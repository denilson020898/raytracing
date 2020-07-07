use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dialetric { ref_idx: f32 },
}

pub fn material_scatter(
    material: &Material,
    r_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
) -> bool {
    match material {
        Material::Lambertian { albedo } => {
            let scatter_direction = rec.normal + Vec3::random_unit_vector();
            *scattered = Ray::new(rec.p, scatter_direction);
            *attenuation = *albedo;
            true
        }
        Material::Metal { albedo, fuzz } => {
            let reflected = Vec3::reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal);
            *scattered = Ray::new(rec.p, reflected + *fuzz * Vec3::random_in_unit_sphere());
            *attenuation = *albedo;
            Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
        }
        Material::Dialetric { ref_idx } => {
            *attenuation = Vec3::new(1.0, 1.0, 1.0);
            let etai_over_etat = if rec.front_face {
                1.0 / *ref_idx
            } else {
                *ref_idx
            };
            let unit_direction = Vec3::unit_vector(r_in.direction());
            let cos_theta = libm::fminf(Vec3::dot(&-unit_direction, &rec.normal), 1.0);
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
            if etai_over_etat * sin_theta > 1.0 {
                let reflected = Vec3::reflect(&unit_direction, &rec.normal);
                *scattered = Ray::new(rec.p, reflected);
                return true;
            }
            let reflect_prob = schlick(cos_theta, etai_over_etat);
            if utils::random_f32() < reflect_prob {
                let reflected = Vec3::reflect(&unit_direction, &rec.normal);
                *scattered = Ray::new(rec.p, reflected);
                return true;
            }

            let refracted = Vec3::refract(&unit_direction, &rec.normal, etai_over_etat);
            *scattered = Ray::new(rec.p, refracted);
            true
        }
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
