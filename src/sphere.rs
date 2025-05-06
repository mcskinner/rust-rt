use approx_derive::AbsDiffEq;

use crate::ray::Ray;
use crate::shape::{Hittable, HittableTrait};
use crate::tuple::Tuple;

#[derive(Debug, Clone, AbsDiffEq)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {}
    }
}

impl HittableTrait for Sphere {
    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = local_ray.origin - Tuple::ORIGIN;

        let a = local_ray.direction.dot(&local_ray.direction);
        let b = 2.0 * local_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else if discriminant == 0.0 {
            let t = -b / (2.0 * a);
            vec![t]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            vec![t1, t2]
        }
    }

    fn local_normal_at(&self, local_point: &Tuple) -> Tuple {
        *local_point - Tuple::ORIGIN
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
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::shape::Shape;
    use crate::tuple::{point, vector};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s: Shape = Sphere::new().into();
        let xs = s.intersect(&r);
        assert_eq!(xs.xs.len(), 2);
        assert_eq!(xs.xs[0].t, 4.0);
        assert_eq!(xs.xs[1].t, 6.0);
    }

    #[test]
    fn test_ray_intersects_a_sphere_at_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s: Shape = Sphere::new().into();
        let xs = s.intersect(&r);
        assert_eq!(xs.xs.len(), 1);
        assert_eq!(xs.xs[0].t, 5.0);
    }

    #[test]
    fn test_ray_misses_a_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s: Shape = Sphere::new().into();
        let xs = s.intersect(&r);
        assert_eq!(xs.xs.len(), 0);
    }

    #[test]
    fn test_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::ORIGIN, vector(0.0, 0.0, 1.0));
        let s: Shape = Sphere::new().into();
        let xs = s.intersect(&r);
        assert_eq!(xs.xs.len(), 2);
        assert_eq!(xs.xs[0].t, -1.0);
        assert_eq!(xs.xs[1].t, 1.0);
    }

    #[test]
    fn test_sphere_behind_a_ray() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s: Shape = Sphere::new().into();
        let xs = s.intersect(&r);
        assert_eq!(xs.xs.len(), 2);
        assert_eq!(xs.xs[0].t, -6.0);
        assert_eq!(xs.xs[1].t, -4.0);
    }

    #[test]
    fn test_intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s: Shape = Sphere::new().into();
        let xs = s.intersect(&r);
        assert_eq!(xs.xs.len(), 2);
        assert_eq!(xs.xs[0].object, &s);
        assert_eq!(xs.xs[1].object, &s);
    }

    #[test]
    fn test_a_spheres_default_transformation() {
        let s: Shape = Sphere::new().into();
        assert_eq!(s.transform, Matrix::identity(4));
    }

    #[test]
    fn test_changing_a_spheres_transformation() {
        let m = Matrix::translation(2.0, 3.0, 4.0);
        let mut s: Shape = Sphere::new().into();
        s.set_transform(&m);
        assert_eq!(s.transform, m);
    }

    #[test]
    fn test_intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s: Shape = Sphere::new().into();
        s.set_transform(&Matrix::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.xs.len(), 2);
        assert_eq!(xs.xs[0].t, 3.0);
        assert_eq!(xs.xs[1].t, 7.0);
    }

    #[test]
    fn test_intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s: Shape = Sphere::new().into();
        s.set_transform(&Matrix::translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.xs.len(), 0);
    }

    #[test]
    fn test_normal_at_a_point_on_the_x_axis() {
        let s: Shape = Sphere::new().into();
        let n = s.normal_at(&point(1.0, 0.0, 0.0));
        assert_eq!(n, vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_normal_at_a_point_on_the_y_axis() {
        let s: Shape = Sphere::new().into();
        let n = s.normal_at(&point(0.0, 1.0, 0.0));
        assert_eq!(n, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_normal_at_a_point_on_the_z_axis() {
        let s: Shape = Sphere::new().into();
        let n = s.normal_at(&point(0.0, 0.0, 1.0));
        assert_eq!(n, vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_normal_at_a_nonaxial_point() {
        let s: Shape = Sphere::new().into();
        let sqrt_3_over_3 = (3.0_f64).sqrt() / 3.0;
        let n = s.normal_at(&point(sqrt_3_over_3, sqrt_3_over_3, sqrt_3_over_3));
        assert_eq!(n, vector(sqrt_3_over_3, sqrt_3_over_3, sqrt_3_over_3));
    }

    #[test]
    fn test_normal_is_normalized() {
        let s: Shape = Sphere::new().into();
        let sqrt_3_over_3 = (3.0_f64).sqrt() / 3.0;
        let n = s.normal_at(&point(sqrt_3_over_3, sqrt_3_over_3, sqrt_3_over_3));
        assert_eq!(n.normalize(), n);
    }

    #[test]
    fn test_computing_the_normal_on_a_translated_sphere() {
        let mut s: Shape = Sphere::new().into();
        s.set_transform(&Matrix::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&point(
            0.0,
            1.0 + std::f64::consts::FRAC_1_SQRT_2,
            -std::f64::consts::FRAC_1_SQRT_2,
        ));
        assert_abs_diff_eq!(
            n,
            vector(
                0.0,
                std::f64::consts::FRAC_1_SQRT_2,
                -std::f64::consts::FRAC_1_SQRT_2
            )
        );
    }

    #[test]
    fn test_computing_the_normal_on_a_transformed_sphere() {
        let mut s: Shape = Sphere::new().into();
        s.set_transform(
            &(Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(std::f64::consts::PI / 5.0)),
        );
        let n = s.normal_at(&point(
            0.0,
            std::f64::consts::FRAC_1_SQRT_2,
            -std::f64::consts::FRAC_1_SQRT_2,
        ));
        assert_abs_diff_eq!(n, vector(0.0, 0.97014, -0.24254), epsilon = 0.00001);
    }

    #[test]
    fn test_sphere_has_default_material() {
        let s: Shape = Sphere::new().into();
        assert_eq!(s.material, Material::new());
    }

    #[test]
    fn test_sphere_may_be_assigned_a_material() {
        let m = Material::new().with_ambient(1.0);
        let mut s: Shape = Sphere::new().into();
        s.set_material(&m);
        assert_eq!(s.material, m);
    }
}
