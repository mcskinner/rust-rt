use crate::shape::Shape;
use crate::tuple::Tuple;

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Shape,
}

impl Intersection<'_> {
    pub fn new(t: f64, object: &Shape) -> Intersection {
        Intersection { t, object }
    }

    pub fn prepare_computations(&self, r: &crate::ray::Ray) -> Computations {
        let eyev = -r.direction;
        let point = r.position(self.t);
        let mut normalv = self.object.normal_at(&point);
        let inside = eyev.dot(&normalv) < 0.0;
        if inside {
            normalv = -normalv;
        }
        let over_point = point + normalv * 1e-9;
        Computations {
            object: self.object,
            point,
            eyev,
            normalv,
            over_point,
        }
    }
}

#[allow(dead_code)]
pub struct Computations<'a> {
    pub object: &'a Shape,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub over_point: Tuple,
}

pub struct Intersections<'a> {
    pub xs: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new(mut xs: Vec<Intersection<'a>>) -> Intersections<'a> {
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Intersections { xs }
    }

    pub fn hit(&self) -> Option<&Intersection> {
        let mut hit: Option<&Intersection> = None;

        for i in &self.xs {
            if i.t > 0.0 {
                if hit.is_none() || i.t < hit.unwrap().t {
                    hit = Some(i);
                }
            }
        }

        hit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::Matrix;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::{point, vector};

    #[test]
    fn test_intersection() {
        let s = Sphere::new().into();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn test_aggregate_intersections() {
        let s = Sphere::new().into();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.xs.len(), 2);
        assert_eq!(xs.xs[0].t, 1.0);
        assert_eq!(xs.xs[1].t, 2.0);
    }

    #[test]
    fn test_hit() {
        let s = Sphere::new().into();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = Intersections::new(vec![i2, i1]);
        assert_eq!(xs.hit().unwrap().t, 1.0);
    }

    #[test]
    fn test_hit_with_some_intersections_negative() {
        let s = Sphere::new().into();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.hit().unwrap().t, 1.0);
    }

    #[test]
    fn test_hit_with_all_negative_intersections() {
        let s = Sphere::new().into();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = Intersections::new(vec![i2, i1]);
        assert_eq!(xs.hit().is_none(), true);
    }

    #[test]
    fn test_hit_is_always_the_lowest_positive_intersection() {
        let s = Sphere::new().into();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let expected = i4.clone();
        let xs = Intersections::new(vec![i1, i2, i3, i4]);
        assert_eq!(xs.hit().unwrap(), &expected);
    }

    #[test]
    fn test_precomputing_the_state_of_an_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new().into();
        let i = Intersection::new(4.0, &s);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_hit_from_the_outside() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::new().into();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_hit_from_the_inside() {
        let r = Ray::new(Tuple::ORIGIN, vector(0.0, 0.0, 1.0));
        let shape = Sphere::new().into();
        let i = Intersection::new(1.0, &shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_hit_should_offset_the_point() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut shape: Shape = Sphere::new().into();
        shape.set_transform(&Matrix::translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &shape);
        let comps = i.prepare_computations(&r);
        assert!(comps.over_point.z < -1e-9 / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
