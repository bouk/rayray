use vec3::Vec3;
use super::{Material, Hit, Hittable, Ray, EPSILON};

#[derive(Clone, Copy)]
pub struct Plane {
    position: Vec3,
    normal: Vec3,
    radius2: f64,
    material: Material,
}

impl Plane {
    pub fn new(position: Vec3, normal: Vec3, radius: f64, material: Material) -> Plane {
        Plane { position, normal: normal.unit(), radius2: radius * radius, material }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let discriminant = ray.direction().dot(self.normal);

        // Parallel
        if discriminant == 0.0 {
            return None;
        }

        let distance = (self.position - ray.origin()).dot(self.normal) / discriminant;
        if distance < EPSILON {
            return None;
        }
        let intersection = ray.advance(distance);
        if (intersection - self.position).length_squared() > self.radius2 {
            return None;
        }

        Some(Hit {
            distance,
            intersection,
            normal: self.normal,
            material: self.material,
        })
    }
}
