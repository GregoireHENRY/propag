use crate::propag;
use itertools::izip;
use propag::{Propag, States};

fn rkfn(propag: &Propag, mut s: States) -> States {
    let mut new_s = s.clone();
    for (name, x, y, z, vx, vy, vz) in izip!(
        propag.names.iter(),
        new_s.x.iter_mut(),
        new_s.y.iter_mut(),
        new_s.z.iter_mut(),
        new_s.vx.iter_mut(),
        new_s.vy.iter_mut(),
        new_s.vz.iter_mut()
    ) {
        let old_vx = *vx;
        let old_vy = *vy;
        let old_vz = *vz;
        *vx = 0.;
        *vy = 0.;
        *vz = 0.;
        for (o_name, o_mass, o_x, o_y, o_z) in izip!(
            propag.names.iter(),
            propag.masses.iter(),
            s.x.iter_mut(),
            s.y.iter_mut(),
            s.z.iter_mut(),
        ) {
            if name == o_name {
                continue;
            }
            let k =
                -propag::G * o_mass / propag::math::norm(*x - *o_x, *y - *o_y, *z - *o_z).powi(2);
            *vx = propag::math::unitx(*x - *o_x, *y - *o_y, *z - *o_z) * k;
            *vy = propag::math::unity(*x - *o_x, *y - *o_y, *z - *o_z) * k;
            *vz = propag::math::unitz(*x - *o_x, *y - *o_y, *z - *o_z) * k;
        }
        *x = old_vx;
        *y = old_vy;
        *z = old_vz;
    }
    new_s
}

pub fn rk(propag: &mut Propag) {
    let mut states = propag.states.clone();
    let mut ps = states[0].clone();
    for s in states.iter_mut().skip(1) {
        let k1 = propag.time_step() * rkfn(propag, ps.clone());
        let k2 = propag.time_step() * rkfn(propag, ps.clone() + k1.clone() / 2.);
        let k3 = propag.time_step() * rkfn(propag, ps.clone() + k2.clone() / 2.);
        let k4 = propag.time_step() * rkfn(propag, ps.clone() + k3.clone());
        *s = ps + (k1 + 2.0 * (k2 + k3) + k4) / 6.0;
        ps = s.clone();
    }
    propag.states = states;
}
