use nalgebra::Matrix3;

use crate::{pack_struct, Pack, Transform};

use super::Object;

// Sphere object which can be added to scenes
pack_struct! {
    struct Sphere {
        pack_id: 0,
        transform: Transform,
    }
}

impl Sphere {
    pub fn default() -> Sphere {
        Sphere::new(Transform::new(Matrix3::identity()))
    }
}

impl Object for Sphere {}
