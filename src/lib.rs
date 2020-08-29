mod canvas;
mod color;
pub mod consts;
mod tuple;

pub use canvas::*;
pub use color::*;
pub use tuple::*;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
