mod propag;

fn main() {
    let frame = "ICRS";
    let origin = "133P";
    let nbody = 2;
    // t0, tf, dt
    let time = propag::set_time(0., -propag::DAY * 15., -10.);

    let mut propag = propag::new(frame, origin, nbody, time);

    // Frozens first
    // name, mass, radius, x, y, z, vx, vy, vz
    propag.add("133P", 1e13, 2e3, 0., 0., 0., 0., 0., 0.);
    propag.propagate_nexts();
    propag.add("spacecraft", 10., 0.01, -5e3, 0., 0., 0., 1., 0.);

    propag.save("spacecraft");

    propag.start();
}
