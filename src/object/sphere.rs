use super::Object;

#[derive(Debug, Clone)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {}
    }
}

impl Object for Sphere {}
