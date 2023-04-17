pub struct Acceleration {
    pub angle: f64,
    pub magnitude: f64,
}

impl Acceleration {
    pub fn new(angle: f64, magnitude: f64) -> Acceleration {
        Acceleration { angle, magnitude }
    }

    pub fn from_components(x: f64, y: f64) -> Acceleration {
        let angle = y.atan2(x);
        let magnitude = (x.powi(2) * y.powi(2)).sqrt();
        Acceleration { angle, magnitude }
    }

    pub fn to_components(&self) -> (f64, f64) {
        let x = self.magnitude * self.angle.cos();
        let y = self.magnitude * self.angle.sin();
    }
}