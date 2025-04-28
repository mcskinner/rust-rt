use crate::intersection::Intersection;
use crate::matrix::Matrix;
use crate::tuple::{point, vector, Tuple};
use crate::ray::Ray;

#[derive(Debug)]
pub struct Sphere {
    transform: Matrix,
    inverse_transform: Matrix,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix::identity(4),
            inverse_transform: Matrix::identity(4),
        }
    }

    pub fn set_transform(&mut self, m: &Matrix) {
        self.transform = m.clone();
        self.inverse_transform = m.inverse();
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let ray_t = ray.transform(&self.inverse_transform);
        let sphere_to_ray = ray_t.origin - point(0.0, 0.0, 0.0);

        let a = ray_t.direction.dot(&ray_t.direction);
        let b = 2.0 * ray_t.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        } else if discriminant == 0.0 {
            let t = -b / (2.0 * a);
            return vec![Intersection::new(t, self)];
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            return vec![Intersection::new(t1, self), Intersection::new(t2, self)];
        }
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        world_point - point(0.0, 0.0, 0.0)
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn test_ray_intersects_a_sphere_at_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 5.0);
    }

    #[test]
    fn test_ray_misses_a_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_ray_originates_inside_a_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn test_sphere_behind_a_ray() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn test_intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].object, &s);
    }

    #[test]
    fn test_a_spheres_default_transformation() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix::identity(4));
    }

    #[test]
    fn test_changing_a_spheres_transformation() {
        let mut s = Sphere::new();
        let m = Matrix::translation(2.0, 3.0, 4.0);
        s.set_transform(&m);
        assert_eq!(s.transform, m);
    }

    #[test]
    fn test_intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(&Matrix::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn test_intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(&Matrix::translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_normal_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(point(1.0, 0.0, 0.0));
        assert_eq!(n, vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_normal_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(point(0.0, 1.0, 0.0));
        assert_eq!(n, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_normal_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(point(0.0, 0.0, 1.0));
        assert_eq!(n, vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_normal_at_a_nonaxial_point() {
        let s = Sphere::new();
        let sqrt_3_over_3 = (3.0_f64).sqrt() / 3.0;
        let n = s.normal_at(point(sqrt_3_over_3, sqrt_3_over_3, sqrt_3_over_3));
        assert_eq!(n, vector(sqrt_3_over_3, sqrt_3_over_3, sqrt_3_over_3));
    }

    #[test]
    fn test_normal_is_normalized() {
        let s = Sphere::new();
        let sqrt_3_over_3 = (3.0_f64).sqrt() / 3.0;
        let n = s.normal_at(point(sqrt_3_over_3, sqrt_3_over_3, sqrt_3_over_3));
        assert_eq!(n.normalize(), n);
    }
}