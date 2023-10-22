// Cube rotation code 

pub mod vec; 
use crate::vec::{Vector, Rotation};

use std::time::Duration;

// costumizable CONSTS.
const SPEED: f32 = 0.05;
const BG_CHAR: char = ' '; 
const CUBE_LEN: i32 = 15;

// points calc CONSTS.
const CUBE_DISTANCE: f32 = 60.; // K2
const SCREEN_DISTANCE: f32 = 40.; // K1

const X: f32 = 0.;
const Y: f32 = 0.;
const Z: f32 = 0.;

const WIDTH: usize = 160;
const HEIGHT: usize = 44;
const FRAME_DELAY: u64 = 10;


fn parse_surface(
    cube_x: i32,
    cube_y: i32,
    cube_z: i32,
    angles: [f32; 3],
    ch: char, 
    mut z_buffer: Vec<Vec<i32>>,
    outpuf_buffer: &mut Vec<Vec<char>> 
) 
{
    let x_theta = angles[0];
    let y_theta = angles[1];
    let z_theta = angles[2];
    
    let mut current_vec: Vector = Vector {
        x: cube_x as f32, 
        y: cube_y as f32,  
        z: cube_z as f32 
    };

    // rotate the vector.
    current_vec.rotate_all(x_theta, y_theta, z_theta);  

    let w_offset = (WIDTH as usize) as f32 / 2.;
    let h_offset = (HEIGHT as usize) as f32 / 2.;

    // calc needed arguments.
    let ooz: f32 = 1.0 / (CUBE_DISTANCE + current_vec.z);
    let xp: usize = (w_offset + SCREEN_DISTANCE * ooz * current_vec.x * 2.) as usize; // * 2. because width of char is smaller than its height.
    let yp: usize = (h_offset + SCREEN_DISTANCE * ooz * current_vec.y) as usize;

    if xp >= WIDTH || yp >= HEIGHT { 
        return 
    };

    if ooz > z_buffer[yp][xp] as f32 { 

        // update the Z buffer.
        z_buffer[yp][xp] = ooz as i32;

        // set the luminace of the point.
        outpuf_buffer[yp][xp] = ch;

    }

}

fn render_cube() {

    let mut output_buffer: Vec<Vec<char>> = vec![vec![BG_CHAR; WIDTH]; HEIGHT]; // output on the screen.
    let mut depth_checker: Vec<Vec<i32>> = vec![vec![0; WIDTH]; HEIGHT]; // z buffer.
    let mut rotation_angles: [f32; 3] = [X, Y, Z];

    loop {

        // parse every surface into the output buffer. 
        for cube_x in -CUBE_LEN..CUBE_LEN {

            for cube_y in -CUBE_LEN..CUBE_LEN {
                
                parse_surface(cube_x, cube_y, -CUBE_LEN, rotation_angles,'~',
                    depth_checker.clone(),
                    &mut output_buffer, 
                );

                parse_surface(-cube_x, cube_y, CUBE_LEN, rotation_angles,'*',
                depth_checker.clone(),
                &mut output_buffer, 
                );

                parse_surface(CUBE_LEN, cube_y, cube_x, rotation_angles,'+',
                depth_checker.clone(),
                &mut output_buffer, 
                );

                parse_surface(-CUBE_LEN, cube_y, -cube_x, rotation_angles,'^',
                depth_checker.clone(),
                &mut output_buffer, 
                );

                parse_surface(cube_x, -CUBE_LEN, -cube_y, rotation_angles,'!',
                depth_checker.clone(),
                &mut output_buffer, 
                );

                parse_surface(cube_x, CUBE_LEN, cube_y, rotation_angles,'.',
                depth_checker.clone(),
                &mut output_buffer, 
                );

            }
        }

        for y in 0..HEIGHT {
            for x in 0..WIDTH { 
                print!("{}", output_buffer[y][x]);
            }
            print!("\n");
        }

        // return to the top.
        println!("\x1b[H");

        // inc the angles.
        rotation_angles[0] += SPEED;
        rotation_angles[1] += SPEED;
        rotation_angles[2] += SPEED;

        // renew buffers. 
        output_buffer = vec![vec![BG_CHAR; WIDTH]; HEIGHT]; 
        depth_checker = vec![vec![0; WIDTH]; HEIGHT];

        std::thread::sleep(Duration::from_millis(FRAME_DELAY));
    }

}

fn main() {
    // start animation.
    println!("\x1b[2J\r\x1b[H");
    render_cube();
}