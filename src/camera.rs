use crate::matrix::Matrix;

struct Camera {
    width: usize,
    height: usize,
    fov: f64,
    transform: Matrix,
}

impl Camera {
    pub fn new(width: usize, height: usize, fov: f64) -> Self {
        Self {
            width,
            height,
            fov,
            transform: Matrix::identity(4),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructing_a_camera() {
        let c = Camera::new(160, 120, std::f64::consts::FRAC_PI_2);
        assert_eq!(c.width, 160);
        assert_eq!(c.height, 120);
        assert_eq!(c.fov, std::f64::consts::FRAC_PI_2);
        assert_eq!(c.transform, Matrix::identity(4));
    }
}
