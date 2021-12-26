use nalgebra::{Rotation3, Vector3};
use ocl::{builders::KernelBuilder, prm};

// Basic pinhole camera
#[derive(Debug, Clone)]
pub struct Camera {
    pos: Vector3<f32>,
    ori: Rotation3<f32>,
    pub fov: f32,
}

impl Camera {
    pub fn new(pos: Vector3<f32>, ori: Rotation3<f32>) -> Camera {
        Camera {
            pos,
            ori,
            fov: 1.0f32,
        }
    }

    pub fn default() -> Camera {
        Camera::new(Vector3::default(), Rotation3::identity())
    }

    pub fn add_args(&self, kb: &mut KernelBuilder) {
        let mut ori_buffer = [0f32; 16];
        let ori_matrix = self.ori.matrix();
        ori_buffer[0..3].copy_from_slice(&ori_matrix.as_slice()[0..3]);
        ori_buffer[4..7].copy_from_slice(&ori_matrix.as_slice()[3..6]);
        ori_buffer[8..11].copy_from_slice(&ori_matrix.as_slice()[6..9]);

        let mut pos_buffer = [0f32; 3];
        pos_buffer.copy_from_slice(&self.pos.as_slice());

        kb.arg(prm::Float3::from(pos_buffer));
        kb.arg(prm::Float16::from(ori_buffer));
        kb.arg(self.fov);
    }
}
