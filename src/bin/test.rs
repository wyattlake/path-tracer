use std::path::Path;

use nalgebra::{Rotation3, Vector3};
use path_tracer::{
    image::PostProcessor, Camera, Context, Material, Renderer, Result, Scene, Sphere, Transform,
};
use std::time::Instant;

fn main() -> Result<()> {
    let context = Context::new()?;

    let mut scene = Scene::new();

    let s = Sphere::new(Transform::identity(), Material::from_color((1.0, 0.0, 0.0)));
    scene.add_object(s);

    let camera = Camera::new(
        Vector3::new(0.0f32, 0.0f32, 5f32),
        Rotation3::from_euler_angles(0.0f32, 0.0f32, 0.0f32),
    );

    let mut renderer = Renderer::new(&scene, &camera, (1000, 1000), &context)?;

    println!("Render started...");
    let now = Instant::now();
    renderer.render_direct_lighting()?;

    let duration = now.elapsed();
    println!("Image successfully rendered.");
    println!(
        "{} milliseconds elapsed.",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );

    let path = Path::new("./output/image.png");
    let raw_image = renderer.raw_image();
    let processor = PostProcessor::new(&raw_image, &context)?;
    println!("Saving image...");
    processor.save_image(path)?;
    println!("Image saved.");

    Ok(())
}
