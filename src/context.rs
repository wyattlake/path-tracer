use ocl::{Device, Platform, Queue};

use crate::Result;

/// Stores OpenCl Context, Platform, and Device
#[derive(Debug, Clone)]
pub struct Context {
    context: ocl::Context,
    platform: Platform,
    device: Device,
    queue: Queue,
}

impl Context {
    pub fn new() -> Result<crate::Context> {
        let platform = Platform::default();
        let device = Device::first(platform)?;
        let context = ocl::Context::builder()
            .platform(platform)
            .devices(device.clone())
            .build()?;

        let queue = ocl::Queue::new(&context, device, None)?;

        Ok(Context {
            context,
            platform,
            device,
            queue,
        })
    }

    pub fn get_context(&self) -> &ocl::Context {
        &self.context
    }

    pub fn get_device(&self) -> &ocl::Device {
        &self.device
    }

    pub fn get_platform(&self) -> &ocl::Platform {
        &self.platform
    }

    pub fn get_queue(&self) -> &ocl::Queue {
        &self.queue
    }
}
