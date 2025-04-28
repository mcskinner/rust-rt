use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Tuple, intensity: Color) -> Light {
        Light { position, intensity }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::color;
    use crate::tuple::point;

    #[test]
    fn test_creating_a_light() {
        let position = point(0.0, 10.0, -10.0);
        let intensity = color(1.0, 1.0, 1.0);
        let light = Light::new(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}