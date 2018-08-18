mod vecmath;

use vecmath::Vec3;

pub struct Ray {
  origin: Vec3,
  direction: Vec3,
}

impl Ray {
  pub fn new() -> {
    Ray { Vec3::new(), Vec3::new() }
  }
}