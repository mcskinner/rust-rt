use crate::light::Light;
use crate::sphere::Sphere;

struct World {
    lights: Vec<Light>,
    objects: Vec<Sphere>,
}

impl World {
    fn new() -> World {
        World {
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_a_world() {
        let w = World::new();
        assert_eq!(w.lights.len(), 0);
        assert_eq!(w.objects.len(), 0);
    }
}
