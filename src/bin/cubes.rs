use ray_tracer_lib::*;
use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut floor = Plane::default();
    let mut floor_material = Material::default();
    floor_material.pattern = checkers_pattern(WHITE, BLACK, None);
    floor_material.reflective = 0.4;
    floor_material.diffuse = 0.7;
    floor_material.specular = 0.3;
    floor.material = floor_material;

    let mut brick1 = Cube::default();
    // brick1.transform = scale(0.5, 0.25, 1.5).translate(-1.0, 0.25, 1.0);
    let mut brick_material = Material::default();
    brick_material.pattern = solid_pattern(color(0.3, 0, 0));
    brick_material.diffuse = 0.7;
    brick_material.ambient = 0.8;
    brick_material.specular = 0.7;
    brick_material.shininess = 90.0;
    brick1.material = brick_material;

    let mut brick2 = brick1.clone();
    brick2.transform(translate(1.1, 0.0, 0.0));

    let mut brick3 = brick2.clone();
    brick3.transform(translate(1.1, 0.0, 0.0));

    let mut brick4 = brick1.clone();
    brick4.transform(translate(0.0, 0.5, 0.0).rotate_y(180.0));

    
    let world = World::new(
        vec![
            Box::new(brick1),
            Box::new(brick2),
            Box::new(brick3),
            Box::new(brick4),
            Box::new(floor),
        ],
        vec![PointLight::new(point(-10, 10, -10), color(1, 1, 1))],
    );

    let camera = Camera::new(
        500,
        250,
        PI / 3.0,
        view_transform(point(0, 1.5, -5), point(0, 1, 0), vector(0, 1, 0)),
    );

    let canvas = camera.render(world);

    let ppm = canvas.to_ppm();
    let mut file = File::create("cubes.ppm")?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
