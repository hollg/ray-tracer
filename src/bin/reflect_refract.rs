use ray_tracer_lib::*;
use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let wall_material = Material {
        ambient: 0.0,
        diffuse: 0.4,
        specular: 0.0,
        pattern: stripe_pattern(
            color(0.45, 0.45, 0.45),
            color(0.55, 0.55, 0.55),
            scale(0.25, 0.25, 0.25).rotate_y(1.5708),
        ),
        shininess: 200.0,
        reflective: 0.0,
        transparency: 0.0,
        refractive_index: 1.0,
    };

    let mut floor_material = Material::default();
    floor_material.pattern =
        checkers_pattern(color(0.35, 0.35, 0.35), color(0.65, 0.65, 0.65), None);
    floor_material.specular = 0.0;
    floor_material.reflective = 0.4;

    let floor = Plane::new(floor_material, rotate_y(0.31415));

    let mut ceiling_material = Material::default();
    ceiling_material.pattern = solid_pattern(color(0.8, 0.8, 0.8));
    ceiling_material.specular = 0.0;
    ceiling_material.ambient = 0.3;
    let ceiling = Plane::new(ceiling_material, translate(0, 5, 0));

    let west_wall = Plane::new(
        wall_material.clone(),
        rotate_y(1.5708).rotate_z(1.5708).translate(-5.0, 0.0, 0.0),
    );
    let east_wall = Plane::new(
        wall_material.clone(),
        rotate_y(1.5708).rotate_z(1.5708).translate(5.0, 0.0, 0.0),
    );

    let world = World::new(
        vec![Box::new(floor), Box::new(ceiling), Box::new(west_wall), Box::new(east_wall)],
        vec![PointLight::new(point(-4.9, 4.9, -1), color(1, 1, 1))],
    );

    let camera = Camera::new(
        400,
        200,
        1.152,
        view_transform(
            point(-2.6, 1.5, -3.9),
            point(-0.6, 1, -0.8),
            vector(0, 1, 0),
        ),
    );

    let canvas = camera.render(world);

    let ppm = canvas.to_ppm();
    let mut file = File::create("reflective_spheres.ppm")?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
