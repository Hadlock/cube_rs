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
    let mut vertices = [
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
    let mut angle = 180.0;

    // Camera position
    let mut camera_x = 0.0;
    let mut camera_y = 0.0;

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
        if window.is_key_down(Key::Comma) {
            angle -= 0.01; // Turn camera left
        }
        if window.is_key_down(Key::Period) {
            angle += 0.01; // Turn camera right
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
        if window.is_key_down(Key::Q) {
            cube_y -= 0.1; // Move cube down
        }
        if window.is_key_down(Key::E) {
            cube_y += 0.1; // Move cube up
        }
        if window.is_key_down(Key::J) {
            // Rotate cube around the center of its z-axis
            let center_x = (vertices[0][0] + vertices[6][0]) / 2.0;
            let center_y = (vertices[0][1] + vertices[6][1]) / 2.0;
            let center_z = (vertices[0][2] + vertices[6][2]) / 2.0;
            let rotated_vertices = rotate_around_z(vertices, center_x, center_y, center_z, 0.01);
            for i in 0..vertices.len() {
                vertices[i] = rotated_vertices[i];
            }
        }
        if window.is_key_down(Key::L) {
            // Rotate cube around the center of its z-axis
            let center_x = (vertices[0][0] + vertices[6][0]) / 2.0;
            let center_y = (vertices[0][1] + vertices[6][1]) / 2.0;
            let center_z = (vertices[0][2] + vertices[6][2]) / 2.0;
            let rotated_vertices = rotate_around_z(vertices, center_x, center_y, center_z, -0.01);
            for i in 0..vertices.len() {
                vertices[i] = rotated_vertices[i];
            }
        }
        if window.is_key_down(Key::I) {
            // Rotate cube around the center of its x-axis
            let center_x = (vertices[0][0] + vertices[6][0]) / 2.0;
            let center_y = (vertices[0][1] + vertices[6][1]) / 2.0;
            let center_z = (vertices[0][2] + vertices[6][2]) / 2.0;
            let rotated_vertices = rotate_around_x(vertices, center_x, center_y, center_z, 0.01);
            for i in 0..vertices.len() {
                vertices[i] = rotated_vertices[i];
            }
        }
        if window.is_key_down(Key::K) {
            // Rotate cube around the center of its x-axis
            let center_x = (vertices[0][0] + vertices[6][0]) / 2.0;
            let center_y = (vertices[0][1] + vertices[6][1]) / 2.0;
            let center_z = (vertices[0][2] + vertices[6][2]) / 2.0;
            let rotated_vertices = rotate_around_x(vertices, center_x, center_y, center_z, -0.01);
            for i in 0..vertices.len() {
                vertices[i] = rotated_vertices[i];
            }
        }
        if window.is_key_down(Key::U) {
            // Rotate cube around the center of its y-axis
            let center_x = (vertices[0][0] + vertices[6][0]) / 2.0;
            let center_y = (vertices[0][1] + vertices[6][1]) / 2.0;
            let center_z = (vertices[0][2] + vertices[6][2]) / 2.0;
            let rotated_vertices = rotate_around_y(vertices, center_x, center_y, center_z, 0.01);
            for i in 0..vertices.len() {
                vertices[i] = rotated_vertices[i];
            }
        }
        if window.is_key_down(Key::O) {
            // Rotate cube around the center of its y-axis
            let center_x = (vertices[0][0] + vertices[6][0]) / 2.0;
            let center_y = (vertices[0][1] + vertices[6][1]) / 2.0;
            let center_z = (vertices[0][2] + vertices[6][2]) / 2.0;
            let rotated_vertices = rotate_around_y(vertices, center_x, center_y, center_z, -0.01);
            for i in 0..vertices.len() {
                vertices[i] = rotated_vertices[i];
            }
        }

        // Project and draw the cube edges
        fn is_front_edge(i: usize, j: usize) -> bool {
            // Define the indices of the front edges
            let front_edges = vec![
                (0, 1), (1, 2), (2, 3), (3, 0), // Front face
                //(4, 5), (5, 6), (6, 7), (7, 4), // Back face
                //(0, 4), (1, 5), (2, 6), (3, 7), // Connecting edges
            ];

            // Check if the given indices represent a front edge
            front_edges.contains(&(i, j)) || front_edges.contains(&(j, i))
        }
        fn is_rear_edge(i: usize, j: usize) -> bool {
            // Define the indices of the rear edges
            let rear_edges = vec![
                (4, 5), (5, 6), (6, 7), (7, 4), // Rear face
                //(0, 1), (1, 2), (2, 3), (3, 0), // Front face
                //(0, 4), (1, 5), (2, 6), (3, 7), // Connecting edges
            ];

            // Check if the given indices represent a rear edge
            rear_edges.contains(&(i, j)) || rear_edges.contains(&(j, i))
        }
        fn is_bottom_edge(i: usize, j: usize) -> bool {
            // Define the indices of the bottom edges
            let bottom_edges = vec![
                (4, 5), (5, 6), (6, 7), (7, 4), // Bottom face
                //(0, 4), (1, 5), (2, 6), (3, 7), // Connecting edges
            ];

            // Check if the given indices represent a bottom edge
            bottom_edges.contains(&(i, j)) || bottom_edges.contains(&(j, i))
        }

        for &(i, j) in &edges {
            let p1 = project(vertices[i], angle, camera_x, camera_y, cube_x, cube_y, cube_z);
            let p2 = project(vertices[j], angle, camera_x, camera_y, cube_x, cube_y, cube_z);
            if is_front_edge(i, j) {
                draw_line_with_color(&mut buffer, p1, p2, WIDTH, 0x0000FF); // Set front cube edges to blue
            } else {
                draw_line(&mut buffer, p1, p2, WIDTH);
            }
        }

        // Project and draw the square edges
        for &(i, j) in &square_edges {
            let p1 = project(square_vertices[i], angle, camera_x, camera_y, 0.0, 0.0, -3.0);
            let p2 = project(square_vertices[j], angle, camera_x, camera_y, 0.0, 0.0, -3.0);
            draw_line_with_color(&mut buffer, p1, p2, WIDTH, 0x00FF00); // Set square color to green
        }

        

        // Draw the x-axis line (red)
        let p1 = project([-1.0, 0.0, 0.0], angle, camera_x, camera_y, 0.0, 0.0, 0.0);
        let p2 = project([1.0, 0.0, 0.0], angle, camera_x, camera_y, 0.0, 0.0, 0.0);
        draw_line_with_color(&mut buffer, p1, p2, WIDTH, 0xFF0000);

        // Draw the y-axis line (dark green)
        let p1 = project([0.0, -1.0, 0.0], angle, camera_x, camera_y, 0.0, 0.0, 0.0);
        let p2 = project([0.0, 1.0, 0.0], angle, camera_x, camera_y, 0.0, 0.0, 0.0);
        draw_line_with_color(&mut buffer, p1, p2, WIDTH, 0x006400);

        // Draw the z-axis line (cyan blue)
        let p1 = project([0.0, 0.0, -1.0], angle, camera_x, camera_y, 0.0, 0.0, 0.0);
        let p2 = project([0.0, 0.0, 1.0], angle, camera_x, camera_y, 0.0, 0.0, 0.0);
        draw_line_with_color(&mut buffer, p1, p2, WIDTH, 0x00FFFF);

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
) -> (usize, usize) {
    let x = point[0] + cube_x;
    let y = point[1] + cube_y;
    let z = point[2] + cube_z;

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

fn rotate_around_z(vertices: [[f32; 3]; 8], center_x: f32, center_y: f32, center_z: f32, angle: f32) -> [[f32; 3]; 8] {
    let sin_a = angle.sin();
    let cos_a = angle.cos();

    let mut rotated_vertices = [[0.0; 3]; 8];
    for i in 0..vertices.len() {
        let x = vertices[i][0] - center_x;
        let y = vertices[i][1] - center_y;
        let z = vertices[i][2] - center_z;

        rotated_vertices[i][0] = x * cos_a - y * sin_a + center_x;
        rotated_vertices[i][1] = x * sin_a + y * cos_a + center_y;
        rotated_vertices[i][2] = z + center_z;
    }

    rotated_vertices
}

fn rotate_around_x(vertices: [[f32; 3]; 8], center_x: f32, center_y: f32, center_z: f32, angle: f32) -> [[f32; 3]; 8] {
    let sin_a = angle.sin();
    let cos_a = angle.cos();

    let mut rotated_vertices = [[0.0; 3]; 8];
    for i in 0..vertices.len() {
        let x = vertices[i][0] - center_x;
        let y = vertices[i][1] - center_y;
        let z = vertices[i][2] - center_z;

        rotated_vertices[i][0] = x + center_x;
        rotated_vertices[i][1] = y * cos_a - z * sin_a + center_y;
        rotated_vertices[i][2] = y * sin_a + z * cos_a + center_z;
    }

    rotated_vertices
}

fn rotate_around_y(vertices: [[f32; 3]; 8], center_x: f32, center_y: f32, center_z: f32, angle: f32) -> [[f32; 3]; 8] {
    let sin_a = angle.sin();
    let cos_a = angle.cos();

    let mut rotated_vertices = [[0.0; 3]; 8];
    for i in 0..vertices.len() {
        let x = vertices[i][0] - center_x;
        let y = vertices[i][1] - center_y;
        let z = vertices[i][2] - center_z;

        rotated_vertices[i][0] = x * cos_a + z * sin_a + center_x;
        rotated_vertices[i][1] = y + center_y;
        rotated_vertices[i][2] = -x * sin_a + z * cos_a + center_z;
    }

    rotated_vertices
}