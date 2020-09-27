use crate::intersection::*;
use crate::material::Material;
use crate::matrix::*;
use crate::object::Object;
use crate::ray::*;
use crate::tuple::*;
use uuid::Uuid;

#[derive(PartialEq)]
pub struct Sphere {
    pub transform: Matrix,
    pub material: Material,
    id: Uuid,
}

impl Sphere {
    pub fn default() -> Sphere {
        Sphere {
            transform: Matrix::identity(),
            material: Material::default(),
            id: Uuid::new_v4(),
        }
    }
}

impl Object for Sphere {
    fn id(&self) -> Uuid {
        self.id
    }

    fn material(&self) -> &Material {
        &self.material
    }
    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn transformation(&self) -> Matrix {
        self.transform
    }

    fn transform_mut(&mut self) -> &mut Matrix {
        &mut self.transform
    }

    fn intersect(&self, ray: Ray) -> Result<Vec<Intersection>, ()> {
        // the vector from the sphere's center, to the ray origin
        // remember: the sphere is centered at the world origin
        let matrix = self.transform.inverse();

        match matrix {
            Ok(matrix) => {
                let ray2 = ray.transform(matrix);
                let sphere_to_ray = ray2.origin - point(0.0, 0.0, 0.0);

                let a = ray2.direction.dot(ray2.direction);
                let b = 2.0 * ray2.direction.dot(sphere_to_ray);
                let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

                let discriminant = b.powi(2) - 4.0 * a * c;

                if discriminant < 0.0 {
                    Ok(vec![])
                } else {
                    let t1 = intersection((-b - (discriminant).sqrt()) / (2.0 * a), self);
                    let t2 = intersection((-b + (discriminant).sqrt()) / (2.0 * a), self);
                    Ok(vec![t1, t2])
                }
            }
            _ => Err(()),
        }
    }

    fn normal_at(&self, p: Tuple) -> Tuple {
        let object_point = self.transform.inverse().unwrap() * p;
        let object_normal = object_point - point(0, 0, 0);
        let world_normal_t = self.transform.inverse().unwrap().transpose() * object_normal;
        let world_normal = Tuple {
            x: world_normal_t.x,
            y: world_normal_t.y,
            z: world_normal_t.z,
            w: 0.0,
        };
        world_normal.normalize()
    }
}

pub fn sphere(transform: Matrix, material: Material) -> Sphere {
    Sphere {
        transform,
        material,
        id: Uuid::new_v4()
    }
}

pub fn glass_sphere() -> Sphere {
    let mut s = Sphere::default();
    s.material.refractive_index = 1.5;
    s.material.transparency = 1.0;

    s
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::transformations::*;
    use std::f64::consts::PI;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r).unwrap();
        assert!(xs.len() == 2);
        assert!(xs[0] == intersection(4.0, &s));
        assert!(xs[1] == intersection(6.0, &s));
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r).unwrap();
        assert!(xs.len() == 2);
        assert!(xs[0] == intersection(5.0, &s));
        assert!(xs[1] == intersection(5.0, &s));
    }

    #[test]
    fn ray_misses_sphere() {
        let r = ray(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r).unwrap();
        assert!(xs.len() == 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r).unwrap();

        assert!(xs.len() == 2);
        assert!(xs[0] == intersection(-1.0, &s));
        assert!(xs[1] == intersection(1.0, &s));
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r).unwrap();
        assert!(xs.len() == 2);
        assert!(xs[0] == intersection(-6.0, &s));
        assert!(xs[1] == intersection(-4.0, &s));
    }

    #[test]
    fn intsersect_sets_the_object_on_the_intersection() {
        let r = ray(point(0.0, 0.0, -0.5), vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r).unwrap();
        assert!(xs.len() == 2);
        assert!(xs[0].object.material() == &s.material);
        assert!(xs[0].object.transformation() == s.transform);
        assert!(xs[1].object.material() == &s.material);
        assert!(xs[1].object.transformation() == s.transform);
    }

    #[test]
    fn sphere_default_transformation() {
        let s = Sphere::default();
        assert!(s.transform == Matrix::identity());
    }

    #[test]
    fn changing_sphere_transformation() {
        let mut s = Sphere::default();
        let t = translate(2.0, 3.0, 4.0);

        s.transform = t;

        assert!(s.transform == t)
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::default();

        s.transform = scale(2.0, 2.0, 2.0);
        let xs = s.intersect(r).unwrap();

        assert!(xs.len() == 2);
        assert!(xs[0].t == 3.0);
        assert!(xs[1].t == 7.0);
    }

    #[test]
    fn intersect_translated_sphere_with_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::default();

        s.transform = translate(5.0, 0.0, 0.0);
        let xs = s.intersect(r).unwrap();

        assert!(xs.len() == 0);
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = Sphere::default();

        let n = s.normal_at(point(1, 0, 0));

        assert!(n == vector(1, 0, 0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let s = Sphere::default();

        let n = s.normal_at(point(0, 1, 0));

        assert!(n == vector(0, 1, 0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let s = Sphere::default();

        let n = s.normal_at(point(0, 0, 1));

        assert!(n == vector(0, 0, 1));
    }

    #[test]
    fn normal_on_sphere_at_a_nonaxial_point() {
        let s = Sphere::default();

        let root_3 = f64::sqrt(3.0);
        let n = s.normal_at(point(root_3 / 3.0, root_3 / 3.0, root_3 / 3.0));

        assert!(n == vector(root_3 / 3.0, root_3 / 3.0, root_3 / 3.0));
    }

    #[test]
    fn normal_is_normalised_vector() {
        let s = Sphere::default();
        let root_3 = f64::sqrt(3.0);
        let n = s.normal_at(point(root_3 / 3.0, root_3 / 3.0, root_3 / 3.0));

        assert!(n == n.normalize());
    }

    #[test]
    fn compute_normal_on_translated_sphere() {
        let mut s = Sphere::default();
        s.transform = translate(0, 1, 0);

        let n = s.normal_at(point(0, 1.70711, -0.70711));
        assert!(n == vector(0, 0.70711, -0.70711));
    }

    #[test]
    fn compute_normal_on_transformed_sphere() {
        let mut s = Sphere::default();
        let m = scale(1, 0.5, 1) * rotate_z(PI / 5.0);
        s.transform = m;

        let root_2 = PI.sqrt();
        let n = s.normal_at(point(0, root_2 / 2.0, -root_2 / 2.0));
        assert!(n == vector(0, 0.97014, -0.24254));
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::default();
        assert!(s.material == Material::default());
    }

    #[test]
    fn sphere_may_be_assigned_material() {
        let mut s = Sphere::default();
        let mut m = Material::default();
        m.ambient = 1.0;
        s.material = m;

        assert!(s.material.ambient == 1.0);
    }

    #[test]
    fn helper_for_glass_sphere() {
        let s = glass_sphere();

        assert!(s.material.refractive_index == 1.5);
        assert!(s.material.transparency == 1.0);
    }
}
