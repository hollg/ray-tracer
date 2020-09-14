use crate::matrix::*;
use crate::tuple::*;
#[derive(Debug)]
pub struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Tuple {
        self.origin
    }

    pub fn direction(&self) -> Tuple {
        self.direction
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transform(&self, m: Matrix) -> Ray {
        Ray {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}

pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
    Ray::new(origin, direction)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::transformations::*;

    #[test]
    fn create_and_query_ray() {
        let origin = point(1, 2, 3);
        let direction = vector(4, 5, 6);

        let r = ray(origin, direction);

        assert!(r.origin == origin);
        assert!(r.direction == direction);
    }

    #[test]
    fn compute_point_from_distance() {
        let r = ray(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));

        assert!(r.position(0.0) == point(2.0, 3.0, 4.0));
        assert!(r.position(-1.0) == point(1.0, 3.0, 4.0));
        assert!(r.position(2.5) == point(4.5, 3.0, 4.0));
    }

    #[test]
    fn translate_a_ray() {
        let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = translate(3.0, 4.0, 5.0);

        let r2 = r.transform(m);

        assert!(r2.origin() == point(4.0, 6.0, 8.0));
        assert!(r2.direction() == vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scale_a_ray() {
        let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = scale(2.0, 3.0, 4.0);

        let r2 = r.transform(m);

        assert!(r2.origin() == point(2.0, 6.0, 12.0));
        assert!(r2.direction() == vector(0.0, 3.0, 0.0));
    }
}
