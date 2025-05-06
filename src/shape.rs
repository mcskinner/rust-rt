use approx_derive::AbsDiffEq;

use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

#[derive(Debug, Clone, AbsDiffEq)]
pub struct Shape {
    hittable: Hittable,
    pub transform: Matrix,
    pub inverse_transform: Matrix,
    pub material: Material,
}

pub trait HittableTrait {
    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64>;
    fn local_normal_at(&self, local_point: &Tuple) -> Tuple;
}

#[derive(Debug, Clone, PartialEq, AbsDiffEq)]
pub enum Hittable {
    Sphere(Sphere),
}

impl HittableTrait for Hittable {
    fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        match self {
            Hittable::Sphere(sphere) => sphere.local_intersect(local_ray),
        }
    }

    fn local_normal_at(&self, local_point: &Tuple) -> Tuple {
        match self {
            Hittable::Sphere(sphere) => sphere.local_normal_at(local_point),
        }
    }
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

impl From<Sphere> for Shape {
    fn from(s: Sphere) -> Self {
        Shape::new(Hittable::Sphere(s))
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}
