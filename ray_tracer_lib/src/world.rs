use crate::color::color;
use crate::intersection::Intersection;
use crate::light::{point_light, PointLight};
use crate::material::material;
use crate::ray::Ray;
use crate::sphere::{sphere, Sphere};
use crate::transformations::scale;
use crate::tuple::point;

pub struct World {
    objects: Vec<Sphere>,
    light_source: Option<PointLight>,
}

impl World {
    pub fn default() -> World {
        let mut inner_sphere = sphere();
        inner_sphere.transform = scale(0.5, 0.5, 0.5);

        let mut outer_sphere = sphere();
        let mut m = material();
        m.set_color(color(0.8, 1.0, 0.6));
        m.diffuse = 0.7;
        m.specular = 0.2;
        outer_sphere.material = m;

        World {
            light_source: Some(point_light(point(-10, 10, -10), color(1, 1, 1))),
            objects: vec![inner_sphere, outer_sphere],
        }
    }

    pub fn intersect(&self, r: Ray) -> Vec<Intersection> {
        let mut xs: Vec<Intersection> = vec![];
        for obj in self.objects.iter() {
           xs.append(&mut obj.intersect(r).unwrap());
        }

        xs.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
        xs
    }
}

pub fn world() -> World {
    World {
        objects: vec![],
        light_source: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::ray;
    use crate::tuple::vector;
    #[test]
    fn create_world() {
        let w = world();

        assert!(w.light_source.is_none());
        assert!(w.objects.len() == 0);
    }

    #[test]
    fn default_world() {
        let w = World::default();

        let mut inner_sphere = sphere();
        inner_sphere.transform = scale(0.5, 0.5, 0.5);

        let mut outer_sphere = sphere();
        let mut m = material();
        m.set_color(color(0.8, 1.0, 0.6));
        m.diffuse = 0.7;
        m.specular = 0.2;
        outer_sphere.material = m;

        assert!(w.light_source.unwrap() == (point_light(point(-10, 10, -10), color(1, 1, 1))));
        assert!(w.objects.len() == 2);
        assert!(w.objects[0] == inner_sphere);
        assert!(w.objects[1] == outer_sphere);
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = ray(point(0, 0, -5), vector(0, 0, 1));

        let xs = w.intersect(r);

        assert!(xs.len() == 4);
        assert!(xs[0].t() == 4.0);
        assert!(xs[1].t() == 4.5);
        assert!(xs[2].t() == 5.5);
        assert!(xs[3].t() == 6.0);
    }
}
