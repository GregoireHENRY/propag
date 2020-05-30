use crate::propag;
use propag::{math, Propag};

use ndarray::prelude::*;

fn rkfn(propag: &Propag, p: Array2<f64>)
-> Array2<f64>
{
    let mut new_p = p.clone();
    for ii in propag.propagate_index..propag.nbody {
        new_p[[ii, 3]] = 0.;
        new_p[[ii, 4]] = 0.;
        new_p[[ii, 5]] = 0.;
        for jj in 0..propag.nbody {
            if ii == jj { continue; }
            let distance2 = math::distance(p[[ii, 0]], p[[ii, 1]], p[[ii, 2]], p[[jj, 0]], p[[jj, 1]], p[[jj, 2]]).powi(2);
            new_p[[ii, 3]] -= propag::G * propag.masses[jj] * math::xdirection(p[[ii, 0]], p[[ii, 1]], p[[ii, 2]], p[[jj, 0]], p[[jj, 1]], p[[jj, 2]]) / distance2;
            new_p[[ii, 4]] -= propag::G * propag.masses[jj] * math::ydirection(p[[ii, 0]], p[[ii, 1]], p[[ii, 2]], p[[jj, 0]], p[[jj, 1]], p[[jj, 2]]) / distance2;
            new_p[[ii, 5]] -= propag::G * propag.masses[jj] * math::zdirection(p[[ii, 0]], p[[ii, 1]], p[[ii, 2]], p[[jj, 0]], p[[jj, 1]], p[[jj, 2]]) / distance2;
        }
        new_p[[ii, 0]] = p[[ii, 3]];
        new_p[[ii, 1]] = p[[ii, 4]];
        new_p[[ii, 2]] = p[[ii, 5]];
    }
    new_p
}

pub fn rk(propag: &mut Propag)
{
    let mut p = propag.get_state(0);
    for ii in 0..1 {//time.list.len() {
        let k1 = propag.time_step() * rkfn(&propag, p.clone());
        let k2 = propag.time_step() * rkfn(&propag, p.clone() + k1.clone() / 2.);
        let k3 = propag.time_step() * rkfn(&propag, p.clone() + k2.clone() / 2.);
        let k4 = propag.time_step() * rkfn(&propag, p.clone() + k3.clone());
        p = p + (k1 + 2.0 * (k2 + k3) + k4) / 6.0;
        propag.set_state(ii, &p);
    }
    propag.display_label();
    propag.display(1, 0);
}
