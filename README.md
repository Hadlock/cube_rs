# cube.rs

![cube_rs logo](https://github.com/hadlock/cube_rs/blob/master/static/cube_rs.png)

this should draw a 2d representation of a wireframe cube hovering over a green ~~field~~ square which can be moved

i wrote this entirely with github copilot to see how far I could push it. no code was written by a human I just kept selecting the entire codebase and asking it to add specific functionality

cube:

- w/s forward/back
- a/d left/right
- q/e up/down

cube translation:

- i/k pitch
- u/o yaw
- j/l roll

camera:

- arrow keys up/down/left/right
- ,/. pan left/right

I suggest using ,/. (< / > keys ) over the left/right keys.

## suggestions

1. when starting, press the `,` key to pan so you are directly in front of the cube (blue side facing away)
1. push `w` key to move the cube slightly away from the camera
1. use jkli/u/o keys to translate cube

## building

1. add `minifb = "0.25.0"` to cargo.toml
1. run `cargo run`

## notes

1. runs at 240fps, mostly in an attempt to fix flicker
1. esc key to exit
