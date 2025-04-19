mod tuple;
use tuple::{point, vector, Tuple};

#[derive(Debug, Clone, Copy)]
struct Env {
    gravity: Tuple,
    wind: Tuple,
}

impl Env {
    fn new(gravity: Tuple, wind: Tuple) -> Env {
        Env { gravity, wind }
    }
}

#[derive(Debug, Clone, Copy)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    fn new(position: Tuple, velocity: Tuple) -> Projectile {
        Projectile { position, velocity }
    }

    fn tick(&mut self, env: &Env) {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + env.gravity + env.wind;
    }
}

fn main() {
    let mut p = Projectile::new(point(0.0, 1.0, 0.0), vector(1.0, 1.0, 0.0));
    let env = Env::new(vector(0.0, -0.1, 0.0), vector(-0.01, 0.0, 0.0));
    println!("Initial projectile position and velocity: {:?} {:?}", p.position, p.velocity);
    while p.position.y > 0.0 {
        p.tick(&env);
        println!("Projectile position and velocity: {:?} {:?}", p.position, p.velocity);
    }
}
