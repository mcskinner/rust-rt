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
    Solid(SolidPattern),
    Stripe(StripePattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
    Checkers(CheckersPattern),
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
    pub fn set_transform(&mut self, transform: &Matrix) -> &mut Self {
        self.transform = transform.clone();
        self.inverse_transform = transform.inverse();
        self
    }

    pub fn color_at_object(&self, object: &Shape, point: &Tuple) -> Color {
        let local_point = &object.inverse_transform * point;
        self.color_at_pattern(&local_point)
    }

    fn color_at_pattern(&self, point: &Tuple) -> Color {
        let pattern_point = &self.inverse_transform * point;
        self.local_pattern.color_at(&pattern_point)
    }
}

impl<T: Into<LocalPattern>> From<T> for Pattern {
    fn from(local_pattern: T) -> Self {
        Pattern::new(local_pattern.into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SolidPattern {
    color: Color,
}
impl SolidPattern {
    #[allow(dead_code)]
    pub fn new(color: Color) -> Self {
        SolidPattern { color }
    }
}
impl LocalPatternTrait for SolidPattern {
    fn color_at(&self, _point: &Tuple) -> Color {
        self.color
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StripePattern {
    a: Box<Pattern>,
    b: Box<Pattern>,
}
impl StripePattern {
    pub fn new(a: Pattern, b: Pattern) -> Self {
        StripePattern {
            a: Box::new(a),
            b: Box::new(b),
        }
    }

    #[allow(dead_code)]
    pub fn from_colors(a: Color, b: Color) -> Self {
        Self::new(SolidPattern::new(a).into(), SolidPattern::new(b).into())
    }
}
impl LocalPatternTrait for StripePattern {
    fn color_at(&self, point: &Tuple) -> Color {
        if point.x.rem_euclid(2.0) < 1.0 {
            self.a.color_at_pattern(point)
        } else {
            self.b.color_at_pattern(point)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GradientPattern {
    a: Color,
    b: Color,
}
impl GradientPattern {
    #[allow(dead_code)]
    pub fn new(a: Color, b: Color) -> Self {
        GradientPattern { a, b }
    }
}
impl LocalPatternTrait for GradientPattern {
    fn color_at(&self, point: &Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();
        self.a + distance * fraction
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RingPattern {
    a: Color,
    b: Color,
}
impl RingPattern {
    #[allow(dead_code)]
    pub fn new(a: Color, b: Color) -> Self {
        RingPattern { a, b }
    }
}
impl LocalPatternTrait for RingPattern {
    fn color_at(&self, point: &Tuple) -> Color {
        let distance = (point.x * point.x + point.z * point.z).sqrt();
        if distance.rem_euclid(2.0) < 1.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckersPattern {
    a: Color,
    b: Color,
}
impl CheckersPattern {
    #[allow(dead_code)]
    pub fn new(a: Color, b: Color) -> Self {
        CheckersPattern { a, b }
    }
}
impl LocalPatternTrait for CheckersPattern {
    fn color_at(&self, point: &Tuple) -> Color {
        let total_val = point.x.floor() + point.y.floor() + point.z.floor();
        if total_val.rem_euclid(2.0) < 1.0 {
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
        let pattern = StripePattern::from_colors(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.a, Box::new(SolidPattern::new(Color::WHITE).into()));
        assert_eq!(pattern.b, Box::new(SolidPattern::new(Color::BLACK).into()));
    }

    #[test]
    fn test_stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::from_colors(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(0.0, 1.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(0.0, 2.0, 0.0)), Color::WHITE);
    }

    #[test]
    fn test_stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::from_colors(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 1.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 2.0)), Color::WHITE);
    }

    #[test]
    fn test_stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::from_colors(Color::WHITE, Color::BLACK);
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
        let pattern: Pattern = StripePattern::from_colors(Color::WHITE, Color::BLACK).into();
        let c = pattern.color_at_object(&object, &point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::WHITE);
    }

    #[test]
    fn test_stripes_with_a_pattern_transformation() {
        let object: Shape = Sphere::new().into();
        let mut pattern: Pattern = StripePattern::from_colors(Color::WHITE, Color::BLACK).into();
        pattern.set_transform(&Matrix::scaling(2.0, 2.0, 2.0));
        let c = pattern.color_at_object(&object, &point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::WHITE);
    }

    #[test]
    fn test_stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object: Shape = Sphere::new().into();
        object.set_transform(&Matrix::scaling(2.0, 2.0, 2.0));
        let mut pattern: Pattern = StripePattern::from_colors(Color::WHITE, Color::BLACK).into();
        pattern.set_transform(&Matrix::translation(0.5, 0.0, 0.0));
        let c = pattern.color_at_object(&object, &point(2.5, 0.0, 0.0));
        assert_eq!(c, Color::WHITE);
    }

    #[test]
    fn test_gradient_pattern() {
        let pattern = GradientPattern::new(Color::WHITE, Color::BLACK);
        assert_eq!(
            pattern.color_at(&point(0.0, 0.0, 0.0)),
            Color::new(1.0, 1.0, 1.0)
        );
        assert_eq!(
            pattern.color_at(&point(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at(&point(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at(&point(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn test_ring_pattern() {
        let pattern = RingPattern::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(1.0, 0.0, 0.0)), Color::BLACK);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 1.0)), Color::BLACK);
        assert_eq!(pattern.color_at(&point(0.708, 0.0, 0.708)), Color::BLACK);
    }

    #[test]
    fn test_checkers_repeat_in_x() {
        let pattern = CheckersPattern::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(0.99, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(1.01, 0.0, 0.0)), Color::BLACK);
    }

    #[test]
    fn test_checkers_repeat_in_y() {
        let pattern = CheckersPattern::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(0.0, 0.99, 0.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(0.0, 1.01, 0.0)), Color::BLACK);
    }

    #[test]
    fn test_checkers_repeat_in_z() {
        let pattern = CheckersPattern::new(Color::WHITE, Color::BLACK);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 0.0)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 0.99)), Color::WHITE);
        assert_eq!(pattern.color_at(&point(0.0, 0.0, 1.01)), Color::BLACK);
    }
}
