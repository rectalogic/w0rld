# w0rld

[frei0r](https://dyne.org/software/frei0r/) plugins that render video into a 3D scene using Bevy.

The scene should have a camera named `Camera` - this is usually animated.
The filter should have a UV mapped mesh named `Video1`. The mixer2 should have two meshes named `Video1` and `Video2`.
The mixer3 should have three meshes named `Video1`, `Video2` and `Video3`.

Build:
```
cargo build --workspace; cargo xtask package
```
