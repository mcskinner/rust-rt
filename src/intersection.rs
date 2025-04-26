use crate::sphere::Sphere;

#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl Intersection<'_> {
    pub fn new(t: f64, object: &Sphere) -> Intersection {
        Intersection { t, object }
    }
}

pub fn hit<'a>(xs: &'a [Intersection<'a>]) -> Option<&'a Intersection<'a>> {
    let mut hit: Option<&Intersection> = None;

    for i in xs {
        if i.t > 0.0 {
            if hit.is_none() || i.t < hit.unwrap().t {
                hit = Some(i);
            }
        }
    }

    hit
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::tuple::{point, vector};

    #[test]
    fn test_intersection() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn test_aggregate_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2];
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn test_hit() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i2, i1];
        assert_eq!(hit(&xs).unwrap().t, 1.0);
    }

    #[test]
    fn test_hit_with_some_intersections_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i1, i2];
        assert_eq!(hit(&xs).unwrap().t, 1.0);
    }

    #[test]
    fn test_hit_with_all_negative_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i2, i1];
        assert_eq!(hit(&xs), None);
    }

    #[test]
    fn test_hit_is_always_the_lowest_positive_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2, i3, i4];
        assert_eq!(hit(&xs).unwrap(), &xs[3]);
    }
}
