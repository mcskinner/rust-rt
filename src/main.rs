mod canvas;
mod color;
mod intersection;
mod light;
mod material;
mod matrix;
mod ray;
mod sphere;
mod tuple;
use crate::canvas::canvas;
use crate::color::Color;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::sphere::Sphere;
use crate::tuple::{Tuple, point, vector};

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
            c.write_pixel(x + i, c.height - (y + j) - 1, Color::new(1.0, 0.0, 0.0));
        }
    }
}

fn simulate_projectile() {
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

fn draw_clock() {
    let size = 400;
    let radius = 0.4 * size as f64;
    let noon = point(0.0, radius, 0.0);
    let mut c = canvas(size, size);

    let position_on_canvas = Matrix::translation(size as f64 / 2.0, size as f64 / 2.0, 0.0);

    for i in 0..12 {
        let angle = (i as f64) * std::f64::consts::PI / 6.0;
        let t = &position_on_canvas * &Matrix::rotation_z(-angle);
        let p = t * noon;
        draw_box(&mut c, p.x.round() as usize, p.y.round() as usize);
    }

    let ppm = c.to_ppm();
    let mut file = std::fs::File::create("clock.ppm").unwrap();
    std::io::Write::write_all(&mut file, ppm.as_bytes()).unwrap();
}

fn render_sphere() {
    let size = 400;
    let mut c = canvas(size, size);
    let mut s = Sphere::new();
    let mut m = Material::new();
    m.color = Color::new(1.0, 0.2, 1.0);
    s.set_material(&m);
    s.set_transform(&Matrix::translation(0.0, 0.0, 2.0));

    let camera = point(0.0, 0.0, -5.0);
    let light = light::Light::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    for y in 0..size {
        for x in 0..size {
            let p = point(
                2.0 * (x as f64) / (size as f64) - 1.0,
                1.0 - 2.0 * (y as f64) / (size as f64),
                0.0,
            );

            let direction = (p - camera).normalize();
            let ray = ray::Ray::new(camera, direction);
            if let Some(hit) = s.intersect(&ray).hit() {
                let point = ray.position(hit.t);
                let normal = s.normal_at(point);
                let eyev = (camera - point).normalize();
                let color = m.lighting(&light, &point, &eyev, &normal);
                c.write_pixel(x, y, color);
            }
        }
    }

    let ppm = c.to_ppm();
    let mut file = std::fs::File::create("sphere.ppm").unwrap();
    std::io::Write::write_all(&mut file, ppm.as_bytes()).unwrap();
}

fn main() {
    render_sphere();
}
