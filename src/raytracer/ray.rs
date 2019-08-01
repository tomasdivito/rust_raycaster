use raytracer::vec::Vec3;

#[derive(Copy, Clone)]
pub struct Ray<'a> {
    pub origin: &'a Vec3,
    pub direction: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Vec3, direction: &'a Vec3) -> Ray<'a> {
        Ray { origin, direction }
    }

    pub fn point_at_param(self, t: f32) -> Vec3 {
        self.origin + &(self.direction * t)
    }
}
