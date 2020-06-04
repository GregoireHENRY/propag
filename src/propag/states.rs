use ndarray::prelude::*;

#[derive(Default, Clone)]
pub struct States {
    pub x: Array1<f64>,
    pub y: Array1<f64>,
    pub z: Array1<f64>,
    pub vx: Array1<f64>,
    pub vy: Array1<f64>,
    pub vz: Array1<f64>,
}

pub fn new(nbody: usize) -> States {
    States {
        x: Array1::default(nbody),
        y: Array1::default(nbody),
        z: Array1::default(nbody),
        vx: Array1::default(nbody),
        vy: Array1::default(nbody),
        vz: Array1::default(nbody),
    }
}

impl States {
    pub fn set(&mut self, ibody: usize, x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) {
        self.x[ibody] = x;
        self.y[ibody] = y;
        self.z[ibody] = z;
        self.vx[ibody] = vx;
        self.vy[ibody] = vy;
        self.vz[ibody] = vz;
    }
    pub fn display_body(&self, ibody: usize) {
        println!(
            "{:20.10}{:20.10}{:20.10}{:20.10}{:20.10}{:20.10}",
            self.x[ibody],
            self.y[ibody],
            self.z[ibody],
            self.vx[ibody],
            self.vy[ibody],
            self.vz[ibody]
        );
    }
}
impl std::ops::Mul<f64> for States {
    type Output = Self;
    fn mul(mut self, oth: f64) -> Self {
        self.x *= oth;
        self.y *= oth;
        self.z *= oth;
        self.vx *= oth;
        self.vy *= oth;
        self.vz *= oth;
        self
    }
}
impl std::ops::Mul<States> for f64 {
    type Output = States;
    fn mul(self, oth: States) -> States {
        oth * self
    }
}
impl std::ops::Div<f64> for States {
    type Output = Self;
    fn div(self, oth: f64) -> States {
        self * (1. / oth)
    }
}
impl std::ops::Add for States {
    type Output = Self;
    fn add(mut self, oth: Self) -> Self {
        self.x = self.x + oth.x;
        self.y = self.y + oth.y;
        self.z = self.z + oth.z;
        self.vx = self.vx + oth.vx;
        self.vy = self.vy + oth.vy;
        self.vz = self.vz + oth.vz;
        self
    }
}
impl std::ops::AddAssign for States {
    fn add_assign(&mut self, oth: Self) {
        *self = self.clone() + oth;
    }
}
