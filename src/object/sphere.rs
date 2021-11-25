use nalgebra::Matrix3;

use crate::Pack;

use super::Object;

#[derive(Debug, Clone)]
pub struct Sphere {
    transform: Matrix3<f32>,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix3::identity(),
        }
    }
}

impl Pack for Sphere {
    fn pack_f32(&self, buffer_f32: &mut Vec<f32>) {
        self.transform.pack_f32(buffer_f32);
    }

    fn pack_i32(&self, buffer_i32: &mut Vec<i32>) {
        buffer_i32.append(&mut vec![0, 9]);
    }
}

impl Object for Sphere {}
