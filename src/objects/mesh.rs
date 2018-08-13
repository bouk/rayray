use triangle::Triangle;
use {Material, Hit, Hittable, Ray, EPSILON};

struct Mesh {
    triangles: Vec<Triangle>,
    material: Material,
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        self.triangles.hit(ray)
    }
}
