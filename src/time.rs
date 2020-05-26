use crate::math;
use std::vec::{Vec};

pub struct Time
{
    pub init: f64,
    pub end: f64,
    pub step: f64,
    pub n: usize,
    pub list: Vec<f64>,
}

pub fn set(init: f64, end: f64, step: f64)
-> Time
{
    let n = ((end - init) / step ) as usize + 1;
    Time{init: init, end: end, step: step, n: n, list: math::linspace(init, end, n)}
}
