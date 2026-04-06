// -- IMPORTS

mod load;

use std::path::PathBuf;

use bevy::prelude::*;
use bevy::window::FileDragAndDrop;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use clap::Parser;
use load::{frame_pan_orbit, load_obj, load_xyz, BoundingBox3};

// -- TYPES

#[derive( Parser, Debug )]
#[command( name = "facet", version, about = "Minimal 3D viewer for OBJ and XYZ point clouds" )]
struct Cli
{
    // -- ATTRIBUTES

    /// Path to a `.obj` or `.xyz` file
    file: Option<PathBuf>,
}

// ~~

#[derive( Resource )]
struct InitialPath
(
    Option<PathBuf>
);

// ~~

#[derive( Message )]
struct LoadFacetPath
(
    PathBuf
);

// ~~

#[derive( Component )]
struct FacetContent;

// -- FUNCTIONS

fn main(
    )
{
    let command_line = Cli::parse();

    if let Some( ref file_path ) = command_line.file
    {
        let file_extension
            = file_path
                  .extension()
                  .and_then(
                      | extension_string | extension_string.to_str()
                      )
                  .map(
                      | extension_string | extension_string.to_ascii_lowercase()
                      );

        if file_extension.as_deref() != Some( "obj" )
           && file_extension.as_deref() != Some( "xyz" )
        {
            eprintln!( "Unsupported format" );
            std::process::exit( 1 );
        }

        if !file_path.exists()
        {
            eprintln!( "File not found." );
            std::process::exit( 1 );
        }
    }

    App::new()
        .add_plugins(
            (
                DefaultPlugins.set(
                    WindowPlugin
                    {
                        primary_window:
                            Some(
                                Window
                                {
                                    title: "Facet".into(),
                                    resolution: bevy::window::WindowResolution::new( 1024, 720 ),
                                    ..default()
                                }
                                ),
                        ..default()
                    }
                    ),
                PanOrbitCameraPlugin,
            )
            )
        .add_message::<LoadFacetPath>()
        .insert_resource( InitialPath( command_line.file ) )
        .add_systems(
            Startup,
            ( setup_scene, startup_enqueue_load ).chain()
            )
        .add_systems(
            Update,
            ( on_file_drop, process_load_requests ).chain()
            )
        .run();
}

// ~~

fn setup_scene(
    mut command_buffer: Commands
    )
{
    command_buffer.insert_resource(
        GlobalAmbientLight
        {
            brightness: 130.0,
            ..default()
        }
        );

    command_buffer.spawn(
        (
            DirectionalLight
            {
                illuminance: 18_000.0,
                ..default()
            },
            Transform::from_xyz( 5.0, 12.0, 6.0 ).looking_at( Vec3::ZERO, Vec3::Y ),
        )
        );

    command_buffer.spawn(
        (
            DirectionalLight
            {
                illuminance: 7_500.0,
                ..default()
            },
            Transform::from_xyz( -7.0, 5.0, -6.0 ).looking_at( Vec3::ZERO, Vec3::Y ),
        )
        );

    command_buffer.spawn(
        (
            Transform::from_translation( Vec3::new( 0.0, 0.0, 2.0 ) ),
            PanOrbitCamera
            {
                button_orbit: MouseButton::Left,
                button_pan: MouseButton::Right,
                orbit_smoothness: 0.0,
                pan_smoothness: 0.0,
                zoom_smoothness: 0.0,
                ..default()
            },
        )
        );
}

// ~~

fn startup_enqueue_load(
    initial_path: Res<InitialPath>,
    mut load_path_writer: MessageWriter<LoadFacetPath>
    )
{
    if let Some( cloned_path ) = initial_path.0.clone()
    {
        load_path_writer.write( LoadFacetPath( cloned_path ) );
    }
}

// ~~

fn on_file_drop(
    mut drag_drop_reader: MessageReader<FileDragAndDrop>,
    mut load_path_writer: MessageWriter<LoadFacetPath>
    )
{
    for event in drag_drop_reader.read()
    {
        let FileDragAndDrop::DroppedFile { path_buf: dropped_file_path, .. } = event.clone() else
        {
            continue;
        };

        if !dropped_file_path.exists()
        {
            continue;
        }

        let file_extension
            = dropped_file_path
                .extension()
                .and_then(
                    | extension_string | extension_string.to_str()
                    )
                .map(
                    | extension_string | extension_string.to_ascii_lowercase()
                    );

        if file_extension.as_deref() != Some( "obj" )
           && file_extension.as_deref() != Some( "xyz" )
        {
            continue;
        }

        load_path_writer.write( LoadFacetPath( dropped_file_path ) );
    }
}

// ~~

fn process_load_requests(
    mut load_path_reader: MessageReader<LoadFacetPath>,
    mut command_buffer: Commands,
    mut mesh_asset: ResMut<Assets<Mesh>>,
    mut standard_material_asset: ResMut<Assets<StandardMaterial>>,
    content_entity_query: Query<Entity, With<FacetContent>>,
    mut orbit_camera_query: Query<&mut PanOrbitCamera, With<Camera3d>>,
    )
{
    for LoadFacetPath( file_path ) in load_path_reader.read()
    {
        let content_entity_vector: Vec<Entity> = content_entity_query.iter().collect();

        for content_entity in content_entity_vector
        {
            command_buffer.entity( content_entity ).despawn();
        }

        let file_extension
            = file_path
                .extension()
                .and_then(
                    | extension_string | extension_string.to_str()
                    )
                .map(
                    | extension_string | extension_string.to_ascii_lowercase()
                    );

        let bounding_box_result: Result<BoundingBox3, String>
            = match file_extension.as_deref()
            {
                Some( "xyz" ) =>
                    match load_xyz( file_path.as_path() )
                    {
                        Ok( ( point_mesh, bounding_box ) ) =>
                        {
                            let mesh_handle = mesh_asset.add( point_mesh );
                            let material_handle
                                = standard_material_asset.add(
                                    StandardMaterial
                                    {
                                        base_color: Color::WHITE,
                                        unlit: true,
                                        ..default()
                                    }
                                    );
                            command_buffer.spawn(
                                (
                                    Mesh3d( mesh_handle ),
                                    MeshMaterial3d( material_handle ),
                                    FacetContent,
                                )
                                );

                            Ok( bounding_box )
                        }
                        Err( load_error ) => Err( load_error ),
                    },
                Some( "obj" ) =>
                    match load_obj( file_path.as_path() )
                    {
                        Ok( ( mesh_and_material_vector, bounding_box ) ) =>
                        {
                            for ( triangle_mesh, standard_material ) in mesh_and_material_vector
                            {
                                let mesh_handle = mesh_asset.add( triangle_mesh );
                                let material_handle
                                    = standard_material_asset.add( standard_material );
                                command_buffer.spawn(
                                    (
                                        Mesh3d( mesh_handle ),
                                        MeshMaterial3d( material_handle ),
                                        FacetContent,
                                    )
                                    );
                            }

                            Ok( bounding_box )
                        }
                        Err( load_error ) => Err( load_error ),
                    },
                _ => continue,
            };

        match bounding_box_result
        {
            Ok( bounding_box ) =>
            {
                if let Ok( mut orbit_camera ) = orbit_camera_query.single_mut()
                {
                    frame_pan_orbit( &bounding_box, &mut orbit_camera );
                }
            }
            Err( load_error ) => eprintln!( "Load error: {load_error}" ),
        }
    }
}
