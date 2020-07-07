use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3 },
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
        Material::Metal { albedo } => {
            let reflected = Vec3::reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            *attenuation = *albedo;
            Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
        }
    }
}
