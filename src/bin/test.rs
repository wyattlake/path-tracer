use std::path::Path;

use nalgebra::{Rotation3, Vector3};
use path_tracer::{image::PostProcessor, Camera, Context, Renderer, Result, Scene, Sphere};

fn main() -> Result<()> {
    let context = Context::new()?;

    let mut scene = Scene::new();
    let sphere = Sphere::default();
    scene.add_object(sphere);

    let camera = Camera::new(Vector3::new(0.0f32, 0.0f32, 2.5f32), Rotation3::identity());
    let mut renderer = Renderer::new(&scene, &camera, (1000, 1000), context.get_context())?;
    renderer.render_direct_lighting()?;

    let path = Path::new("./output/image.png");
    let raw_image = renderer.raw_image();
    let processor = PostProcessor::new(&raw_image, context.get_context())?;
    processor.save_image(path)?;

    Ok(())
}
