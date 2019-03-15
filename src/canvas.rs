use super::color::Color;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Canvas {
  width: usize,
  height: usize,
  cells: Vec<Color>,
}

impl Index<(usize, usize)> for Canvas {
  type Output = Color;
  fn index(&self, idx: (usize, usize)) -> &Color {
    &self.cells[idx.1 * self.height + idx.0]
  }
}

impl IndexMut<(usize, usize)> for Canvas {
  fn index_mut(&mut self, idx: (usize, usize)) -> &mut Color {
    &mut self.cells[idx.1 * self.height + idx.0]
  }
}

impl Canvas {
  pub fn new(width: usize, height: usize) -> Canvas {
    let mut cells = Vec::with_capacity(width * height);
    for _ in 0..(width * height) {
      cells.push(Color::new(0.0, 0.0, 0.0));
    }

    Canvas { height, width, cells }
  }

  pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
    x < self.width && y < self.height
  }

  pub fn to_ppm(&self) -> String {
    let mut s = String::with_capacity(self.width * self.height * 12 + 64);
    s.push_str("P3\n");
    s.push_str(&self.width.to_string());
    s.push_str(" ");
    s.push_str(&self.height.to_string());
    s.push_str("\n");
    s.push_str("255\n");

    for y in 0..self.height {
      let mut pixels: Vec<String> = Vec::with_capacity(self.width);

      for x in 0..self.width {
        let pixel = self[(x, y)];
        pixels.push(format!("{} {} {}", pixel.red_u8(), pixel.green_u8(), pixel.blue_u8()));
      }

      s.push_str(&format!("{}\n", &pixels.join(" ")));
    }

    s
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let canvas = Canvas::new(10, 10);
    for x in 0..10 {
      for y in 0..10 {
        assert_eq!(canvas[(x, y)], Color::new(0.0, 0.0, 0.0));
      }
    }
  }

  #[test]
  fn test_set() {
    let mut canvas = Canvas::new(10, 10);
    canvas[(0, 0)] = Color::new(1.0, 0.0, 0.0);
    canvas[(0, 1)] = Color::new(0.0, 1.0, 0.0);
    canvas[(1, 0)] = Color::new(0.0, 0.0, 1.0);

    assert_eq!(canvas[(0, 0)], Color::new(1.0, 0.0, 0.0));
    assert_eq!(canvas[(0, 1)], Color::new(0.0, 1.0, 0.0));
    assert_eq!(canvas[(1, 0)], Color::new(0.0, 0.0, 1.0));
  }

  #[test]
  fn test_to_ppm() {
    let mut canvas = Canvas::new(5, 3);
    canvas[(0, 0)] = Color::new(1.5, 0.0, 0.0);
    canvas[(2, 1)] = Color::new(0.0, 0.5, 0.0);
    canvas[(4, 2)] = Color::new(-0.5, 0.0, 1.0);

    let ppm = canvas.to_ppm();
    let lines: Vec<&str> = ppm.lines().collect();

    assert_eq!(lines[0], "P3");
    assert_eq!(lines[1], "5 3");
    assert_eq!(lines[2], "255");

    assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
    assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
    assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");

    let last_char = ppm.chars().last().unwrap();
    assert_eq!(last_char, '\n');
  }

  /*
  The book recommends having this test, but feh reads the image file just file and wrapping
  the lines was very slow, so I got rid of it.

  #[test]
  fn test_to_ppm_long_lines() {
    let mut canvas = Canvas::new(10, 2);
    for x in 0..10 {
      for y in 0..2 {
        canvas.set(x, y, Color::new(1.0, 0.8, 0.6));
      }
    }

    let ppm = canvas.to_ppm();
    let lines: Vec<&str> = ppm.lines().collect();

    assert_eq!(lines[3], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
    assert_eq!(lines[4], "153 255 204 153 255 204 153 255 204 153 255 204 153");
    assert_eq!(lines[5], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
    assert_eq!(lines[6], "153 255 204 153 255 204 153 255 204 153 255 204 153");
  }
  */
}