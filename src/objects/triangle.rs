use vec3::Vec3;
use {Material, Hit, Hittable, Ray, EPSILON};

pub struct Triangle {
    vertices: [Vec3; 3],
    material: Material,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Material) -> Triangle {
        Triangle {
            vertices: [v0, v1, v2],
            material,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let edge0 = self.vertices[1] - self.vertices[0];
        let edge1 = self.vertices[2] - self.vertices[1];
        let edge2 = self.vertices[0] - self.vertices[2];

        let normal = edge0.cross(self.vertices[2] - self.vertices[0]);
        let discriminant = ray.direction().dot(normal);

        // Parallel
        if discriminant == 0.0 {
            return None;
        }

        let distance = (self.vertices[0] - ray.origin()).dot(normal) / discriminant;
        if distance < EPSILON {
            return None;
        }

        let intersection = ray.advance(distance);
        if normal.dot(edge0.cross(intersection - self.vertices[0])) < 0.0 {
            return None;
        }

        if normal.dot(edge1.cross(intersection - self.vertices[1])) < 0.0 {
            return None;
        }

        if normal.dot(edge2.cross(intersection - self.vertices[2])) < 0.0 {
            return None;
        }

        Some(Hit {
            distance,
            intersection,
            normal,
            material: self.material,
        })
    }
}
