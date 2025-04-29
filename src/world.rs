use crate::color::Color;
use crate::light::Light;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::sphere::Sphere;
use crate::tuple::point;

struct World {
    lights: Vec<Light>,
    objects: Vec<Sphere>,
}

impl World {
    fn new() -> World {
        World {
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }

    fn default() -> World {
        let light = Light::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut s1 = Sphere::new();
        let mut m = Material::new();
        m.color = Color::new(0.8, 1.0, 0.6);
        m.diffuse = 0.7;
        m.specular = 0.2;
        s1.set_material(&m);

        let mut s2 = Sphere::new();
        s2.set_transform(&Matrix::scaling(0.5, 0.5, 0.5));

        World {
            lights: vec![light],
            objects: vec![s1, s2],
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn test_creating_a_world() {
        let w = World::new();
        assert_eq!(w.lights.len(), 0);
        assert_eq!(w.objects.len(), 0);
    }

    #[test]
    fn test_the_default_world() {
        let w = World::default();
        let light = Light::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut s1 = Sphere::new();
        let mut m = Material::new();
        m.color = Color::new(0.8, 1.0, 0.6);
        m.diffuse = 0.7;
        m.specular = 0.2;
        s1.set_material(&m);

        let mut s2 = Sphere::new();
        s2.set_transform(&Matrix::scaling(0.5, 0.5, 0.5));

        assert_eq!(w.lights.len(), 1);
        assert_eq!(w.lights[0], light);

        assert_eq!(w.objects.len(), 2);
        assert_abs_diff_eq!(w.objects[0], s1);
        assert_abs_diff_eq!(w.objects[1], s2);
    }
}
