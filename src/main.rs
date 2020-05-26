extern crate itertools;

mod toolbox;
mod math;
mod system;
mod body;
mod physics;
mod time;

use math::{State};

fn main() {
    let mut system = system::new("ICRS"); 
    system.bodies.add(body::new("133P", 1e13, 2e3, State::zero(), "center"));
    system.bodies.add(body::new("spacecraft", 10., 0.01, State::new(-5e3, 0., 0., 0., 1., 0.), "propagation"));
    let time = time::set(0., system::DAY, 20.);
    physics::rk(&mut system, &time);
}
