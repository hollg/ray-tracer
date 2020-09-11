use crate::ray::*;
use crate::tuple::*;
pub struct Sphere;

impl Sphere {
    pub fn intersects(&self, r: Ray) -> Vec<f64> {
        // the vector from the sphere's center, to the ray origin
        // remember: the sphere is centered at the world origin
        let sphere_to_ray = r.origin() - point(0.0, 0.0, 0.0);

        let a = r.direction().dot(r.direction());
        let b = 2.0 * r.direction().dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        } else {
            let t1 = (-b - (discriminant).sqrt()) / (2.0 * a);
            let t2 = (-b + (discriminant).sqrt()) / (2.0 * a);
            return vec![t1, t2];
        }
    }
}

fn sphere() -> Sphere {
    Sphere {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = s.intersects(r);
        assert!(xs.len() == 2);
        assert!(xs[0] == 4.0);
        assert!(xs[1] == 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = s.intersects(r);
        assert!(xs.len() == 2);
        assert!(xs[0] == 5.0);
        assert!(xs[1] == 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = ray(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = s.intersects(r);
        assert!(xs.len() == 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = s.intersects(r);

        assert!(xs.len() == 2);
        assert!(xs[0] == -1.0);
        assert!(xs[1] == 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = s.intersects(r);
        assert!(xs.len() == 2);
        assert!(xs[0] == -6.0);
        assert!(xs[1] == -4.0);
    }
}
