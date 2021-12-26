use ocl::{flags, prm, Buffer, Context, ProQue};
use ocl_include::{source, Parser};

use crate::{image::RawImage, Camera, Result, Scene, SceneData};
use rand::{thread_rng, Rng};
use uni_path::Path;

/// Renders a Scene using OpenCL
#[derive(Debug, Clone)]
pub struct Renderer<'a> {
    dims: (usize, usize),
    passes: usize,
    color_buffer: Buffer<f32>,
    random_buffer: Buffer<u32>,
    pro_que: ProQue,
    scene: &'a Scene<'a>,
    camera: &'a Camera,
}

impl<'a> Renderer<'a> {
    /// Creates a new Renderer with OpenCL buffers
    pub fn new(
        scene: &'a Scene,
        camera: &'a Camera,
        dims: (usize, usize),
        context: &Context,
    ) -> Result<Renderer<'a>> {
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

        let pro_que = ProQue::builder()
            .context(context.clone())
            .src(src)
            .dims(dims)
            .build()?;

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
            dims,
            scene,
            passes: 0,
            color_buffer,
            random_buffer,
            camera,
            pro_que,
        })
    }

    /// Renders lighting only from point lights
    pub fn render_direct_lighting(&mut self) -> Result<()> {
        self.passes += 1;

        if self.scene.object_count == 0 {
            return Err("Scene with zero objects cannot be rendered".into());
        }

        let mut kb = self.pro_que.kernel_builder("render_direct_lighting");

        // Packed scene data
        let data = SceneData::new(&self.scene, self.pro_que.queue())?;
        kb.arg(prm::Int2::new(self.dims.0 as i32, self.dims.1 as i32));
        kb.arg(&self.color_buffer);
        kb.arg(&self.random_buffer);

        data.add_args(&mut kb);
        self.camera.add_args(&mut kb);

        let kernel = kb.build()?;

        unsafe {
            kernel.enq()?;
        }

        Ok(())
    }

    /// Renders lighting from surrounding objects using Monte Carlo integration
    pub fn render_indirect_lighting(&mut self) -> Result<()> {
        self.passes += 1;

        if self.scene.object_count == 0 {
            return Err("Scene with zero objects cannot be rendered".into());
        }

        let mut kb = self.pro_que.kernel_builder("render_indirect_lighting");

        // Packed scene data
        let data = SceneData::new(self.scene, self.pro_que.queue())?;
        kb.arg(prm::Int2::new(self.dims.0 as i32, self.dims.1 as i32));
        kb.arg(1);
        kb.arg(&self.color_buffer);

        data.add_args(&mut kb);
        self.camera.add_args(&mut kb);

        let kernel = kb.build()?;

        unsafe {
            kernel.enq()?;
        }

        Ok(())
    }

    /// Repeatedly renders indirect lighting
    pub fn render_indirect_passes(&mut self, passes: usize) -> Result<()> {
        for _ in 0..passes {
            self.render_indirect_lighting()?;
        }
        Ok(())
    }

    /// Converts the color buffer to a RawImage for Post Processing
    pub fn raw_image(&self) -> RawImage<'_> {
        RawImage::new(&self.color_buffer, self.passes, self.dims)
    }

    /// Resets the color buffers
    pub fn reset(&mut self) -> Result<()> {
        self.color_buffer.cmd().offset(0).fill(0f32, None).enq()?;
        Ok(())
    }
}
