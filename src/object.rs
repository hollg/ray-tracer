use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;
use uuid::Uuid;

pub trait Object {
    fn intersect(&self, ray: Ray) -> Result<Vec<Intersection>, ()>;
    fn normal_at(&self, p: Tuple) -> Tuple;
    fn transformation(&self) -> Matrix;
    fn transform_mut(&mut self) -> &mut Matrix;
    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;
    fn id(&self) -> Uuid;
}

impl PartialEq for &dyn Object {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}