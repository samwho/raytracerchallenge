use super::float;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Color {
  red: f32,
  green: f32,
  blue: f32,
}

impl Color {
  pub fn new(red: f32, green: f32, blue: f32) -> Color {
    Color { red, green, blue }
  }
}

impl PartialEq for Color {
  fn eq(&self, other: &Color) -> bool {
    float::eq(self.red, other.red)
      && float::eq(self.green, other.green)
      && float::eq(self.blue, other.blue)
  }
}
impl Eq for Color {}

impl Add<Color> for Color {
  type Output = Color;
  fn add(self, other: Color) -> Color {
    Color { red: self.red + other.red, green: self.green + other.green, blue: self.blue + other.blue }
  }
}

impl Sub<Color> for Color {
  type Output = Color;
  fn sub(self, other: Color) -> Color {
    Color { red: self.red - other.red, green: self.green - other.green, blue: self.blue - other.blue }
  }
}

impl Mul<f32> for Color {
  type Output = Color;
  fn mul(self, other: f32) -> Color {
    Color { red: self.red * other, green: self.green * other, blue: self.blue * other }
  }
}

impl Mul<Color> for Color {
  type Output = Color;
  fn mul(self, other: Color) -> Color {
    Color { red: self.red * other.red, green: self.green * other.green, blue: self.blue * other.blue }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let c = Color::new(-0.5, 0.4, 1.7);
    assert!(float::eq(c.red, -0.5));
    assert!(float::eq(c.green, 0.4));
    assert!(float::eq(c.blue, 1.7));
  }

  #[test]
  fn test_add() {
    let a = Color::new(0.9, 0.6, 0.75);
    let b = Color::new(0.7, 0.1, 0.25);
    let result = Color::new(1.6, 0.7, 1.0);
    assert!(a + b == result);
  }

  #[test]
  fn test_sub() {
    let a = Color::new(0.9, 0.6, 0.75);
    let b = Color::new(0.7, 0.1, 0.25);
    let result = Color::new(0.2, 0.5, 0.5);
    assert!(a - b == result);
  }

  #[test]
  fn test_mul() {
    let a = Color::new(0.2, 0.3, 0.4);
    let result = Color::new(0.4, 0.6, 0.8);
    assert!(a * 2.0 == result);
  }

  #[test]
  fn test_mul_color() {
    let a = Color::new(1.0, 0.2, 0.4);
    let b = Color::new(0.9, 1.0, 0.1);
    let result = Color::new(0.9, 0.2, 0.04);
    assert!(a * b == result);
  }
}