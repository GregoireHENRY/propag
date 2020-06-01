use ndarray::prelude::*;

#[derive(Default)]
pub struct States {
    pub x: Array1<f64>,
    pub y: Array1<f64>,
    pub z: Array1<f64>,
    pub vx: Array1<f64>,
    pub vy: Array1<f64>,
    pub vz: Array1<f64>,
}

impl States {
    pub fn init(&mut self, nbody: usize) {
        self.x = Array1::default(nbody);
        self.y = Array1::default(nbody);
        self.z = Array1::default(nbody);
        self.vx = Array1::default(nbody);
        self.vy = Array1::default(nbody);
        self.vz = Array1::default(nbody);
    }
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
