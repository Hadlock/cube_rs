use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "3D Cube",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut angle = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        angle += 5.01;

        // Clear the buffer
        for pixel in buffer.iter_mut() {
            *pixel = 0x0000FF; // Blue background
        }

        // Draw the wireframe cube
        draw_cube(&mut buffer, angle);

        // Update the window
        window.update_with_buffer(&buffer, 800, 600).unwrap();
    }
}

fn draw_cube(buffer: &mut [u32], angle: f32) {
    // Define the cube vertices
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

    // Define the cube edges
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

    // Calculate the rotation matrix
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();
    let rotation_matrix = [
        [cos_angle, -sin_angle, 0.0],
        [sin_angle, cos_angle, 0.0],
        [0.0, 0.0, 1.0],
    ];

    // Project and draw the cube edges
    for (v1, v2) in edges.iter() {
        let p1 = project(vertices[*v1], rotation_matrix);
        let p2 = project(vertices[*v2], rotation_matrix);

        draw_line(buffer, p1, p2);
    }
}

fn project(vertex: [f32; 3], rotation_matrix: [[f32; 3]; 3]) -> (usize, usize) {
    let x = vertex[0] * rotation_matrix[0][0] + vertex[1] * rotation_matrix[1][0];
    let y = vertex[0] * rotation_matrix[0][1] + vertex[1] * rotation_matrix[1][1];
    let z = vertex[0] * rotation_matrix[0][2] + vertex[1] * rotation_matrix[1][2] + vertex[2];

    let scale = WIDTH as f32 / 4.0;
    let offset_x = WIDTH / 2;
    let offset_y = HEIGHT / 2;

    (
        (x * scale + offset_x as f32) as usize,
        (y * scale + offset_y as f32) as usize,
    )
}

fn draw_line(buffer: &mut [u32], p1: (usize, usize), p2: (usize, usize)) {
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
        if x >= 0 && x < WIDTH as isize && y >= 0 && y < HEIGHT as isize {
            buffer[(y as usize) * WIDTH + (x as usize)] = 0xFFFFFF; // White line
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
