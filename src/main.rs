mod color;
mod ray;
mod vec3;

use color::Color;
use ray::Ray;
use vec3::Vec3;

fn color(ray: &Ray) -> Color {
    let sphere = Sphere(Vec3::new(0.0, 0.0, -1.0), 0.5);
    if sphere.intersect(ray) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction();
    let t = (unit_direction.y() + 1.0) / 2.0;

    Color::linear_interpolation(Color::white(), Color::new(0.2, 0.4, 0.8), t)
}

#[derive(Clone, Copy)]
struct Sphere(Vec3, f64);

impl Sphere {
    pub fn intersect(&self, ray: &Ray) -> bool {
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(self.0 - ray.origin());
        let c = self.0.dot(self.0) - 2.0 * self.0.dot(ray.origin()) + ray.origin().dot(ray.origin()) - self.1 * self.1;

        let result = quadratic_formula(a, b, c);

        if let QuadraticResult::None = result {
            false
        } else {
            true
        }
    }
}

enum QuadraticResult {
    Two(f64, f64),
    One(f64),
    None,
}

fn quadratic_formula(a: f64, b: f64, c: f64) -> QuadraticResult {
    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
        QuadraticResult::None
    } else if d == 0.0 {
        QuadraticResult::One(-b / 2.0 * a)
    } else {
        let d_sqrt = d.sqrt();
        QuadraticResult::Two(
            (-b + d_sqrt) / 2.0 * a,
            (-b - d_sqrt) / 2.0 * a,
        )
    }
}

fn main() {
    let width = 200;
    let height = 100;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zero();

    println!("P3");
    println!("{} {}", width, height);
    println!("255");
    for y in (0..height).rev() {
        for x in 0..width {
            let pos_x = x as f64 / width as f64;
            let pos_y = y as f64 / height as f64;
            let ray = Ray::new(origin, lower_left_corner + horizontal * pos_x + vertical * pos_y);

            let c = color(&ray);
            print!("{} {} {}\t", (c.red() * 255.0) as u8, (c.green() * 255.0) as u8, (c.blue() * 255.0) as u8);
        }
        println!();
    }
}
