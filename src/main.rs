mod canvas;
mod color;
mod tuple;
use canvas::canvas;
use color::color;
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

fn draw_box(c: &mut canvas::Canvas, x: usize, y: usize) {
    for i in 0..5 {
        for j in 0..5 {
            c.write_pixel(x + i, c.height - (y + j) - 1, color(1.0, 0.0, 0.0));
        }
    }
}

fn main() {
    let start = point(0.0, 1.0, 0.0);
    let velocity = vector(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut p = Projectile::new(start, velocity);

    let gravity = vector(0.0, -0.1, 0.0);
    let wind = vector(-0.01, 0.0, 0.0);
    let env = Env::new(gravity, wind);

    let mut c = canvas(900, 550);

    while p.position.y > 0.0 {
        p.tick(&env);
        let x = p.position.x.round() as usize;
        let y = p.position.y.round() as usize;
        draw_box(&mut c, x, y);
    }

    let ppm = c.to_ppm();
    let mut file = std::fs::File::create("projectile.ppm").unwrap();
    std::io::Write::write_all(&mut file, ppm.as_bytes()).unwrap();
}
