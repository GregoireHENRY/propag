extern crate itertools;
extern crate ndarray;

mod math;
mod physics;
mod states;

use ndarray::prelude::*;
use states::States;
use std::io::{stdout, Write};

pub const _AU: f64 = 149597870700.;
pub const G: f64 = 6.67408e-11;
pub const DAY: f64 = 86400.;
pub const _SUN_FLUX: f64 = 1371.;
pub const _VLIGHT: f64 = 3e8;

pub struct Propag {
    pub frame: &'static str,
    pub names: Array1<&'static str>,
    pub masses: Array1<f64>,
    pub radii: Array1<f64>,
    pub states: Array1<States>,
    pub time: Array1<f64>,
    pub build_index: usize,
    pub nfrozen: usize,
}

pub fn new(frame: &'static str, nbody: usize, time: Array1<f64>) -> Propag {
    let ntime = time.len();
    let mut states: Array1<States> = Array1::default(ntime);
    for s in states.iter_mut() {
        s.init(nbody);
    }
    Propag {
        frame,
        names: Array1::default(nbody),
        masses: Array1::default(nbody),
        radii: Array1::default(nbody),
        states,
        time,
        build_index: 0,
        nfrozen: 0,
    }
}

impl Propag {
    pub fn add(
        &mut self,
        name: &'static str,
        mass: f64,
        radius: f64,
        x: f64,
        y: f64,
        z: f64,
        vx: f64,
        vy: f64,
        vz: f64,
    ) {
        self.names[self.build_index] = name;
        self.masses[self.build_index] = mass;
        self.radii[self.build_index] = radius;
        self.states[0].set(self.build_index, x, y, z, vx, vy, vz);
        self.build_index += 1;
    }
    pub fn propagate_nexts(&mut self) {
        self.nfrozen = self.build_index;
    }
    pub fn start(&mut self) {
        physics::rk(self);
    }
    pub fn time_step(&self) -> f64 {
        self.time[1] - self.time[0]
    }
    pub fn _display_label(&self) {
        println!(
            "{:>11}{:>20}{:>20}{:>20}{:>20}{:>20}{:>20}",
            "t", "x", "y", "z", "vx", "vy", "vz"
        );
    }
    pub fn _display(&self, ibody: usize, itime: usize) {
        print!("{:11.1}", self.time[itime]);
        stdout().flush().unwrap();
        self.states[itime].display_body(ibody);
    }
}

pub fn set_time(init: f64, end: f64, step: f64) -> Array1<f64> {
    let n = ((end - init) / step) as usize + 1;
    Array1::linspace(init, end, n)
}
