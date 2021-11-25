use ocl::{flags, prm, Buffer, ProQue};

use crate::{DataBuffer, Result, Scene};
use std::fs;

pub struct Renderer<'a> {
    pub dims: (usize, usize),
    pub passes: usize,
    color_buffer: Buffer<f32>,
    random_buffer: Buffer<u32>,
    pro_que: ProQue,
    scene: &'a Scene<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(scene: &'a Scene, dims: (usize, usize), passes: usize) -> Result<Renderer<'a>> {
        let len = dims.0 * dims.1;

        let src = fs::read_to_string("ocl/main.cl")?;
        let pro_que = ProQue::builder().src(src).dims(dims).build()?;

        let color_buffer = Buffer::<f32>::builder()
            .queue(pro_que.queue().clone())
            .flags(flags::MEM_READ_WRITE)
            .len(3 * len)
            .fill_val(0f32)
            .build()?;

        let random_buffer = Buffer::<u32>::builder()
            .queue(pro_que.queue().clone())
            .flags(flags::MEM_READ_ONLY)
            .len(3 * len)
            .fill_val(0u32)
            .build()?;

        Ok(Renderer {
            dims: dims,
            scene: scene,
            passes: passes,
            color_buffer: color_buffer,
            random_buffer: random_buffer,
            pro_que: pro_que,
        })
    }

    pub fn render(&self) -> Result<()> {
        if self.scene.object_count == 0 {
            return Err("Scene with zero objects cannot be rendered".into());
        }

        let mut kb = self.pro_que.kernel_builder("render");

        let data = DataBuffer::new(self.scene, self.pro_que.queue())?;
        kb.arg(prm::Int2::zero());
        kb.arg(&self.color_buffer);
        kb.arg(&self.random_buffer);
        data.add_args(&mut kb);

        let kernel = kb.build()?;

        unsafe {
            kernel.enq()?;
        }

        Ok(())
    }

    pub fn get_color_buffer(&self) -> &'_ Buffer<f32> {
        &self.color_buffer
    }

    pub fn get_random_buffer(&self) -> &'_ Buffer<u32> {
        &self.random_buffer
    }
}
