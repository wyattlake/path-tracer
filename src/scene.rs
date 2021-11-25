use crate::{Object, Pack};

#[derive(Debug)]
pub struct Scene<'a> {
    pub object_count: usize,
    objects: Vec<Box<dyn Object + 'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        return Scene {
            objects: Vec::new(),
            object_count: 0,
        };
    }

    pub fn add_object<O: Object + 'a>(&mut self, object: O) {
        self.objects.push(Box::new(object));
        self.object_count += 1;
    }
}

impl Pack for Scene<'_> {
    fn pack_f32(&self, buffer_f32: &mut Vec<f32>) {
        for object in &self.objects {
            object.pack_f32(buffer_f32);
        }
    }

    fn pack_i32(&self, buffer_i32: &mut Vec<i32>) {
        for object in &self.objects {
            object.pack_i32(buffer_i32);
        }
    }
}
