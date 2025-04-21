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
}
