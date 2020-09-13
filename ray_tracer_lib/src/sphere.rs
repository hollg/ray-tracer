use crate::intersection::*;
use crate::ray::*;
use crate::tuple::*;

#[derive(PartialEq)]
pub struct Sphere;

impl Sphere {
    pub fn intersects(&self, r: Ray) -> Vec<Intersection> {
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
            let t1 = intersection((-b - (discriminant).sqrt()) / (2.0 * a), self);
            let t2 = intersection((-b + (discriminant).sqrt()) / (2.0 * a), self);
            return vec![t1, t2];
        }
    }
}

pub fn sphere() -> Sphere {
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
        assert!(xs[0] == intersection(4.0, &s));
        assert!(xs[1] == intersection(6.0, &s));
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = s.intersects(r);
        assert!(xs.len() == 2);
        assert!(xs[0] == intersection(5.0, &s));
        assert!(xs[1] == intersection(5.0, &s));
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
        assert!(xs[0] == intersection(-1.0, &s));
        assert!(xs[1] == intersection(1.0, &s));
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = s.intersects(r);
        assert!(xs.len() == 2);
        assert!(xs[0] == intersection(-6.0, &s));
        assert!(xs[1] == intersection(-4.0, &s));
    }
}
