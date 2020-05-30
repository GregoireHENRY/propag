use crate::propag;
use propag::{Propag};

fn rkfn(propag: Propag, _it: usize)
-> Propag
{
    let mut new_propag = propag.clone();
    for ii in propag.propagate_index..propag.nbody {
        new_propag.vx[ii] = 0.;
        new_propag.vy[ii] = 0.;
        new_propag.vz[ii] = 0.;
        for jj in 0..propag.nbody {
            if propag.names[ii] == propag.names[jj] { continue; }
            let distance2 = propag.distance(ii, jj).powi(2);
            new_propag.vx[ii] -= propag::G * propag.masses[jj] * propag.xdirection(ii, jj) / distance2;
            new_propag.vy[ii] -= propag::G * propag.masses[jj] * propag.ydirection(ii, jj) / distance2;
            new_propag.vz[ii] -= propag::G * propag.masses[jj] * propag.zdirection(ii, jj) / distance2;
        }
        new_propag.x[ii] = propag.vx[ii];
        new_propag.y[ii] = propag.vy[ii];
        new_propag.z[ii] = propag.vz[ii];
    }
    new_propag
}

pub fn rk(mut propag: &mut Propag)
{
    for ii in 0..1 {//time.list.len() {
        let k1 = propag.time_step() * rkfn(propag.clone(), ii);
        let k2 = propag.time_step() * rkfn(propag.clone() + k1.clone() / 2., ii);
        let k3 = propag.time_step() * rkfn(propag.clone() + k2.clone() / 2., ii);
        let k4 = propag.time_step() * rkfn(propag.clone() + k3.clone(), ii);
        propag = propag + (k1 + 2.0 * (k2 + k3) + k4) / 6.0;
    }
    propag.display_label();
    propag.display(1);
}
