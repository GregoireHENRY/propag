mod math;
mod physics;
mod table;

use table::Table;
use ndarray::prelude::*;

//pub const AU:       f64 = 149597870700.; 
pub const G:        f64 = 6.67408e-11; 
pub const DAY:      f64 = 86400.;
//pub const SUN_FLUX: f64 = 1371.;
//pub const VLIGHT:   f64 = 3e8;

#[derive(Debug, Clone)]
pub struct Propag
{
    pub frame: &'static str,
    pub nbody: usize,
    pub time: Array1<f64>,
    pub table: Array1<Table>,
    pub current_index: usize,
    pub propagate_index: usize,
}

pub fn new(frame: &'static str, nbody: usize, time: Array1<f64>)
-> Propag
{
    Propag{
        frame: frame,
        nbody: nbody,
        time: time,
        table: Table::init(time.len()),

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
        self.x[self.current_index] = x;
        self.y[self.current_index] = y;
        self.z[self.current_index] = z;
        self.vx[self.current_index] = vx;
        self.vy[self.current_index] = vy;
        self.vz[self.current_index] = vz;
        self.current_index += 1;
    }
    pub fn propagate_nexts(&mut self) { self.propagate_index = self.current_index; }
    pub fn start(&mut self) { physics::rk(self); }
    pub fn sub(&self, ii: usize, jj: usize) -> (f64, f64, f64) { (self.x[ii]-self.x[jj], self.y[ii]-self.y[jj], self.z[ii]-self.z[jj]) }
    pub fn distance(&self, ii: usize, jj: usize) -> f64 { math::norm(self.sub(ii, jj)) }
    pub fn xdirection(&self, ii: usize, jj: usize) -> f64 { math::unitx(self.sub(ii, jj)) }
    pub fn ydirection(&self, ii: usize, jj: usize) -> f64 { math::unity(self.sub(ii, jj)) }
    pub fn zdirection(&self, ii: usize, jj: usize) -> f64 { math::unitz(self.sub(ii, jj)) }
    pub fn time_step(&self) -> f64 { self.time[1] - self.time[0] }
    pub fn display_label(&self) { println!("{:>20}{:>20}{:>20}{:>20}{:>20}{:>20}", "x", "y", "z", "vx", "vy", "vz"); }
    pub fn display(&self, body: usize) { println!("{:20.10}{:20.10}{:20.10}{:20.10}{:20.10}{:20.10}", self.x[body], self.y[body], self.z[body], self.vx[body], self.vy[body], self.vz[body]); }
}

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

pub fn set_time(init: f64, end: f64, step: f64) -> Array1<f64> { math::linspace(init, end, step) }
