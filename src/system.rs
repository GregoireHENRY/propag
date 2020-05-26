use crate::body::{Bodies};
use std::collections::{HashMap};

//pub const AU:       f64 = 149597870700.; 
pub const G:        f64 = 6.67408e-11; 
pub const DAY:      f64 = 86400.;
//pub const SUN_FLUX: f64 = 1371.;
//pub const VLIGHT:   f64 = 3e8;

pub struct System
{
    pub frame: &'static str,
    pub bodies: Bodies,
}

pub fn new(frame: &'static str)
-> System
{
    System{frame: frame, bodies: Bodies(HashMap::new())}
}


