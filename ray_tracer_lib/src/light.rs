use crate::color::*;
use crate::tuple::*;
#[derive(Copy, Clone)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Tuple,
}

pub fn point_light(position: Tuple, intensity: Color) -> PointLight {
    PointLight {
        position,
        intensity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = color(1, 1, 1);
        let position = point(0, 0, 0);

        let light = point_light(position, intensity);

        assert!(light.position == position);
        assert!(light.intensity == intensity);
    }
}
