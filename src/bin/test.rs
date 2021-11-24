use path_tracer::{object::Sphere, scene::*};
use std::io::Error;

fn main() -> Result<(), Error> {
    let mut scene = Scene::new();
    let sphere = Sphere::new();
    let s2 = sphere.clone();
    scene.add_object(sphere);
    scene.add_object(s2);

    Ok(())
}
