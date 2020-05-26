use crate::math::{Vector};
use crate::system;
use system::{System};
use crate::body::{Bodies};
use crate::time::{Time};
use std::collections::{HashMap};

fn rkfn(mut propa: Bodies, bodies: &Bodies, _it: usize)
-> Bodies
{
    for body in propa.values_mut() {
        let v = body.state.v;
        body.state.v = Vector::zero();
        for oth in bodies.values() {
            if body.name == oth.name { continue; }
            body.state.v -= system::G * oth.mass * body.direction(oth) / body.distance(oth).powi(2);
        }
        body.state.x = v;
    }
    propa
}

pub fn rk(system: &mut System, time: &Time)
{
    let mut propa = Bodies(HashMap::new()) ;
    for body in system.bodies.values() {
        if body.method == "propagation" {
            propa.add(body.clone());
        }
    }
    for ii in 0..1 {//time.list.len() {
        let k1 = time.step * rkfn(propa.clone(), &system.bodies, ii);
        let k2 = time.step * rkfn(propa.clone() + k1.clone() / 2.0, &system.bodies, ii);
        let k3 = time.step * rkfn(propa.clone() + k2.clone() / 2.0, &system.bodies, ii);
        let k4 = time.step * rkfn(propa.clone() + k3.clone(), &system.bodies, ii);
        propa = propa + (k1 + 2.0 * (k2 + k3) + k4) / 6.0;
    }
}
