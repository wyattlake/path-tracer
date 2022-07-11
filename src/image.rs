use crate::{Context, Result};
use image::ColorType;
use ocl::{flags, prm, Buffer, Program};
use ocl_include::{source, Parser};
use std::path;
use uni_path;
#[derive(Debug, Clone)]

/// Contains raw render output for Post Processing
pub struct RawImage<'a> {
    dims: (usize, usize),
    raw_buffer: &'a Buffer<f32>,
    passes: usize,
}

impl<'a> RawImage<'a> {
    pub fn new(raw_buffer: &'a Buffer<f32>, passes: usize, dims: (usize, usize)) -> Self {
        Self {
            raw_buffer,
            passes,
            dims,
        }
    }

    pub fn len(&self) -> usize {
        self.raw_buffer.len()
    }

    pub fn get_passes(&self) -> usize {
        self.passes
    }

    pub fn get_dims(&self) -> (usize, usize) {
        self.dims
    }
}

/// Struct for manipulating raw images
#[derive(Debug, Clone)]
pub struct PostProcessor {
    mean_buffer: Buffer<f32>,
    rgb_buffer: Buffer<u8>,
    program: Program,
    context: Context,
    passes: usize,
    dims: (usize, usize),
}

impl PostProcessor {
    pub fn new(image: &RawImage, context: &Context) -> Result<PostProcessor> {
        let parser = Parser::builder()
            .add_source(
                source::Fs::builder()
                    .include_dir(&uni_path::Path::new("./ocl"))
                    .unwrap()
                    .build(),
            )
            .build();
        let node = parser.parse(uni_path::Path::new("processor.cl")).unwrap();
        let (src, _) = node.collect();

        let program = Program::builder()
            .devices(context.get_device())
            .source(src)
            .build(context.get_context())?;

        let mean_buffer = Buffer::<f32>::builder()
            .queue(context.get_queue().clone())
            .flags(flags::MEM_READ_WRITE)
            .len(image.len())
            .fill_val(0f32)
            .build()?;

        let rgb_buffer = Buffer::<u8>::builder()
            .queue(context.get_queue().clone())
            .flags(flags::MEM_READ_WRITE)
            .len(image.len())
            .fill_val(0u8)
            .build()?;

        let mut processor = PostProcessor {
            mean_buffer,
            rgb_buffer,
            program,
            context: context.clone(),
            passes: 0,
            dims: image.get_dims(),
        };

        processor.process_image(image)?;

        Ok(processor)
    }

    /// Averages MonteCarlo estimations and converts to rgb
    pub fn process_image(&mut self, image: &RawImage) -> Result<()> {
        self.build_mean(image)?;
        self.build_rgb()?;
        Ok(())
    }

    /// Averages MonteCarlo estimations
    pub fn build_mean(&mut self, image: &RawImage) -> Result<()> {
        let queue = self.context.get_queue().clone();

        let mut kb = ocl::Kernel::builder();
        kb.program(&self.program).name("mean").queue(queue.clone());

        kb.arg(prm::Int2::new(self.dims.0 as i32, self.dims.1 as i32));
        kb.arg(self.passes);
        kb.arg(image.get_passes());
        kb.arg(&self.mean_buffer);
        kb.arg(image.raw_buffer);

        let kernel = kb.build()?;

        unsafe {
            kernel.cmd().global_work_size(self.dims).enq()?;
        }

        self.context.get_queue().finish()?;

        self.passes += image.get_passes();

        Ok(())
    }

    /// Converts from float buffer to rgb
    pub fn build_rgb(&self) -> Result<()> {
        let queue = self.context.get_queue().clone();

        let mut kb = ocl::Kernel::builder();
        kb.program(&self.program).name("rgb").queue(queue.clone());

        kb.arg(prm::Int2::new(self.dims.0 as i32, self.dims.1 as i32));
        kb.arg(&self.mean_buffer);
        kb.arg(&self.rgb_buffer);

        let kernel = kb.build()?;

        unsafe {
            kernel.cmd().global_work_size(self.dims).enq()?;
        }

        self.context.get_queue().finish()?;

        Ok(())
    }

    /// Saves rgb buffer to specified path
    pub fn save_image(&self, path: &path::Path) -> Result<()> {
        let mut buf = vec![0u8; self.rgb_buffer.len()];
        self.rgb_buffer.cmd().offset(0).read(&mut buf).enq()?;

        let res = image::save_buffer(
            path,
            &buf[..],
            self.dims.0 as u32,
            self.dims.1 as u32,
            ColorType::Rgb8,
        );

        match res {
            Ok(()) => Ok(()),
            Err(e) => {
                panic!("ImageError:\n{:?}", e);
            }
        }
    }
}
