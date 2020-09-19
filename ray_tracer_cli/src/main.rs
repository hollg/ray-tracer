use ray_tracer_lib::*;
use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut floor = Sphere::default();
    floor.transform = scale(10, 0.01, 10);
    let mut floor_material = Material::default();
    floor_material.color = color(1, 0.9, 0.9);
    floor_material.specular = 0.0;
    floor.material = floor_material;

    let mut left_wall = Sphere::default();
    left_wall.transform =
        translate(0, 0, 5) * rotate_y(-PI / 4.0) * rotate_x(PI / 2.0) * scale(10, 0.01, 10.0);
    left_wall.material = floor_material;

    let mut right_wall = Sphere::default();
    right_wall.transform =
        translate(0, 0, 5) * rotate_y(PI / 4.0) * rotate_x(PI / 2.0) * scale(10, 0.01, 10);
    right_wall.material = floor_material;

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
    world.light_source = Some(PointLight::new(point(-10, 10, -10), color(1, 1, 1)));
    world.objects = vec![left, right, middle, floor, left_wall, right_wall];
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
