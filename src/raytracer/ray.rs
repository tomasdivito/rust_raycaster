use raytracer::vec::Vec3;

pub struct Ray {
  origin: Vec3,
  direction: Vec3,
}

impl Ray {
  pub fn new() -> Ray {
    Ray { origin: Vec3::new(), direction: Vec3::new() }
  }
}