use ocl::{builders::KernelBuilder, flags, Buffer, Queue};

use crate::{Result, Scene};

/// Encapsulates packed data and transfers data to buffers
#[derive(Debug, Clone)]
pub struct SceneData {
    len_buffer: ocl::Buffer<u32>,
    buffer_u8: ocl::Buffer<u8>,
    buffer_f32: ocl::Buffer<f32>,
}

impl SceneData {
    pub fn new(scene: &Scene, queue: &Queue) -> Result<SceneData> {
        let mut len_vec = Vec::new();
        let mut vec_f32 = Vec::new();
        let mut vec_u8 = Vec::new();
        scene.pack_scene(&mut len_vec, &mut vec_f32, &mut vec_u8);

        let len_buffer = Buffer::<u32>::builder()
            .queue(queue.clone())
            .flags(flags::MEM_READ_ONLY)
            .len(len_vec.len())
            .fill_val(0u32)
            .build()?;

        let buffer_u8 = Buffer::<u8>::builder()
            .queue(queue.clone())
            .flags(flags::MEM_READ_ONLY)
            .len(vec_u8.len())
            .fill_val(0u8)
            .build()?;

        let buffer_f32 = Buffer::<f32>::builder()
            .queue(queue.clone())
            .flags(flags::MEM_READ_ONLY)
            .len(vec_f32.len())
            .fill_val(0f32)
            .build()?;

        len_buffer.cmd().offset(0).write(&len_vec).enq()?;
        buffer_u8.cmd().offset(0).write(&vec_u8).enq()?;
        buffer_f32.cmd().offset(0).write(&vec_f32).enq()?;

        let data = SceneData {
            len_buffer,
            buffer_u8,
            buffer_f32,
        };

        Ok(data)
    }

    pub fn add_args(self, kb: &mut KernelBuilder) {
        kb.arg(self.len_buffer);
        kb.arg(self.buffer_u8);
        kb.arg(self.buffer_f32);
    }
}
