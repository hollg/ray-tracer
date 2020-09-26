use crate::consts::EPSILON;
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::Tuple;
#[derive(Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a dyn Object,
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        f64::abs(self.t - other.t) < EPSILON && self.object.material() == other.object.material()
    }
}

impl<'a> Intersection<'a> {
    pub fn prepare(&self, r: Ray, xs: &[Intersection]) -> ComputedIntersection {
        let object = self.object;
        let t = self.t;
        let point = r.position(t);
        let eye_v = -r.direction;
        let mut normal_v = self.object.normal_at(r.position(self.t));

        let mut is_inside = false;
        if normal_v.dot(eye_v) < 0.0 {
            is_inside = true;
            normal_v = -normal_v;
        }

        let reflect_v = r.direction.reflect(normal_v);

        let over_point = point + normal_v * EPSILON;
        let under_point = point - normal_v * EPSILON;

        let mut containers: Vec<&dyn Object> = vec![];
        let mut n1 = 1.0;
        let mut n2 = 1.0;
        for x in xs {
            if x == self {
                if containers.is_empty() {
                    n1 = 1.0;
                } else {
                    n1 = containers.last().unwrap().material().refractive_index;
                    //TODO: remove unwrap
                }
            }

            if containers.contains(&x.object) {
                containers.retain(|c| *c != x.object);
            } else {
                containers.push(x.object);
            }

            if x == self {
                if containers.is_empty() {
                    n2 = 1.0;
                } else {
                    n2 = containers.last().unwrap().material().refractive_index;
                    //TODO: remove unwrap
                }
            }
        }

        ComputedIntersection {
            object,
            t,
            point,
            normal_v,
            eye_v,
            is_inside,
            n1,
            n2,
            reflect_v,
            over_point,
            under_point,
        }
    }
}

pub fn intersection<A: Into<f64>>(t: A, object: &dyn Object) -> Intersection {
    Intersection {
        t: t.into(),
        object: object,
    }
}

pub trait Hit {
    fn hit(&mut self) -> Option<&Intersection>;
}

impl<'a> Hit for Vec<&Intersection<'a>> {
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
    pub object: &'a dyn Object,
    pub point: Tuple,
    pub eye_v: Tuple,
    pub normal_v: Tuple,
    pub reflect_v: Tuple,
    pub t: f64,
    pub is_inside: bool,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub n1: f64,
    pub n2: f64,
}

impl<'a> ComputedIntersection<'a> {
    pub fn schlick(&self) -> f64 {
        let mut cos = self.eye_v.dot(self.normal_v);

        // total internal reflection can only occur if n1 > n2
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powf(2.0) * (1.0 - cos.powf(2.0));
            if sin2_t > 1.0 {
                return 1.0;
            } else {
                let cos_t = f64::sqrt(1.0 - sin2_t);
                cos = cos_t;
            }
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::plane::Plane;
    use crate::ray::ray;
    use crate::sphere::{glass_sphere, Sphere};
    use crate::transformations::{scale, translate};
    use crate::tuple::{point, vector};
    use std::collections::HashMap;
    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::default();

        let i = intersection(3.5, &s);
        assert!(i.t == 3.5);
        assert!(i.object.material() == &s.material);
        assert!(i.object.transform() == s.transform);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::default();

        let i1 = intersection(1.0, &s);
        let i2 = intersection(2.0, &s);

        let xs = vec![&i1, &i2];

        assert!(xs.len() == 2);
        assert!(xs[0] == &i1);
        assert!(xs[1] == &i2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = intersection(1.0, &s);
        let i2 = intersection(2.0, &s);
        let mut xs = vec![&i2, &i1];

        let i = xs.hit().unwrap();
        assert!(i == &i1);
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = intersection(-1.0, &s);
        let i2 = intersection(1.0, &s);
        let mut xs = vec![&i2, &i1];

        let i = xs.hit().unwrap();
        assert!(i == &i2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = intersection(-2.0, &s);
        let i2 = intersection(-1.0, &s);
        let mut xs = vec![&i2, &i1];

        let i = xs.hit();
        assert!(i == None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = intersection(5.0, &s);
        let i2 = intersection(7.0, &s);
        let i3 = intersection(-3.0, &s);
        let i4 = intersection(2.0, &s);
        let mut xs = vec![&i1, &i2, &i3, &i4];

        let i = xs.hit().unwrap();
        assert!(i == &i4);
    }

    #[test]
    fn precompute_state_of_intersection() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let s = Sphere::default();

        let i = intersection(4, &s);
        let i2 = i.clone();
        let comps = i.prepare(r, &[i2]);

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
        let i = intersection(1, &s);
        let i2 = i.clone();
        let comps = i.prepare(r, &[i2]);

        assert!(!comps.is_inside);
    }

    #[test]
    fn the_hit_when_intersection_occurs_on_inside() {
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let s = Sphere::default();
        let i = intersection(1, &s);
        let i2 = i.clone();
        let comps = i.prepare(r, &[i2]);

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

        let i = intersection(5, &s);
        let i2 = i.clone();
        let comps = i.prepare(r, &[i2]);

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precompute_reflection_vector() {
        let shape = Plane::default();

        let root_2 = f64::sqrt(2.0);
        let r = ray(point(0, 1, -1), vector(0, -root_2 / 2.0, root_2 / 2.0));
        let i = intersection(root_2, &shape);
        let i2 = i.clone();
        let comps = i.prepare(r, &[i2]);
        assert!(comps.reflect_v == vector(0, root_2 / 2.0, root_2 / 2.0));
    }

    #[test]
    fn find_n1_and_n2_at_various_intersections() {
        let mut a = glass_sphere();
        a.transform = scale(2, 2, 2);

        let mut b = glass_sphere();
        b.material.refractive_index = 2.0;
        b.transform = translate(0, 0, -0.25);

        let mut c = glass_sphere();
        c.transform = translate(0, 0, 0.25);
        c.material.refractive_index = 2.5;

        let r = ray(point(0, 0, -4), vector(0, 0, 1));
        let xs = vec![
            intersection(2.0, &a),
            intersection(2.75, &b),
            intersection(3.25, &c),
            intersection(4.75, &b),
            intersection(5.25, &c),
            intersection(6.0, &a),
        ];

        let expected: HashMap<usize, (f64, f64)> = [
            (0, (1.0, 1.5)),
            (1, (1.5, 2.0)),
            (2, (2.0, 2.5)),
            (3, (2.5, 2.5)),
            (4, (2.5, 1.5)),
            (5, (1.5, 1.0)),
        ]
        .iter()
        .cloned()
        .collect();

        for (i, intersection) in xs.iter().enumerate() {
            let comps = intersection.prepare(r, &xs);
            assert!(comps.n1 == expected.get(&i).unwrap().0);
            assert!(comps.n2 == expected.get(&i).unwrap().1);
        }
    }

    #[test]
    fn under_point_is_offset_below_surface() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let mut shape = glass_sphere();
        shape.transform = translate(0, 0, 1);
        let i = intersection(5, &shape);

        let comps = i.prepare(r, &[i.clone()]);
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn schlick_approximation_with_total_internal_reflection() {
        let shape = glass_sphere();
        let r = ray(point(0, 0, f64::sqrt(2.0) / 2.0), vector(0, 1, 0));
        let xs = vec![
            intersection(-f64::sqrt(2.0) / 2.0, &shape),
            intersection(f64::sqrt(2.0) / 2.0, &shape),
        ];

        let comps = xs[1].prepare(r, &xs);
        let reflectance = comps.schlick();
        assert!(reflectance == 1.0);
    }

    #[test]
    fn schlick_approximation_with_perpendicular_viewing_angle() {
        let shape = glass_sphere();
        let r = ray(point(0, 0, 0), vector(0, 1, 0));
        let xs = vec![intersection(-1.0, &shape), intersection(1, &shape)];

        let comps = xs[0].prepare(r, &xs);
        let reflectance = comps.schlick();
        assert!(f64::abs(reflectance - 0.04) < EPSILON);
    }

    #[test]
    fn schlick_approximation_with_small_angle_and_n2_gt_n1() {
        let shape = glass_sphere();
        let r = ray(point(0, 0.99, -2), vector(0, 0, 1));
        let xs = vec![intersection(1.8589, &shape)];
        let comps = xs[0].prepare(r, &xs);
        let reflectance = comps.schlick();
        assert!(f64::abs(reflectance - 0.48873) < EPSILON);
    }

    
}
