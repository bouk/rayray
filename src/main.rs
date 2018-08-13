extern crate rand;
extern crate rayon;

use rayon::prelude::*;

mod ray;
mod plane;
mod sphere;
mod vec3;

use ray::Ray;
use vec3::Vec3;
use rand::random;
use sphere::Sphere;
use plane::Plane;

type Color = Vec3;

pub struct Hit {
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
    color: Color,
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

struct World {
    objects: Vec<Box<Hittable>>,
}

impl World {
    pub fn hit(&self, ray: &Ray) -> Color {
        self._hit(ray, 10)
    }

    fn _hit(&self, ray: &Ray, depth: usize) -> Color {
        if depth == 0 {
            return Color::black();
        }

        if let Some(hit) = self.objects.hit(ray) {
            let target = hit.material.scatter(&hit);

            return hit.material.color * self._hit(&Ray::new(hit.intersection, target - hit.intersection), depth - 1);
        }

        let unit_direction = ray.direction();
        let t = (unit_direction.y() + 1.0) / 2.0;

        Color::linear_interpolation(Color::white(), Color::new(0.2, 0.4, 0.8), t)
    }
}

fn main() {
    let width = 400;
    let height = 200;
    let anti_aliasing = 20;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zero();

    let matte = Material { scatter: true, color: Color::new(0.8, 0.5, 0.5) };
    let red = Material { scatter: true, color: Color::new(1.0, 0.5, 0.5) };
    let reflect = Material { scatter: false, color: Color::new(0.8, 1.0, 0.8) };
    let world = World {
        objects: vec![
            Box::new(Plane::new(-1.0, matte)),
            Box::new(Sphere::new(Vec3::new(0.0, -0.25, 2.0), 0.5, matte)),
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, red)),
            Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.8), 0.5, reflect)),
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.8), 0.5, reflect)),
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
