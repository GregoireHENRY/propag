use ndarray::prelude::*;

pub fn linspace(init: f64, end: f64, step: f64)
-> Array1<f64>
{
    let n = ((end - init) / step ) as usize + 1;
    let s = (end - init) / (n - 1) as f64;
    let mut arr = Array1::default(n);
    for ii in 0..n {
        arr[ii] = init + ii as f64 * s;
    }
    arr
}

pub fn dot((x1, y1, z1): (f64, f64, f64), (x2, y2, z2): (f64, f64, f64)) -> f64 { x1 * x2 + y1 * y2 + z1 * z2 }
pub fn norm((x, y, z): (f64, f64, f64)) -> f64 { dot((x, y, z), (x, y, z)).sqrt() }
pub fn unitx((x, y, z): (f64, f64, f64)) -> f64 { x / norm((x, y, z)) }
pub fn unity((x, y, z): (f64, f64, f64)) -> f64 { y / norm((x, y, z)) }
pub fn unitz((x, y, z): (f64, f64, f64)) -> f64 { z / norm((x, y, z)) }
