use crate::color::Color;
use crate::intersection::{Computations, Intersections};
use crate::light::Light;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::{Tuple, point};

pub struct World {
    lights: Vec<Light>,
    objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> World {
        World {
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn default() -> World {
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

    pub fn add_object(mut self, floor: Sphere) -> Self {
        self.objects.push(floor);
        self
    }

    pub fn add_light(mut self, light: Light) -> Self {
        self.lights.push(light);
        self
    }

    fn intersect(&self, r: &Ray) -> Intersections<'_> {
        let mut intersections = Vec::new();
        for object in &self.objects {
            let xs = object.intersect(r);
            intersections.extend(xs.xs);
        }
        Intersections::new(intersections)
    }

    fn shade_hit(&self, comps: &Computations) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        for light in &self.lights {
            color = color
                + comps.object.material.lighting(
                    light,
                    &comps.point,
                    &comps.eyev,
                    &comps.normalv,
                    false,
                );
        }
        color
    }

    pub fn color_at(&self, r: &Ray) -> Color {
        let i = self.intersect(r);
        if let Some(hit) = i.hit() {
            let comps = hit.prepare_computations(r);
            self.shade_hit(&comps)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }

    fn is_shadowed(&self, point: &Tuple) -> bool {
        let to_light = self.lights[0].position - *point;
        let ray = Ray::new(*point, to_light.normalize());
        self.intersect(&ray)
            .hit()
            .is_some_and(|hit| hit.t < to_light.magnitude())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intersection::Intersection;
    use crate::tuple::vector;
    use approx::assert_abs_diff_eq;

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

    #[test]
    fn test_intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = w.intersect(&r);
        assert_eq!(xs.xs.len(), 4);
        assert_abs_diff_eq!(xs.xs[0].t, 4.0);
        assert_abs_diff_eq!(xs.xs[1].t, 4.5);
        assert_abs_diff_eq!(xs.xs[2].t, 5.5);
        assert_abs_diff_eq!(xs.xs[3].t, 6.0);
    }

    #[test]
    fn test_shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_abs_diff_eq!(c, Color::new(0.38066, 0.47583, 0.2855), epsilon = 0.00001);
    }

    #[test]
    fn test_shading_an_intersection_from_inside() {
        let mut w = World::default();
        w.lights[0] = Light::new(point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_abs_diff_eq!(c, Color::new(0.90498, 0.90498, 0.90498), epsilon = 0.00001);
    }

    #[test]
    fn test_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r);
        assert_abs_diff_eq!(c, Color::new(0.38066, 0.47583, 0.2855), epsilon = 0.00001);
    }

    #[test]
    fn test_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();

        let mut outer = w.objects[0].clone();
        let mut outer_material = outer.material.clone();
        outer_material.ambient = 1.0;
        outer.set_material(&outer_material);

        let mut inner = w.objects[1].clone();
        let mut inner_material = inner.material.clone();
        inner_material.ambient = 1.0;
        inner.set_material(&inner_material);

        w.objects = vec![outer, inner];

        let r = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let c = w.color_at(&r);
        assert_eq!(c, inner_material.color);
    }

    #[test]
    fn test_no_shadow_when_nothing_collinear_with_point_and_light() {
        let w = World::default();
        let p = point(0.0, 10.0, 0.0);
        assert_eq!(w.is_shadowed(&p), false);
    }

    #[test]
    fn test_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = point(10.0, -10.0, 10.0);
        assert_eq!(w.is_shadowed(&p), true);
    }

    #[test]
    fn test_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = point(-20.0, 20.0, -20.0);
        assert_eq!(w.is_shadowed(&p), false);
    }

    #[test]
    fn test_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = point(-2.0, 2.0, -2.0);
        assert_eq!(w.is_shadowed(&p), false);
    }
}
