use crate::color::{color, Color};
use crate::intersection::Hit;
use crate::intersection::{ComputedIntersection, Intersection};
use crate::light::PointLight;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::transformations::scale;
use crate::tuple::{point, Tuple};

pub struct World {
    pub objects: Vec<Sphere>,
    pub light_sources: Vec<PointLight>,
}

impl World {
    pub fn new(objects: Vec<Sphere>, light_sources: Vec<PointLight>) -> World {
        World {
            objects,
            light_sources,
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
            light_sources: vec![PointLight::new(point(-10, 10, -10), color(1, 1, 1))],
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
        self.light_sources
            .iter()
            .fold(color(0, 0, 0), |color, light_source| {
                color
                    + comps.object.material.lighting(
                        light_source,
                        comps.over_point,
                        comps.eye_v,
                        comps.normal_v,
                        self.is_shadowed(comps.over_point, light_source),
                    )
            })
    }

    fn is_shadowed(&self, point: Tuple, light_source: &PointLight) -> bool {
        let v = light_source.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(point, direction);

        let mut intersections = self.intersect(r);

        let h = intersections.hit();

        match h {
            Some(hit) => hit.t < distance,
            _ => false,
        }
    }
}

pub fn world() -> World {
    World {
        objects: vec![],
        light_sources: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intersection::intersection;
    use crate::ray::ray;
    use crate::transformations::translate;
    use crate::tuple::vector;
    #[test]
    fn create_world() {
        let w = world();

        assert!(w.light_sources.is_empty());
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

        assert!(w.light_sources[0] == PointLight::new(point(-10, 10, -10), color(1, 1, 1)));
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
        assert!(xs[0].t == 4.0);
        assert!(xs[1].t == 4.5);
        assert!(xs[2].t == 5.5);
        assert!(xs[3].t == 6.0);
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
        w.light_sources = vec![PointLight::new(point(0, 0.25, 0), color(1, 1, 1))];
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
        assert!(c == inner.material.color);
    }

    #[test]
    fn no_shadow_when_nothing_is_colinear_with_point_and_light() {
        let w = World::default();
        let p = point(0, 10, 0);

        assert!(w.is_shadowed(p, &w.light_sources[0]) == false);
    }

    #[test]
    fn shadow_when_object_is_between_point_and_light() {
        let w = World::default();
        let p = point(10, -10, 10);

        assert!(w.is_shadowed(p, &w.light_sources[0]) == true);
    }

    #[test]
    fn no_shadow_when_object_is_behind_light() {
        let w = World::default();
        let p = point(-20, 20, -20);

        assert!(w.is_shadowed(p, &w.light_sources[0]) == false);
    }

    #[test]
    fn no_shadow_when_object_is_behind_point() {
        let w = World::default();
        let p = point(-2, 2, -2);

        assert!(w.is_shadowed(p, &w.light_sources[0]) == false);
    }

    #[test]
    fn shade_hit_given_an_intersection_in_shadow() {
        let s1 = Sphere::default();
        let mut s2 = Sphere::default();
        s2.transform = translate(0, 0, 10);
        let w = World::new(
            vec![s1, s2],
            vec![PointLight::new(point(0, 0, -10), color(1, 1, 1))],
        );
        let r = ray(point(0, 0, 5), vector(0, 0, 1));
        let i = intersection(4, &s2);

        let comps = i.prepare(r);

        assert!(w.shade_hit(comps) == color(0.1, 0.1, 0.1));
    }
}