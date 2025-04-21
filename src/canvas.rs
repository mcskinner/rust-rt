use crate::color::{color, Color};

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

pub fn canvas(width: usize, height: usize) -> Canvas {
    let pixels = vec![vec![color(0.0, 0.0, 0.0); width]; height];
    Canvas { width, height, pixels }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::color;

    #[test]
    fn test_canvas() {
        let c = canvas(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for x in 0..10 {
            for y in 0..20 {
                assert_eq!(c.pixels[y][x], color(0.0, 0.0, 0.0));
            }
        }
    }
}