use ray_tracer_lib::*;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;

    let wall_size = 7.0;
    let canvas_pixels = 100;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color(1.0, 0.0, 0.0);
    let shape = sphere();

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = point(world_x, world_y, wall_z);
            let r = ray(ray_origin, (position - ray_origin).normalize());
            println!("ray: {:?}", r);
            let mut xs = shape.intersect(r).unwrap();

            println!("xs: {:?}", xs);
            if let Some(hit) = xs.hit() {
                println!("{}", hit.t());
                canvas.write_pixel(x, y, color)
            } else {
                println!("no hit!")
            }
        }
    }

    let ppm = canvas.to_ppm();
    let mut file = File::create("shadow.ppm")?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
