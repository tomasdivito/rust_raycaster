use std;

pub struct Vec3 {
  x: f32,
  y: f32,
  z: f32,
}

impl Vec3 {
  pub fn new() -> Vec3 {
    Vec3 { x: 0.0, y: 0.0, z: 0.0 }
  }

  pub fn from(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x: x, y: y, z: z }
  }

  pub fn length(&self) -> f32 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }

  pub fn make_unit_vector(&self) -> Vec3 {
    let k = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    Vec3 { x: self.x * k, y: self.y * k, z: self.z * k }
  }
}

impl std::fmt::Display for Vec3 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{} {} {}", self.x, self.y, self.z)
  }
}

impl std::ops::Add for Vec3 {
  type Output = Vec3;

  fn add(self, other: Vec3) -> Vec3 {
    Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
  }
}

impl std::ops::Sub for Vec3 {
  type Output = Vec3;

  fn sub(self, other: Vec3) -> Vec3 {
    Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
  }
}

impl std::ops::Div for Vec3 {
  type Output = Vec3;

  fn div(self, other: Vec3) -> Vec3 {
    Vec3 { x: self.x / other.x, y: self.y / other.y, z: self.z / other.z }
  }
}

impl std::ops::Mul for Vec3 {
  type Output = Vec3;

  fn mul(self, other: Vec3) -> Vec3 {
    Vec3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
  }
}