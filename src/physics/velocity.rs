pub struct Velocity {
    pub angle: f64,
    pub magnitude: f64,
}

impl Velocity {
    pub fn new(angle: f64, magnitude: f64) -> Velocity {
        Velocity { angle, magnitude }
    }

    pub fn from_components(x: f64, y: f64) -> Velocity {
        let angle = y.atan2(x);
        let magnitude = (x.powi(2) + y.powi(2)).sqrt();
        Velocity { angle, magnitude }
    }

    pub fn to_components(&self) -> (f64, f64) {
        let x = self.magnitude * self.angle.cos();
        let y = self.magnitude * self.angle.sin();
        (x, y)
    }
}