mod tuple;
mod float;
mod color;
mod canvas;
mod matrix;

use matrix::*;

fn main() -> std::io::Result<()> {
    let m = Mat4::new([
        1.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]);

    let a = Mat4::new([
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
    ]);

    println!("{}", a * m);

    Ok(())
}
