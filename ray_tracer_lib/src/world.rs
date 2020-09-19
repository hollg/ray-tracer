use crate::color::{color, Color};
use crate::intersection::Hit;
use crate::intersection::{ComputedIntersection, Intersection};
use crate::light::PointLight;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::transformations::scale;
use crate::tuple::point;

pub struct World {
    pub objects: Vec<Sphere>,
    pub light_source: Option<PointLight>,
}

impl World {
    pub fn new<T: Into<Option<PointLight>>>(objects: Vec<Sphere>, light_source: T) -> World {
        World {
            objects,
            light_source: light_source.into(),
        }
    }

    pub fn default() -> World {
        let mut inner_sphere = Sphere::default();
        inner_sphere.transform = scale(0.5, 0.5, 0.5);

        let mut outer_sphere = Sphere::default();
        let mut m = Material::default();
        m.color = color(0.8, 1.0, 0.6);
        m.diffuse = 0.7;
        m.specular = 0.2;
        outer_sphere.material = m;

        World {
            light_source: Some(PointLight::new(point(-10, 10, -10), color(1, 1, 1))),
            objects: vec![outer_sphere, inner_sphere],
        }
    }

    pub fn color_at(&self, r: Ray) -> Color {
        let mut intersections = self.intersect(r);

        let hit_option = intersections.hit();

        match hit_option {
            Some(hit) => {
                let comps = hit.prepare(r);
                self.shade_hit(comps)
            }
            None => color(0, 0, 0),
        }
    }

    fn intersect(&self, r: Ray) -> Vec<Intersection> {
        let mut xs: Vec<Intersection> = vec![];
        for obj in self.objects.iter() {
            xs.append(&mut obj.intersect(r).unwrap());
        }

        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    fn shade_hit(&self, comps: ComputedIntersection) -> Color {
        comps.object.material.lighting(
            self.light_source.unwrap(),
            comps.point,
            comps.eye_v,
            comps.normal_v,
        )
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
    use crate::intersection::intersection;
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

        let mut inner_sphere = Sphere::default();
        inner_sphere.transform = scale(0.5, 0.5, 0.5);

        let mut outer_sphere = Sphere::default();
        let mut m = Material::default();
        m.color = color(0.8, 1.0, 0.6);
        m.diffuse = 0.7;
        m.specular = 0.2;
        outer_sphere.material = m;

        assert!(w.light_source.unwrap() == (point_light(point(-10, 10, -10), color(1, 1, 1))));
        assert!(w.objects.len() == 2);
        assert!(w.objects[0] == outer_sphere);
        assert!(w.objects[1] == inner_sphere);
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

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let shape = &w.objects[0];

        let i = intersection(4, shape);
        let comps = i.prepare(r);
        let c = w.shade_hit(comps);
        assert!(c == color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.light_source = Some(point_light(point(0, 0.25, 0), color(1, 1, 1)));
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let shape = &w.objects[1];

        let i = intersection(0.5, shape);
        let comps = i.prepare(r);
        let c = w.shade_hit(comps);
        assert!(c == color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = ray(point(0, 0, -5), vector(0, 1, 0));

        let c = w.color_at(r);
        assert!(c == color(0, 0, 0));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = ray(point(0, 0, -5), vector(0, 0, 1));

        let c = w.color_at(r);
        assert!(c == color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();

        let mut outer = w.objects.remove(0);
        let mut inner = w.objects.remove(0);

        outer.material.ambient = 1.0;
        inner.material.ambient = 1.0;

        w.objects = vec![outer, inner];
        let r = ray(point(0, 0, 0.75), vector(0, 0, -1));
        let c = w.color_at(r);
        dbg!(c);
        dbg!(inner.material.color);
        assert!(c == inner.material.color);
    }
}
