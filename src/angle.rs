pub enum Angle {
    Deg(f64),
    Rad(f64),
}

impl Angle {
    pub fn as_rad(self) -> f64 {
        match self {
            Angle::Deg(d) => d.to_radians(),
            Angle::Rad(r) => r,
        }
    }

    pub fn as_deg(self) -> f64 {
        match self {
            Angle::Deg(d) => d,
            Angle::Rad(r) => r.to_degrees(),
        }
    }
}
