use crate::canvas::{canvas, Canvas};
use crate::matrix::Matrix;
use crate::ray::{ray, Ray};
use crate::tuple::point;
use crate::world::World;
use std::time::Instant;

pub struct Camera {
    h_size: usize,
    v_size: usize,
    transform: Matrix,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new<T: Into<f64>, U: Into<Option<Matrix>>>(
        h_size: usize,
        v_size: usize,
        field_of_view: T,
        transform: U,
    ) -> Camera {
        let fov = field_of_view.into();

        let half_view = (fov / 2.0).tan();
        let aspect = h_size as f64 / v_size as f64;
        let mut half_width = half_view * aspect;
        let mut half_height = half_view;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        }
        let pixel_size = (half_width * 2.0) / h_size as f64;

        Camera {
            h_size,
            v_size,
            pixel_size,
            half_height,
            half_width,
            transform: match transform.into() {
                None => Matrix::identity(),
                Some(t) => t,
            },
        }
    }

    fn ray_for_pixel<A: Into<f64>, B: Into<f64>>(&self, px: A, py: B) -> Ray {
        let pixel_x = px.into();
        let pixel_y = py.into();
        // the offset from the edge of the canvas to the pixel's center
        let x_offset = (pixel_x + 0.5) * self.pixel_size;
        let y_offset = (pixel_y + 0.5) * self.pixel_size;

        //  the untransformed coordinates of the pixel in world space.
        // # (remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;
        // # using the camera matrix, transform the canvas point and the origin, # and then compute the ray's direction vector.
        // # (remember that the canvas is at z=-1)
        let pixel = self.transform.inverse().unwrap() * point(world_x, world_y, -1);
        let origin = self.transform.inverse().unwrap() * point(0, 0, 0);
        let direction = (pixel - origin).normalize();
        ray(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        let time = Instant::now();
        let mut image = canvas(self.h_size, self.v_size);

        for y in 0..self.v_size {
            for x in 0..self.h_size {
                let r = self.ray_for_pixel(x as f64, y as f64);
                let color = world.color_at(r, 5);
                image.write_pixel(x, y, color);
            }
        }
        println!("Renderd in {} seconds", time.elapsed().as_secs());
        image
    }
}

pub fn camera<T: Into<f64>, U: Into<Option<Matrix>>>(
    h_size: usize,
    v_size: usize,
    field_of_view: T,
    transform: U,
) -> Camera {
    Camera::new(h_size, v_size, field_of_view, transform)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::EPSILON;
    use crate::transformations::{rotate_y, translate, view_transform};
    use crate::tuple::vector;
    use std::f64::consts::PI;

    use crate::color::color;
    #[test]
    fn constructing_a_camera() {
        let h_size = 160;
        let v_size = 120;
        let field_of_view = PI / 2.0;

        let c = camera(h_size, v_size, field_of_view, None);
        assert!(c.h_size == 160);
        assert!(c.v_size == 120);
        assert!(c.transform == Matrix::identity());
    }

    #[test]
    fn pixel_size_for_a_horizontal_canvas() {
        let c = camera(200, 125, PI / 2.0, None);

        assert!((c.pixel_size - 0.01).abs() < EPSILON);
    }

    #[test]
    fn pixel_size_for_a_verticalal_canvas() {
        let c = camera(125, 200, PI / 2.0, None);
        assert!((c.pixel_size - 0.01).abs() < EPSILON);
    }

    #[test]
    fn constructing_a_ray_through_the_centre_of_the_canvas() {
        let c = camera(201, 101, PI / 2.0, None);
        let r = c.ray_for_pixel(100.0, 50.0);

        assert!(r.origin == point(0, 0, 0));
        assert!(r.direction == vector(0, 0, -1));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = camera(201, 101, PI / 2.0, None);
        let r = c.ray_for_pixel(0.0, 0.0);

        assert!(r.origin == point(0, 0, 0));
        assert!(r.direction == vector(0.66519, 0.33259, -0.66851));
    }
    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = camera(201, 101, PI / 2.0, None);
        c.transform = rotate_y(PI / 4.0) * translate(0, -2, 5);
        let r = c.ray_for_pixel(100, 50);

        let root_2 = f64::sqrt(2.0);
        assert!(r.origin == point(0, 2, -5));
        assert!(r.direction == vector(root_2 / 2.0, 0, -root_2 / 2.0));
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::default();
        let from = point(0, 0, -5);
        let to = point(0, 0, 0);
        let up = vector(0, 1, 0);
        let c = camera(11, 11, PI / 2.0, view_transform(from, to, up));

        let image = c.render(w);

        assert!(image.get_pixel(5, 5) == &color(0.38066, 0.47583, 0.2855));
    }
}
