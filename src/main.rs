extern crate rand;
extern crate rayon;

use rayon::prelude::*;

mod objects;
mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;
use rand::random;
use objects::*;

const EPSILON: f64 = 0.001;

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

            return hit.material.color * self._hit(&Ray::new(hit.intersection, target - hit.intersection), depth - 1) * 0.99;
        }

        let unit_direction = ray.direction();
        let t = (unit_direction.y() + 1.0) / 2.0;

        Color::linear_interpolation(Color::white(), Color::new(0.2, 0.4, 0.8), t)
    }
}

fn main() {
    let width = 800;
    let height = 400;
    let anti_aliasing = 10;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zero();

    let mut objects: Vec<Box<Hittable>> = vec![
        Box::new(Plane::new(Vec3(0.0, -1.0, -7.0), Vec3(0.0, 1.0, 0.0), 8.0, Material { scatter: true, color: Color::new(1.0, 0.2, 0.2) })),
    ];
    
    for _ in 0..40 {
        let radius = 0.4 + random::<f64>() * 0.1;
        objects.push(Box::new(
                Sphere::new(
                    Vec3(10.0 * random::<f64>() - 5.0, radius - 1.0, random::<f64>() * -10.0 - 2.0),
                    radius,
                    Material { scatter: random::<f64>() < 0.7, color: Color::random() })));
    }
    /*
    objects.push(Box::new(Triangle::new(
                Vec3(-4.0, 1.0, -4.0),
                Vec3(5.0, 1.0, -4.0),
                Vec3(-6.0, 5.0, -4.0),
                Material { scatter: true, color: Color::random() })));
                */
    let world = World {
        objects,
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

            print!("{} {} {}\t", (sum.red().sqrt() * 255.0) as u8, (sum.green().sqrt() * 255.0) as u8, (sum.blue().sqrt() * 255.0) as u8);
        }
        println!();
    }
}
