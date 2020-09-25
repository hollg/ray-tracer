use crate::color::Color;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::tuple::Tuple;
#[derive(Clone)]
pub enum Kind {
    Test,
    Checkers(Color, Color),
    Gradient(Color, Color),
    Rings(Color, Color),
    Stripe(Color, Color),
}

impl Kind {
    pub fn color_at(&self, point: Tuple) -> Color {
        match self {
            Kind::Test => Color(point.x, point.y, point.z),
            Kind::Checkers(a, b) => {
                match (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0 == 0.0 {
                    true => *a,
                    false => *b,
                }
            }
            Kind::Stripe(a, b) => match point.x.floor() % 2.0 == 0.0 {
                true => *a,
                false => *b,
            },
            Kind::Gradient(a, b) => {
                let distance = *b - *a;
                let fraction = point.x - point.x.floor();
                *a + distance * fraction
            }
            Kind::Rings(a, b) => {
                match (point.x * point.x + point.z * point.z).sqrt().floor() % 2.0 == 0.0 {
                    true => *a,
                    false => *b,
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Pattern {
    pub kind: Kind,
    pub transform: Matrix,
}

impl Pattern {
    pub fn color_at(&self, point: Tuple) -> Color {
        self.kind.color_at(point)
    }

    pub fn color_at_object(&self, object: &dyn Object, world_point: Tuple) -> Result<Color, ()> {
        let object_point = object.transform().inverse()? * world_point;
        let pattern_point = self.transform.inverse()? * object_point;

        Ok(self.kind.color_at(pattern_point))
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        match (&self.kind, &other.kind) {
            (Kind::Stripe(a1, a2), Kind::Stripe(b1, b2)) => a1 == b1 && a2 == b2,
            (Kind::Gradient(a1, a2), Kind::Gradient(b1, b2)) => a1 == b1 && a2 == b2,
            _ => false,
        }
    }
}

pub fn stripe_pattern<T: Into<Option<Matrix>>>(
    color_a: Color,
    color_b: Color,
    transform: T,
) -> Pattern {
    let m = match transform.into() {
        Some(matrix) => matrix,
        None => Matrix::identity(),
    };
    Pattern {
        kind: Kind::Stripe(color_a, color_b),
        transform: m,
    }
}

pub fn gradient_pattern<T: Into<Option<Matrix>>>(
    color_a: Color,
    color_b: Color,
    transform: T,
) -> Pattern {
    let m = match transform.into() {
        Some(matrix) => matrix,
        None => Matrix::identity(),
    };
    Pattern {
        kind: Kind::Gradient(color_a, color_b),
        transform: m,
    }
}

pub fn ring_pattern<T: Into<Option<Matrix>>>(
    color_a: Color,
    color_b: Color,
    transform: T,
) -> Pattern {
    let m = match transform.into() {
        Some(matrix) => matrix,
        None => Matrix::identity(),
    };
    Pattern {
        kind: Kind::Rings(color_a, color_b),
        transform: m,
    }
}

pub fn checkers_pattern<T: Into<Option<Matrix>>>(
    color_a: Color,
    color_b: Color,
    transform: T,
) -> Pattern {
    let m = match transform.into() {
        Some(matrix) => matrix,
        None => Matrix::identity(),
    };
    Pattern {
        kind: Kind::Checkers(color_a, color_b),
        transform: m,
    }
}

pub fn test_pattern<T: Into<Option<Matrix>>>(transform: T) -> Pattern {
    let m = match transform.into() {
        Some(matrix) => matrix,
        None => Matrix::identity(),
    };
    Pattern {
        kind: Kind::Test,
        transform: m,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{color, BLACK, WHITE};
    use crate::sphere::Sphere;
    use crate::transformations::{scale, translate};
    use crate::tuple::point;
    // #[test]
    // fn creating_a_stripe_pattern() {
    //     let pattern = stripe_pattern(WHITE, BLACK);
    //     assert!(pattern.kind. .0 == WHITE);
    //     assert!(pattern.kind.1 == BLACK);
    // }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = stripe_pattern(WHITE, BLACK, None);

        assert!(pattern.color_at(point(0, 0, 0)) == WHITE);
        assert!(pattern.color_at(point(0, 1, 0)) == WHITE);
        assert!(pattern.color_at(point(0, 2, 0)) == WHITE);
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = stripe_pattern(WHITE, BLACK, None);

        assert!(pattern.color_at(point(0, 0, 0)) == WHITE);
        assert!(pattern.color_at(point(0, 0, 1)) == WHITE);
        assert!(pattern.color_at(point(0, 0, 2)) == WHITE);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = stripe_pattern(WHITE, BLACK, None);

        assert!(pattern.color_at(point(0, 0, 0)) == WHITE);
        assert!(pattern.color_at(point(0.9, 0, 0)) == WHITE);
        assert!(pattern.color_at(point(1, 0, 0)) == BLACK);
        assert!(pattern.color_at(point(-0.1, 0, 0)) == BLACK);
        assert!(pattern.color_at(point(-1, 0, 0)) == BLACK);
        assert!(pattern.color_at(point(-1.1, 0, 0)) == WHITE);
    }

    #[test]
    fn stripes_with_object_transformation() {
        let mut object = Sphere::default();
        object.transform = scale(2, 2, 2);
        let pattern = stripe_pattern(WHITE, BLACK, None);
        let c = pattern.color_at_object(&object, point(1.5, 0, 0)).unwrap();

        assert!(c == WHITE);
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let object = Sphere::default();
        let pattern = stripe_pattern(WHITE, BLACK, scale(2, 2, 2));
        let c = pattern.color_at_object(&object, point(1.5, 0, 0)).unwrap();
        assert!(c == WHITE);
    }

    #[test]
    fn stripes_with_both_pattern_and_object_transformation() {
        let mut object = Sphere::default();
        object.transform = scale(2, 2, 2);
        object.material.pattern = Some(stripe_pattern(WHITE, BLACK, translate(0.5, 0, 0)));
        let c = object
            .material()
            .pattern()
            .as_ref()
            .unwrap()
            .color_at_object(&object, point(2.5, 0, 0));
        assert!(c.unwrap() == WHITE);
    }

    #[test]
    fn grandient_linearly_interpolates_two_colors() {
        let pattern = gradient_pattern(WHITE, BLACK, None);

        assert!(pattern.color_at(point(0, 0, 0)) == WHITE);
        assert!(pattern.color_at(point(0.25, 0, 0)) == color(0.75, 0.75, 0.75));
        assert!(pattern.color_at(point(0.5, 0, 0)) == color(0.5, 0.5, 0.5));
        assert!(pattern.color_at(point(0.75, 0, 0)) == color(0.25, 0.25, 0.25));
    }
    #[test]
    fn ring_extends_in_both_x_and_z() {
        let pattern = ring_pattern(WHITE, BLACK, None);

        assert!(pattern.color_at(point(0, 0, 0)) == WHITE);
        assert!(pattern.color_at(point(1, 0, 0)) == BLACK);
        assert!(pattern.color_at(point(0, 0, 1)) == BLACK);
        assert!(pattern.color_at(point(0.708, 0, 0.708)) == BLACK);
    }
    #[test]
    fn checkers_extend_in_x() {
        let pattern = checkers_pattern(WHITE, BLACK, None);

        assert!(pattern.color_at(point(0, 0, 0)) == WHITE);
        assert!(pattern.color_at(point(0.99, 0, 0)) == WHITE);
        assert!(pattern.color_at(point(1.01, 0, 0)) == BLACK);
    }

    #[test]
    fn checkers_extend_in_y() {
        let pattern = checkers_pattern(WHITE, BLACK, None);

        assert!(pattern.color_at(point(0, 0, 0)) == WHITE);
        assert!(pattern.color_at(point(0, 0.99, 0)) == WHITE);
        assert!(pattern.color_at(point(0, 1.01, 0)) == BLACK);
    }

    #[test]
    fn checkers_extend_in_z() {
        let pattern = checkers_pattern(WHITE, BLACK, None);

        assert!(pattern.color_at(point(0, 0, 0)) == WHITE);
        assert!(pattern.color_at(point(0, 0, 0.99)) == WHITE);
        assert!(pattern.color_at(point(0, 0, 1.01)) == BLACK);
    }
}
