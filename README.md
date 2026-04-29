# w0rld

[frei0r](https://dyne.org/software/frei0r/) plugins that render video into a 3D scene using Bevy.

The scene should have a camera node named `Camera` - this is usually animated.
The filter should have a material named `Video1`. The mixer2 should have two materials named `Video1` and `Video2`.
The mixer3 should have three materials named `Video1`, `Video2` and `Video3`.
These materials should be applied to a UV mapped mesh.

Build:
```
cargo build --workspace; cargo xtask package
```
