use super::color::{Color};

struct Canvas {
  cells: Vec<Vec<Color>>
}

impl Canvas {
  fn new(width: usize, height: usize) -> Canvas {
    let mut cells = Vec::with_capacity(width);
    for _ in 0..width {
      cells.push(Vec::with_capacity(height));
    }

    for x in 0..width {
      for _ in 0..height {
        cells[x].push(Color::new(0.0, 0.0, 0.0));
      }
    }

    Canvas { cells }
  }

  fn set(&mut self, x: usize, y: usize, color: Color) {
    self.cells[x][y] = color
  }

  fn get(&self, x: usize, y: usize) -> Color {
    self.cells[x][y]
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
        assert_eq!(canvas.get(x, y), Color::new(0.0, 0.0, 0.0));
      }
    }
  }

  #[test]
  fn test_set() {
    let mut canvas = Canvas::new(10, 10);
    canvas.set(0, 0, Color::new(1.0, 0.0, 0.0));
    canvas.set(0, 1, Color::new(0.0, 1.0, 0.0));
    canvas.set(1, 0, Color::new(0.0, 0.0, 1.0));

    assert_eq!(canvas.get(0, 0), Color::new(1.0, 0.0, 0.0));
    assert_eq!(canvas.get(0, 1), Color::new(0.0, 1.0, 0.0));
    assert_eq!(canvas.get(1, 0), Color::new(0.0, 0.0, 1.0));
  }
}