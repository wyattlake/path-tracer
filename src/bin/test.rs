use path_tracer::{Result, Scene, Sphere};

fn main() -> Result<()> {
    let mut scene = Scene::new();
    let sphere = Sphere::new();
    let s2 = sphere.clone();
    scene.add_object(sphere);
    scene.add_object(s2);

    Ok(())
}
