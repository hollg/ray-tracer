use ray_tracer_lib::*;
use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut floor = Plane::default();
    let mut floor_material = Material::default();
    floor_material.pattern = Some(checkers_pattern(WHITE, BLACK, None));
    floor_material.reflective = 0.4;
    floor_material.diffuse = 0.7;
    floor_material.specular = 0.3;
    floor.material = floor_material;

    let mut sphere = Sphere::default();
    sphere.transform(translate(-0.5, 1, 0.5));
    let mut sphere_material = Material::default();
    sphere_material.color = color(0.2, 0, 0);
    sphere_material.diffuse = 0.1;
    sphere_material.ambient = 0.1;
    sphere_material.specular = 0.1;
    sphere_material.reflective = 0.9;
    sphere_material.refractive_index = 1.5;
    sphere_material.transparency = 1.0;
    sphere_material.shininess = 300.0;
    sphere.material = sphere_material;

    let world = World::new(
        vec![Box::new(sphere), Box::new(floor)],
        vec![PointLight::new(point(-10, 10, -10), color(1, 1, 1))],
    );

    let camera = Camera::new(
        2000,
        1000,
        PI / 3.0,
        view_transform(point(0, 1.5, -5), point(0, 1, 0), vector(0, 1, 0)),
    );

    let canvas = camera.render(world);

    let ppm = canvas.to_ppm();
    let mut file = File::create("spheres.ppm")?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
