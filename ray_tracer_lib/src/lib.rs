mod canvas;
mod color;
pub mod consts;
mod intersection;
mod matrix;
mod ray;
mod sphere;
mod transformations;
mod tuple;

pub use canvas::*;
pub use color::*;
pub use intersection::*;
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
