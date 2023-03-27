#[derive(Copy, Clone)]
pub struct Vec3 {
    p: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {p: [x, y, z]}
    }

    pub fn x(&self) -> f64 {
        self.p[0]
    }
    
    pub fn r(&self) -> f64 {
        self.p[0]
    }

    pub fn y(&self) -> f64 {
        self.p[1]
    }
    
    pub fn g(&self) -> f64 {
        self.p[1]
    }
    pub fn z(&self) -> f64 {
        self.p[2]
    }
    
    pub fn b(&self) -> f64 {
        self.p[2]
    }

}