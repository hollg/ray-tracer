use crate::sphere::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &Sphere {
        self.object
    }
}

pub fn intersection<A: Into<f64>>(t: A, object: &Sphere) -> Intersection {
    Intersection { t: t.into(), object }
}

pub trait Hit {
    fn hit(&mut self) -> Option<&Intersection>;
}

impl<'a> Hit for Vec<Intersection<'a>> {
    fn hit(&mut self) -> Option<&Intersection> {
        self.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        self.iter().find(|i| i.t >= 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = sphere();

        let i = intersection(3.5, &s);
        assert!(i.t == 3.5);
        assert!(i.object == &s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = sphere();

        let i1 = intersection(1.0, &s);
        let i2 = intersection(2.0, &s);

        let xs = vec![i1, i2];

        assert!(xs.len() == 2);
        assert!(xs[0] == i1);
        assert!(xs[1] == i2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = sphere();
        let i1 = intersection(1.0, &s);
        let i2 = intersection(2.0, &s);
        let mut xs = vec![i2, i1];

        let i = xs.hit().unwrap();
        assert!(i == &i1);
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-1.0, &s);
        let i2 = intersection(1.0, &s);
        let mut xs = vec![i2, i1];

        let i = xs.hit().unwrap();
        assert!(i == &i2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-2.0, &s);
        let i2 = intersection(-1.0, &s);
        let mut xs = vec![i2, i1];

        let i = xs.hit();
        assert!(i == None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = sphere();
        let i1 = intersection(5.0, &s);
        let i2 = intersection(7.0, &s);
        let i3 = intersection(-3.0, &s);
        let i4 = intersection(2.0, &s);
        let mut xs = vec![i1, i2, i3, i4];

        let i = xs.hit().unwrap();
        assert!(i == &i4);
    }
}
