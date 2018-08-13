use vec3::Vec3;
use super::{Material, Hit, Hittable, Ray};

#[derive(Clone, Copy)]
pub struct Plane {
    position: f64,
    material: Material,
}

impl Plane {
    pub fn new(position: f64, material: Material) -> Plane {
        Plane { position, material }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let a = ray.origin().y();
        let b = ray.direction().y();
        if b == 0.0 {
            return None;
        }

        let distance = (self.position - a) / b;
        if distance < 0.0001 {
            return None;
        }
        let intersection = ray.advance(distance);

        Some(Hit {
            distance,
            intersection,
            normal: Vec3::new(0.0, - b / b.abs(), 0.0),
            material: self.material,
        })
    }
}
