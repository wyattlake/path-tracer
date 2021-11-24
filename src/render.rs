use crate::{Result, Scene};

pub struct RenderWorker<'a> {
    scene: &'a Scene<'a>,
}

impl<'a> RenderWorker<'a> {
    pub fn new(scene: &'a Scene) -> RenderWorker<'a> {
        RenderWorker { scene: scene }
    }

    pub fn render() -> Result<()> {
        Ok(())
    }
}
