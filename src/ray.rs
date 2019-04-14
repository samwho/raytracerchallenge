use super::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
  origin: Tuple,
  direction: Tuple,
}

impl Ray {
  pub fn new(origin: Tuple, direction: Tuple) -> Ray {
    if !origin.is_point() {
      panic!("Ray origin must be a point");
    }

    if !direction.is_vector() {
      panic!("Ray direction must be a vector");
    }

    Ray { origin, direction }
  }

  pub fn origin(&self) -> Tuple {
    self.origin
  }

  pub fn direction(&self) -> Tuple {
    self.direction
  }

  pub fn position_at_time(&self, t: f32) -> Tuple {
    self.origin + (self.direction * t)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let ray = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(4.0, 5.0, 6.0));
    assert_eq!(ray.origin(), Tuple::point(1.0, 2.0, 3.0));
    assert_eq!(ray.direction(), Tuple::vector(4.0, 5.0, 6.0));
  }

  #[test]
  fn test_position_at_time() {
    let ray = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));
    assert_eq!(ray.origin(), Tuple::point(1.0, 2.0, 3.0));
    assert_eq!(ray.direction(), Tuple::vector(4.0, 5.0, 6.0));
  }
}