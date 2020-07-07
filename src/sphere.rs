use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;

            if temp < t_max && temp > t_min {
                let rec_t = temp;
                let rec_p = r.at(rec_t);
                let outward_normal = (rec_p - self.center) / self.radius;

                let mut record = HitRecord {
                    t: rec_t,
                    p: rec_p,
                    normal: Vec3::default(),
                    front_face: bool::default(),
                    material: self.material,
                };
                record.set_face_normal(r, &outward_normal);
                return Some(record);
            }
            // temp = (-half_b + root) / a;
            // if temp < t_max && temp > t_min {
            //     rec.t = temp;
            //     rec.p = r.at(rec.t);
            //     let outward_normal = (rec.p - self.center) / self.radius;
            //     rec.set_face_normal(r, &outward_normal);
            //     return true;
            // }
        }

        None
    }
}
