use crate::tuple::{point, vector, Tuple};

struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_and_querying_a_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, point(1.0, 2.0, 3.0));
        assert_eq!(r.direction, vector(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_computing_a_point_from_a_distance() {
        let r = Ray::new(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), point(4.5, 3.0, 4.0));
    }
}