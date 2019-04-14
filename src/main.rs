#![allow(dead_code)]

mod tuple;
mod float;
mod color;
mod canvas;
mod matrix;
mod ray;

use matrix::*;
use tuple::*;
use canvas::*;
use color::*;

use std::f32::consts::{PI};
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let clock_size = 200.0;
    let padding = 20.0;
    let dim = (clock_size * 2.0 + padding * 2.0) as usize;

    let mut c = Canvas::new(dim, dim);
    let p = Tuple::point(0.0, 0.0, 1.0);

    for i in 0..12 {
        let transform =
            Mat4::identity()
                .rotate_y(i as f32 * (PI/6.0))
                .translate(1.0, 0.0, 1.0)
                .scale(clock_size, 0.0, clock_size)
                .translate(padding, 0.0, padding);
        let np = transform * p;
        let x = np.x() as usize;
        let y = np.z() as usize;
        c[(x, y)] = Color::new(255.0, 255.0, 255.0);
    }

    File::create("out.ppm")?.write_all(c.to_ppm().as_bytes())?;
    Ok(())
}
