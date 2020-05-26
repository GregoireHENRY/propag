use std::vec::{Vec};

pub fn linspace(a: f64, b: f64, n: usize)
-> Vec<f64>
{
    let s = (b - a) / (n - 1) as f64;
    let mut v = vec![0.; n];
    for ii in 0..n {
        v[ii] = a + ii as f64 * s;
    }
    v
}

pub fn dot(u: &Vector, v: &Vector)
-> f64
{
    u.x * v.x + u.y * v.y + u.z * v.z
}

#[derive(Debug, Copy, Clone)]
pub struct Vector
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector
{
    pub fn new(x: f64, y: f64, z: f64)
    -> Self
    {
        Self{x: x, y: y, z: z}
    }
    pub fn zero()
    -> Self
    {
        Self::new(0., 0., 0.)
    }
    pub fn norm(&self)
    -> f64
    {
        dot(self, self).sqrt()
    }
    pub fn unit(&self)
    -> Self
    {
        *self / self.norm()
    }
}

impl std::ops::Mul<f64> for Vector
{
    type Output = Self;
    fn mul(mut self, oth: f64)
    -> Self
    {
        self.x *= oth;
        self.y *= oth;
        self.z *= oth;
        self
    }
}

impl std::ops::Mul<Vector> for f64
{
    type Output = Vector;
    fn mul(self, oth: Vector)
    -> Vector
    {
        oth * self
    }
}

impl std::ops::Div<f64> for Vector
{
    type Output = Self;
    fn div(self, oth: f64)
    -> Self
    {
        self * (1./oth)
    }
}

impl std::ops::Neg for Vector
{
    type Output = Self;
    fn neg(self)
    -> Self
    {
        self * -1.
    }
}  

impl std::ops::Add for Vector
{
    type Output = Self;
    fn add(mut self, oth: Self)
    -> Self
    {
        self.x += oth.x;
        self.y += oth.y;
        self.z += oth.z;
        self
    }
}  

impl std::ops::Sub for Vector
{
    type Output = Self;
    fn sub(self, oth: Self)
    -> Self
    {
        self + -oth
    }
}

impl std::ops::AddAssign for Vector { fn add_assign(&mut self, oth: Self) { *self = *self + oth; } }  
impl std::ops::SubAssign for Vector { fn sub_assign(&mut self, oth: Self) { *self = *self - oth; } }  
impl std::ops::MulAssign<f64> for Vector { fn mul_assign(&mut self, oth: f64)  { *self = *self * oth; } }  

#[derive(Debug, Clone)]
pub struct State
{
    pub x: Vector,
    pub v: Vector,
}

impl State
{
    pub fn from_vec(x: Vector, v: Vector)
    -> Self
    {
        Self{x: x, v: v}
    }
    pub fn new(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64)
    -> Self
    {
        Self::from_vec(Vector::new(x, y, z), Vector::new(vx, vy, vz))
    }
    pub fn zero()
    -> Self
    {
        Self::from_vec(Vector::zero(), Vector::zero())
    }
}
