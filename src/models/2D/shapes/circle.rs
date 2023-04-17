pub struct Circle{
    pub body: Body,
    pub size: f64,
}

impl Circle {
    pub fn new(position: Point, velocity: Velocity, mass: f64, size: f64) -> Circle {
        let body = Body {
            position,
            velocity,
            acceleration: Acceleration::new(0.0, 0.0),
            mass,
        };
        Circle {
            body,
            size,
        }
    }
}