use std::ops::RangeInclusive;

use crate::types::{HitRecord, Hittable, Point3, Ray};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: if radius > 0.0 { radius } else { 0.0 },
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: RangeInclusive<f64>) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();

        let mut root = (h - sqrt_d) / a;
        if !ray_t.contains(&root) {
            root = (h + sqrt_d) / a;
            if !ray_t.contains(&root) {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;
        Some(HitRecord::new(ray, root, p, outward_normal))
    }
}
