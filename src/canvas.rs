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

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::new();
        ppm.push_str("P3\n");
        ppm.push_str(&format!("{} {}\n", self.width, self.height));
        ppm.push_str("255\n");
        for row in &self.pixels {
            let mut line = String::new();
            for pixel in row {
                line.push_str(&format!("{} {} {} ", (pixel.red * 255.0).round() as u8, (pixel.green * 255.0).round() as u8, (pixel.blue * 255.0).round() as u8));
            }
            if !line.is_empty() {
                line.pop();
            }
            ppm.push_str(&line);
            ppm.push('\n');
        }
        ppm
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

    #[test]
    fn test_ppm_header() {
        let c = canvas(5, 3);
        let ppm = c.to_ppm();
        assert_eq!(ppm.lines().nth(0).unwrap(), "P3");
        assert_eq!(ppm.lines().nth(1).unwrap(), "5 3");
        assert_eq!(ppm.lines().nth(2).unwrap(), "255");
    }

    #[test]
    fn test_ppm_pixel_data() {
        let mut c = canvas(5, 3);
        let c1 = color(1.5, 0.0, 0.0);
        let c2 = color(0.0, 0.5, 0.0);
        let c3 = color(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = c.to_ppm();
        assert_eq!(ppm.lines().nth(3).unwrap(), "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(ppm.lines().nth(4).unwrap(), "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(ppm.lines().nth(5).unwrap(), "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }
}