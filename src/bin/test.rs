use std::path::Path;

use path_tracer::{image::PostProcessor, Renderer, Result, Scene, Sphere};

fn main() -> Result<()> {
    let mut scene = Scene::new();
    let sphere = Sphere::new();
    scene.add_object(sphere);

    let mut renderer = Renderer::new(&scene, (100, 100))?;
    renderer.render()?;

    let path = Path::new("./output/image.png");
    let raw_image = renderer.raw_image();
    let processor = PostProcessor::new(&raw_image)?;
    processor.save_image(path)?;

    Ok(())
}
