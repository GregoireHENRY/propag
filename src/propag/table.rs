use ndarray::prelude::*;

#[derive(Debug, Clone)]
pub struct Table
{
    pub names: Array1<&'static str>,
    pub masses: Array1<f64>,
    pub radii: Array1<f64>,
    pub x: Array1<f64>,
    pub y: Array1<f64>,
    pub z: Array1<f64>,
    pub vx: Array1<f64>,
    pub vy: Array1<f64>,
    pub vz: Array1<f64>,
}

impl Table
{
    pub fn init(nbody: usize)
    -> Self
    {
        Self{
            names: Array1::default(nbody),
            masses: Array1::default(nbody),
            radii: Array1::default(nbody),
            x: Array1::default(nbody),
            y: Array1::default(nbody),
            z: Array1::default(nbody),
            vx: Array1::default(nbody),
            vy: Array1::default(nbody),
            vz: Array1::default(nbody),
        }
    }
}
