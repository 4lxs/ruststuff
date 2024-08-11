use super::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            dir: direction,
        }
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }
}
