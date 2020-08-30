use ray_tracer_lib::*;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Projectile {
    pos: Tuple,
    velocity: Tuple,
}
#[derive(Copy, Clone, Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(environment: Environment, projectile: Projectile) -> Projectile {
    Projectile {
        pos: projectile.pos + projectile.velocity,
        velocity: projectile.velocity + environment.gravity + environment.wind,
    }
}

fn main() -> std::io::Result<()> {
    let mut canvas = Canvas::new(900, 550);
    // let mut canvas = Canvas::new(100, 100);

    let mut p = Projectile {
        pos: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    while (canvas.height() as i32 - p.pos.y().round() as i32) >= 0
        && (canvas.height() as i32 - p.pos.y().round() as i32) <= canvas.height() as i32
    {
        // println!("pos: {:?}", p.position);

        canvas.write_pixel(
            p.pos.x().round() as usize,
            (canvas.height() as i32 - p.pos.y().round() as i32) as usize,
            Color(1.0, 0.0, 0.0),
        );
        p = tick(e, p);
    }

    let ppm = canvas.to_ppm();
    // println!("{:?}", ppm);
    let mut file = File::create("trajectory.ppm")?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
