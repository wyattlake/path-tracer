use nalgebra::Matrix3;

use crate::Pack;

#[derive(Debug, Clone)]
pub struct Transform(Matrix3<f32>);

impl Transform {
    pub fn new(matrix: Matrix3<f32>) -> Transform {
        Transform(matrix)
    }

    pub fn mult(&mut self, matrix: Matrix3<f32>) {
        self.0 = matrix * self.0;
    }
}

impl Pack for Transform {
    fn pack(&self, buffer_f32: &mut Vec<f32>, buffer_u8: &mut Vec<u8>) {
        self.0.pack(buffer_f32, buffer_u8);
        self.0.try_inverse().unwrap().pack(buffer_f32, buffer_u8)
    }
}
