use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

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

    // Cube vertices
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

    // Camera rotation angle
    let mut angle = 0.0;

    // Camera position
    let mut camera_x = 0.0;
    let mut camera_y = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the window
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        window.update_with_buffer(&buffer, 800, 600).unwrap();

        // Rotate the camera
        angle += 0.01;

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

        // Project and draw the cube edges
        for &(i, j) in &edges {
            let p1 = project(vertices[i], angle, camera_x, camera_y);
            let p2 = project(vertices[j], angle, camera_x, camera_y);
            draw_line(&mut buffer, p1, p2, WIDTH);
        }

        // Update the window with the buffer
        window.update_with_buffer(&buffer, 800, 600).unwrap();
    }
}

// Project a 3D point onto a 2D plane
fn project(point: [f32; 3], angle: f32, camera_x: f32, camera_y: f32) -> (usize, usize) {
    let x = point[0];
    let y = point[1];
    let z = point[2];

    let sin_a = angle.sin();
    let cos_a = angle.cos();

    let x2 = x * cos_a - z * sin_a;
    let y2 = y + camera_y;
    let z2 = x * sin_a + z * cos_a;

    let scale = 2.0 / (z2 + 3.0);
    let x3 = x2 * scale + camera_x;
    let y3 = y2 * scale;

    let screen_x = (WIDTH as f32 / 2.0 + x3 * WIDTH as f32 / 4.0) as usize;
    let screen_y = (HEIGHT as f32 / 2.0 - y3 * HEIGHT as f32 / 4.0) as usize;

    (screen_x, screen_y)
}

// Draw a line between two points using Bresenham's line algorithm
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
