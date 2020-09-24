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

    let mut middle = Sphere::default();
    middle.transform = translate(-0.5, 1, 0.5);
    let mut middle_material = Material::default();
    middle_material.color = color(0.1, 1, 0.5);
    middle_material.diffuse = 0.7;
    middle_material.specular = 0.3;
    middle_material.reflective = 0.1;
    middle.material = middle_material;

    let mut right = Sphere::default();
    right.transform = translate(1.5, 0.5, 1.5) * scale(0.5, 0.5, 0.5);
    let mut right_material = Material::default();
    right_material.color = WHITE;
    right_material.diffuse = 0.7;
    right_material.specular = 0.2;
    right_material.reflective = 1.0;
    right_material.shininess = 1.0;
    right.material = right_material;

    let mut left = Sphere::default();
    left.transform = translate(-1.5, 0.33, -0.75) * scale(0.33, 0.33, 0.33);
    let mut left_material = Material::default();
    left_material.color = color(1, 0.8, 0.1);
    left_material.diffuse = 0.7;
    left_material.specular = 0.3;
    left_material.reflective = 0.1;
    left_material.pattern = Some(gradient_pattern(
        color(1, 1, 0),
        color(1, 0, 1),
        rotate_x(30),
    ));
    left.material = left_material;

    let world = World::new(
        vec![
            Box::new(left),
            Box::new(right),
            Box::new(middle),
            Box::new(floor),
        ],
        vec![PointLight::new(point(-10, 10, -10), color(1, 1, 1))],
    );

    let camera = Camera::new(
        4000,
        2000,
        PI / 3.0,
        view_transform(point(0, 1.5, -5), point(0, 1, 0), vector(0, 1, 0)),
    );

    let canvas = camera.render(world);

    let ppm = canvas.to_ppm();
    let mut file = File::create("spheres.ppm")?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
