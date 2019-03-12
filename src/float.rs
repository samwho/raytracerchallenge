const EPSILON: f32 = 0.00001;

pub fn eq(a: f32, b: f32) -> bool {
  (a - b).abs() < EPSILON
}