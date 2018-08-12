extern crate rand;
extern crate rayon;

mod color;
mod ray;
mod vec3;

use color::Color;
use ray::Ray;
use vec3::Vec3;
use rand::random;
use rayon::prelude::*;

pub struct Hit {
    pub color: Color,
    pub distance: f64,
    pub intersection: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

trait Hittable: Sync {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

impl <'a> Hittable for Vec<&'a Hittable> {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        self.iter()
            .filter_map(|h| h.hit(ray))
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }
}

impl Hittable for Vec<Box<Hittable>> {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        self.iter()
            .filter_map(|h| h.hit(ray))
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p;
    loop {
        p = Vec3::new(random(), random(), random()) * 2.0 - Vec3::one();
        if p.length_squared() < 1.0 {
            break;
        }
    }
    p
}

#[derive(Clone, Copy)]
pub struct Material {
    scatter: bool,
    reflection_rate: f64,
}

impl Material {
    pub fn scatter(&self, hit: &Hit) -> Vec3 {
        if self.scatter {
            hit.intersection + hit.normal + random_in_unit_sphere()
        } else {
            hit.intersection - hit.normal * hit.intersection.dot(hit.normal) * 2.0
        }
    }
}

#[derive(Clone, Copy)]
struct Sphere {
    position: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    fn new(position: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere { position, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin() - self.position;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;

        match quadratic_formula(a, b, c) {
            None => None,
            Some(distance) => {
                if distance < 0.00001 {
                    return None;
                }

                let intersection = ray.advance(distance);
                let v = ((intersection - self.position).unit() + Vec3::one()) / 2.0;
                Some(Hit { 
                    color: Color::new(v.x(), v.y(), v.z()),
                    distance: distance,
                    intersection: intersection,
                    normal: (intersection - self.position) / self.radius,
                    material: self.material,
                })
            }
        }
    }
}

fn quadratic_formula(a: f64, b: f64, c: f64) -> Option<f64> {
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    // We are only interested in the lowest value
    // The higher value would be useful if the camera would be inside the sphere but w/e
    let d_sqrt = discriminant.sqrt();
    Some((-b - d_sqrt) / 2.0 * a)
}

struct World {
    objects: Vec<Box<Hittable>>,
}

impl World {
    fn hit(&self, ray: &Ray) -> Color {
        if let Some(hit) = self.objects.hit(ray) {
            let target = hit.material.scatter(&hit);

            return self.hit(&Ray::new(hit.intersection, target - hit.intersection)) * hit.material.reflection_rate;
        }

        let unit_direction = ray.direction();
        let t = (unit_direction.y() + 1.0) / 2.0;

        Color::linear_interpolation(Color::white(), Color::new(0.2, 0.4, 0.8), t)
    }
}

fn main() {
    let width = 1920;
    let height = 1080;
    let anti_aliasing = 20;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zero();

    let matte = Material { scatter: true, reflection_rate: 0.5 };
    let reflect = Material { scatter: false, reflection_rate: 0.9 };
    let world = World {
        objects: vec![
            Box::new(Sphere::new(Vec3::new(0.0, -0.25, 2.0), 0.5, matte)),
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, reflect)),
            Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.8), 0.5, reflect)),
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.8), 0.5, reflect)),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -2.0), 100.0, matte)),
        ],
    };

    println!("P3");
    println!("{} {}", width, height);
    println!("255");
    let mut anti_aliases = vec![];
    for ax in 0..anti_aliasing {
        for ay in 0..anti_aliasing {
            anti_aliases.push((ax, ay));
        }
    }

    for y in (0..height).rev() {
        for x in 0..width {
            let mut sum = anti_aliases
                .par_iter()
                .map(|(ax, ay)| {
                    let pos_x = (x * anti_aliasing + ax) as f64 / (width * anti_aliasing) as f64;
                    let pos_y = (y * anti_aliasing + ay) as f64 / (height * anti_aliasing) as f64;
                    let ray = Ray::new(origin, lower_left_corner + horizontal * pos_x + vertical * pos_y);
                    world.hit(&ray)
                })
                .reduce(Color::black, |a, b| a + b);

            sum /= anti_aliases.len() as f64;

            print!("{} {} {}\t", (sum.red() * 255.0) as u8, (sum.green() * 255.0) as u8, (sum.blue() * 255.0) as u8);
        }
        println!();
    }
}
