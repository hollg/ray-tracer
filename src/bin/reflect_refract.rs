use ray_tracer_lib::*;
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
        rotate_y(-1.5708).rotate_z(-1.5708).translate(5.0, 0.0, 0.0),
    );

    let north_wall = Plane::new(
        wall_material.clone(),
        rotate_x(1.5708).translate(0.0, 0.0, 5.0),
    );

    let south_wall = Plane::new(
        wall_material.clone(),
        rotate_x(1.5708).translate(0.0, 0.0, -5.0),
    );

    let mut red_sphere = Sphere::default();
    red_sphere.transform(translate(-0.6, 1, 0.6));
    red_sphere.material.pattern = solid_pattern(color(1, 0.3, 0.2));
    red_sphere.material.specular = 0.4;
    red_sphere.material.shininess = 5.0;

    let mut blue_glass_sphere = Sphere::default();
    blue_glass_sphere.transform(scale(0.7, 0.7, 0.7).translate(0.6, 0.7, -0.6));
    blue_glass_sphere.material.pattern = solid_pattern(color(0, 0, 0.2));
    blue_glass_sphere.material.ambient = 0.0;
    blue_glass_sphere.material.diffuse = 0.4;
    blue_glass_sphere.material.specular = 0.9;
    blue_glass_sphere.material.shininess = 300.0;
    blue_glass_sphere.material.reflective = 0.9;
    blue_glass_sphere.material.transparency = 0.9;
    blue_glass_sphere.material.refractive_index = 1.5;

    let mut green_glass_sphere = Sphere::default();
    green_glass_sphere.transform(scale(0.5, 0.5, 0.5).translate(-0.7, 0.5, -0.8));
    green_glass_sphere.material.pattern = solid_pattern(color(0, 0.2, 0));
    green_glass_sphere.material.ambient = 0.0;
    green_glass_sphere.material.diffuse = 0.4;
    green_glass_sphere.material.specular = 0.9;
    green_glass_sphere.material.shininess = 300.0;
    green_glass_sphere.material.reflective = 0.9;
    green_glass_sphere.material.transparency = 0.9;
    green_glass_sphere.material.refractive_index = 1.5;

    let mut bg_sphere_1 = Sphere::default();
    bg_sphere_1.transform(scale(0.4, 0.4, 0.4).translate(4.6, 0.4, 1.0));
    bg_sphere_1.material.pattern = solid_pattern(color(0.8, 0.5, 0.3));
    bg_sphere_1.material.shininess = 50.0;

    let mut bg_sphere_2 = Sphere::default();
    bg_sphere_2.transform(scale(0.3, 0.3, 0.3).translate(4.7, 0.3, 0.4));
    bg_sphere_2.material.pattern = solid_pattern(color(0.9, 0.4, 0.5));
    bg_sphere_2.material.shininess = 50.0;

    let mut bg_sphere_3 = Sphere::default();
    bg_sphere_3.transform(scale(0.5, 0.5, 0.5).translate(-1.0, 0.5, 4.5));
    bg_sphere_3.material.pattern = solid_pattern(color(0.4, 0.9, 0.6));
    bg_sphere_3.material.shininess = 50.0;

    let mut bg_sphere_4 = Sphere::default();
    bg_sphere_4.transform(scale(0.3, 0.3, 0.3).translate(-1.7, 0.3, 4.7));
    bg_sphere_4.material.pattern = solid_pattern(color(0.4, 0.6, 0.9));
    bg_sphere_4.material.shininess = 50.0;

    let world = World::new(
        vec![
            Box::new(floor),
            Box::new(ceiling),
            Box::new(west_wall),
            Box::new(east_wall),
            Box::new(north_wall),
            Box::new(south_wall),
            Box::new(red_sphere),
            Box::new(blue_glass_sphere),
            Box::new(green_glass_sphere),
            Box::new(bg_sphere_1),
            Box::new(bg_sphere_2),
            Box::new(bg_sphere_3),
            Box::new(bg_sphere_4),
        ],
        vec![PointLight::new(point(-4.9, 4.9, -1), color(1, 1, 1))],
    );

    let camera = Camera::new(
        1000,
        750,
        1.152,
        view_transform(
            point(-2.6, 1.5, -5.9),
            point(-0.6, 2.5, -0.8),
            vector(0, 1, 0),
        ),
    );

    let canvas = camera.render(world);

    let ppm = canvas.to_ppm();
    let mut file = File::create("reflective_spheres.ppm")?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
