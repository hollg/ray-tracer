mod canvas;
mod color;
pub mod consts;
mod intersection;
mod light;
mod material;
mod matrix;
mod ray;
mod sphere;
mod transformations;
mod tuple;
mod world;

pub use world::*;
pub use canvas::*;
pub use color::*;
pub use intersection::*;
pub use light::*;
pub use material::*;
pub use matrix::*;
pub use ray::*;
pub use sphere::*;
pub use transformations::*;
pub use tuple::*;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
