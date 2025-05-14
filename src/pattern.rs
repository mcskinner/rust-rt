use crate::color::Color;
use crate::matrix::Matrix;
use crate::shape::Shape;
use crate::tuple::Tuple;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait LocalPatternTrait {
    fn color_at(&self, point: &Tuple) -> Color;
}

#[enum_dispatch(LocalPatternTrait)]
#[derive(Debug, Clone, PartialEq)]
pub enum LocalPattern {
    Stripe(StripePattern),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    local_pattern: LocalPattern,
    transform: Matrix,
    inverse_transform: Matrix,
}

impl Pattern {
    pub fn new(local_pattern: LocalPattern) -> Self {
        Pattern {
            local_pattern: local_pattern.clone(),
            transform: Matrix::identity(4),
            inverse_transform: Matrix::identity(4),
        }
    }

    #[allow(dead_code)]
    fn set_transform(&mut self, transform: &Matrix) -> &mut Self {
        self.transform = transform.clone();
        self.inverse_transform = transform.inverse();
        self
    }

    pub fn color_at_object(&self, object: &Shape, point: &Tuple) -> Color {
        let local_point = &object.inverse_transform * point;
        let pattern_point = &self.inverse_transform * local_point;
        self.local_pattern.color_at(&pattern_point)
    }
}

impl<T: Into<LocalPattern>> From<T> for Pattern {
    fn from(local_pattern: T) -> Self {
        Pattern::new(local_pattern.into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StripePattern {
    a: Color,
    b: Color,
}

impl StripePattern {
    #[allow(dead_code)]
    pub fn new(a: Color, b: Color) -> Self {
        StripePattern { a, b }
    }
}

impl LocalPatternTrait for StripePattern {
    fn color_at(&self, point: &Tuple) -> Color {
        if point.x.rem_euclid(2.0) < 1.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;
    use crate::tuple::point;

    #[test]
    fn test_creating_a_stripe_pattern() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.a, Color::WHITE);
        assert_eq!(pattern.b, Color::BLACK);
    }

    #[test]
    fn test_stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(0.0, 1.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(0.0, 2.0, 0.0)), Color::WHITE);
    }

    #[test]
    fn test_stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 1.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 2.0)), Color::WHITE);
    }

    #[test]
    fn test_stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.color_at(&point(-1.1, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(-1.0, 0.0, 0.0)), Color::BLACK);
        assert_eq!(pattern.color_at(&point(-0.1, 0.0, 0.0)), Color::BLACK);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(0.9, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(1.0, 0.0, 0.0)), Color::BLACK);
    }

    #[test]
    fn test_stripes_with_an_object_transformation() {
        let mut object: Shape = Sphere::new().into();
        object.set_transform(&Matrix::scaling(2.0, 2.0, 2.0));
        let pattern: Pattern = StripePattern::new(Color::WHITE, Color::BLACK).into();
        let c = pattern.color_at_object(&object, &point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::WHITE);
    }

    #[test]
    fn test_stripes_with_a_pattern_transformation() {
        let object: Shape = Sphere::new().into();
        let mut pattern: Pattern = StripePattern::new(Color::WHITE, Color::BLACK).into();
        pattern.set_transform(&Matrix::scaling(2.0, 2.0, 2.0));
        let c = pattern.color_at_object(&object, &point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::WHITE);
    }

    #[test]
    fn test_stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object: Shape = Sphere::new().into();
        object.set_transform(&Matrix::scaling(2.0, 2.0, 2.0));
        let mut pattern: Pattern = StripePattern::new(Color::WHITE, Color::BLACK).into();
        pattern.set_transform(&Matrix::translation(0.5, 0.0, 0.0));
        let c = pattern.color_at_object(&object, &point(2.5, 0.0, 0.0));
        assert_eq!(c, Color::WHITE);
    }
}
