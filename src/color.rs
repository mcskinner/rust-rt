use approx::assert_abs_diff_eq;
use approx_derive::AbsDiffEq;
use derive_more::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, AbsDiffEq, Add, Sub)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

pub fn color(red: f64, green: f64, blue: f64) -> Color {
    Color { red, green, blue }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let c = color(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn test_adding_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_subtracting_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_abs_diff_eq!(c1 - c2, color(0.2, 0.5, 0.5));
    }
}
