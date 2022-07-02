use nalgebra::{Matrix4, Vector3};

use crate::Pack;

#[derive(Debug, Clone)]
pub struct Transform {
    pub pos: Vector3<f32>,
    pub ori: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    pub fn new(pos: Vector3<f32>, ori: Vector3<f32>, scale: Vector3<f32>) -> Transform {
        Transform {
            pos: pos,
            ori: ori,
            scale: scale,
        }
    }

    pub fn identity() -> Transform {
        Transform::new(
            Vector3::new(0.0f32, 0.0f32, 0.0f32),
            Vector3::new(0.0f32, 0.0f32, 0.0f32),
            Vector3::new(1.0f32, 1.0f32, 1.0f32),
        )
    }
}

impl Pack for Transform {
    fn pack(&self, buffer_f32: &mut Vec<f32>, buffer_u8: &mut Vec<u8>) {
        let res: Matrix4<f32> = Matrix4::new_translation(&self.pos)
            * Matrix4::from_euler_angles(self.ori.x, self.ori.y, self.ori.z)
            * Matrix4::new_nonuniform_scaling(&self.scale);

        let inverse = res.try_inverse().unwrap();
        inverse.pack(buffer_f32, buffer_u8);
        inverse.clone().transpose().pack(buffer_f32, buffer_u8);
    }
}
