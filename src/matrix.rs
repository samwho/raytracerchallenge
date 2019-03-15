use std::ops::{Index, IndexMut};
use super::float;

#[derive(Debug, Copy, Clone)]
pub struct Matrix {
  dimensions: (usize, usize),
  matrix: [f32; 16],
}

impl Index<(usize, usize)> for Matrix {
  type Output = f32;
  fn index(&self, idx: (usize, usize)) -> &f32 {
    &self.matrix[idx.0 * self.dimensions.0 + idx.1]
  }
}

impl IndexMut<(usize, usize)> for Matrix {
  fn index_mut(&mut self, idx: (usize, usize)) -> &mut f32 {
    &mut self.matrix[idx.0 * 4 + idx.1]
  }
}

impl Matrix {
  pub fn mat4(matrix: [f32; 16]) -> Matrix {
    Matrix { dimensions: (4, 4), matrix }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let m4 = Matrix::mat4([
      1.0,  2.0,  3.0,  4.0,
      5.5,  6.5,  7.5,  8.5,
      9.0,  10.0, 11.0, 12.0,
      13.5, 14.5, 15.5, 16.5
    ]);

    assert!(float::eq(m4[(0, 0)], 1.0));
    assert!(float::eq(m4[(0, 3)], 4.0));
    assert!(float::eq(m4[(1, 0)], 5.5));
    assert!(float::eq(m4[(1, 2)], 7.5));
    assert!(float::eq(m4[(2, 2)], 11.0));
    assert!(float::eq(m4[(3, 0)], 13.5));
    assert!(float::eq(m4[(3, 2)], 15.5));
  }
}