use std::path::Path;

use path_tracer::{image::PostProcessor, Context, Renderer, Result, Scene, Sphere};

fn main() -> Result<()> {
    let context = Context::new()?;

    let mut scene = Scene::new();
    let sphere = Sphere::default();
    scene.add_object(sphere);

    let mut renderer = Renderer::new(&scene, (100, 100), context.get_context())?;
    renderer.render_direct_lighting()?;

    let path = Path::new("./output/image.png");
    let raw_image = renderer.raw_image();
    let processor = PostProcessor::new(&raw_image, context.get_context())?;
    processor.save_image(path)?;

    Ok(())
}
