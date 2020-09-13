use crate::intersection::*;
use crate::matrix::*;
use crate::ray::*;
use crate::transformations::*;
use crate::tuple::*;

#[derive(PartialEq)]
pub struct Sphere {
    pub transform: Matrix,
}

impl Sphere {
    pub fn transform(&self) -> Matrix {
        self.transform
    }

    pub fn set_transform(&mut self, m: Matrix) {
        self.transform = m;
    }

    pub fn intersect(&self, r: Ray) -> Result<Vec<Intersection>, ()> {
        // the vector from the sphere's center, to the ray origin
        // remember: the sphere is centered at the world origin
        let m = self.transform().inverse();

        match m {
            Ok(matrix) => {
                let r2 = r.transform(matrix);
                let sphere_to_ray = r2.origin() - point(0.0, 0.0, 0.0);

                let a = r2.direction().dot(r2.direction());
                let b = 2.0 * r2.direction().dot(sphere_to_ray);
                let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

                let discriminant = b.powi(2) - 4.0 * a * c;

                if discriminant < 0.0 {
                    Ok(vec![])
                } else {
                    let t1 = intersection((-b - (discriminant).sqrt()) / (2.0 * a), self);
                    let t2 = intersection((-b + (discriminant).sqrt()) / (2.0 * a), self);
                    Ok(vec![t1, t2])
                }
            },
            _ => Err(())
        }
        
    }
}

pub fn sphere() -> Sphere {
    Sphere {
        transform: Matrix::identity(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = s.intersect(r).unwrap();
        assert!(xs.len() == 2);
        assert!(xs[0] == intersection(4.0, &s));
        assert!(xs[1] == intersection(6.0, &s));
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = s.intersect(r).unwrap();
        assert!(xs.len() == 2);
        assert!(xs[0] == intersection(5.0, &s));
        assert!(xs[1] == intersection(5.0, &s));
    }

    #[test]
    fn ray_misses_sphere() {
        let r = ray(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = s.intersect(r).unwrap();
        assert!(xs.len() == 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = s.intersect(r).unwrap();

        assert!(xs.len() == 2);
        assert!(xs[0] == intersection(-1.0, &s));
        assert!(xs[1] == intersection(1.0, &s));
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = s.intersect(r).unwrap();
        assert!(xs.len() == 2);
        assert!(xs[0] == intersection(-6.0, &s));
        assert!(xs[1] == intersection(-4.0, &s));
    }

    #[test]
    fn intsersect_sets_the_object_on_the_intersection() {
        let r = ray(point(0.0, 0.0, -0.5), vector(0.0, 0.0, 1.0));
        let s = sphere();

        let xs = s.intersect(r).unwrap();
        assert!(xs.len() == 2);
        assert!(xs[0].object() == &s);
        assert!(xs[1].object() == &s);
    }

    #[test]
    fn sphere_default_transformation() {
        let s = sphere();
        assert!(s.transform == Matrix::identity());
    }

    #[test]
    fn changing_sphere_transformation() {
        let mut s = sphere();
        let t = translate(2.0, 3.0, 4.0);

        s.set_transform(t);

        assert!(s.transform == t)
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = sphere();

        s.set_transform(scale(2.0, 2.0, 2.0));
        let xs = s.intersect(r).unwrap();

        assert!(xs.len() == 2);
        assert!(xs[0].t() == 3.0);
        assert!(xs[1].t() == 7.0);
    }

    #[test]
    fn intersect_translated_sphere_with_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = sphere();

        s.set_transform(translate(5.0, 0.0, 0.0));
        let xs = s.intersect(r).unwrap();

        assert!(xs.len() == 0);
    }
}
