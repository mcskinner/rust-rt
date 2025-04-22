use approx::assert_abs_diff_eq;
use approx_derive::AbsDiffEq;
use derive_more::{Add, Sub, Neg, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq, AbsDiffEq, Add, Sub, Neg, Mul, Div)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple { x, y, z, w }
}
pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    tuple(x, y, z, 1.0)
}
pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    tuple(x, y, z, 0.0)
}

impl Tuple {
    pub fn is_point(&self) -> bool {
        return self.w == 1.0;
    }

    pub fn is_vector(&self) -> bool {
        return self.w == 0.0;
    }

    pub fn magnitude(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        *self / self.magnitude()
    }

    pub fn dot(&self, other: &Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tuple_with_w1_is_a_point() {
        let a = tuple(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert_eq!(a.is_point(), true);
        assert_eq!(a.is_vector(), false);
    }

    #[test]
    fn test_tuple_with_w0_is_a_vector() {
        let a = tuple(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert_eq!(a.is_point(), false);
        assert_eq!(a.is_vector(), true);
    }

    #[test]
    fn test_point() {
        let a = point(4.0, -4.0, 3.0);
        assert_eq!(a, tuple(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn test_vector() {
        let a = vector(4.0, -4.0, 3.0);
        assert_eq!(a, tuple(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn test_adding_two_tuples() {
        let a1 = tuple(3.0, -2.0, 5.0, 1.0);
        let a2 = tuple(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a1 + a2, tuple(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn test_subtracting_two_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_subtracting_a_vector_from_a_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert_eq!(p - v, point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_subtracting_two_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_subtracting_a_vector_from_the_zero_vector() {
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);
        assert_eq!(zero - v, vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_negating_a_tuple() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, tuple(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn test_multiplying_a_tuple_by_a_scalar() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, tuple(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn test_multiplying_a_tuple_by_a_fraction() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 0.5, tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_dividing_a_tuple_by_a_scalar() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn computing_the_magnitude_of_vector_100() {
        let v = vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_010() {
        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_001() {
        let v = vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_123() {
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn computing_the_magnitude_of_vector_negative_123() {
        let v = vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn normalizing_vector_400() {
        let v = vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalizing_vector_123() {
        let v = vector(1.0, 2.0, 3.0);
        assert_abs_diff_eq!(v.normalize(), vector(0.26726, 0.53452, 0.80178), epsilon = 0.00001);
    }

    #[test]
    fn magnitude_of_a_normalized_vector() {
        let v = vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        assert_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(&b), vector(-1.0, 2.0, -1.0));
    }
}