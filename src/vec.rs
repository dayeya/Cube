#[derive(Debug)]
pub struct Vector {
    pub x: f32, 
    pub y: f32, 
    pub z: f32 
}

pub trait Rotation {
    
    // Rotate Self by the x-axis
    fn rotate_x(&self, theta: f32) -> Vector;

    // Rotate Self by the y-axis
    fn rotate_y(&self, theta: f32) -> Vector;

    // Rotate Self by the z-axis
    fn rotate_z(&self, theta: f32) -> Vector;

    // Rotate Self by x, y and z axis.
    fn rotate_all(&mut self, x_theta: f32, y_theta: f32, z_theta: f32);

}

impl Rotation for Vector {
    fn rotate_x(&self, theta: f32) -> Vector {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        // Return the rotated vector.
        Vector {
            x: self.x, 
            y: self.y * cos_theta + self.z * sin_theta, 
            z: self.z * cos_theta - self.y * sin_theta
        }
    }

    fn rotate_y(&self, theta: f32) -> Vector {

        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        // Return the rotated vector.
        Vector {
            x: self.x * cos_theta - self.z * sin_theta, 
            y: self.y, 
            z: self.x * sin_theta + self.z * cos_theta
        }
    }

    fn rotate_z(&self, theta: f32) -> Vector {

        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        // Return the rotated vector.
        Vector {
            x: self.x * cos_theta - self.y * sin_theta, 
            y: self.x * sin_theta + self.y * cos_theta,
            z: self.z
        }
    }

    fn rotate_all(&mut self, x_theta: f32, y_theta: f32, z_theta: f32) {
        // Takes the origin vector "SELF" and rotating it by x, y, and z axis. 
        *self = self.rotate_x(x_theta).rotate_y(y_theta).rotate_z(z_theta);
    }
}
