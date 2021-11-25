use crate::Result;
use image::ColorType;
use ocl::{flags, prm, Buffer, Context, ProQue};
use ocl_include::{source, Parser};
use std::path;
use uni_path;
#[derive(Debug, Clone)]
pub struct RawImage<'a> {
    dims: (usize, usize),
    raw_buffer: &'a Buffer<f32>,
    passes: usize,
    context: &'a Context,
}

impl<'a> RawImage<'a> {
    pub fn new(
        raw_buffer: &'a Buffer<f32>,
        passes: usize,
        dims: (usize, usize),
        context: &'a Context,
    ) -> Self {
        Self {
            raw_buffer,
            passes,
            dims,
            context,
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

#[derive(Debug, Clone)]
pub struct PostProcessor {
    mean_buffer: Buffer<f32>,
    rgb_buffer: Buffer<u8>,
    pro_que: ProQue,
    passes: usize,
    dims: (usize, usize),
}

impl PostProcessor {
    pub fn new(image: &RawImage) -> Result<PostProcessor> {
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
        let pro_que = ProQue::builder()
            .src(src)
            .dims(image.get_dims())
            .context(image.context.clone())
            .build()?;

        let mean_buffer = Buffer::<f32>::builder()
            .queue(pro_que.queue().clone())
            .flags(flags::MEM_READ_WRITE)
            .len(image.len())
            .fill_val(0f32)
            .build()?;

        let rgb_buffer = Buffer::<u8>::builder()
            .queue(pro_que.queue().clone())
            .flags(flags::MEM_READ_WRITE)
            .len(image.len())
            .fill_val(0u8)
            .build()?;

        let mut processor = PostProcessor {
            mean_buffer,
            rgb_buffer,
            pro_que,
            passes: 0,
            dims: image.get_dims(),
        };

        processor.process_image(image)?;

        Ok(processor)
    }

    pub fn process_image(&mut self, image: &RawImage) -> Result<()> {
        self.build_mean(image)?;
        self.build_rgb()?;
        Ok(())
    }

    pub fn build_mean(&mut self, image: &RawImage) -> Result<()> {
        let mut kb = self.pro_que.kernel_builder("mean");
        kb.arg(prm::Int2::new(self.dims.0 as i32, self.dims.1 as i32));
        kb.arg(self.passes);
        kb.arg(image.get_passes());
        kb.arg(&self.mean_buffer);
        kb.arg(image.raw_buffer);

        let kernel = kb.build()?;

        unsafe {
            kernel.enq()?;
        }

        self.passes += image.get_passes();

        Ok(())
    }

    pub fn build_rgb(&self) -> Result<()> {
        let mut kb = self.pro_que.kernel_builder("rgb");
        kb.arg(prm::Int2::new(self.dims.0 as i32, self.dims.1 as i32));
        kb.arg(&self.mean_buffer);
        kb.arg(&self.rgb_buffer);

        let kernel = kb.build()?;

        unsafe {
            kernel.enq()?;
        }

        Ok(())
    }

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
