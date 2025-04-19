#[derive(Debug, PartialEq)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    fn is_point(&self) -> bool {
        return self.w == 1.0;
    }

    fn is_vector(&self) -> bool {
        return self.w == 0.0;
    }
}

fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple { x, y, z, w }
}
fn point(x: f64, y: f64, z: f64) -> Tuple {
    tuple(x, y, z, 1.0)
}
fn vector(x: f64, y: f64, z: f64) -> Tuple {
    tuple(x, y, z, 0.0)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tuple_with_w1_is_a_point() {
        let a = tuple(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert_eq!(a.is_point(), true);
        assert_eq!(a.is_vector(), false);
    }

    #[test]
    fn test_tuple_with_w0_is_a_vector() {
        let a = tuple(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert_eq!(a.is_point(), false);
        assert_eq!(a.is_vector(), true);
    }

    #[test]
    fn test_point() {
        let a = point(4.0, -4.0, 3.0);
        assert_eq!(a, tuple(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn test_vector() {
        let a = vector(4.0, -4.0, 3.0);
        assert_eq!(a, tuple(4.0, -4.0, 3.0, 0.0));
    }
}

fn main() {
    println!("Hello, world!");
}
