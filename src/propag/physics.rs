use crate::propag;
use itertools::izip;
use propag::{Propag, States};

use crossbeam::{channel, thread, Receiver, Sender};
use std::fs::File;
use std::io::Write;
//use std::sync::mpsc;
//use std::sync::mpsc::{Receiver, Sender};
//use std::thread;

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
        let old = (*vx, *vy, *vz);
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
            let dx = *x - *o_x;
            let dy = *y - *o_y;
            let dz = *z - *o_z;
            let k = -propag::G * o_mass / propag::math::norm(dx, dy, dz).powi(2);
            *vx += propag::math::unitx(dx, dy, dz) * k;
            *vy += propag::math::unity(dx, dy, dz) * k;
            *vz += propag::math::unitz(dx, dy, dz) * k;
        }
        *x = old.0;
        *y = old.1;
        *z = old.2;
    }
    new_s
}

pub fn rk(propag: &mut Propag) {
    let (tx, rx): (Sender<States>, Receiver<States>) = channel::unbounded();

    let time = propag.time.clone();
    let (do_save, file) = match propag.save {
        Some(_) => (
            true,
            Some(
                File::create(format!(
                    "rsc/out/{}.txt",
                    propag.names[propag.save.unwrap()]
                ))
                .unwrap(),
            ),
        ),
        None => (false, None),
    };
    let mut file = file.unwrap();
    if do_save {
        file.write(propag.format_label().as_bytes()).unwrap();
        file.write(b"\n").unwrap();
    }

    thread::scope(|s| {
        s.spawn(|_| {
            // THREAD RECEIVER
            for time in time.iter() {
                let data = rx.recv().unwrap();
                if time.abs() % propag::DAY == 0. {
                    if do_save {
                        file.write(format!("{:>11}", time).as_bytes()).unwrap();
                        file.write(data.format_body(1).as_bytes()).unwrap();
                        file.write(b"\n").unwrap();
                    }
                }
            }
        });

        // THREAD SENDER
        for _ in propag.time.iter() {
            let k1 = propag.time_step() * rkfn(&propag, propag.states.clone());
            let k2 = propag.time_step() * rkfn(&propag, propag.states.clone() + k1.clone() / 2.);
            let k3 = propag.time_step() * rkfn(&propag, propag.states.clone() + k2.clone() / 2.);
            let k4 = propag.time_step() * rkfn(&propag, propag.states.clone() + k3.clone());
            propag.states += (k1 + 2.0 * (k2 + k3) + k4) / 6.0;
            tx.send(propag.states.clone()).unwrap();
        }
    })
    .unwrap();
}
