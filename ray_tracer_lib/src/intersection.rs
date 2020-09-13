use crate::sphere::*;

#[derive(PartialEq, Copy, Clone)]
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

pub fn intersection(t: f64, object: &Sphere) -> Intersection {
    Intersection { t, object }
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
}
