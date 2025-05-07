use approx_derive::AbsDiffEq;

use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Clone, AbsDiffEq)]
pub struct Shape {
    hittable: Hittable,
    pub transform: Matrix,
    pub inverse_transform: Matrix,
    pub material: Material,
}

#[enum_dispatch]
pub trait HittableTrait {
    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64>;
    fn local_normal_at(&self, local_point: &Tuple) -> Tuple;
}

#[derive(Debug, Clone, PartialEq, AbsDiffEq)]
#[enum_dispatch(HittableTrait)]
pub enum Hittable {
    Sphere(Sphere),
}

impl Shape {
    pub fn new(hittable: Hittable) -> Shape {
        Shape {
            hittable,
            transform: Matrix::identity(4),
            inverse_transform: Matrix::identity(4),
            material: Material::new(),
        }
    }

    pub fn set_transform(&mut self, m: &Matrix) -> &mut Self {
        self.transform = m.clone();
        self.inverse_transform = m.inverse();
        self
    }

    pub fn set_material(&mut self, m: &Material) -> &mut Self {
        self.material = m.clone();
        self
    }

    pub fn intersect(&self, r: &Ray) -> Intersections {
        let local_ray = r.transform(&self.inverse_transform);
        let hit_ts = self.hittable.local_intersect(&local_ray);
        let mut intersections = Vec::new();
        for t in hit_ts {
            intersections.push(Intersection::new(t, self));
        }
        Intersections::new(intersections)
    }

    pub fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let local_point = &self.inverse_transform * world_point;
        let local_normal = self.hittable.local_normal_at(&local_point);
        let mut world_normal = self.inverse_transform.transpose() * local_normal;
        world_normal.w = 0.0; // Ensure it's a vector
        world_normal.normalize()
    }
}

impl<T: Into<Hittable>> From<T> for Shape {
    fn from(hittable: T) -> Self {
        Self::new(hittable.into())
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_shape() -> Shape {
        Sphere::new().into()
    }

    #[test]
    fn test_the_default_transformation() {
        let s: Shape = test_shape();
        assert_eq!(s.transform, Matrix::identity(4));
    }

    #[test]
    fn test_assigning_a_transformation() {
        let mut s: Shape = test_shape();
        let m = Matrix::translation(2.0, 3.0, 4.0);
        s.set_transform(&m);
        assert_eq!(s.transform, m);
    }

    #[test]
    fn test_the_default_material() {
        let s: Shape = test_shape();
        assert_eq!(s.material, Material::new());
    }

    #[test]
    fn test_assigning_a_material() {
        let mut s: Shape = test_shape();
        let m = Material::new().with_ambient(1.0);
        s.set_material(&m);
        assert_eq!(s.material, m);
    }
}
