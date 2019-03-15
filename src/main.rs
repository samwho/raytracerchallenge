mod tuple;
mod float;
mod color;
mod canvas;

use tuple::Tuple;

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

fn main() {
    let mut projectile = Projectile{
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize(),
    };

    let environment = Environment{
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(0.01, 0.0, 0.0),
    };

    while projectile.position.y() > 0.0 {
        println!("{:?}", projectile);
        projectile = tick(&environment, &projectile);
    }
}
