mod propag;

use std::io::{stdout, Write};
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

    let tic = Instant::now();
    print!("Retro propagation started..");
    stdout().flush().unwrap();
    propag.start();
    println!(" elapsed time {:.2} seconds.", tic.elapsed().as_secs_f64());

    propag.states[0].display_body(1);
    propag.states[propag.time.len() - 1].display_body(1);
}
