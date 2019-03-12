use super::float;

#[derive(Debug)]
struct Tuple {
  x: f32,
  y: f32,
  z: f32,
  w: f32,
}

impl Tuple {
  fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
    Tuple { x, y, z, w }
  }

  fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple::new(x, y, z, 1.0)
  }

  fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple::new(x, y, z, 0.0)
  }

  fn is_point(&self) -> bool {
    float::eq(self.w, 1.0)
  }

  fn is_vector(&self) -> bool {
    float::eq(self.w, 0.0)
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
}
