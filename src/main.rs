mod camera;
mod canvas;
mod color;
mod consts;
mod intersection;
mod light;
mod material;
mod matrix;
mod pattern;
mod plane;
mod ray;
mod shape;
mod sphere;
mod tuple;
mod world;

use crate::camera::Camera;
use crate::canvas::canvas;
use crate::color::Color;
use crate::light::Light;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::pattern::CheckersPattern;
use crate::plane::Plane;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::tuple::{Tuple, point, vector};
use crate::world::World;

use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

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
    let m = Material::new().with_rgb(1.0, 0.2, 1.0);
    let mut s: Shape = Sphere::new().into();
    s.set_material(&m)
        .set_transform(&Matrix::translation(0.0, 0.0, 2.0));

    let camera = point(0.0, 0.0, -5.0);
    let light = light::Light::new(point(-10.0, 10.0, -10.0), Color::WHITE);

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
                let position = ray.position(hit.t);
                let normalv = s.normal_at(&position);
                let eyev = (camera - position).normalize();
                let color = m.lighting(hit.object, &light, &position, &eyev, &normalv, false);
                c.write_pixel(x, y, color);
            }
        }
    }

    let ppm = c.to_ppm();
    let mut file = std::fs::File::create("sphere.ppm").unwrap();
    std::io::Write::write_all(&mut file, ppm.as_bytes()).unwrap();
}

fn render_chapter7_scene() {
    let m = Material::new()
        .with_pattern(&CheckersPattern::new(Color::BLACK, Color::WHITE).into())
        .with_specular(0.0);
    let mut floor: Shape = Plane::new().into();
    floor.set_material(&m);

    let mut left_wall: Shape = Plane::new().into();
    left_wall.set_material(&m).set_transform(
        &(Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(-FRAC_PI_4)
            * Matrix::rotation_x(FRAC_PI_2)),
    );

    let mut right_wall: Shape = Plane::new().into();
    right_wall.set_material(&m).set_transform(
        &(Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(FRAC_PI_4)
            * Matrix::rotation_x(FRAC_PI_2)),
    );

    let m = Material::new()
        .with_rgb(0.1, 1.0, 0.5)
        .with_diffuse(0.7)
        .with_specular(0.3);
    let mut middle: Shape = Sphere::new().into();
    middle
        .set_material(&m)
        .set_transform(&Matrix::translation(-0.5, 1.0, 0.5));

    let m = Material::new()
        .with_rgb(0.5, 1.0, 0.1)
        .with_diffuse(0.7)
        .with_specular(0.3);
    let mut right: Shape = Sphere::new().into();
    right
        .set_material(&m)
        .set_transform(&(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5)));

    let m = Material::new()
        .with_rgb(1.0, 0.8, 0.1)
        .with_diffuse(0.7)
        .with_specular(0.3);
    let mut left: Shape = Sphere::new().into();
    left.set_material(&m).set_transform(
        &(Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33)),
    );

    let light = Light::new(point(-10.0, 10.0, -10.0), Color::WHITE);

    let mut camera = Camera::new(400, 200, FRAC_PI_3);
    camera.set_transform(&Matrix::view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    ));

    let world = World::new()
        .add_object(floor.into())
        .add_object(left_wall.into())
        .add_object(right_wall.into())
        .add_object(middle.into())
        .add_object(left.into())
        .add_object(right.into())
        .add_light(light);

    let canvas = camera.render(&world);

    let ppm = canvas.to_ppm();
    let mut file = std::fs::File::create("chapter7.ppm").unwrap();
    std::io::Write::write_all(&mut file, ppm.as_bytes()).unwrap();
}

fn main() {
    let demo = "chapter7";
    match demo {
        "projectile" => simulate_projectile(),
        "clock" => draw_clock(),
        "sphere" => render_sphere(),
        "chapter7" => render_chapter7_scene(),
        _ => (),
    }
}
