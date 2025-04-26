use crate::sphere::Sphere;

#[derive(Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl Intersection<'_> {
    pub fn new(t: f64, object: &Sphere) -> Intersection {
        Intersection { t, object }
    }
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
}
