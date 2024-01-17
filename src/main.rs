use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const FRAME_RATE: u64 = 240;

fn main() {
    // Create a window with blue background
    let mut window = Window::new(
        "3D Wireframe Cube",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: minifb::Scale::X1,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.set_background_color(0xFF, 0xFF, 0xFF);

    // Cube vertices, edges, and initial positions
    let vertices = [
        [-1.0, -1.0, -1.0],
        [1.0, -1.0, -1.0],
        [1.0, 1.0, -1.0],
        [-1.0, 1.0, -1.0],
        [-1.0, -1.0, 1.0],
        [1.0, -1.0, 1.0],
        [1.0, 1.0, 1.0],
        [-1.0, 1.0, 1.0],
    ];

    // Cube edges
    let edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];

    // Cube position
    let mut cube_x = 0.0;
    let mut cube_y = 0.0;
    let mut cube_z = 0.0;

    // Square vertices and edges
    let square_vertices = [
        [-2.0, -4.0, -2.0],
        [2.0, -4.0, -2.0],
        [2.0, -4.0, 2.0],
        [-2.0, -4.0, 2.0],
    ];

    let square_edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
    ];

    // Camera rotation angle
    let mut angle = 0.0;

    // Camera position
    let mut camera_x = 0.0;
    let mut camera_y = 0.0;

    // Cube tilt
    let mut cube_tilt = 0.0;

    // Cube roll
    let mut cube_roll = 0.0;

    let frame_duration = Duration::from_secs_f64(1.0 / FRAME_RATE as f64);
    let mut last_frame_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed = last_frame_time.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
        last_frame_time = Instant::now();

        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        // Rotate the camera
        //angle += 0.001;

        // Handle camera movement
        if window.is_key_down(Key::Up) {
            camera_y += 0.1;
        }
        if window.is_key_down(Key::Down) {
            camera_y -= 0.1;
        }
        if window.is_key_down(Key::Left) {
            camera_x -= 0.1;
        }
        if window.is_key_down(Key::Right) {
            camera_x += 0.1;
        }

        // Handle cube movement
        if window.is_key_down(Key::W) {
            cube_z += 0.1;
        }
        if window.is_key_down(Key::S) {
            cube_z -= 0.1;
        }
        if window.is_key_down(Key::A) {
            cube_x -= 0.1;
        }
        if window.is_key_down(Key::D) {
            cube_x += 0.1;
        }

        // Handle cube tilt
        if window.is_key_down(Key::I) {
            cube_tilt += 0.3;
        }
        if window.is_key_down(Key::K) {
            cube_tilt -= 0.3;
        }

        // Handle cube roll
        if window.is_key_down(Key::J) {
            cube_roll -= 0.3;
        }
        if window.is_key_down(Key::L) {
            cube_roll += 0.3;
        }

        // Project and draw the cube edges
        for &(i, j) in &edges {
            let p1 = project(vertices[i], angle, camera_x, camera_y, cube_x, cube_y, cube_z, cube_tilt, cube_roll);
            let p2 = project(vertices[j], angle, camera_x, camera_y, cube_x, cube_y, cube_z, cube_tilt, cube_roll);
            draw_line(&mut buffer, p1, p2, WIDTH);
        }

        // Project and draw the square edges
        for &(i, j) in &square_edges {
            let p1 = project(square_vertices[i], angle, camera_x, camera_y, 0.0, 0.0, -3.0, 0.0, 0.0);
            let p2 = project(square_vertices[j], angle, camera_x, camera_y, 0.0, 0.0, -3.0, 0.0, 0.0);
            draw_line(&mut buffer, p1, p2, WIDTH);
        }

        // Set one side of the cube to blue
        for &(i, j) in &edges {
            if i == 0 && j == 1 {
                let p1 = project(vertices[i], angle, camera_x, camera_y, cube_x, cube_y, cube_z, cube_tilt, cube_roll);
                let p2 = project(vertices[j], angle, camera_x, camera_y, cube_x, cube_y, cube_z, cube_tilt, cube_roll);
                draw_line_with_color(&mut buffer, p1, p2, WIDTH, 0x0000FF);
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

// Project a 3D point onto a 2D plane
fn project(
    point: [f32; 3],
    angle: f32,
    camera_x: f32,
    camera_y: f32,
    cube_x: f32,
    cube_y: f32,
    cube_z: f32,
    cube_tilt: f32,
    cube_roll: f32,
) -> (usize, usize) {
    let x = point[0] + cube_x;
    let y = point[1] + cube_y;
    let z = point[2] + cube_z;

    let sin_a = angle.sin();
    let cos_a = angle.cos();

    let x2 = x * cos_a - z * sin_a;
    let y2 = y + camera_y;
    let z2 = x * sin_a + z * cos_a;

    let sin_t = cube_tilt.sin();
    let cos_t = cube_tilt.cos();

    let x3 = x2 * cos_t - y2 * sin_t;
    let y3 = x2 * sin_t + y2 * cos_t;

    let sin_r = cube_roll.sin();
    let cos_r = cube_roll.cos();

    let x4 = x3 * cos_r - z2 * sin_r;
    let y4 = y3;
    let z4 = x3 * sin_r + z2 * cos_r;

    let scale = 2.0 / (z4 + 3.0);
    let x5 = x4 * scale + camera_x;
    let y5 = y4 * scale;

    let screen_x = (WIDTH as f32 / 2.0 + x5 * WIDTH as f32 / 4.0) as usize;
    let screen_y = (HEIGHT as f32 / 2.0 - y5 * HEIGHT as f32 / 4.0) as usize;

    (screen_x, screen_y)
}

fn draw_line(buffer: &mut Vec<u32>, p1: (usize, usize), p2: (usize, usize), width: usize) {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let dx = (x2 as isize - x1 as isize).abs();
    let dy = (y2 as isize - y1 as isize).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1 as isize;
    let mut y = y1 as isize;

    while x != x2 as isize || y != y2 as isize {
        if x >= 0 && x < width as isize && y >= 0 && y < HEIGHT as isize {
            buffer[(y as usize) * width + (x as usize)] = 0xFFFFFF;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

fn draw_line_with_color(buffer: &mut Vec<u32>, p1: (usize, usize), p2: (usize, usize), width: usize, color: u32) {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let dx = (x2 as isize - x1 as isize).abs();
    let dy = (y2 as isize - y1 as isize).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1 as isize;
    let mut y = y1 as isize;

    while x != x2 as isize || y != y2 as isize {
        if x >= 0 && x < width as isize && y >= 0 && y < HEIGHT as isize {
            buffer[(y as usize) * width + (x as usize)] = color;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
