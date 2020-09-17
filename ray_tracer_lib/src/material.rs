use crate::color::{color, Color};
use crate::light::PointLight;
use crate::tuple::Tuple;
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn default() -> Material {
        Material {
            color: color(1, 1, 1),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn set_color(&mut self, c: Color) {
        self.color = c;
    }

    pub fn set_ambient<T: Into<f64>>(&mut self, a: T) {
        self.ambient = a.into();
    }

    pub fn set_diffuse<T: Into<f64>>(&mut self, d: T) {
        self.diffuse = d.into();
    }

    pub fn set_specular<T: Into<f64>>(&mut self, s: T) {
        self.specular = s.into();
    }

    pub fn set_shininess<T: Into<f64>>(&mut self, s: T) {
        self.shininess = s.into();
    }

    pub fn lighting(
        &self,
        light: PointLight,
        point: Tuple,
        eye_v: Tuple,
        normal_v: Tuple,
    ) -> Color {
        let effective_color = self.color * light.intensity;
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
        ambient + diffuse + specular
    }
}

pub fn material(
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
) -> Material {
    Material {
        ambient,
        color,
        diffuse,
        specular,
        shininess,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::light::point_light;
    use crate::tuple::{point, vector};

    #[test]
    fn default_material() {
        let m = Material::default();
        assert!(m.color == color(1, 1, 1));
        assert!(m.ambient == 0.1);
        assert!(m.diffuse == 0.9);
        assert!(m.specular == 0.9);
        assert!(m.shininess == 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::default();
        let p = point(0, 0, 0);

        let eye_v = vector(0, 0, -1);
        let normal_v = vector(0, 0, -1);
        let light = point_light(point(0, 0, -10), color(1, 1, 1));

        let result = m.lighting(light, p, eye_v, normal_v);
        assert!(result == color(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_degs() {
        let m = Material::default();
        let p = point(0, 0, 0);

        let root_2 = f64::sqrt(2.0);
        let eye_v = vector(0, root_2 / 2.0, root_2 / 2.0);
        let normal_v = vector(0, 0, -1);
        let light = point_light(point(0, 0, -10), color(1, 1, 1));

        let result = m.lighting(light, p, eye_v, normal_v);
        assert!(result == color(1, 1, 1));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_light_offset_45_degs() {
        let m = Material::default();
        let p = point(0, 0, 0);

        let eye_v = vector(0, 0, -1);
        let normal_v = vector(0, 0, -1);
        let light = point_light(point(0, 10, -10), color(1, 1, 1));

        let result = m.lighting(light, p, eye_v, normal_v);
        assert!(result == color(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_reflection_vector() {
        let m = Material::default();
        let p = point(0, 0, 0);

        let root_2 = f64::sqrt(2.0);
        let eye_v = vector(0, -root_2 / 2.0, -root_2 / 2.0);
        let normal_v = vector(0, 0, -1);
        let light = point_light(point(0, 10, -10), color(1, 1, 1));

        let result = m.lighting(light, p, eye_v, normal_v);
        assert!(result == color(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = Material::default();
        let p = point(0, 0, 0);

        let eye_v = vector(0, 0, -1);
        let normal_v = vector(0, 0, -1);
        let light = point_light(point(0, 0, 10), color(1, 1, 1));

        let result = m.lighting(light, p, eye_v, normal_v);
        assert!(result == color(0.1, 0.1, 0.1));
    }
}
