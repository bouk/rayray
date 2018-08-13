use vec3::Vec3;
use super::{Material, Hit, Hittable, Ray};

#[derive(Clone, Copy)]
pub struct Sphere {
    position: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f64, material: Material) -> Sphere {
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
                Some(Hit { 
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
