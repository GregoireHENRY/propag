pub fn dot(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 { x1 * x2 + y1 * y2 + z1 * z2 }
pub fn norm(x: f64, y: f64, z: f64) -> f64 { dot(x, y, z, x, y, z).sqrt() }
pub fn unitx(x: f64, y: f64, z: f64) -> f64 { x / norm(x, y, z) }
pub fn unity(x: f64, y: f64, z: f64) -> f64 { y / norm(x, y, z) }
pub fn unitz(x: f64, y: f64, z: f64) -> f64 { z / norm(x, y, z) }
pub fn distance(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 { norm(x1-x2, y1-y2, z1-z2) }
pub fn xdirection(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 { unitx(x1-x2, y1-y2, z1-z2) }
pub fn ydirection(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 { unity(x1-x2, y1-y2, z1-z2) }
pub fn zdirection(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 { unitz(x1-x2, y1-y2, z1-z2) }
