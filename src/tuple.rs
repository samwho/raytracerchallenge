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

  fn is_point(&self) -> bool {
    self.w == 1.0
  }

  fn is_vector(&self) -> bool {
    self.w == 0.0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_point() {
    let p = Tuple::new(0.0, 0.0, 0.0, 1.0);
    assert!(p.is_point());
    assert!(!p.is_vector());
  }

  #[test]
  fn test_is_vector() {
    let v = Tuple::new(0.0, 0.0, 0.0, 0.0);
    assert!(v.is_vector());
    assert!(!v.is_point());
  }
}
