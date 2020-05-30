mod math;
mod physics;

use ndarray::prelude::*;
use ndarray::{stack};

//pub const AU:       f64 = 149597870700.; 
pub const G:        f64 = 6.67408e-11; 
pub const DAY:      f64 = 86400.;
//pub const SUN_FLUX: f64 = 1371.;
//pub const VLIGHT:   f64 = 3e8;

#[derive(Debug, Clone)]
pub struct Propag
{
    /* axis 0: bodies
     * axis 1: time
     */
    pub time: Array1<f64>,
    pub names: Array1<&'static str>,
    pub masses: Array1<f64>,
    pub radii: Array1<f64>,
    pub x: Array2<f64>,
    pub y: Array2<f64>,
    pub z: Array2<f64>,
    pub vx: Array2<f64>,
    pub vy: Array2<f64>,
    pub vz: Array2<f64>,
    pub frame: &'static str,
    pub nbody: usize,
    pub ntime: usize,
    pub current_index: usize,
    pub propagate_index: usize,
}

pub fn new(frame: &'static str, nbody: usize, time: Array1<f64>)
-> Propag
{
    let ntime = time.len();
    let new_array = Array2::default((nbody, ntime));
    Propag{
        frame,
        nbody,
        time,
        ntime,
        names: Array1::default(nbody),
        masses: Array1::default(nbody),
        radii: Array1::default(nbody),
        x: new_array.clone(),
        y: new_array.clone(),
        z: new_array.clone(),
        vx: new_array.clone(),
        vy: new_array.clone(),
        vz: new_array.clone(),
        current_index: 0,
        propagate_index: 0,
    }
}

impl Propag
{
    pub fn add(&mut self, name: &'static str, mass: f64, radius: f64, x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64)
    {
        self.names[self.current_index] = name;
        self.masses[self.current_index] = mass;
        self.radii[self.current_index] = radius;
        self.x[[self.current_index, 0]] = x;
        self.y[[self.current_index, 0]] = y;
        self.z[[self.current_index, 0]] = z;
        self.vx[[self.current_index, 0]] = vx;
        self.vy[[self.current_index, 0]] = vy;
        self.vz[[self.current_index, 0]] = vz;
        self.current_index += 1;
    }
    pub fn propagate_nexts(&mut self) { self.propagate_index = self.current_index; }
    pub fn start(&mut self) { physics::rk(self); }
    pub fn time_step(&self) -> f64 { self.time[1] - self.time[0] }
    pub fn display_label(&self) { println!("{:>20}{:>20}{:>20}{:>20}{:>20}{:>20}", "x", "y", "z", "vx", "vy", "vz"); }
    pub fn display(&self, body: usize, kk: usize) { println!("{:20.10}{:20.10}{:20.10}{:20.10}{:20.10}{:20.10}", self.x[[body, kk]], self.y[[body, kk]], self.z[[body, kk]], self.vx[[body, kk]], self.vy[[body, kk]], self.vz[[body, kk]]); }
    pub fn get_state(&self, kk: usize)
    -> Array2<f64>
    {
        stack(Axis(1),
            &[
                self.x.slice(s![.., kk]) .broadcast((1, self.nbody)).unwrap().t(),
                self.y.slice(s![.., kk]) .broadcast((1, self.nbody)).unwrap().t(),
                self.z.slice(s![.., kk]) .broadcast((1, self.nbody)).unwrap().t(),
                self.vx.slice(s![.., kk]).broadcast((1, self.nbody)).unwrap().t(),
                self.vy.slice(s![.., kk]).broadcast((1, self.nbody)).unwrap().t(),
                self.vz.slice(s![.., kk]).broadcast((1, self.nbody)).unwrap().t(),
            ]
        ).unwrap()
    }
    pub fn set_state(&mut self, kk: usize, p: &Array2<f64>)
    {
        self.x.column_mut(kk) .assign(&p.slice(s![.., 0]));
        self.y.column_mut(kk) .assign(&p.slice(s![.., 1]));
        self.z.column_mut(kk) .assign(&p.slice(s![.., 2]));
        self.vx.column_mut(kk).assign(&p.slice(s![.., 3]));
        self.vy.column_mut(kk).assign(&p.slice(s![.., 4]));
        self.vz.column_mut(kk).assign(&p.slice(s![.., 5]));
    }
}

pub fn set_time(init: f64, end: f64, step: f64)
-> Array1<f64>
{
    let n = ((end - init) / step ) as usize + 1;
    Array1::linspace(init, end, n)
}

/*
impl std::ops::Mul<f64> for Propag
{
    type Output = Self;
    fn mul(mut self, oth: f64)
    -> Self
    {
        for ii in self.propagate_index..self.nbody {
            self.x[ii] *= oth;
            self.y[ii] *= oth;
            self.z[ii] *= oth;
            self.vx[ii] *= oth;
            self.vy[ii] *= oth;
            self.vz[ii] *= oth;
        }
        self
    }
}

impl std::ops::Mul<Propag> for f64
{
    type Output = Propag;
    fn mul(self, oth: Propag)
    -> Propag
    {
        oth * self
    }
}

impl std::ops::Div<f64> for Propag
{
    type Output = Self;
    fn div(self, oth: f64)
    -> Self
    {
        self * (1./oth)
    }
}

impl std::ops::Add<f64> for Propag
{
    type Output = Self;
    fn add(mut self, oth: f64)
    -> Self
    {
        for ii in self.propagate_index..self.nbody {
            self.x[ii] += oth;
            self.y[ii] += oth;
            self.z[ii] += oth;
            self.vx[ii] += oth;
            self.vy[ii] += oth;
            self.vz[ii] += oth;
        }
        self
    }
}

impl std::ops::Add<Propag> for f64
{
    type Output = Propag;
    fn add(self, oth: Propag)
    -> Propag
    {
        oth + self
    }
}

impl std::ops::Add<Propag> for Propag
{
    type Output = Self;
    fn add(mut self, oth: Propag)
    -> Self
    {
        for ii in self.propagate_index..self.nbody {
            self.x[ii] += oth.x[ii];
            self.y[ii] += oth.y[ii];
            self.z[ii] += oth.z[ii];
            self.vx[ii] += oth.vx[ii];
            self.vy[ii] += oth.vy[ii];
            self.vz[ii] += oth.vz[ii];
        }
        self
    }
}

impl std::ops::Add<Propag> for &mut Propag
{
    type Output = Self;
    fn add(self, oth: Propag)
    -> Self
    {
        for ii in self.propagate_index..self.nbody {
            self.x[ii] += oth.x[ii];
            self.y[ii] += oth.y[ii];
            self.z[ii] += oth.z[ii];
            self.vx[ii] += oth.vx[ii];
            self.vy[ii] += oth.vy[ii];
            self.vz[ii] += oth.vz[ii];
        }
        self
    }
}
*/

