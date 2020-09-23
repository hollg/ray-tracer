use crate::color::Color;
use crate::tuple::Tuple;
pub enum Kind {
    Stripe(Color, Color),
}

impl Kind {
    pub fn color_at(&self, point: Tuple) -> Color {
        match self {
            Kind::Stripe(a, b) => match point.x.floor() % 2.0 == 0.0 {
                true => *a,
                false => *b,
            },
        }
    }
}

pub struct Pattern {
    pub kind: Kind,
}

impl Pattern {
    pub fn color_at(&self, point: Tuple) -> Color {
        self.kind.color_at(point)
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        match (&self.kind, &other.kind) {
            (Kind::Stripe(a1, a2), Kind::Stripe(b1, b2)) => a1 == b1 && a2 == b2,
        }
    }
}

pub fn stripe_pattern(color_a: Color, color_b: Color) -> Pattern {
    Pattern {
        kind: Kind::Stripe(color_a, color_b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{BLACK, WHITE};
    use crate::tuple::point;
    // #[test]
    // fn creating_a_stripe_pattern() {
    //     let pattern = stripe_pattern(WHITE, BLACK);
    //     assert!(pattern.kind. .0 == WHITE);
    //     assert!(pattern.kind.1 == BLACK);
    // }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = stripe_pattern(WHITE, BLACK);

        assert!(pattern.color_at(point(0, 0, 0)) == WHITE);
        assert!(pattern.color_at(point(0, 1, 0)) == WHITE);
        assert!(pattern.color_at(point(0, 2, 0)) == WHITE);
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = stripe_pattern(WHITE, BLACK);

        assert!(pattern.color_at(point(0, 0, 0)) == WHITE);
        assert!(pattern.color_at(point(0, 0, 1)) == WHITE);
        assert!(pattern.color_at(point(0, 0, 2)) == WHITE);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = stripe_pattern(WHITE, BLACK);

        assert!(pattern.color_at(point(0, 0, 0)) == WHITE);
        assert!(pattern.color_at(point(0.9, 0, 0)) == WHITE);
        assert!(pattern.color_at(point(1, 0, 0)) == BLACK);
        assert!(pattern.color_at(point(-0.1, 0, 0)) == BLACK);
        assert!(pattern.color_at(point(-1, 0, 0)) == BLACK);
        assert!(pattern.color_at(point(-1.1, 0, 0)) == WHITE);
    }
}
