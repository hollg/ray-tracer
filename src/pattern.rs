use crate::color::Color;
use crate::matrix::Matrix;
use crate::tuple::Tuple;
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Template {
    Test,
    Solid(Color),
    Checkers(Color, Color),
    Gradient(Color, Color),
    Rings(Color, Color),
    Stripe(Color, Color),
}

impl Template {
    pub fn color_at(&self, point: Tuple) -> Color {
        match self {
            Template::Solid(c) => *c,
            Template::Test => Color(point.x, point.y, point.z),
            Template::Checkers(a, b) => {
                match (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0 == 0.0 {
                    true => *a,
                    false => *b,
                }
            }
            Template::Stripe(a, b) => match point.x.floor() % 2.0 == 0.0 {
                true => *a,
                false => *b,
            },
            Template::Gradient(a, b) => {
                let distance = *b - *a;
                let fraction = point.x - point.x.floor();
                *a + distance * fraction
            }
            Template::Rings(a, b) => {
                match (point.x * point.x + point.z * point.z).sqrt().floor() % 2.0 == 0.0 {
                    true => *a,
                    false => *b,
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pattern {
    template: Template,
    transform: Matrix,
    inverse: Matrix,
}

impl Pattern {
    pub fn color_at(&self, point: Tuple) -> Color {
        self.template.color_at(point)
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
        template: Template::Stripe(color_a, color_b),
        transform: m,
        inverse: m.inverse().unwrap(),
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
        template: Template::Gradient(color_a, color_b),
        transform: m,
        inverse: m.inverse().unwrap(),
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
        template: Template::Rings(color_a, color_b),
        transform: m,
        inverse: m.inverse().unwrap(),
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
        template: Template::Checkers(color_a, color_b),
        transform: m,
        inverse: m.inverse().unwrap(),
    }
}

pub fn solid_pattern(c: Color) -> Pattern {
    Pattern {
        template: Template::Solid(c),
        transform: Matrix::identity(),
        inverse: Matrix::identity(),
    }
}

pub fn test_pattern<T: Into<Option<Matrix>>>(transform: T) -> Pattern {
    let m = match transform.into() {
        Some(matrix) => matrix,
        None => Matrix::identity(),
    };
    Pattern {
        template: Template::Test,
        transform: m,
        inverse: m.inverse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{color, BLACK, WHITE};
    use crate::object::Object;
    use crate::shapes::Sphere;
    use crate::transformations::{scale, translate};
    use crate::tuple::point;

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
        object.transform(scale(2, 2, 2));
        let pattern = stripe_pattern(WHITE, BLACK, None);

        let object_point = object.inverse() * point(1.5, 0, 0);
        let pattern_point = pattern.inverse * object_point;

        let c = pattern.color_at(pattern_point);

        assert!(c == WHITE);
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let object = Sphere::default();
        let pattern = stripe_pattern(WHITE, BLACK, scale(2, 2, 2));
        let object_point = object.inverse() * point(1.5, 0, 0);
        let pattern_point = pattern.inverse * object_point;
        let c = pattern.color_at(pattern_point);
        assert!(c == WHITE);
    }

    #[test]
    fn stripes_with_both_pattern_and_object_transformation() {
        let mut object = Sphere::default();
        object.transform(scale(2, 2, 2));
        object.material.pattern = stripe_pattern(WHITE, BLACK, translate(0.5, 0, 0));
        let object_point = object.inverse() * point(2.5, 0, 0);
        let pattern_point = object.material().pattern().inverse * object_point;

        let c = object.material().pattern().color_at(pattern_point);
        assert!(c == WHITE);
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

    #[test]
    fn solid_is_solid() {
        let pattern = solid_pattern(BLACK);

        assert!(pattern.color_at(point(0, 0, 0)) == BLACK);
        assert!(pattern.color_at(point(9, 1, 10)) == BLACK);
    }
}
