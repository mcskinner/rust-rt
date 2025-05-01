use crate::matrix::Matrix;

struct Camera {
    width: usize,
    height: usize,
    fov: f64,
    pixel_size: f64,
    transform: Matrix,
}

impl Camera {
    pub fn new(width: usize, height: usize, fov: f64) -> Self {
        let half_view = (fov / 2.0).tan();
        let aspect = width as f64 / height as f64;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.0) / width as f64;
        Self {
            width,
            height,
            fov,
            pixel_size,
            transform: Matrix::identity(4),
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn test_constructing_a_camera() {
        let c = Camera::new(160, 120, std::f64::consts::FRAC_PI_2);
        assert_eq!(c.width, 160);
        assert_eq!(c.height, 120);
        assert_eq!(c.fov, std::f64::consts::FRAC_PI_2);
        assert_eq!(c.transform, Matrix::identity(4));
    }

    #[test]
    fn test_the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, std::f64::consts::FRAC_PI_2);
        assert_abs_diff_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn test_the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, std::f64::consts::FRAC_PI_2);
        assert_abs_diff_eq!(c.pixel_size, 0.01);
    }
}
