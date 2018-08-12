use vec3::Vec3;

pub struct Ray(Vec3, Vec3);

impl Ray {
    // direction is unitized
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray(origin, direction.unit())
    }

    #[inline]
    pub fn origin(&self) -> Vec3 {
        self.0
    }

    #[inline]
    pub fn direction(&self) -> Vec3 {
        self.1
    }

    pub fn advance(&self, t: f64) -> Vec3 {
        self.0 + self.1 * t
    }
}
