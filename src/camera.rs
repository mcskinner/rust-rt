use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::point;

struct Camera {
    width: usize,
    height: usize,
    fov: f64,
    half_width: f64,
    half_height: f64,
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
            half_width,
            half_height,
            pixel_size,
            transform: Matrix::identity(4),
        }
    }

    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let x_offset = (px as f64 + 0.5) * self.pixel_size;
        let y_offset = (py as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;
        let pixel = point(world_x, world_y, -1.0);
        let origin = point(0.0, 0.0, 0.0);
        Ray::new(origin, (pixel - origin).normalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::vector;
    use approx::assert_abs_diff_eq;

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

    #[test]
    fn test_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, std::f64::consts::FRAC_PI_2);
        let r = c.ray_for_pixel(100, 50);
        assert_abs_diff_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_abs_diff_eq!(r.direction, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, std::f64::consts::FRAC_PI_2);
        let r = c.ray_for_pixel(0, 0);
        assert_abs_diff_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_abs_diff_eq!(
            r.direction,
            vector(0.66519, 0.33259, -0.66851),
            epsilon = 0.00001
        );
    }
}
