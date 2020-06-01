pub fn dot(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    x1 * x2 + y1 * y2 + z1 * z2
}
pub fn norm(x: f64, y: f64, z: f64) -> f64 {
    dot(x, y, z, x, y, z).sqrt()
}
pub fn unitx(x: f64, y: f64, z: f64) -> f64 {
    x / norm(x, y, z)
}
pub fn unity(x: f64, y: f64, z: f64) -> f64 {
    y / norm(x, y, z)
}
pub fn unitz(x: f64, y: f64, z: f64) -> f64 {
    z / norm(x, y, z)
}
