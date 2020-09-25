use crate::consts::EPSILON;
use crate::intersection::{intersection, Intersection};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::{vector, Tuple};
use uuid::Uuid;

#[macro_use]
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}
#[macro_use]
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}
pub struct Cube {
    pub material: Material,
    pub transform: Matrix,
    id: Uuid,
}

impl Cube {
    pub fn default() -> Cube {
        Cube {
            material: Material::default(),
            transform: Matrix::identity(),
            id: Uuid::new_v4(),
        }
    }

    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let t_min_numerator = -1.0 - origin;
        let t_max_numerator = 1.0 - origin;

        let t_min;
        let t_max;
        if f64::abs(direction) >= EPSILON {
            t_min = t_min_numerator / direction;
            t_max = t_max_numerator / direction;
        } else {
            t_min = t_min_numerator * std::f64::INFINITY;
            t_max = t_max_numerator * std::f64::INFINITY;
        };

        if t_min > t_max {
            (t_max, t_min)
        } else {
            (t_min, t_max)
        }
    }
}

impl Object for Cube {
    fn id(&self) -> Uuid {
        self.id
    }

    fn normal_at(&self, p: Tuple) -> Tuple {
        let max_c = max!(f64::abs(p.x), f64::abs(p.y), f64::abs(p.z));

        if max_c == p.x.abs() {
            return vector(p.x, 0, 0);
        } else if max_c == p.y.abs() {
            vector(0, p.y, 0)
        } else {
            vector(0, 0, p.z)
        }
    }

    fn transform(&self) -> Matrix {
        self.transform
    }

    fn transform_mut(&mut self) -> &mut Matrix {
        &mut self.transform
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn intersect(&self, ray: Ray) -> Result<Vec<Intersection>, ()> {
        let ray2 = ray.transform(self.transform().inverse()?);

        let (x_t_min, x_t_max) = Self::check_axis(ray2.origin.x, ray2.direction.x);
        let (y_t_min, y_t_max) = Self::check_axis(ray2.origin.y, ray2.direction.y);
        let (z_t_min, z_t_max) = Self::check_axis(ray2.origin.z, ray2.direction.z);

        let t_min = max!(x_t_min, y_t_min, z_t_min);
        let t_max = min!(x_t_max, y_t_max, z_t_max);

        return match t_min > t_max {
            true => Ok(vec![]),
            false => Ok(vec![intersection(t_min, self), intersection(t_max, self)]),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::ray;
    use crate::tuple::point;
    #[test]
    fn ray_intersects_cube() {
        let expected = [
            (point(5, 0.5, 0), vector(-1, 0, 0), 4.0, 6.0),
            (point(-5, 0.5, 0), vector(1, 0, 0), 4.0, 6.0),
            (point(0.5, 5, 0), vector(0, -1, 0), 4.0, 6.0),
            (point(0.5, -5, 0), vector(0, 1, 0), 4.0, 6.0),
            (point(0.5, 0, 5), vector(0, 0, -1), 4.0, 6.0),
            (point(0.5, 0, 5), vector(0, 0, -1), 4.0, 6.0),
            (point(0.5, 0, -5), vector(0, 0, 1), 4.0, 6.0),
            (point(0, 0.5, 0), vector(0, 0, 1), -1.0, 1.0),
        ];

        for (p, v, t1, t2) in expected.iter() {
            let c = Cube::default();
            let r = ray(*p, *v);
            let xs = c.intersect(r).unwrap();

            assert!(xs.len() == 2);
            assert!(xs[0].t == *t1);
            assert!(xs[1].t == *t2);
        }
    }

    #[test]
    fn ray_misses_cube() {
        let expected = [
            ((point(-2, 0, 0), vector(0.2673, 0.5345, 0.8018))),
            ((point(0, -2, 0), vector(0.8018, 0.2673, 0.5345))),
            ((point(0, 0, -2), vector(0.5345, 0.8018, 0.2673))),
            ((point(2, 0, 2), vector(0, 0, -1))),
            ((point(0, 2, 2), vector(0, -1, 0))),
            ((point(2, 2, 0), vector(-1, 0, 0))),
        ];

        for (p, v) in expected.iter() {
            let cube = Cube::default();
            let r = ray(*p, *v);
            let xs = cube.intersect(r).unwrap();

            assert!(xs.len() == 0);
        }
    }

    #[test]
    fn normal_of_surface_of_cube() {
        let expected = [
            (point(1, 0.5, -0.8), vector(1, 0, 0)),
            (point(-1, -0.2, 0.9), vector(-1, 0, 0)),
            (point(-0.4, 1, -0.1), vector(0, 1, 0)),
            (point(0.3, -1, -0.7), vector(0, -1, 0)),
            (point(-0.6, 0.3, 1), vector(0, 0, 1)),
            (point(0.4, 0.4, -1), vector(0, 0, -1)),
            (point(1, 1, 1), vector(1, 0, 0)),
            (point(-1, -1, -1), vector(-1, 0, 0)),
        ];

        for (p, n) in expected.iter() {
            let cube = Cube::default();
            assert!(cube.normal_at(*p) == *n);
        }
    }
}
