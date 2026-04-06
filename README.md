![](https://github.com/senselogic/FACET/blob/master/LOGO/facet.png)

# Facet

Minimal 3D viewer for OBJ models and XYZ point clouds.

## Features

- Loads `.obj` models and `.xyz` point clouds (`X Y Z R G B` per line) by drag and drop or through the command line.
- Reframes the camera at loading based on the geometry bounding box.
- Allows to orbit the camera with the mouse : left drag to rotate, right drag to pan, mouse wheel to zoom.

## Usage

```bash
facet
facet model.obj
facet cloud.xyz
```

## Limitations

- Can't handle large point clouds.

## Version

0.0.1

## Author

Eric Pelzer (ecstatic.coder@gmail.com).

## License

This project is licensed under the GNU General Public License version 3.

See the [LICENSE](LICENSE.md) file for details.
