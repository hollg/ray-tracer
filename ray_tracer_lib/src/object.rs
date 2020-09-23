use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;
pub trait Object {
    fn intersect(&self, ray: Ray) -> Result<Vec<Intersection>, ()>;
    fn normal_at(&self, p: Tuple) -> Tuple;
    fn transform(&self) -> Matrix;
    fn transform_mut(&mut self) -> &mut Matrix;
    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;
}

// pub trait ObjectClone {
//     fn clone_box(&self) -> Box<dyn Object>;
// }

// impl<T> ObjectClone for T
// where
//     T: 'static + Object + Clone,
// {
//     fn clone_box(&self) -> Box<dyn Object> {
//         Box::new(self.clone())
//     }
// }

// impl Clone for Box<dyn Object> {
//     fn clone(&self) -> Box<dyn Object> {
//         self.clone_box()
//     }
// }
