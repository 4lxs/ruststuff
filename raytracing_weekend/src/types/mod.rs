mod hittable;
mod ray;
mod x3;

use x3::X3;

pub use ray::*;

pub use hittable::*;
pub mod objects;

pub type Vec3 = X3;
pub type Point3 = X3;
pub type Color = X3;
