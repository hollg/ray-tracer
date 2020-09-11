mod canvas;
mod color;
pub mod consts;
mod matrix;
mod transformations;
mod tuple;
mod ray;

pub use ray::*;
pub use canvas::*;
pub use color::*;
pub use matrix::*;
pub use transformations::*;
pub use tuple::*;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
