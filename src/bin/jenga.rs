use rand::Rng;
use ray_tracer_lib::*;
use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

enum RowDirection {
    X,
    Z,
}
enum RowType {
    Full,
    Middle,
    Edges,
}

fn gen_row_positions() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1, 5);

    let row_type: RowType;
    if num <= 2 {
        row_type = RowType::Edges;
    } else if num == 3 {
        row_type = RowType::Full;
    } else {
        row_type = RowType::Middle;
    }

    match row_type {
        RowType::Full => vec![0, 1, 2],
        RowType::Middle => vec![1],
        RowType::Edges => vec![0, 2],
    }
}

fn main() -> std::io::Result<()> {
    let floor_material = Material {
        ambient: 0.1,
        diffuse: 0.7,
        specular: 0.3,
        shininess: 250.0,
        reflective: 0.2,
        transparency: 0.0,
        refractive_index: 0.0,
        pattern: checkers_pattern(WHITE, BLACK, None),
    };
    let floor = Plane::new(floor_material.clone(), Matrix::identity());

    let mirror = Cube::new(
        // Material {
        //     ambient: 0.1,
        //     diffuse: 0.7,
        //     specular: 1.0,
        //     shininess: 1.0,
        //     reflective: 1.0,
        //     transparency: 0.0,
        //     refractive_index: 1.0,
        //     pattern: solid_pattern(WHITE),
        // },
        floor_material.clone(),
        scale(15, 15.0, 0.1)
            // .rotate_x(PI / 2.0)
            // .rotate_y(PI / 2.0)
            .translate(0.0, 0.0, 6.0),
    );

    let mut block_x_material = Material::default();
    block_x_material.pattern = solid_pattern(color(0.4, 0, 0));
    block_x_material.diffuse = 0.7;
    block_x_material.ambient = 0.7;
    block_x_material.specular = 0.4;
    block_x_material.reflective = 0.01;
    block_x_material.shininess = 200.0;

    let mut block_x = Cube::default();
    block_x.transform(
        scale(0.5, 0.25, 1.6)
            .rotate_y(PI / 2.0)
            .translate(0.1, 0.25, -0.6),
    );
    block_x.material = block_x_material.clone();

    let mut block_z_material = block_x_material.clone();
    block_z_material.pattern = solid_pattern(color(0, 0.4, 0));

    let mut block_z = Cube::default();
    block_z.transform(scale(0.5, 0.25, 1.6).translate(-1.0, 0.25, 0.5));
    block_z.material = block_z_material.clone();

    fn build_row(of: &Cube, direction: RowDirection, level: i32) -> Vec<Box<dyn Object>> {
        let positions = gen_row_positions();
        let mut row: Vec<Box<dyn Object>> = vec![];

        for i in positions {
            let mut new_block = of.clone();
            let x_pos = match direction {
                RowDirection::X => 0.0,
                RowDirection::Z => i as f64 * 1.1,
            };

            let y_pos = level as f64 * 0.5;
            let z_pos = match direction {
                RowDirection::X => i as f64 * 1.1,
                RowDirection::Z => 0.0,
            };

            new_block.transform(translate(x_pos, y_pos, z_pos));
            row.push(Box::new(new_block))
        }

        row
    }

    let mut objects: Vec<Box<dyn Object>> = vec![Box::new(floor)];

    for i in 0..21 {
        match i % 2 == 0 {
            false => {
                let row = &mut build_row(&block_z, RowDirection::Z, i);
                objects.append(row);
            }
            true => {
                let row = &mut build_row(&block_x, RowDirection::X, i);
                objects.append(row)
            }
        };
    }

    let world = World {
        objects,
        light_sources: vec![PointLight::new(point(9.5, 5.0, -14), color(1, 1, 1))],
    };

    let camera = Camera::new(
        2000,
        4000,
        PI / 3.0,
        // view_transform(point(8.5, 5.0, -13), point(0, 2.5, 3), vector(0, 1, 0)),
        view_transform(point(8.5, 1.0, -4), point(0, 5.5, 0), vector(0, 1, 0)),
    );

    let canvas = camera.render(world);
    let ppm = canvas.to_ppm();
    let mut file = File::create("jenga.ppm")?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
