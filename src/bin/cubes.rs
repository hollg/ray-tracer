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

    let mut middle = Cube::default();
    middle.transform = translate(1, 1, -1).rotate_y(40.0).scale(0.75, 0.75, 0.75);
    let mut middle_material = Material::default();
    middle_material.color = color(0.3, 0, 0);
    middle_material.diffuse = 0.7;
    middle_material.ambient = 0.8;
    middle_material.specular = 0.7;
    middle_material.shininess = 90.0;
    middle.material = middle_material;

    let mut right = Cube::default();
    right.transform = translate(1.5, 0.5, 1.5) * scale(0.5, 0.5, 0.5);

    let world = World::new(
        vec![
            // Box::new(left),
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
    let mut file = File::create("cubes.ppm")?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
