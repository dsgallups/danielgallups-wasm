pub struct Space {
    stars: Vec<Star>,
}

impl Space {
    pub fn new() -> Self {
        Space { stars: Vec::new() }
    }
}

struct Star {
    mass: f64,
    position: (f64, f64),
    radius: f64,
}
