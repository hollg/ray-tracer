use crate::tuple::*;

pub struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }
}

pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
    Ray::new(origin, direction)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_query_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);

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
}
