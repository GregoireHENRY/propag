extern crate ndarray;
extern crate itertools;

mod propag;

fn main() {
    let frame = "ICRS";
    let nbody = 2;
    let time = propag::set_time(0., propag::DAY, 20.);

    let mut propag = propag::new(frame, nbody, time);

    // Create every frozen before, then add the body to be propagated
    // name, mass, radius, x, y, z, vx, vy, vz
    propag.add("133P", 1e13, 2e3, 0., 0., 0., 0., 0. ,0.);
    propag.propagate_nexts();
    propag.add("spacecraft", 10., 0.01, -5e3, 0., 0., 0., 1., 0.);

    propag.start();
}
