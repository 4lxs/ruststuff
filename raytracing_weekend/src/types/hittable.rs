use std::{ops::RangeInclusive, rc::Rc};

use super::{Point3, Ray, Vec3};

pub struct HitRecord {
    t: f64,
    p: Point3,
    normal: Vec3,
    front_face: bool,
}

impl HitRecord {
    /// Creates a new [`HitRecord`].
    ///
    /// * `outward_normal` - assumed to have unit length
    pub fn new(ray: &Ray, t: f64, p: Vec3, outward_normal: Vec3) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            t,
            p,
            normal,
            front_face,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: RangeInclusive<f64>) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl From<Rc<dyn Hittable>> for HittableList {
    fn from(value: Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![value],
        }
    }
}

impl From<Vec<Rc<dyn Hittable>>> for HittableList {
    fn from(objects: Vec<Rc<dyn Hittable>>) -> Self {
        Self { objects }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: RangeInclusive<f64>) -> Option<HitRecord> {
        let start = *ray_t.start();
        self.objects
            .iter()
            .fold((*ray_t.end(), None), |(nearest_hit, rec), obj| {
                if let Some(hit_record) = obj.hit(ray, start..=(nearest_hit)) {
                    (hit_record.t, Some(hit_record))
                } else {
                    (nearest_hit, rec)
                }
            })
            .1
    }
}
