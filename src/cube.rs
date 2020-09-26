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

    fn normal_at(&self, _p: Tuple) -> Tuple {
        vector(0, 1, 0)
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
        let (y_t_min, y_t_max) = Self::check_axis(ray2.origin.x, ray2.direction.x);
        let (z_t_min, z_t_max) = Self::check_axis(ray2.origin.x, ray2.direction.x);

        let t_min = max!(x_t_min, y_t_min, z_t_min);
        let t_max = min!(x_t_max, y_t_max, z_t_max);

        Ok(vec![intersection(t_min, self), intersection(t_max, self)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::ray;
    use crate::tuple::point;
    use std::collections::HashMap;
    #[test]
    fn ray_intersects_cube() {
        let table: HashMap<i32, (Tuple, Tuple, f64, f64)> = [
            (0, (point(0, 0.5, 0), vector(-1, 0, 0), 4.0, 6.0)),
            (1, (point(-5, 0.5, 0), vector(1, 0, 0), 4.0, 6.0)),
            (2, (point(0.5, 5, 0), vector(0, -1, 0), 4.0, 6.0)),
            (3, (point(0.5, -5, 0), vector(0, 1, 0), 4.0, 6.0)),
            (4, (point(0.5, 0, 5), vector(0, 0, -1), 4.0, 6.0)),
            (5, (point(0.5, 0, -5), vector(0, 0, -1), 4.0, 6.0)),
            (6, (point(0.5, 0, -5), vector(0, 0, 1), 4.0, 6.0)),
            (7, (point(0, 0.5, 0), vector(0, 0, 1), -1.0, 1.0)),
        ]
        .iter()
        .cloned()
        .collect();
        for i in 0..7 {
            let values = table.get(&i).unwrap();
            let c = Cube::default();
            let r = ray(values.0, values.1);
            let xs = c.intersect(r).unwrap();

            assert!(xs.len() == 2);
            assert!(xs[0].t == values.2);
            assert!(xs[1].t == values.3);
        }
    }
}
