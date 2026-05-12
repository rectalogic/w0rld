# w0rld

[frei0r](https://dyne.org/software/frei0r/) plugins that render video into a 3D scene using Bevy.

The scene should have a camera node named `Camera` - this is usually animated.
The filter should have an empty named `Video1`. The mixer2 should have two empties named `Video1` and `Video2`.
The mixer3 should have three empties named `Video1`, `Video2` and `Video3`.
These empties will have [ForwardDecal](https://docs.rs/bevy/latest/bevy/pbr/decal/struct.ForwardDecal.html) inserted on them.

Build:
```
cargo build --workspace; cargo xtask package
```

[MLT](https://www.mltframework.org) transition (mixer2):
```
FREI0R_PATH=target/debug melt https://assets.mixkit.co/videos/11007/11007-720.mp4 out=119 https://assets.mixkit.co/videos/1479/1479-720.mp4 out=119 -mix 120 -mixer frei0r.w0rld_mixer2 0=demo/room.glb -consumer avformat:output-melt.mp4
```

https://github.com/user-attachments/assets/ca5d6169-3259-439b-a5dd-7bb04b7f95d9


[ffmpeg](https://ffmpeg.org) filter (ignoring `Video2` in the scene):
```
FREI0R_PATH=target/debug ffmpeg -t 3s -i https://assets.mixkit.co/videos/1479/1479-720.mp4 -vf frei0r=w0rld_filter:demo/room.glb -pix_fmt yuv420p -y output-ffmpeg.mp4
```

https://github.com/user-attachments/assets/f23f3d98-2e86-45a5-9eb4-3db5fc80ce8f
