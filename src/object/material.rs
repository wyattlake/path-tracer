use crate::{pack_struct, Pack};

pub type Color = (f32, f32, f32);

pack_struct! {
    struct Material {
        color: Color,
        emission_color: Color,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        invisible: bool,
    }
}

impl Material {
    pub fn default() -> Material {
        Material {
            color: (1.0, 1.0, 1.0),
            emission_color: (0.0, 0.0, 0.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            invisible: false,
        }
    }

    pub fn from_color(color: Color) -> Material {
        Material {
            color,
            emission_color: (0.0, 0.0, 0.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            invisible: false,
        }
    }

    pub fn light(color: Color) -> Material {
        Material {
            color: (1.0, 1.0, 1.0),
            emission_color: color,
            ambient: 0.0,
            diffuse: 0.0,
            specular: 0.0,
            invisible: true,
        }
    }
}
