use ray_tracer_lib::*;
use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    
    let mut floor = Plane::default();
    floor.material_mut().pattern = Some(stripe_pattern(BLACK, WHITE));
    
    let mut middle = Sphere::default();
    middle.transform = translate(-0.5, 1, 0.5);
    let mut middle_material = Material::default();
    middle_material.color = color(0.1, 1, 0.5);
    middle_material.diffuse = 0.7;
    middle_material.specular = 0.3;
    middle.material = middle_material;

    let mut right = Sphere::default();
    right.transform = translate(1.5, 0.5, -0.5) * scale(0.5, 0.5, 0.5);
    let mut right_material = Material::default();
    right_material.color = color(0.5, 1, 0.1);
    right_material.diffuse = 0.7;
    right_material.specular = 0.3;
    right.material = right_material;

    let mut left = Sphere::default();
    left.transform = translate(-1.5, 0.33, -0.75) * scale(0.33, 0.33, 0.33);
    let mut left_material = Material::default();
    left_material.color = color(1, 0.8, 0.1);
    left_material.diffuse = 0.7;
    left_material.specular = 0.3;
    left.material = left_material;

    let mut world = World::default();
    world.light_sources = vec![
        PointLight::new(point(-10, 10, -10), color(1, 1, 1))
    ];
    world.objects = vec![
        Box::new(left),
        Box::new(right),
        Box::new(middle),
        Box::new(floor),
    ];
    let c = camera(
        500,
        250,
        PI / 3.0,
        view_transform(point(0, 1.5, -5), point(0, 1, 0), vector(0, 1, 0)),
    );

    let canvas = c.render(world);

    let ppm = canvas.to_ppm();
    let mut file = File::create("spheres.ppm")?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
