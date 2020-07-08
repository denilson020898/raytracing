mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::material::{material_scatter, Material};
use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Vec3};

use std::io::{self, Write};

fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f32;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let ir = (256.0 * utils::clamp(r, 0.0, 0.999)) as i32;
    let ig = (256.0 * utils::clamp(g, 0.0, 0.999)) as i32;
    let ib = (256.0 * utils::clamp(b, 0.0, 0.999)) as i32;
    println!("{} {} {}\n", ir, ig, ib)
}

fn ray_color(r: &Ray, world: &mut HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, std::f32::MAX) {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::default();
        if material_scatter(&rec.material, &r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
fn main() {
    let aspect_ratio = 16.0 / 9.0f32;
    let max_depth = 50;
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples = 100;

    println!("P3\n{} {}\n255\n", image_width, image_height);

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        },
    )));

    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dialetric { ref_idx: 1.5 },
    )));

    objects.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    )));

    objects.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::random_f32();
            let center = Vec3::new(
                a as f32 + utils::random_f32(),
                0.2,
                b as f32 + 0.9 * utils::random_f32(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    objects.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian { albedo },
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_min_max(0.5, 1.0);
                    let fuzz = utils::random_f32_min_max(0.0, 0.5);
                    objects.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Metal { albedo, fuzz },
                    )));
                } else {
                    // glass
                    objects.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Dialetric { ref_idx: 1.5 },
                    )));
                }
            }
        }
    }

    let mut world = HittableList::new(objects);

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0f32;
    let aperture = 0.1f32;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    for j in (0..image_height).rev() {
        // eprint!("\rScalines remaining: {}", j);
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (i as f32 + utils::random_f32()) / image_width as f32;
                let v = (j as f32 + utils::random_f32()) / image_height as f32;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &mut world, max_depth);
            }
            write_color(pixel_color, samples);
        }
    }
    eprintln!("\nDone.\n");
}
