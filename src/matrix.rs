use std::ops::{Index, IndexMut};
use std::fmt;
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

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
          f,
          "[{:.2}, {:.2}, {:.2}, {:.2},\n {:.2}, {:.2}, {:.2}, {:.2},\n {:.2}, {:.2}, {:.2}, {:.2},\n {:.2}, {:.2}, {:.2}, {:.2}]",
          self.matrix[0], self.matrix[1], self.matrix[2], self.matrix[3],
          self.matrix[4], self.matrix[5], self.matrix[6], self.matrix[7],
          self.matrix[8], self.matrix[9], self.matrix[10], self.matrix[11],
          self.matrix[12], self.matrix[13], self.matrix[14], self.matrix[15])
    }
}

impl Mat4 {
  pub fn new(matrix: [f32; 16]) -> Mat4 {
    Mat4 { matrix }
  }

  pub fn translation(x: f32, y: f32, z: f32) -> Mat4 {
    Mat4 { matrix: [
      1.0, 0.0, 0.0, x,
      0.0, 1.0, 0.0, y,
      0.0, 0.0, 1.0, z,
      0.0, 0.0, 0.0, 1.0,
    ] }
  }

  pub fn scaling(x: f32, y: f32, z: f32) -> Mat4 {
    Mat4 { matrix: [
      x, 0.0, 0.0, 0.0,
      0.0, y, 0.0, 0.0,
      0.0, 0.0, z, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ] }
  }

  pub fn rotation_x(rad: f32) -> Mat4 {
    Mat4 { matrix: [
      1.0, 0.0, 0.0, 0.0,
      0.0, rad.cos(), -rad.sin(), 0.0,
      0.0, rad.sin(), rad.cos(), 0.0,
      0.0, 0.0, 0.0, 1.0,
    ] }
  }

  pub fn identity() -> Mat4 {
    Mat4::new([
      1.0, 0.0, 0.0, 0.0,
      0.0, 1.0, 0.0, 0.0,
      0.0, 0.0, 1.0, 0.0,
      0.0, 0.0, 0.0, 1.0
    ])
  }

  pub fn transpose(&self) -> Mat4 {
    Mat4::new([
      self.matrix[0], self.matrix[4], self.matrix[8], self.matrix[12],
      self.matrix[1], self.matrix[5], self.matrix[9], self.matrix[13],
      self.matrix[2], self.matrix[6], self.matrix[10], self.matrix[14],
      self.matrix[3], self.matrix[7], self.matrix[11], self.matrix[15],
    ])
  }

  pub fn submatrix(&self, row: usize, col: usize) -> Mat3 {
    let mut m3 = [0.0; 9];
    let mut idx = 0;

    for i in 0..4 {
      if i == row {
        continue;
      }

      for j in 0..4 {
        if j == col {
          continue;
        }

        m3[idx] = self[(i, j)];
        idx += 1;
      }
    }

    Mat3::new(m3)
  }

  pub fn minor(&self, row: usize, col: usize) -> f32 {
    self.submatrix(row, col).determinant()
  }

  pub fn cofactor(&self, row: usize, col: usize) -> f32 {
    if (row + col) % 2 == 0 {
      self.minor(row, col)
    } else {
      -self.minor(row, col)
    }
  }

  pub fn determinant(&self) -> f32 {
    let mut total = 0.0;
    for idx in 0..4 {
      total += self[(0, idx)] * self.cofactor(0, idx);
    }
    total
  }

  pub fn is_invertible(&self) -> bool {
    !float::eq(self.determinant(), 0.0)
  }

  pub fn inverse(&self) -> Mat4 {
    let mut m4 = [0.0; 16];
    let determinant = self.determinant();
    for row in 0..4 {
      for col in 0..4 {
        let c = self.cofactor(row, col);
        m4[col * 4 + row] = c / determinant;
      }
    }
    Mat4::new(m4)
  }
}

impl Mat3 {
  pub fn new(matrix: [f32; 9]) -> Mat3 {
    Mat3 { matrix }
  }

  pub fn submatrix(&self, row: usize, col: usize) -> Mat2 {
    let mut m2 = [0.0; 4];
    let mut idx = 0;

    for i in 0..3 {
      if i == row {
        continue;
      }

      for j in 0..3 {
        if j == col {
          continue;
        }

        m2[idx] = self[(i, j)];

        idx += 1;
      }
    }

    Mat2::new(m2)
  }

  pub fn minor(&self, row: usize, col: usize) -> f32 {
    self.submatrix(row, col).determinant()
  }

  pub fn cofactor(&self, row: usize, col: usize) -> f32 {
    if (row + col) % 2 == 0 {
      self.minor(row, col)
    } else {
      -self.minor(row, col)
    }
  }

  pub fn determinant(&self) -> f32 {
    let mut total = 0.0;
    for idx in 0..3 {
      total += self[(0, idx)] * self.cofactor(0, idx);
    }
    total
  }
}

impl Mat2 {
  pub fn new(matrix: [f32; 4]) -> Mat2 {
    Mat2 { matrix }
  }

  pub fn determinant(&self) -> f32 {
    self.matrix[0] * self.matrix[3] - self.matrix[1] * self.matrix[2]
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

  #[test]
  fn test_mat4_transpose() {
    let a = Mat4::new([
      0.0, 9.0, 3.0, 0.0,
      9.0, 8.0, 0.0, 8.0,
      1.0, 8.0, 5.0, 3.0,
      0.0, 0.0, 5.0, 8.0,
    ]);

    let result = Mat4::new([
      0.0, 9.0, 1.0, 0.0,
      9.0, 8.0, 8.0, 0.0,
      3.0, 0.0, 5.0, 5.0,
      0.0, 8.0, 3.0, 8.0,
    ]);

    assert_eq!(a.transpose(), result);
  }

  #[test]
  fn test_mat4_identity_transpose() {
    assert_eq!(Mat4::identity().transpose(), Mat4::identity());
  }

  #[test]
  fn test_mat2_determinant() {
    assert!(float::eq(Mat2::new([1.0, 5.0, -3.0, 2.0]).determinant(), 17.0));
  }

  #[test]
  fn test_mat4_submatrix() {
    let a = Mat4::new([
      -6.0, 1.0, 1.0, 6.0,
      -8.0, 5.0, 8.0, 6.0,
      -1.0, 0.0, 8.0, 2.0,
      -7.0, 1.0, -1.0, 1.0,
    ]);

    let result = Mat3::new([
      -6.0, 1.0, 6.0,
      -8.0, 8.0, 6.0,
      -7.0, -1.0, 1.0,
    ]);

    assert_eq!(a.submatrix(2, 1), result);
  }

  #[test]
  fn test_mat3_submatrix() {
    let a = Mat3::new([
      1.0, 5.0, 0.0,
      -3.0, 2.0, 7.0,
      0.0, 6.0, -3.0,
    ]);

    let result = Mat2::new([
      -3.0, 2.0,
      0.0, 6.0,
    ]);

    assert_eq!(a.submatrix(0, 2), result);
  }

  #[test]
  fn test_mat3_minor() {
    let a = Mat3::new([
      3.0, 5.0, 0.0,
      2.0, -1.0, -7.0,
      6.0, -1.0, 5.0,
    ]);

    assert!(float::eq(a.minor(1, 0), 25.0));
  }

  #[test]
  fn test_mat3_cofactor() {
    let a = Mat3::new([
      3.0, 5.0, 0.0,
      2.0, -1.0, -7.0,
      6.0, -1.0, 5.0,
    ]);

    assert!(float::eq(a.cofactor(1, 0), -25.0));
  }

  #[test]
  fn test_mat3_determinant() {
    let a = Mat3::new([
      1.0, 2.0, 6.0,
      -5.0, 8.0, -4.0,
      2.0, 6.0, 4.0,
    ]);

    assert!(float::eq(a.cofactor(0, 0), 56.0));
    assert!(float::eq(a.cofactor(0, 1), 12.0));
    assert!(float::eq(a.cofactor(0, 2), -46.0));
    assert!(float::eq(a.determinant(), -196.0));
  }

  #[test]
  fn test_mat4_determinant() {
    let a = Mat4::new([
      -2.0, -8.0, 3.0, 5.0,
      -3.0, 1.0, 7.0, 3.0,
      1.0, 2.0, -9.0, 6.0,
      -6.0, 7.0, 7.0, -9.0,
    ]);

    assert!(float::eq(a.cofactor(0, 0), 690.0));
    assert!(float::eq(a.cofactor(0, 1), 447.0));
    assert!(float::eq(a.cofactor(0, 2), 210.0));
    assert!(float::eq(a.cofactor(0, 3), 51.0));
    assert!(float::eq(a.determinant(), -4071.0));
  }

  #[test]
  fn test_mat4_is_invertible() {
    let a = Mat4::new([
      6.0, 4.0, 4.0, 4.0,
      5.0, 5.0, 7.0, 6.0,
      4.0, -9.0, 3.0, -7.0,
      9.0, 1.0, 7.0, -6.0,
    ]);

    assert!(a.is_invertible())
  }

  #[test]
  fn test_mat4_is_not_invertible() {
    let a = Mat4::new([
      -4.0, 2.0, -2.0, -3.0,
      9.0, 6.0, 2.0, 6.0,
      0.0, -5.0, 1.0, -5.0,
      0.0, 0.0, 0.0, 0.0,
    ]);

    assert!(!a.is_invertible())
  }

  #[test]
  fn test_mat4_inverse() {
    let a = Mat4::new([
      -5.0, 2.0, 6.0, -8.0,
      1.0, -5.0, 1.0, 8.0,
      7.0, 7.0, -6.0, -7.0,
      1.0, -3.0, 7.0, 4.0,
    ]);

    let b = a.inverse();
    let b_result = Mat4::new([
      0.21805, 0.45113, 0.24060, -0.04511,
      -0.80827, -1.45677, -0.44361, 0.52068,
      -0.07895, -0.22368, -0.05263, 0.19737,
      -0.52256, -0.81391, -0.30075, 0.30639,
    ]);

    assert!(float::eq(a.determinant(), 532.0));
    assert!(float::eq(a.cofactor(2, 3), -160.0));
    assert!(float::eq(b[(3, 2)], -160.0/532.0));
    assert!(float::eq(a.cofactor(3, 2), 105.0));
    assert!(float::eq(b[(2, 3)], 105.0/532.0));

    assert_eq!(b, b_result);
  }

  #[test]
  fn test_mat4_inverse_2() {
    let a = Mat4::new([
      8.0, -5.0, 9.0, 2.0,
      7.0, 5.0, 6.0, 1.0,
      -6.0, 0.0, 9.0, 6.0,
      -3.0, 0.0, -9.0, -4.0,
    ]);

    let b = a.inverse();
    let b_result = Mat4::new([
      -0.15385, -0.15385, -0.28205, -0.53846,
      -0.07692, 0.12308, 0.02564, 0.03077,
      0.35897, 0.35897, 0.43590, 0.92308,
      -0.69231, -0.69231, -0.76923, -1.92308,
    ]);

    assert_eq!(b, b_result);
  }

  #[test]
  fn test_mat4_inverse_3() {
    let a = Mat4::new([
      9.0, 3.0, 0.0, 9.0,
      -5.0, -2.0, -6.0, -3.0,
      -4.0, 9.0, 6.0, 4.0,
      -7.0, 6.0, 6.0, 2.0,
    ]);

    let b = a.inverse();
    let b_result = Mat4::new([
      -0.04074, -0.07778, 0.14444, -0.22222,
      -0.07778, 0.03333, 0.36667, -0.33333,
      -0.02901, -0.14630, -0.10926, 0.12963,
      0.17778, 0.06667, -0.26667, 0.33333,
    ]);

    assert_eq!(b, b_result);
  }

  #[test]
  fn test_mat4_product_inverse() {
    let a = Mat4::new([
      3.0, -9.0, 7.0, 3.0,
      3.0, -8.0, 2.0, -9.0,
      -4.0, 4.0, 4.0, 1.0,
      -6.0, 5.0, -1.0, 1.0,
    ]);

    let b = Mat4::new([
      8.0, 2.0, 2.0, 2.0,
      3.0, -1.0, 7.0, 0.0,
      7.0, 0.0, 5.0, 4.0,
      6.0, -2.0, 0.0, 5.0,
    ]);

    let c = a * b;

    assert_eq!(c * b.inverse(), a);
  }

  #[test]
  fn test_mat4_translation() {
    let t = Mat4::translation(5.0, -3.0, 2.0);
    let p = Tuple::point(-3.0, 4.0, 5.0);
    let r = Tuple::point(2.0, 1.0, 7.0);

    assert_eq!(t * p, r);
  }

  #[test]
  fn test_mat4_translation_inverse() {
    let t = Mat4::translation(5.0, -3.0, 2.0);
    let p = Tuple::point(-3.0, 4.0, 5.0);
    let r = Tuple::point(-8.0, 7.0, 3.0);

    assert_eq!(t.inverse() * p, r);
  }

  #[test]
  fn test_mat4_translation_against_vector() {
    let t = Mat4::translation(5.0, -3.0, 2.0);
    let v = Tuple::vector(-3.0, 4.0, 5.0);

    assert_eq!(t * v, v);
  }

  #[test]
  fn test_mat4_scaling() {
    let t = Mat4::scaling(2.0, 3.0, 4.0);
    let p = Tuple::point(-4.0, 6.0, 8.0);
    let r = Tuple::point(-8.0, 18.0, 32.0);

    assert_eq!(t * p, r);
  }

  #[test]
  fn test_mat4_scaling_vector() {
    let t = Mat4::scaling(2.0, 3.0, 4.0);
    let p = Tuple::vector(-4.0, 6.0, 8.0);
    let r = Tuple::vector(-8.0, 18.0, 32.0);

    assert_eq!(t * p, r);
  }

  #[test]
  fn test_mat4_scaling_vector_inverse() {
    let t = Mat4::scaling(2.0, 3.0, 4.0);
    let p = Tuple::vector(-4.0, 6.0, 8.0);
    let r = Tuple::vector(-2.0, 2.0, 2.0);

    assert_eq!(t.inverse() * p, r);
  }

  #[test]
  fn test_mat4_scaling_reflection() {
    let t = Mat4::scaling(-1.0, 1.0, 1.0);
    let p = Tuple::vector(2.0, 3.0, 4.0);
    let r = Tuple::vector(-2.0, 3.0, 4.0);

    assert_eq!(t * p, r);
  }

  #[test]
  fn test_mat4_rot_x() {
    let p = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter = Mat4::rotation_x(std::f32::consts::PI / 4.0);
    let full_quarter = Mat4::rotation_x(std::f32::consts::PI / 2.0);

    assert_eq!(half_quarter * p, Tuple::point(0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0));
    assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
  }

  #[test]
  fn test_mat4_rot_x_inverse() {
    let p = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter = Mat4::rotation_x(std::f32::consts::PI / 4.0);
    let full_quarter = Mat4::rotation_x(std::f32::consts::PI / 2.0);

    assert_eq!(half_quarter * p, Tuple::point(0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0));
    assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
  }
}