use crate::consts::EPSILON;
use crate::intersection::{intersection, Intersection};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::{vector, Tuple};
use uuid::Uuid;

#[derive(PartialEq)]
pub struct Plane {
    pub material: Material,
    pub transform: Matrix,
    id: Uuid
}

impl Plane {
    pub fn default() -> Plane {
        Plane {
            material: Material::default(),
            transform: Matrix::identity(),
            id: Uuid::new_v4()
        }
    }
}

impl Object for Plane {
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
        if f64::abs(ray2.direction.y) < EPSILON {
            return Ok(vec![]);
        }

        let t = -ray2.origin.y / ray2.direction.y;
        Ok(vec![intersection(t, self)])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::ray;
    use crate::tuple::point;

    #[test]
    fn normal_of_plane_is_constant() {
        let p = Plane::default();

        let n1 = p.normal_at(point(0, 0, 0));
        let n2 = p.normal_at(point(10, 0, -10));
        let n3 = p.normal_at(point(-5, 0, 150));

        assert!(n1 == vector(0, 1, 0));
        assert!(n2 == vector(0, 1, 0));
        assert!(n3 == vector(0, 1, 0));
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane() {
        let p = Plane::default();
        let r = ray(point(0, 10, 0), vector(0, 0, 1));
        let xs = p.intersect(r).unwrap();

        assert!(xs.len() == 0);
    }

    #[test]
    fn ray_intersects_plane_from_above() {
        let p = Plane::default();
        let r = ray(point(0, 1, 0), vector(0, -1, 0));

        let xs = p.intersect(r).unwrap();
        assert!(xs.len() == 1);
        assert!(xs[0].t == 1.0)
    }

    #[test]
    fn ray_intersects_plane_from_below() {
        let p = Plane::default();
        let r = ray(point(0, -1, 0), vector(0, 1, 0));

        let xs = p.intersect(r).unwrap();
        assert!(xs.len() == 1);
        assert!(xs[0].t == 1.0)
    }
}
