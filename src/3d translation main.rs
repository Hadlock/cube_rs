use minifb::{Key, Window, WindowOptions};
use vector3::Vector3;
use std::default::Default;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    // Create a window with the specified dimensions
    let mut window = Window::new(
        "3D Wireframe Cube",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Create a buffer to store the pixel data
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Define the cube's vertices
    let vertices = [
        [-0.5, -0.5, -0.5], // Front bottom left
        [0.5, -0.5, -0.5],  // Front bottom right
        [0.5, 0.5, -0.5],   // Front top right
        [-0.5, 0.5, -0.5],  // Front top left
        [-0.5, -0.5, 0.5],  // Back bottom left
        [0.5, -0.5, 0.5],   // Back bottom right
        [0.5, 0.5, 0.5],    // Back top right
        [-0.5, 0.5, 0.5],   // Back top left
    ];

    // Define the cube's edges
    let edges = [
        (0, 1), (1, 2), (2, 3), (3, 0), // Front face
        (4, 5), (5, 6), (6, 7), (7, 4), // Back face
        (0, 4), (1, 5), (2, 6), (3, 7), // Connecting lines
    ];

    // Define the rotation angles
    let mut angle_x: f64 = 0.0;
    let mut angle_y: f64 = 0.0;

    // Start the main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer with a blue background
        buffer.iter_mut().for_each(|pixel| *pixel = 0x0000FF);

        // Calculate the rotation matrix
        let rotation_x = [
            [1.0, 0.0, 0.0],
            [0.0, angle_x.cos(), -angle_x.sin()],
            [0.0, angle_x.sin(), angle_x.cos()],
        ];

        /*
        impl Default for Vector3 {
            fn default() -> Self {
                Vector3 { x: 0.0, y: 0.0, z: 0.0 }
            }
        }

        let mut projected_vertices: [Vector3; 8] = [Default::default(); 8];
        */

        let mut projected_vertices: [Vector3; 8] = [
            Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        ];

        // Draw the edges of the cube
        for (i, j) in edges.iter() {
            let start = projected_vertices[*i];
            let end = projected_vertices[*j];

            // Convert the 3D coordinates to 2D screen coordinates
            let start_x = ((start.x + 1.0) * (WIDTH as f64 / 2.0)) as usize;
            let start_y = ((start.y + 1.0) * (HEIGHT as f64 / 2.0)) as usize;
            let end_x = ((end.x + 1.0) * (WIDTH as f64 / 2.0)) as usize;
            let end_y = ((end.y + 1.0) * (HEIGHT as f64 / 2.0)) as usize;

            // Draw a white line between the two points
            draw_line(&mut buffer, start_x, start_y, end_x, end_y, 0xFFFFFF);
        }

        // Update the window with the new buffer
        window.update_with_buffer(&buffer, 800, 600).unwrap();

        // Increment the rotation angles
        angle_x += 0.01;
        angle_y += 0.02;
    }
}

// Draw a line using Bresenham's line algorithm
fn draw_line(buffer: &mut [u32], x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
    let dx = (x1 as isize - x0 as isize).abs();
    let dy = (y1 as isize - y0 as isize).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x0 as isize;
    let mut y = y0 as isize;

    while x != x1 as isize || y != y1 as isize {
        if x >= 0 && x < WIDTH as isize && y >= 0 && y < HEIGHT as isize {
            buffer[(y as usize * WIDTH) + x as usize] = color;
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
