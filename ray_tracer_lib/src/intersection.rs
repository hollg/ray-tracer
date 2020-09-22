use crate::consts::EPSILON;
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::{point, Tuple};
#[derive(Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Box<dyn Object>,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        f64::abs(self.t - other.t) < EPSILON && self.object.material() == other.object.material()
    }
}

impl Intersection {
    pub fn prepare(&self, r: Ray) -> ComputedIntersection {
        let mut comps = ComputedIntersection {
            object: &self.object,
            t: self.t,
            point: r.position(self.t),
            eye_v: -r.direction,
            normal_v: self.object.normal_at(r.position(self.t)),
            is_inside: false,
            over_point: point(0, 0, 0), // TODO: avoid this temp value
        };

        if comps.normal_v.dot(comps.eye_v) < 0.0 {
            comps.is_inside = true;
            comps.normal_v = -comps.normal_v;
        }

        comps.over_point = comps.point + comps.normal_v * EPSILON;

        comps
    }
}

pub fn intersection<A: Into<f64>>(t: A, object: Box<dyn Object>) -> Intersection {
    Intersection {
        t: t.into(),
        object,
    }
}

pub trait Hit {
    fn hit(&mut self) -> Option<&Intersection>;
}

impl Hit for Vec<&Intersection> {
    fn hit(&mut self) -> Option<&Intersection> {
        self.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        let result = self.iter().find(|i| i.t >= 0.0);

        match result {
            Some(intersection) => Some(*intersection),
            _ => None,
        }
    }
}

pub struct ComputedIntersection<'a> {
    pub object: &'a Box<dyn Object>,
    pub point: Tuple,
    pub eye_v: Tuple,
    pub normal_v: Tuple,
    pub t: f64,
    pub is_inside: bool,
    pub over_point: Tuple,
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::ray;
    use crate::sphere::Sphere;
    use crate::transformations::translate;
    use crate::tuple::{point, vector};
    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::default();

        let i = intersection(3.5, Box::new(s));
        assert!(i.t == 3.5);
        assert!(i.object.material() == s.material);
        assert!(i.object.transform() == s.transform);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::default();

        let i1 = intersection(1.0, Box::new(s));
        let i2 = intersection(2.0, Box::new(s));

        let xs = vec![&i1, &i2];

        assert!(xs.len() == 2);
        assert!(xs[0] == &i1);
        assert!(xs[1] == &i2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = intersection(1.0, Box::new(s));
        let i2 = intersection(2.0, Box::new(s));
        let mut xs = vec![&i2, &i1];

        let i = xs.hit().unwrap();
        assert!(i == &i1);
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = intersection(-1.0, Box::new(s));
        let i2 = intersection(1.0, Box::new(s));
        let mut xs = vec![&i2, &i1];

        let i = xs.hit().unwrap();
        assert!(i == &i2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = intersection(-2.0, Box::new(s));
        let i2 = intersection(-1.0, Box::new(s));
        let mut xs = vec![&i2, &i1];

        let i = xs.hit();
        assert!(i == None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = intersection(5.0, Box::new(s));
        let i2 = intersection(7.0, Box::new(s));
        let i3 = intersection(-3.0, Box::new(s));
        let i4 = intersection(2.0, Box::new(s));
        let mut xs = vec![&i1, &i2, &i3, &i4];

        let i = xs.hit().unwrap();
        assert!(i == &i4);
    }

    #[test]
    fn precompute_state_of_intersection() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let s = Sphere::default();

        let i = intersection(4, Box::new(s));
        let comps = i.prepare(r);

        assert!(comps.t == i.t);
        assert!(comps.object.material() == i.object.material());
        assert!(comps.object.transform() == i.object.transform());
        assert!(comps.point == point(0, 0, -1));
        assert!(comps.eye_v == vector(0, 0, -1));
        assert!(comps.normal_v == vector(0, 0, -1));
    }
    #[test]
    fn the_hit_when_intersection_occurs_on_outside() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let s = Sphere::default();
        let i = intersection(1, Box::new(s));
        let comps = i.prepare(r);

        assert!(!comps.is_inside);
    }

    #[test]
    fn the_hit_when_intersection_occurs_on_inside() {
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let s = Sphere::default();
        let i = intersection(1, Box::new(s));
        let comps = i.prepare(r);

        assert!(comps.point == point(0, 0, 1));
        assert!(comps.eye_v == vector(0, 0, -1));
        assert!(comps.normal_v == vector(0, 0, -1));
        assert!(comps.is_inside);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let mut s = Sphere::default();
        s.transform = translate(0, 0, 1);

        let i = intersection(5, Box::new(s));
        let comps = i.prepare(r);

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
