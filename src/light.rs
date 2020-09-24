use crate::color::*;
use crate::tuple::*;
#[derive(Copy, Clone, PartialEq)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Tuple,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = color(1, 1, 1);
        let position = point(0, 0, 0);

        let light = PointLight::new(position, intensity);

        assert!(light.position == position);
        assert!(light.intensity == intensity);
    }
}
