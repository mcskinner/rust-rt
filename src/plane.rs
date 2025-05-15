use approx_derive::AbsDiffEq;

use crate::consts::EPSILON;
use crate::ray::Ray;
use crate::shape::HittableTrait;
use crate::tuple::{Tuple, vector};

#[derive(Debug, Clone, PartialEq, AbsDiffEq)]
pub struct Plane {}

impl Plane {
    pub fn new() -> Self {
        Self {}
    }
}

impl HittableTrait for Plane {
    fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        if ray.direction.y.abs() < EPSILON {
            return vec![];
        }
        vec![-ray.origin.y / ray.direction.y]
    }

    fn local_normal_at(&self, _point: &Tuple) -> Tuple {
        vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::{point, vector};

    #[test]
    fn test_the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.local_normal_at(&point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(&point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(&point(-5.0, 0.0, 150.0));
        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::new();
        let r = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_intersect_with_a_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_intersect_with_a_ray_from_above() {
        let p = Plane::new();
        let r = Ray::new(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.0);
    }

    #[test]
    fn test_intersect_with_a_ray_from_below() {
        let p = Plane::new();
        let r = Ray::new(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.0);
    }
}
