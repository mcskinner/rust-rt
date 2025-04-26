use crate::tuple::{point, vector, Tuple};

struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
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
}