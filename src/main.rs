struct Tuple([f64; 4]);

impl Tuple {
    fn is_point(&self) -> bool {
        let Tuple([_, _, _, w]) = *self;
        w == 1.0
    }

    fn is_vector(&self) -> bool {
        let Tuple([_, _, _, w]) = *self;
        w == 0.0
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tuple_with_w1_is_a_point() {
        let a = Tuple([4.3, -4.2, 3.1, 1.0]);
        let Tuple([x, y, z, w]) = a;
        assert_eq!(x, 4.3);
        assert_eq!(y, -4.2);
        assert_eq!(z, 3.1);
        assert_eq!(w, 1.0);
        assert_eq!(a.is_point(), true);
        assert_eq!(a.is_vector(), false);
    }

    #[test]
    fn test_tuple_with_w0_is_a_vector() {
        let a = Tuple([4.3, -4.2, 3.1, 0.0]);
        let Tuple([x, y, z, w]) = a;
        assert_eq!(x, 4.3);
        assert_eq!(y, -4.2);
        assert_eq!(z, 3.1);
        assert_eq!(w, 0.0);
        assert_eq!(a.is_point(), false);
        assert_eq!(a.is_vector(), true);
    }
}

fn main() {
    println!("Hello, world!");
}
