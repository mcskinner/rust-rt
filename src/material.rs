use approx_derive::AbsDiffEq;

use crate::color::Color;
use crate::light::Light;
use crate::tuple::Tuple;

#[derive(Debug, Clone, PartialEq, AbsDiffEq)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: Color::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn with_rgb(mut self, r: f64, g: f64, b: f64) -> Self {
        self.color = Color::new(r, g, b);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(mut self, specular: f64) -> Self {
        self.specular = specular;
        self
    }

    pub fn with_shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn lighting(
        &self,
        light: &Light,
        position: &Tuple,
        eyev: &Tuple,
        normalv: &Tuple,
        in_shadow: bool,
    ) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - *position).normalize();
        let ambient = effective_color * self.ambient;

        let light_dot_normal = lightv.dot(normalv);
        if in_shadow || light_dot_normal <= 0.0 {
            return ambient;
        }

        let diffuse = effective_color * self.diffuse * light_dot_normal;

        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);
        if reflect_dot_eye <= 0.0 {
            return ambient + diffuse;
        }

        let factor = reflect_dot_eye.powf(self.shininess);
        let specular = light.intensity * self.specular * factor;
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::{point, vector};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_the_default_material() {
        let m = Material::new();
        assert_eq!(m.color, Color::WHITE);
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn test_lighting_with_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Tuple::ORIGIN;
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0), Color::WHITE);
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn test_lighting_with_eye_between_light_and_surface_eye_offset_45deg() {
        let m = Material::new();
        let position = Tuple::ORIGIN;
        let eyev = vector(
            0.0,
            std::f64::consts::FRAC_1_SQRT_2,
            -std::f64::consts::FRAC_1_SQRT_2,
        );
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0), Color::WHITE);
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::WHITE);
    }

    #[test]
    fn test_lighting_with_eye_opposite_surface_light_offset_45deg() {
        let m = Material::new();
        let position = Tuple::ORIGIN;
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 10.0, -10.0), Color::WHITE);
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_abs_diff_eq!(
            result,
            Color::new(0.7364, 0.7364, 0.7364),
            epsilon = 0.00001
        );
    }

    #[test]
    fn test_lighting_with_eye_in_the_path_of_reflection_vector() {
        let m = Material::new();
        let position = Tuple::ORIGIN;
        let eyev = vector(
            0.0,
            -std::f64::consts::FRAC_1_SQRT_2,
            -std::f64::consts::FRAC_1_SQRT_2,
        );
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 10.0, -10.0), Color::WHITE);
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_abs_diff_eq!(
            result,
            Color::new(1.6364, 1.6364, 1.6364),
            epsilon = 0.00001
        );
    }

    #[test]
    fn test_lighting_with_the_light_behind_the_surface() {
        let m = Material::new();
        let position = Tuple::ORIGIN;
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, 10.0), Color::WHITE);
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_lighting_with_the_surface_in_shadow() {
        let m = Material::new();
        let position = Tuple::ORIGIN;
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0), Color::WHITE);
        let result = m.lighting(&light, &position, &eyev, &normalv, true);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
