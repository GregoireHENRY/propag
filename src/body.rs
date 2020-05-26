use crate::math::{State, Vector};
use std::collections::{HashMap};
use itertools::{izip};

#[derive(Debug, Clone)]
pub struct Body
{
    pub name:   &'static str,
    pub mass:   f64,
    pub radius: f64,
    pub state:  State,
    pub method: String,
}

pub fn new(name: &'static str, mass: f64, radius: f64, state: State, method: &str)
-> Body
{
    Body{name: name, mass: mass, radius: radius, state: state, method: method.to_string()}
}

impl Body
{
    pub fn distance(&self, oth: &Self)
    -> f64
    {
        (self.state.x - oth.state.x).norm()
    }
    pub fn direction(&self, oth: &Self)
    -> Vector
    {
        (self.state.x - oth.state.x).unit()
    }
}

#[derive(Debug, Clone)]
pub struct Bodies(pub HashMap<&'static str, Body>);

impl std::ops::Deref for Bodies
{
    type Target = HashMap<&'static str, Body>;
    fn deref(&self)
    -> &HashMap<&'static str, Body>
    {
        &self.0
    }
}

impl std::ops::DerefMut for Bodies
{
    fn deref_mut(&mut self)
    -> &mut HashMap<&'static str, Body>
    {
        &mut self.0
    }
}

impl Bodies
{
    pub fn add(&mut self, body: Body)
    {
        self.insert(body.name, body);
    }
}

/*
pub type Bodies = HashMap<&'static str, Body>;

trait Ops
{
    fn mul(mut self, oth: f64) -> Bodies;
}

trait OpsRv
{
    fn mul(self, oth: Bodies);
}

impl Ops for Bodies
{
    fn mul(mut self, oth: f64)
    -> Bodies
    {
        for body in self.values_mut() {
            body.state.x *= oth;
            body.state.v *= oth;
        }
        self
    }
}
*/

impl std::ops::Mul<f64> for Bodies
{
    type Output = Self;
    fn mul(mut self, oth: f64)
    -> Self
    {
        for body in self.values_mut() {
            body.state.x *= oth;
            body.state.v *= oth;
        }
        self
    }
}

impl std::ops::Mul<Bodies> for f64
{
    type Output = Bodies;
    fn mul(self, oth: Bodies)
    -> Bodies
    {
        oth * self
    }
}

impl std::ops::Div<f64> for Bodies
{
    type Output = Self;
    fn div(self, oth: f64)
    -> Self
    {
        self * (1./oth)
    }
}

impl std::ops::Add for Bodies
{
    type Output = Self;
    fn add(mut self, mut oths: Self)
    -> Self
    {
        for (body, oth) in izip!(self.values_mut(), oths.values_mut()) {
            body.state.x += oth.state.x;
            body.state.v += oth.state.v;
        }
        self
    }
}  
