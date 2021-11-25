use image::{ImageBuffer, RgbImage};
use ocl::{flags, prm, Buffer, ProQue};
use ocl_include::{source, Parser};

use crate::{image::RawImage, DataBuffer, Result, Scene};
use rand::{thread_rng, Rng};
use uni_path::Path;

pub struct Renderer<'a> {
    dims: (usize, usize),
    passes: usize,
    color_buffer: Buffer<f32>,
    random_buffer: Buffer<u32>,
    pro_que: ProQue,
    scene: &'a Scene<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(scene: &'a Scene, dims: (usize, usize)) -> Result<Renderer<'a>> {
        let len = dims.0 * dims.1;

        let parser = Parser::builder()
            .add_source(
                source::Fs::builder()
                    .include_dir(&Path::new("./ocl"))
                    .unwrap()
                    .build(),
            )
            .build();
        let node = parser.parse(Path::new("main.cl")).unwrap();
        let (src, _) = node.collect();

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
            .len(len)
            .fill_val(0u32)
            .build()?;

        let mut seed = vec![0u32; len];
        thread_rng().fill(&mut seed[..]);

        random_buffer.cmd().offset(0).write(&seed).enq()?;

        Ok(Renderer {
            dims: dims,
            scene: scene,
            passes: 0,
            color_buffer: color_buffer,
            random_buffer: random_buffer,
            pro_que: pro_que,
        })
    }

    pub fn render(&mut self) -> Result<()> {
        self.passes += 1;

        if self.scene.object_count == 0 {
            return Err("Scene with zero objects cannot be rendered".into());
        }

        let mut kb = self.pro_que.kernel_builder("render");

        let data = DataBuffer::new(self.scene, self.pro_que.queue())?;
        kb.arg(prm::Int2::new(self.dims.0 as i32, self.dims.1 as i32));
        kb.arg(&self.color_buffer);
        kb.arg(&self.random_buffer);
        data.add_args(&mut kb);

        let kernel = kb.build()?;

        unsafe {
            kernel.enq()?;
        }

        let len = self.dims.0 * self.dims.1;
        let mut res = vec![0f32; len * 3];
        self.color_buffer.cmd().offset(0).read(&mut res).enq()?;

        Ok(())
    }

    pub fn render_passes(&mut self, passes: usize) -> Result<()> {
        for _ in 0..passes {
            self.render()?;
        }
        Ok(())
    }

    pub fn raw_image(&self) -> RawImage<'_> {
        RawImage::new(
            &self.color_buffer,
            self.passes,
            self.dims,
            self.pro_que.context(),
        )
    }
}
