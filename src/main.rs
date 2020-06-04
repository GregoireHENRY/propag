mod propag;

use std::time::Instant;

fn main() {
    let frame = "ICRS";
    let nbody = 2;
    let time = propag::set_time(0., -propag::DAY * 15., -10.);

    let mut propag = propag::new(frame, nbody, time);

    // Create every frozen before, then add bodies to be propagated
    // name, mass, radius, x, y, z, vx, vy, vz
    propag.add("133P", 1e13, 2e3, 0., 0., 0., 0., 0., 0.);
    propag.propagate_nexts();
    propag.add("spacecraft", 10., 0.01, -5e3, 0., 0., 0., 1., 0.);

    propag.display_label();
    propag.display(0, 1);
    let tic = Instant::now();
    println!("Retro propagation started..");
    propag.start();
    println!("\telapsed time {:.2} seconds.", tic.elapsed().as_secs_f64());
    propag.display(propag.time.len() - 1, 1);
}
