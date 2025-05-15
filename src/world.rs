use crate::color::Color;
use crate::intersection::{Computations, Intersections};
use crate::light::Light;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::Tuple;

pub struct World {
    lights: Vec<Light>,
    objects: Vec<Shape>,
}

impl World {
    pub fn new() -> World {
        World {
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn add_object(mut self, object: Shape) -> Self {
        self.objects.push(object);
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
        let mut color = Color::BLACK;
        for light in &self.lights {
            color = color
                + comps.object.material.lighting(
                    comps.object,
                    light,
                    &comps.point,
                    &comps.eyev,
                    &comps.normalv,
                    self.is_shadowed(&comps.point, &light),
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
            Color::BLACK
        }
    }

    fn is_shadowed(&self, point: &Tuple, light: &Light) -> bool {
        let to_light = light.position - *point;
        let ray = Ray::new(*point, to_light.normalize());
        self.intersect(&ray)
            .hit()
            .is_some_and(|hit| hit.t < to_light.magnitude())
    }

    fn reflected_color(&self, comps: &Computations<'_>) -> Color {
        if comps.object.material.reflective > 0.0 {
            let r = Ray::new(comps.point, comps.reflectv);
            self.color_at(&r) * comps.object.material.reflective
        } else {
            Color::BLACK
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, SQRT_2};

    use super::*;
    use crate::intersection::Intersection;
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::plane::Plane;
    use crate::sphere::Sphere;
    use crate::tuple::{point, vector};
    use approx::assert_abs_diff_eq;

    fn default_world() -> World {
        let light = Light::new(point(-10.0, 10.0, -10.0), Color::WHITE);

        let m = Material::new()
            .with_rgb(0.8, 1.0, 0.6)
            .with_diffuse(0.7)
            .with_specular(0.2);

        let mut s1: Shape = Sphere::new().into();
        s1.set_material(&m);

        let mut s2: Shape = Sphere::new().into();
        s2.set_transform(&Matrix::scaling(0.5, 0.5, 0.5));

        World {
            lights: vec![light],
            objects: vec![s1, s2],
        }
    }

    #[test]
    fn test_creating_a_world() {
        let w = World::new();
        assert_eq!(w.lights.len(), 0);
        assert_eq!(w.objects.len(), 0);
    }

    #[test]
    fn test_the_default_world() {
        let w = default_world();
        let light = Light::new(point(-10.0, 10.0, -10.0), Color::WHITE);

        let m = Material::new()
            .with_rgb(0.8, 1.0, 0.6)
            .with_diffuse(0.7)
            .with_specular(0.2);
        let mut s1: Shape = Sphere::new().into();
        s1.set_material(&m);

        let mut s2: Shape = Sphere::new().into();
        s2.set_transform(&Matrix::scaling(0.5, 0.5, 0.5));

        assert_eq!(w.lights.len(), 1);
        assert_eq!(w.lights[0], light);

        assert_eq!(w.objects.len(), 2);
        assert_eq!(w.objects[0], s1);
        assert_eq!(w.objects[1], s2);
    }

    #[test]
    fn test_intersect_a_world_with_a_ray() {
        let w = default_world();
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
        let w = default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_abs_diff_eq!(c, Color::new(0.38066, 0.47583, 0.2855), epsilon = 0.00001);
    }

    #[test]
    fn test_shading_an_intersection_from_inside() {
        let mut w = default_world();
        w.lights[0] = Light::new(point(0.0, 0.25, 0.0), Color::WHITE);
        let r = Ray::new(Tuple::ORIGIN, vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_abs_diff_eq!(c, Color::new(0.90498, 0.90498, 0.90498), epsilon = 0.00001);
    }

    #[test]
    fn test_color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::BLACK);
    }

    #[test]
    fn test_color_when_a_ray_hits() {
        let w = default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r);
        assert_abs_diff_eq!(c, Color::new(0.38066, 0.47583, 0.2855), epsilon = 0.00001);
    }

    #[test]
    fn test_color_with_an_intersection_behind_the_ray() {
        let mut w = default_world();

        let mut outer = w.objects[0].clone();
        let outer_material = outer.material.clone().with_ambient(1.0);
        outer.set_material(&outer_material);

        let mut inner = w.objects[1].clone();
        let inner_color = Color::new(1.0, 0.6, 0.2);
        let inner_material = inner
            .material
            .clone()
            .with_ambient(1.0)
            .with_color(inner_color);
        inner.set_material(&inner_material);

        w.objects = vec![outer, inner];

        let r = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let c = w.color_at(&r);
        assert_eq!(c, inner_color);
    }

    #[test]
    fn test_no_shadow_when_nothing_collinear_with_point_and_light() {
        let w = default_world();
        let p = point(0.0, 10.0, 0.0);
        assert_eq!(w.is_shadowed(&p, &w.lights[0]), false);
    }

    #[test]
    fn test_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = default_world();
        let p = point(10.0, -10.0, 10.0);
        assert_eq!(w.is_shadowed(&p, &w.lights[0]), true);
    }

    #[test]
    fn test_no_shadow_when_an_object_is_behind_the_light() {
        let w = default_world();
        let p = point(-20.0, 20.0, -20.0);
        assert_eq!(w.is_shadowed(&p, &w.lights[0]), false);
    }

    #[test]
    fn test_no_shadow_when_an_object_is_behind_the_point() {
        let w = default_world();
        let p = point(-2.0, 2.0, -2.0);
        assert_eq!(w.is_shadowed(&p, &w.lights[0]), false);
    }

    #[test]
    fn test_shade_hit_is_given_an_intersection_in_shadow() {
        let s1: Shape = Sphere::new().into();
        let mut s2: Shape = Sphere::new().into();
        s2.set_transform(&Matrix::translation(0.0, 0.0, 10.0));
        let w = World::new()
            .add_light(Light::new(point(0.0, 0.0, -10.0), Color::WHITE))
            .add_object(s1)
            .add_object(s2);
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, &w.objects[1]);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_abs_diff_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_reflected_color_for_a_nonreflective_material() {
        let mut w = default_world();
        let mut shape = w.objects[1].clone();
        shape.set_material(&shape.material.clone().with_ambient(1.0));
        w.objects[1] = shape;

        let r = Ray::new(Tuple::ORIGIN, vector(0.0, 0.0, 1.0));
        let i = Intersection::new(1.0, &w.objects[1]);
        let comps = i.prepare_computations(&r);
        let c = w.reflected_color(&comps);
        assert_eq!(c, Color::BLACK);
    }

    #[test]
    fn test_reflected_color_for_a_reflective_material() {
        let mut shape: Shape = Plane::new().into();
        shape.set_material(&shape.material.clone().with_reflective(0.5));
        shape.set_transform(&Matrix::translation(0.0, -1.0, 0.0));
        let w = default_world().add_object(shape);

        let r = Ray::new(
            point(0.0, 0.0, -3.0),
            vector(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        );
        let i = Intersection::new(SQRT_2, &w.objects[2]);
        let comps = i.prepare_computations(&r);
        let c = w.reflected_color(&comps);
        assert_abs_diff_eq!(c, Color::new(0.19033, 0.23791, 0.14275), epsilon = 0.00001);
    }
}
