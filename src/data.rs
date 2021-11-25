use ocl::{builders::KernelBuilder, flags, Buffer, Queue};

use crate::{Pack, Result};

#[derive(Debug, Clone)]
pub struct DataBuffer {
    buffer_i32: ocl::Buffer<i32>,
    buffer_f32: ocl::Buffer<f32>,
}

impl DataBuffer {
    pub fn new<P: Pack>(scene: &P, queue: &Queue) -> Result<DataBuffer> {
        let mut vec_i32 = Vec::new();
        let mut vec_f32 = Vec::new();
        scene.pack_i32(&mut vec_i32);
        scene.pack_f32(&mut vec_f32);

        let buffer_i32 = Buffer::<i32>::builder()
            .queue(queue.clone())
            .flags(flags::MEM_READ_ONLY)
            .len(vec_i32.len())
            .fill_val(0i32)
            .build()?;

        let buffer_f32 = Buffer::<f32>::builder()
            .queue(queue.clone())
            .flags(flags::MEM_READ_ONLY)
            .len(vec_f32.len())
            .fill_val(0f32)
            .build()?;

        buffer_i32.cmd().offset(0).write(&vec_i32).enq()?;
        buffer_f32.cmd().offset(0).write(&vec_f32).enq()?;

        let data = DataBuffer {
            buffer_i32: buffer_i32,
            buffer_f32: buffer_f32,
        };

        Ok(data)
    }

    pub fn add_args(self, kb: &mut KernelBuilder) {
        kb.arg(self.buffer_i32);
        kb.arg(self.buffer_f32);
    }
}
