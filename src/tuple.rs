use super::float;
use std::ops;
use std::fmt;
use std::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
  x: f32,
  y: f32,
  z: f32,
  w: f32,
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

const ZERO_VECTOR: Tuple = Tuple { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };

impl Tuple {
  pub fn new<F: Into<f32>>(x: F, y: F, z: F, w: F) -> Tuple {
    Tuple { x: x.into(), y: y.into(), z: z.into(), w: w.into() }
  }

  pub fn point<F: Into<f32>>(x: F, y: F, z: F) -> Tuple {
    Tuple::new(x, y, z, 1.0f32)
  }

  pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple::new(x, y, z, 0.0)
  }

  pub fn x(&self) -> f32 {
    self.x
  }

  pub fn y(&self) -> f32 {
    self.y
  }

  pub fn z(&self) -> f32 {
    self.z
  }

  pub fn w(&self) -> f32 {
    self.w
  }

  pub fn is_point(&self) -> bool {
    float::eq(self.w, 1.0)
  }

  pub fn is_vector(&self) -> bool {
    float::eq(self.w, 0.0)
  }

  pub fn magnitude(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
  }

  pub fn normalize(&self) -> Tuple {
    let m = self.magnitude();
    Tuple::new(self.x / m, self.y / m, self.z / m, self.w / m )
  }

  pub fn dot(&self, other: Tuple) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
  }
}

impl PartialEq for Tuple {
  fn eq(&self, other: &Tuple) -> bool {
    float::eq(self.x, other.x)
      && float::eq(self.y, other.y)
      && float::eq(self.z, other.z)
      && float::eq(self.w, other.w)
  }
}
impl Eq for Tuple {}

impl ops::Add<Tuple> for Tuple {
  type Output = Tuple;
  fn add(self, other: Tuple) -> Tuple {
    Tuple::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
  }
}

impl ops::Sub<Tuple> for Tuple {
  type Output = Tuple;
  fn sub(self, other: Tuple) -> Tuple {
    Tuple::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
  }
}

impl ops::Mul<f32> for Tuple {
  type Output = Tuple;
  fn mul(self, other: f32) -> Tuple {
    Tuple::new(self.x * other, self.y * other, self.z * other, self.w * other)
  }
}

impl ops::Mul<Tuple> for Tuple {
  type Output = Tuple;
  // cross-product
  fn mul(self, other: Tuple) -> Tuple {
    Tuple::vector(
      self.y * other.z - self.z * other.y,
      self.z * other.x - self.x * other.z,
      self.x * other.y - self.y * other.x)
  }
}

impl ops::Div<f32> for Tuple {
  type Output = Tuple;
  fn div(self, other: f32) -> Tuple {
    Tuple::new(self.x / other, self.y / other, self.z / other, self.w / other)
  }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
      ZERO_VECTOR - self
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_point() {
    let p = Tuple::point(0.0, 0.0, 0.0);
    assert!(p.is_point());
    assert!(!p.is_vector());
  }

  #[test]
  fn test_is_vector() {
    let v = Tuple::vector(0.0, 0.0, 0.0);
    assert!(v.is_vector());
    assert!(!v.is_point());
  }

  #[test]
  fn test_eq() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(1.0, 2.0, 3.0);
    assert!(a == b);
    assert!(b == a);
  }

  #[test]
  fn test_add_point_vector() {
    let p = Tuple::point(3.0, -2.0, 5.0);
    let v = Tuple::vector(-2.0, 3.0, 1.0);
    let result = Tuple::point(1.0, 1.0, 6.0);
    assert!(p + v == result);
  }

  #[test]
  fn test_add_vector_point() {
    let v = Tuple::vector(3.0, -2.0, 5.0);
    let p = Tuple::point(-2.0, 3.0, 1.0);
    let result = Tuple::point(1.0, 1.0, 6.0);
    assert!(p + v == result);
  }

  #[test]
  fn test_add_vector_vector() {
    let v1 = Tuple::vector(3.0, -2.0, 5.0);
    let v2 = Tuple::vector(-2.0, 3.0, 1.0);
    let result = Tuple::vector(1.0, 1.0, 6.0);
    assert!(v1 + v2 == result);
  }

  #[test]
  fn test_sub_point_point() {
    let p1 = Tuple::point(3.0, 2.0, 1.0);
    let p2 = Tuple::point(5.0, 6.0, 7.0);
    let result = Tuple::vector(-2.0, -4.0, -6.0);
    assert!(p1 - p2 == result);
  }

  #[test]
  fn test_sub_point_vector() {
    let p = Tuple::point(3.0, 2.0, 1.0);
    let v = Tuple::vector(5.0, 6.0, 7.0);
    let result = Tuple::point(-2.0, -4.0, -6.0);
    assert!(p - v == result);
  }

  #[test]
  fn test_sub_vector_vector() {
    let v1 = Tuple::vector(3.0, 2.0, 1.0);
    let v2 = Tuple::vector(5.0, 6.0, 7.0);
    let result = Tuple::vector(-2.0, -4.0, -6.0);
    assert!(v1 - v2 == result);
  }

  #[test]
  fn test_negate() {
    let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
    let result = Tuple::new(-1.0, 2.0, -3.0, 4.0);
    assert!(-t == result);
  }

  #[test]
  fn test_mul_scalar() {
    let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
    let result = Tuple::new(3.5, -7.0, 10.5, -14.0);
    assert!(t * 3.5 == result);
  }

  #[test]
  fn test_mul_scalar_fraction() {
    let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
    let result = Tuple::new(0.5, -1.0, 1.5, -2.0);
    assert!(t * 0.5 == result);
  }

  #[test]
  fn test_div_scalar() {
    let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
    let result = Tuple::new(0.5, -1.0, 1.5, -2.0);
    assert!(t / 2.0 == result);
  }

  #[test]
  fn test_magnitude() {
    assert!(float::eq(Tuple::vector(1.0, 0.0, 0.0).magnitude(), 1.0));
    assert!(float::eq(Tuple::vector(0.0, 1.0, 0.0).magnitude(), 1.0));
    assert!(float::eq(Tuple::vector(0.0, 0.0, 1.0).magnitude(), 1.0));
    assert!(float::eq(Tuple::vector(1.0, 2.0, 3.0).magnitude(), (14.0 as f32).sqrt()));
    assert!(float::eq(Tuple::vector(-1.0, -2.0, -3.0).magnitude(), (14.0 as f32).sqrt()));
  }

  #[test]
  fn test_normalize() {
    assert!(Tuple::vector(4.0, 0.0, 0.0).normalize() == Tuple::vector(1.0, 0.0, 0.0));
    assert!(Tuple::vector(1.0, 2.0, 3.0).normalize() == Tuple::vector(0.26726, 0.53452, 0.80178));
    assert!(float::eq(Tuple::vector(1.0, 2.0, 3.0).normalize().magnitude(), 1.0));
  }

  #[test]
  fn test_dot_product() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(2.0, 3.0, 4.0);
    assert!(float::eq(a.dot(b), 20.0));
  }

  #[test]
  fn test_cross_product() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(2.0, 3.0, 4.0);
    let result = Tuple::vector(-1.0, 2.0, -1.0);
    assert!(a * b == result);
    assert!(b * a == -result);
  }
}
