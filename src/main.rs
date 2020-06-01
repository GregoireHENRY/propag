mod propag;

//use std::io::{stdout, Write};
//use std::time::Instant;

fn main() {
    let frame = "ICRS";
    let nbody = 2;
    let time = propag::set_time(0., -propag::DAY * 1., -20.);

    let mut propag = propag::new(frame, nbody, time);

    // Create every frozen before, then add bodies to be propagated
    // name, mass, radius, x, y, z, vx, vy, vz
    propag.add("133P", 1e13, 2e3, 0., 0., 0., 0., 0., 0.);
    propag.propagate_nexts();

    propag.states[0].display_body(1);
    propag.add("spacecraft", 10., 0.01, -5e3, 0., 0., 0., 1., 0.);
    propag.states[0].display_body(1);

    /*
    let tic = Instant::now();
    print!("Retro propagation started..");
    stdout().flush().unwrap();
    propag.start();
    println!(" elapsed time {:.2} seconds.", tic.elapsed().as_secs_f64());

    propag.display_label();
    propag.display(1, 0);
    propag.display(1, propag.ntime - 1);
    */
}
