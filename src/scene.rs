use crate::Object;

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

    /// Packs scene to f32 and u8 buffers while storing object data lengths
    pub fn pack_scene(
        &self,
        len_buffer: &mut Vec<u32>,
        buffer_f32: &mut Vec<f32>,
        buffer_u8: &mut Vec<u8>,
    ) {
        for object in &self.objects {
            let lens_before: (usize, usize) = (buffer_f32.len(), buffer_u8.len());
            object.pack(buffer_f32, buffer_u8);

            len_buffer.append(&mut vec![
                (buffer_f32.len() - lens_before.0) as u32,
                (buffer_u8.len() - lens_before.1) as u32,
            ]);
        }
    }
}
