mod tuple;
mod float;
mod color;
mod canvas;
mod matrix;

use tuple::Tuple;
use canvas::Canvas;
use color::Color;

use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct Projectile {
   position: Tuple,
   velocity: Tuple,
}

#[derive(Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    Projectile{
        position: projectile.position + projectile.velocity,
        velocity: projectile.velocity + environment.gravity + environment.wind,
    }
}

fn main() -> std::io::Result<()> {
    let mut projectile = Projectile{
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let environment = Environment{
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(900, 550);

    while projectile.position.y() > 0.0 {
        projectile = tick(&environment, &projectile);

        let x = projectile.position.x().round() as usize;
        let y = (550.0 - projectile.position.y()).round() as usize;
        if !canvas.is_in_bounds(x, y) {
            continue;
        }
        canvas[(x, y)] = Color::new(1.0, 1.0, 1.0);
    }

    println!("writing file...");
    let mut file = File::create("out.ppm")?;
    write!(file, "{}", canvas.to_ppm())?;

    Ok(())
}
