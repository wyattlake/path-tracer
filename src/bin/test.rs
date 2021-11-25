use path_tracer::{Renderer, Result, Scene, Sphere};

fn main() -> Result<()> {
    let mut scene = Scene::new();
    let sphere = Sphere::new();
    scene.add_object(sphere);

    let renderer = Renderer::new(&scene, (10, 10), 1)?;
    renderer.render()?;

    Ok(())
}
