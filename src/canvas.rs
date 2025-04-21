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

impl Canvas {
    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y][x] = color;
    }
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

    #[test]
    fn test_write_pixel() {
        let mut c = canvas(10, 20);
        let red = color(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }
}