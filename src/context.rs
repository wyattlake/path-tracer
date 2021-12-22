use ocl::{Device, Platform};

use crate::Result;

/// Stores OpenCl Context, Platform, and Device
pub struct Context {
    context: ocl::Context,
    platform: Platform,
    device: Device,
}

impl Context {
    pub fn new() -> Result<crate::Context> {
        let platform = Platform::default();
        let device = Device::first(platform)?;
        let context = ocl::Context::builder()
            .platform(platform)
            .devices(device.clone())
            .build()?;
        Ok(Context {
            context,
            platform,
            device,
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
}
