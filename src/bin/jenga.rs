use rand::Rng;
use ray_tracer_lib::*;
use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let floor = Plane::new(
        Material {
            color: BLACK,
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.3,
            shininess: 250.0,
            reflective: 0.6,
            transparency: 0.0,
            refractive_index: 0.0,
            pattern: Some(checkers_pattern(WHITE, BLACK, None)),
        },
        Matrix::identity(),
    );

    let mut block_x_material = Material::default();
    block_x_material.color = color(0.4, 0, 0);
    block_x_material.diffuse = 0.5;
    block_x_material.ambient = 0.5;
    block_x_material.specular = 0.2;
    block_x_material.shininess = 90.0;

    let mut block_y_material = block_x_material.clone();
    block_y_material.color = color(0, 0.4, 0);

    let mut block_x = Cube::default();
    block_x.transform = scale(0.5, 0.25, 1.5).translate(-0.5, 0.25, 0.5);
    block_x.material = block_x_material.clone();

    let mut block_y = Cube::default();
    block_y.transform = scale(0.5, 0.25, 1.5)
        .rotate_y(PI / 2.0)
        .translate(0.5, 0.25, -0.5);
    block_y.material = block_y_material.clone();

    fn build_x_row(of: &Cube, level: i32) -> Vec<Box<dyn Object>> {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(1, 5);

        if num <= 2 {
            build_z_row_edges(of, level)
        } else if num == 3 {
            build_z_row_full(of, level)
        } else {
            build_z_row_middle(of, level)
        }
    }

    fn build_x_row_full(of: &Cube, level: i32) -> Vec<Box<dyn Object>> {
        let mut row: Vec<Box<dyn Object>> = vec![];
        for i in 0..3 {
            let mut new_block = of.clone();
            let x_pos = i as f64 * 1.1;
            let y_pos = level as f64 * 0.5;
            new_block.transform = new_block.transform.translate(x_pos, y_pos, 0.0);
            row.push(Box::new(new_block))
        }

        row
    }

    fn build_x_row_middle(of: &Cube, level: i32) -> Vec<Box<dyn Object>> {
        let mut row: Vec<Box<dyn Object>> = vec![];
        for i in 0..3 {
            if i == 1 {
                let mut new_block = of.clone();
                let x_pos = i as f64 * 1.1;
                let y_pos = level as f64 * 0.5;
                new_block.transform = new_block.transform.translate(x_pos, y_pos, 0.0);
                row.push(Box::new(new_block))
            }
        }

        row
    }

    fn build_x_row_edges(of: &Cube, level: i32) -> Vec<Box<dyn Object>> {
        let mut row: Vec<Box<dyn Object>> = vec![];
        for i in 0..3 {
            if i != 1 {
                let mut new_block = of.clone();
                let x_pos = i as f64 * 1.1;
                let y_pos = level as f64 * 0.5;
                new_block.transform = new_block.transform.translate(x_pos, y_pos, 0.0);
                row.push(Box::new(new_block))
            }
        }

        row
    }

    fn build_z_row(of: &Cube, level: i32) -> Vec<Box<dyn Object>> {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(1, 5);

        if num <= 2 {
            build_x_row_edges(of, level)
        } else if num == 3 {
            build_x_row_full(of, level)
        } else {
            build_x_row_middle(of, level)
        }
    }

    fn build_z_row_full(of: &Cube, level: i32) -> Vec<Box<dyn Object>> {
        let mut row: Vec<Box<dyn Object>> = vec![];
        for i in 0..3 {
            let mut new_block = of.clone();
            let z_pos = i as f64 * 1.1;
            let y_pos = level as f64 * 0.5;
            new_block.transform = new_block.transform.translate(0.0, y_pos, z_pos);
            row.push(Box::new(new_block))
        }

        row
    }

    fn build_z_row_edges(of: &Cube, level: i32) -> Vec<Box<dyn Object>> {
        let mut row: Vec<Box<dyn Object>> = vec![];
        for i in 0..3 {
            if i != 1 {
                let mut new_block = of.clone();
                let z_pos = i as f64 * 1.1;
                let y_pos = level as f64 * 0.5;
                new_block.transform = new_block.transform.translate(0.0, y_pos, z_pos);
                row.push(Box::new(new_block))
            }
        }

        row
    }

    fn build_z_row_middle(of: &Cube, level: i32) -> Vec<Box<dyn Object>> {
        let mut row: Vec<Box<dyn Object>> = vec![];
        for i in 0..3 {
            if i == 1 {
                let mut new_block = of.clone();
                let z_pos = i as f64 * 1.1;
                let y_pos = level as f64 * 0.5;
                new_block.transform = new_block.transform.translate(0.0, y_pos, z_pos);
                row.push(Box::new(new_block))
            }
        }

        row
    }

    let mut objects: Vec<Box<dyn Object>> = vec![Box::new(floor)];
    let mut rows = vec![];

    for i in 0..21{
        rows.push(match i % 2 == 0 {
            false => build_z_row(&block_x, i),
            true => build_x_row(&block_y, i),
        });
    }

    for mut row in rows {
        objects.append(&mut row);
    }

    let world = World {
        objects,
        light_sources: vec![PointLight::new(point(0, 5, -10), color(1, 1, 1))],
    };

    let camera = Camera::new(
        250,
        500,
        PI / 3.0,
        view_transform(point(8.5, 1.0, -4), point(0, 5.5, 0), vector(0, 1, 0)),
    );

    let canvas = camera.render(world);
    let ppm = canvas.to_ppm();
    let mut file = File::create("jenga.ppm")?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
