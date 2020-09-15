use ray_tracer_lib::*;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let ray_origin = point(0, 0, -5);
    let wall_z = 10.0;

    let wall_size = 7.0;
    let canvas_pixels = 300;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut m = material();
    m.set_color(color(1, 0.2, 1));
    let mut shape = sphere();
    shape.set_material(m);

    let light_position = point(-10, 10, -10);
    let light_color = color(1, 1, 1);
    let light = point_light(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = point(world_x, world_y, wall_z);
            let r = ray(ray_origin, (position - ray_origin).normalize());

            let mut xs = shape.intersect(r).unwrap();

            if let Some(hit) = xs.hit() {
                let p = r.position(hit.t());
                let normal = hit.object().normal_at(p);
                let eye = -r.direction();

                let color = hit.object().material.lighting(light, p, eye, normal);
                canvas.write_pixel(x, y, color)
            }
        }
    }

    let ppm = canvas.to_ppm();
    let mut file = File::create("sphere.ppm")?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
