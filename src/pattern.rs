use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Debug, Clone, PartialEq)]
pub struct StripePattern {
    a: Color,
    b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        StripePattern { a, b }
    }

    pub fn color_at(&self, point: &Tuple) -> Color {
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
}
