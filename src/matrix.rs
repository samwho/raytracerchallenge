use std::ops::{Index, IndexMut};
use super::float;
use super::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Mat4 {
  matrix: [f32; 16],
}

#[derive(Debug, Copy, Clone)]
pub struct Mat3 {
  matrix: [f32; 9],
}

#[derive(Debug, Copy, Clone)]
pub struct Mat2 {
  matrix: [f32; 4],
}

impl Index<(usize, usize)> for Mat4 {
  type Output = f32;
  fn index(&self, idx: (usize, usize)) -> &f32 {
    &self.matrix[idx.0 * 4 + idx.1]
  }
}

impl IndexMut<(usize, usize)> for Mat4 {
  fn index_mut(&mut self, idx: (usize, usize)) -> &mut f32 {
    &mut self.matrix[idx.0 * 4 + idx.1]
  }
}

impl Index<(usize, usize)> for Mat3 {
  type Output = f32;
  fn index(&self, idx: (usize, usize)) -> &f32 {
    &self.matrix[idx.0 * 3 + idx.1]
  }
}

impl IndexMut<(usize, usize)> for Mat3 {
  fn index_mut(&mut self, idx: (usize, usize)) -> &mut f32 {
    &mut self.matrix[idx.0 * 3 + idx.1]
  }
}

impl Index<(usize, usize)> for Mat2 {
  type Output = f32;
  fn index(&self, idx: (usize, usize)) -> &f32 {
    &self.matrix[idx.0 * 2 + idx.1]
  }
}

impl IndexMut<(usize, usize)> for Mat2 {
  fn index_mut(&mut self, idx: (usize, usize)) -> &mut f32 {
    &mut self.matrix[idx.0 * 2 + idx.1]
  }
}

impl PartialEq for Mat4 {
  fn eq(&self, other: &Mat4) -> bool {
    for i in 0..(self.matrix.len()) {
      if !float::eq(self.matrix[i], other.matrix[i]) {
        return false
      }
    }

    return true
  }
}
impl Eq for Mat4 {}

impl PartialEq for Mat3 {
  fn eq(&self, other: &Mat3) -> bool {
    for i in 0..(self.matrix.len()) {
      if !float::eq(self.matrix[i], other.matrix[i]) {
        return false
      }
    }

    return true
  }
}
impl Eq for Mat3 {}

impl PartialEq for Mat2 {
  fn eq(&self, other: &Mat2) -> bool {
    for i in 0..(self.matrix.len()) {
      if !float::eq(self.matrix[i], other.matrix[i]) {
        return false
      }
    }

    return true
  }
}
impl Eq for Mat2 {}

impl std::ops::Mul<Mat4> for Mat4 {
  type Output = Mat4;
  fn mul(self, other: Mat4) -> Mat4 {
    let mut m = [0.0; 16];

    for row in 0..4 {
      for col in 0..4 {
        m[row * 4 + col] =
          self[(row, 0)] * other[(0, col)] +
          self[(row, 1)] * other[(1, col)] +
          self[(row, 2)] * other[(2, col)] +
          self[(row, 3)] * other[(3, col)];
      }
    }

    Mat4::new(m)
  }
}

impl std::ops::Mul<Tuple> for Mat4 {
  type Output = Tuple;
  fn mul(self, other: Tuple) -> Tuple {
    Tuple::new(
      self[(0, 0)] * other.x() + self[(0, 1)] * other.y() + self[(0, 2)] * other.z() + self[(0, 3)] * other.w(),
      self[(1, 0)] * other.x() + self[(1, 1)] * other.y() + self[(1, 2)] * other.z() + self[(1, 3)] * other.w(),
      self[(2, 0)] * other.x() + self[(2, 1)] * other.y() + self[(2, 2)] * other.z() + self[(2, 3)] * other.w(),
      self[(3, 0)] * other.x() + self[(3, 1)] * other.y() + self[(3, 2)] * other.z() + self[(3, 3)] * other.w()
    )
  }
}

impl Mat4 {
  pub fn new(matrix: [f32; 16]) -> Mat4 {
    Mat4 { matrix }
  }

  pub fn identity() -> Mat4 {
    Mat4::new([
      1.0, 0.0, 0.0, 0.0,
      0.0, 1.0, 0.0, 0.0,
      0.0, 0.0, 1.0, 0.0,
      0.0, 0.0, 0.0, 1.0
    ])
  }
}

impl Mat3 {
  pub fn new(matrix: [f32; 9]) -> Mat3 {
    Mat3 { matrix }
  }
}

impl Mat2 {
  pub fn new(matrix: [f32; 4]) -> Mat2 {
    Mat2 { matrix }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_mat4_new() {
    let m = Mat4::new([
      1.0,  2.0,  3.0,  4.0,
      5.5,  6.5,  7.5,  8.5,
      9.0,  10.0, 11.0, 12.0,
      13.5, 14.5, 15.5, 16.5
    ]);

    assert!(float::eq(m[(0, 0)], 1.0));
    assert!(float::eq(m[(0, 3)], 4.0));
    assert!(float::eq(m[(1, 0)], 5.5));
    assert!(float::eq(m[(1, 2)], 7.5));
    assert!(float::eq(m[(2, 2)], 11.0));
    assert!(float::eq(m[(3, 0)], 13.5));
    assert!(float::eq(m[(3, 2)], 15.5));
  }

  #[test]
  fn test_mat3_new() {
    let m = Mat3::new([
      -3.0, 5.0, 0.0,
      1.0, -2.0, -7.0,
      0.0, 1.0, 1.0,
    ]);

    assert!(float::eq(m[(0, 0)], -3.0));
    assert!(float::eq(m[(1, 1)], -2.0));
    assert!(float::eq(m[(2, 2)], 1.0));
  }

  #[test]
  fn test_mat2_new() {
    let m = Mat2::new([
      -3.0, 5.0,
      1.0, -2.0,
    ]);

    assert!(float::eq(m[(0, 0)], -3.0));
    assert!(float::eq(m[(0, 1)], 5.0));
    assert!(float::eq(m[(1, 0)], 1.0));
    assert!(float::eq(m[(1, 1)], -2.0));
  }

  #[test]
  fn test_mat4_eq() {
    let a = Mat4::new([
      1.0,  2.0,  3.0,  4.0,
      5.5,  6.5,  7.5,  8.5,
      9.0,  10.0, 11.0, 12.0,
      13.5, 14.5, 15.5, 16.5
    ]);

    let b = Mat4::new([
      1.0,  2.0,  3.0,  4.0,
      5.5,  6.5,  7.5,  8.5,
      9.0,  10.0, 11.0, 12.0,
      13.5, 14.5, 15.5, 16.5
    ]);

    assert_eq!(a, b);
  }

  #[test]
  fn test_mat3_eq() {
    let a = Mat3::new([
      1.0,  2.0,  3.0,
      5.5,  6.5,  7.5,
      9.0,  10.0, 11.0,
    ]);

    let b = Mat3::new([
      1.0,  2.0,  3.0,
      5.5,  6.5,  7.5,
      9.0,  10.0, 11.0,
    ]);

    assert_eq!(a, b);
  }

  #[test]
  fn test_mat2_eq() {
    let a = Mat2::new([
      1.0,  2.0,
      5.5,  6.5,
    ]);

    let b = Mat2::new([
      1.0,  2.0,
      5.5,  6.5,
    ]);

    assert_eq!(a, b);
  }

  #[test]
  fn test_mat4_not_eq() {
    let a = Mat4::new([
      2.0,  2.0,  3.0,  4.0,
      5.5,  6.5,  7.5,  8.5,
      9.0,  10.0, 11.0, 12.0,
      13.5, 14.5, 15.5, 16.5
    ]);

    let b = Mat4::new([
      1.0,  2.0,  3.0,  4.0,
      5.5,  6.5,  7.5,  8.5,
      9.0,  10.0, 11.0, 12.0,
      13.5, 14.5, 15.5, 16.5
    ]);

    assert_ne!(a, b);
  }

  #[test]
  fn test_mat3_not_eq() {
    let a = Mat3::new([
      2.0,  2.0,  3.0,
      5.5,  6.5,  7.5,
      9.0,  10.0, 11.0,
    ]);

    let b = Mat3::new([
      1.0,  2.0,  3.0,
      5.5,  6.5,  7.5,
      9.0,  10.0, 11.0,
    ]);

    assert_ne!(a, b);
  }

  #[test]
  fn test_mat2_not_eq() {
    let a = Mat2::new([
      2.0,  2.0,
      5.5,  6.5,
    ]);

    let b = Mat2::new([
      1.0,  2.0,
      5.5,  6.5,
    ]);

    assert_ne!(a, b);
  }

  #[test]
  fn test_mat4_mat4_mul() {
    let a = Mat4::new([
      1.0, 2.0, 3.0, 4.0,
      5.0, 6.0, 7.0, 8.0,
      9.0, 8.0, 7.0, 6.0,
      5.0, 4.0, 3.0, 2.0,
    ]);

    let b = Mat4::new([
      -2.0, 1.0, 2.0, 3.0,
      3.0, 2.0, 1.0, -1.0,
      4.0, 3.0, 6.0, 5.0,
      1.0, 2.0, 7.0, 8.0,
    ]);

    let result = Mat4::new([
      20.0, 22.0, 50.0, 48.0,
      44.0, 54.0, 114.0, 108.0,
      40.0, 58.0, 110.0, 102.0,
      16.0, 26.0, 46.0, 42.0,
    ]);

    assert_eq!(a * b, result);
  }

  #[test]
  fn test_mat4_tuple_mul() {
    let a = Mat4::new([
      1.0, 2.0, 3.0, 4.0,
      2.0, 4.0, 4.0, 2.0,
      8.0, 6.0, 4.0, 1.0,
      0.0, 0.0, 0.0, 1.0,
    ]);

    let b = Tuple::new(
      1.0, 2.0, 3.0, 1.0
    );

    let result = Tuple::new(
      18.0, 24.0, 33.0, 1.0
    );

    assert_eq!(a * b, result);
  }

  #[test]
  fn test_mat4_identity_mul() {
    let a = Mat4::new([
      1.0, 2.0, 3.0, 4.0,
      5.0, 6.0, 7.0, 8.0,
      9.0, 8.0, 7.0, 6.0,
      5.0, 4.0, 3.0, 2.0,
    ]);

    let b = Mat4::identity();

    assert_eq!(a * b, a);
  }
}