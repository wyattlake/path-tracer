use crate::object::Object;

#[derive(Debug)]
pub struct Scene<'a> {
    objects: Vec<Box<dyn Object + 'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        return Scene {
            objects: Vec::new(),
        };
    }
    pub fn add_object<O: Object + 'a>(&mut self, object: O) {
        self.objects.push(Box::new(object))
    }
}
