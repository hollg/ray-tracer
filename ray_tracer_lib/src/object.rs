use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;
pub trait Object {
    fn intersect(&self, ray: Ray) -> Result<Vec<Intersection>, ()>;
    fn normal_at(&self, p: Tuple) -> Tuple;
    fn transform(&self) -> Matrix;
    fn material(&self) -> Material;
}

