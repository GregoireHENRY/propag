extern crate crossbeam;
extern crate itertools;
extern crate log;
extern crate ndarray;
extern crate simplelog;

mod math;
mod physics;
mod states;

use log::*;
use ndarray::prelude::*;
use simplelog::*;
use states::States;
use std::io::{stdout, Write};
use std::time::Instant;
use std::vec::Vec;

pub const _AU: f64 = 149597870700.;
pub const G: f64 = 6.67408e-11;
pub const DAY: f64 = 86400.;
pub const _SUN_FLUX: f64 = 1371.;
pub const _VLIGHT: f64 = 3e8;

pub struct Propag {
    pub frame: &'static str,
    pub origin: &'static str,
    pub names: Array1<&'static str>,
    pub masses: Array1<f64>,
    pub radii: Array1<f64>,
    pub states: States,
    pub time: Array1<f64>,
    pub build_index: usize,
    pub nfrozen: usize,
    pub saves: Vec<usize>,
}

pub fn new(frame: &'static str, origin: &'static str, nbody: usize, time: Array1<f64>) -> Propag {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Warn,
        Config::default(),
        TerminalMode::Mixed,
    )])
    .unwrap();
    Propag {
        frame,
        origin,
        names: Array1::default(nbody),
        masses: Array1::default(nbody),
        radii: Array1::default(nbody),
        states: states::new(nbody),
        time,
        build_index: 0,
        nfrozen: 0,
        saves: vec![],
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
        self.states.set(self.build_index, x, y, z, vx, vy, vz);
        self.build_index += 1;
    }
    pub fn propagate_nexts(&mut self) {
        self.nfrozen = self.build_index;
    }
    pub fn start(&mut self) {
        let tic = Instant::now();
        print!("Retro propagation started..");
        stdout().flush().unwrap();
        physics::rk(self);
        println!(" elapsed time {:.2} seconds.", tic.elapsed().as_secs_f64());
    }
    pub fn time_step(&self) -> f64 {
        self.time[1] - self.time[0]
    }
    pub fn format_label(&self) -> String {
        format!(
            "{:>11}{:>20}{:>20}{:>20}{:>20}{:>20}{:>20}",
            "t", "x", "y", "z", "vx", "vy", "vz"
        )
    }
    #[allow(dead_code)]
    pub fn display_label(&self) {
        println!("{}", self.format_label());
    }
    #[allow(dead_code)]
    pub fn display(&self, itime: usize, ibody: usize) {
        print!("{:11.1}", self.time[itime]);
        self.states.display_body(ibody);
    }
    pub fn save(&mut self, name: &'static str) {
        match self.names.iter().position(|&x| x == name) {
            Some(ibody) => self.saves.push(ibody),
            None => print!("Body {} cannot be saved for it is not found", name),
        };
    }
}

pub fn set_time(init: f64, end: f64, step: f64) -> Array1<f64> {
    let n = ((end - init) / step) as usize + 1;
    Array1::linspace(init, end, n)
}
