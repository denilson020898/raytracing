use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn default() -> HitRecord {
        HitRecord {
            p: Vec3::default(),
            normal: Vec3::default(),
            t: f32::default(),
            front_face: bool::default(),
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

pub trait Hittable {
    fn hit(&mut self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
