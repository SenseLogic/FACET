// -- IMPORTS

use std::path::Path;

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

// -- TYPES

#[derive( Clone, Copy, Debug, Default )]
pub struct BoundingBox3
{
    // -- ATTRIBUTES

    pub min: Vec3,
    pub max: Vec3,
}

// ~~

impl BoundingBox3
{
    // -- CONSTRUCTORS

    pub fn new_invalid(
        ) -> Self
    {
        Self
        {
            min: Vec3::splat( f32::MAX ),
            max: Vec3::splat( f32::MIN ),
        }
    }

    // -- INQUIRIES

    pub fn center(
        &self
        ) -> Vec3
    {
        ( self.min + self.max ) * 0.5
    }

    // ~~

    pub fn size(
        &self
        ) -> f32
    {
        let extent = self.max - self.min;

        extent.x.max( extent.y ).max( extent.z )
    }

    // ~~

    pub fn is_empty(
        &self
        ) -> bool
    {
        self.min.x > self.max.x
    }

    // -- OPERATIONS

    pub fn include(
        &mut self,
        point: Vec3
        )
    {
        self.min = self.min.min( point );
        self.max = self.max.max( point );
    }
}

// -- FUNCTIONS

pub fn load_xyz(
    path: &Path
    ) -> Result<( Mesh, BoundingBox3 ), String>
{
    let file_text = std::fs::read_to_string( path ).map_err( | io_error | io_error.to_string() )?;
    let mut bounding_box = BoundingBox3::new_invalid();
    let mut position_vector: Vec<Vec3> = Vec::new();
    let mut color_vector: Vec<[f32; 4]> = Vec::new();

    for line in file_text.lines()
    {
        if line.trim().is_empty()
        {
            continue;
        }

        let mut whitespace_part_iterator = line.split_whitespace();
        let coordinate_x: f32
            = whitespace_part_iterator
                  .next()
                  .ok_or_else( || "xyz: missing x".to_string() )?
                  .parse()
                  .map_err( | parse_error: std::num::ParseFloatError | parse_error.to_string() )?;
        let coordinate_y: f32
            = whitespace_part_iterator
                  .next()
                  .ok_or_else( || "xyz: missing y".to_string() )?
                  .parse()
                  .map_err( | parse_error: std::num::ParseFloatError | parse_error.to_string() )?;
        let coordinate_z: f32
            = whitespace_part_iterator
                  .next()
                  .ok_or_else( || "xyz: missing z".to_string() )?
                  .parse()
                  .map_err( | parse_error: std::num::ParseFloatError | parse_error.to_string() )?;
        let color_red: u8
            = whitespace_part_iterator
                  .next()
                  .ok_or_else( || "xyz: missing r".to_string() )?
                  .parse()
                  .map_err( | parse_error: std::num::ParseIntError | parse_error.to_string() )?;
        let color_green: u8
            = whitespace_part_iterator
                  .next()
                  .ok_or_else( || "xyz: missing g".to_string() )?
                  .parse()
                  .map_err( | parse_error: std::num::ParseIntError | parse_error.to_string() )?;
        let color_blue: u8
            = whitespace_part_iterator
                  .next()
                  .ok_or_else( || "xyz: missing b".to_string() )?
                  .parse()
                  .map_err( | parse_error: std::num::ParseIntError | parse_error.to_string() )?;

        let point = Vec3::new( coordinate_x, coordinate_y, coordinate_z );
        bounding_box.include( point );
        position_vector.push( point );
        color_vector.push(
            [
                color_red as f32 / 255.0,
                color_green as f32 / 255.0,
                color_blue as f32 / 255.0,
                1.0,
            ]
            );
    }

    let mut point_mesh
        = Mesh::new( PrimitiveTopology::PointList, RenderAssetUsages::default() );
    point_mesh.insert_attribute( Mesh::ATTRIBUTE_POSITION, position_vector );
    point_mesh.insert_attribute( Mesh::ATTRIBUTE_COLOR, color_vector );

    Ok( ( point_mesh, bounding_box ) )
}

// ~~

fn tobj_material_to_standard(
    tobj_material_slice: &[tobj::Material],
    optional_material_index: Option<usize>
    ) -> StandardMaterial
{
    let mut standard_material
        = StandardMaterial
        {
            perceptual_roughness: 0.5,
            metallic: 0.0,
            ..default()
        };

    if let Some( material_index ) = optional_material_index
    {
        if let Some( tobj_material ) = tobj_material_slice.get( material_index )
        {
            if let Some( diffuse_linear_rgb ) = tobj_material.diffuse
            {
                standard_material.base_color
                    = Color::srgb( diffuse_linear_rgb[ 0 ], diffuse_linear_rgb[ 1 ], diffuse_linear_rgb[ 2 ] );
            }
        }
    }

    standard_material
}

// ~~

pub fn load_obj(
    path: &Path
    ) -> Result<( Vec<( Mesh, StandardMaterial )>, BoundingBox3 ), String>
{
    let ( model_vector, material_load_result )
        = tobj::load_obj( path, &tobj::GPU_LOAD_OPTIONS ).map_err( | load_error | load_error.to_string() )?;
    let tobj_material_vector
        = material_load_result.unwrap_or_else( | _material_load_error | Vec::new() );

    let mut bounding_box = BoundingBox3::new_invalid();
    let mut mesh_and_material_vector: Vec<( Mesh, StandardMaterial )> = Vec::new();

    for tobj_model in model_vector
    {
        let tobj_mesh = &tobj_model.mesh;

        if tobj_mesh.indices.is_empty()
        {
            continue;
        }

        let vertex_count = tobj_mesh.positions.len() / 3;

        for vertex_index in 0..vertex_count
        {
            let position_float_index = vertex_index * 3;
            bounding_box.include(
                Vec3::new(
                    tobj_mesh.positions[ position_float_index ],
                    tobj_mesh.positions[ position_float_index + 1 ],
                    tobj_mesh.positions[ position_float_index + 2 ],
                )
                );
        }

        let position_triplet_vector: Vec<[f32; 3]>
            = tobj_mesh
                  .positions
                  .chunks_exact( 3 )
                  .map(
                      | coordinate_chunk | [ coordinate_chunk[ 0 ], coordinate_chunk[ 1 ], coordinate_chunk[ 2 ] ]
                      )
                  .collect();

        let mut triangle_mesh
            = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::default(),
                );
        triangle_mesh.insert_attribute( Mesh::ATTRIBUTE_POSITION, position_triplet_vector );

        let normal_length_matches_position = tobj_mesh.normals.len() == tobj_mesh.positions.len();

        if normal_length_matches_position
        {
            let normal_triplet_vector: Vec<[f32; 3]>
                = tobj_mesh
                      .normals
                      .chunks_exact( 3 )
                      .map(
                          | coordinate_chunk | [ coordinate_chunk[ 0 ], coordinate_chunk[ 1 ], coordinate_chunk[ 2 ] ]
                          )
                      .collect();
            triangle_mesh.insert_attribute( Mesh::ATTRIBUTE_NORMAL, normal_triplet_vector );
        }

        let expected_vertex_count = tobj_mesh.positions.len() / 3;
        let texture_coordinate_float_count = expected_vertex_count * 2;

        if tobj_mesh.texcoords.len() == texture_coordinate_float_count
        {
            let texture_coordinate_pair_vector: Vec<[f32; 2]>
                = tobj_mesh
                      .texcoords
                      .chunks_exact( 2 )
                      .map(
                          | coordinate_chunk | [ coordinate_chunk[ 0 ], coordinate_chunk[ 1 ] ]
                          )
                      .collect();
            triangle_mesh.insert_attribute( Mesh::ATTRIBUTE_UV_0, texture_coordinate_pair_vector );
        }

        triangle_mesh.insert_indices( Indices::U32( tobj_mesh.indices.clone() ) );

        if !normal_length_matches_position
        {
            triangle_mesh.compute_normals();
        }

        let standard_material
            = tobj_material_to_standard( &tobj_material_vector, tobj_mesh.material_id );
        mesh_and_material_vector.push( ( triangle_mesh, standard_material ) );
    }

    Ok( ( mesh_and_material_vector, bounding_box ) )
}

// ~~

pub fn frame_pan_orbit(
    bounding_box: &BoundingBox3,
    orbit_camera: &mut PanOrbitCamera
    )
{
    let ( center, size )
        = if bounding_box.is_empty()
        {
            ( Vec3::ZERO, 1.0 )
        }

        else
        {
            let raw_size = bounding_box.size();
            let safe_size
                = if raw_size <= 0.0
                     || !raw_size.is_finite()
                  {
                      1.0
                  }
                  else
                  {
                      raw_size
                  };

            ( bounding_box.center(), safe_size )
        };

    orbit_camera.target_focus = center;
    orbit_camera.target_radius = size * 2.0;
    orbit_camera.force_update = true;
}
