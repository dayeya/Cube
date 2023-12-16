// Cube rotation code in rust.

pub mod vec; 
use crate::vec::{Vector, Rotation};

use std::thread;
use std::time::Duration;

// cube parameters.
const SPEED: f32 = 0.05;
const BG_CHAR: char = ' '; 
const CUBE_LEN: i32 = 15;
const CUBE_DISTANCE: f32 = 60.; // K2
const SCREEN_DISTANCE: f32 = 40.; // K1

// angles of rotation on each axis.
const X: f32 = 0.;
const Y: f32 = 0.;
const Z: f32 = 0.;

// terminal and rendering.
const WIDTH: usize = 160;
const HEIGHT: usize = 44;
const FRAME_DELAY: u64 = 50;

fn parse_surface(
    mut origin_vector: Vector,
    angles: [f32; 3],
    ch: char, 
    z_buffer: &mut Vec<Vec<i32>>,
    output_buffer: &mut Vec<Vec<char>>
) 
{
    let x_theta = angles[0];
    let y_theta = angles[1];
    let z_theta = angles[2];

    // rotate the vector.
    origin_vector.rotate_all(x_theta, y_theta, z_theta);  

    let w_offset = WIDTH as f32 / 2.;
    let h_offset = HEIGHT as f32 / 2.;

    // Calc one over Z.
    let ooz: f32 = 1.0 / (CUBE_DISTANCE + origin_vector.z);

    // xp is multiplied by 2. since the width of any char is smaller than its height.
    let xp: usize = (w_offset + SCREEN_DISTANCE * ooz * origin_vector.x * 2.) as usize; 
    let yp: usize = (h_offset + SCREEN_DISTANCE * ooz * origin_vector.y) as usize;

    if xp >= WIDTH || yp >= HEIGHT { 
        return 
    };

    if ooz > z_buffer[yp][xp] as f32 { 
        // Update the Z-buffer and plot the point.
        z_buffer[yp][xp] = ooz as i32;
        output_buffer[yp][xp] = ch;
    }
}

fn render_cube() {
    let mut output_buffer: Vec<Vec<char>> = vec![vec![BG_CHAR; WIDTH]; HEIGHT]; // Output on the screen.
    let mut depth_checker: Vec<Vec<i32>> = vec![vec![0; WIDTH]; HEIGHT]; // Z buffer.
    let mut rotation_angles: [f32; 3] = [X, Y, Z];

    // Origin Vectors.
    let mut v1: Vector;
    let mut v2: Vector;
    let mut v3: Vector;
    let mut v4: Vector;
    let mut v5: Vector;
    let mut v6: Vector;

    loop {
        // Parse all 6 sides of the cube into the buffer.
        for cube_x in -CUBE_LEN..CUBE_LEN {
            for cube_y in -CUBE_LEN..CUBE_LEN {

                v1 = Vector {x:    cube_x as f32, y:    cube_y as f32, z: -CUBE_LEN as f32 };
                v2 = Vector {x:  CUBE_LEN as f32, y:    cube_y as f32, z:    cube_x as f32 };
                v3 = Vector {x: -CUBE_LEN as f32, y:    cube_y as f32, z:   -cube_x as f32 };
                v4 = Vector {x:   -cube_x as f32, y:    cube_y as f32, z:  CUBE_LEN as f32 };
                v5 = Vector {x:    cube_x as f32, y: -CUBE_LEN as f32, z:   -cube_y as f32 };
                v6 = Vector {x:    cube_x as f32, y:  CUBE_LEN as f32, z:    cube_y as f32 };

                parse_surface(v1, rotation_angles, '$', &mut depth_checker, &mut output_buffer);
                parse_surface(v2, rotation_angles, '!', &mut depth_checker, &mut output_buffer);
                parse_surface(v3, rotation_angles, '~', &mut depth_checker, &mut output_buffer);
                parse_surface(v4, rotation_angles, '+', &mut depth_checker, &mut output_buffer);
                parse_surface(v5, rotation_angles, '@', &mut depth_checker, &mut output_buffer);
                parse_surface(v6, rotation_angles, '.', &mut depth_checker, &mut output_buffer);
            }
        }

        // Plot the buffer.
        for row in &output_buffer {
            for px in row { 
                print!("{}", px);
            }
            println!();
        }
        println!("\x1b[H"); // Return to the HOME offset in the terminal.

        // Inc the angles.
        rotation_angles[0] += SPEED;
        rotation_angles[1] += SPEED;
        rotation_angles[2] += SPEED;

        // Renew buffers. 
        output_buffer = vec![vec![BG_CHAR; WIDTH]; HEIGHT]; 
        depth_checker = vec![vec![0; WIDTH]; HEIGHT];
        thread::sleep(Duration::from_millis(FRAME_DELAY));
    }

}

fn main() {
    // Start animation.
    println!("\x1b[2J\r\x1b[H");
    render_cube();
}
