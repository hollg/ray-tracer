use crate::color::{color, Color};
use crate::consts::EPSILON;
use crate::light::PointLight;
use crate::object::Object;
use crate::pattern::{solid_pattern, Pattern};
use crate::tuple::Tuple;

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    pub pattern: Pattern,
}

impl Material {
    pub fn new(
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        reflective: f64,
        transparency: f64,
        refractive_index: f64,
        pattern: Pattern,
    ) -> Material {
        Material {
            ambient,
            diffuse,
            specular,
            shininess,
            reflective,
            transparency,
            refractive_index,
            pattern: pattern,
        }
    }

    pub fn default() -> Material {
        Material::new(
            0.1,
            0.9,
            0.9,
            200.0,
            0.0,
            0.0,
            1.0,
            solid_pattern(color(1, 1, 1)),
        )
    }

    // TODO: don't calculate specular and diffuse if in shadow
    pub fn lighting(
        &self,
        object: &dyn Object,
        light: &PointLight,
        point: Tuple,
        eye_v: Tuple,
        normal_v: Tuple,
        in_shadow: bool,
    ) -> Color {
        let object_point = object.inverse() * point;
        let start_color = self.pattern.color_at_object(object_point);

        let effective_color = start_color * light.intensity;
        let light_v = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = light_v.dot(normal_v);

        let diffuse: Color;
        let specular: Color;

        if light_dot_normal < 0.0 {
            diffuse = Color::default();
            specular = Color::default();
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflect_v = -light_v.reflect(normal_v);
            let reflect_dot_eye = reflect_v.dot(eye_v);

            if reflect_dot_eye <= 0.0 {
                specular = Color::default();
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        if in_shadow {
            ambient
        } else {
            ambient + diffuse + specular
        }
    }

    pub fn pattern(&self) -> Pattern {
        self.pattern
    }
}

pub fn material<T: Into<Option<Pattern>>>(
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    transparency: f64,
    refractive_index: f64,
    pattern: Pattern,
) -> Material {
    Material::new(
        ambient,
        diffuse,
        specular,
        shininess,
        reflective,
        transparency,
        refractive_index,
        pattern,
    )
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        f64::abs(self.ambient - other.ambient) < EPSILON
            && f64::abs(self.diffuse - other.diffuse) < EPSILON
            && f64::abs(self.specular - other.specular) < EPSILON
            && f64::abs(self.shininess - other.shininess) < EPSILON
            && self.pattern == other.pattern
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::light::PointLight;
    use crate::pattern::stripe_pattern;
    use crate::shapes::Sphere;
    use crate::tuple::{point, vector};

    #[test]
    fn default_material() {
        let m = Material::default();
        dbg!(&m.pattern);
        dbg!(solid_pattern(color(1, 1, 1)));
        assert!(m.pattern == solid_pattern(color(1, 1, 1)));
        assert!(m.ambient == 0.1);
        assert!(m.diffuse == 0.9);
        assert!(m.specular == 0.9);
        assert!(m.shininess == 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let object = Sphere::default();
        let m = Material::default();
        let p = point(0, 0, 0);

        let eye_v = vector(0, 0, -1);
        let normal_v = vector(0, 0, -1);
        let light = PointLight::new(point(0, 0, -10), color(1, 1, 1));

        let result = m.lighting(&object, &light, p, eye_v, normal_v, false);
        assert!(result == color(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_degs() {
        let object = Sphere::default();
        let m = Material::default();
        let p = point(0, 0, 0);

        let root_2 = f64::sqrt(2.0);
        let eye_v = vector(0, root_2 / 2.0, root_2 / 2.0);
        let normal_v = vector(0, 0, -1);
        let light = PointLight::new(point(0, 0, -10), color(1, 1, 1));

        let result = m.lighting(&object, &light, p, eye_v, normal_v, false);
        assert!(result == color(1, 1, 1));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_light_offset_45_degs() {
        let object = Sphere::default();
        let m = Material::default();
        let p = point(0, 0, 0);

        let eye_v = vector(0, 0, -1);
        let normal_v = vector(0, 0, -1);
        let light = PointLight::new(point(0, 10, -10), color(1, 1, 1));

        let result = m.lighting(&object, &light, p, eye_v, normal_v, false);
        assert!(result == color(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_reflection_vector() {
        let object = Sphere::default();
        let m = Material::default();
        let p = point(0, 0, 0);

        let root_2 = f64::sqrt(2.0);
        let eye_v = vector(0, -root_2 / 2.0, -root_2 / 2.0);
        let normal_v = vector(0, 0, -1);
        let light = PointLight::new(point(0, 10, -10), color(1, 1, 1));

        let result = m.lighting(&object, &light, p, eye_v, normal_v, false);
        assert!(result == color(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let object = Sphere::default();
        let m = Material::default();
        let p = point(0, 0, 0);

        let eye_v = vector(0, 0, -1);
        let normal_v = vector(0, 0, -1);
        let light = PointLight::new(point(0, 0, 10), color(1, 1, 1));

        let result = m.lighting(&object, &light, p, eye_v, normal_v, false);
        assert!(result == color(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let object = Sphere::default();
        let m = Material::default();
        let p = point(0, 0, 0);
        let eye_v = vector(0, 0, -1);
        let normal_v = vector(0, 0, -1);
        let light = PointLight::new(point(0, 0, -10), color(1, 1, 1));
        let in_shadow = true;

        let result = m.lighting(&object, &light, p, eye_v, normal_v, in_shadow);

        assert!(result == color(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let object = Sphere::default();
        let mut m = Material::default();
        m.pattern = stripe_pattern(color(1, 1, 1), color(0, 0, 0), None);
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;

        let eye_v = vector(0, 0, -1);
        let normal_v = vector(0, 0, -1);

        let light = PointLight::new(point(0, 0, -10), color(1, 1, 1));
        let c1 = m.lighting(&object, &light, point(0.9, 0, 0), eye_v, normal_v, false);
        let c2 = m.lighting(&object, &light, point(1.1, 0, 0), eye_v, normal_v, false);

        assert!(c1 == color(1, 1, 1));
        assert!(c2 == color(0, 0, 0));
    }

    #[test]
    fn reflectivity_for_default_material() {
        let m = Material::default();
        assert!(m.reflective == 0.0);
    }

    #[test]
    fn transparency_and_refractive_index_for_default_material() {
        let m = Material::default();

        assert!(m.refractive_index == 1.0);
        assert!(m.transparency == 0.0);
    }
}
