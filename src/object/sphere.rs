use nalgebra::Matrix3;

use crate::{pack_object, Pack, Transform};

use super::Object;

// Sphere object which can be added to scenes
pack_object! {
    struct Sphere {
        object_id: 0,
        transform: Transform,
        invisible: bool,
    }
}

impl Sphere {
    pub fn default() -> Sphere {
        Sphere::new(Transform::new(Matrix3::identity()), false)
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        return &mut self.transform;
    }
}

impl Object for Sphere {}
