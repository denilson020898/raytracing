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
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambertian {
            albedo: Vec3::new(0.1, 0.2, 0.5),
        },
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.0),
        },
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal {
            albedo: Vec3::new(0.8, 0.6, 0.2),
            fuzz: 0.3,
        },
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        // Material::Metal {
        //     albedo: Vec3::new(0.8, 0.8, 0.8),
        //     fuzz: 0.1,
        // },
        Material::Dialetric { ref_idx: 1.5 },
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        // Material::Metal {
        //     albedo: Vec3::new(0.8, 0.8, 0.8),
        //     fuzz: 0.1,
        // },
        Material::Dialetric { ref_idx: 1.5 },
    )));

    let mut world = HittableList::new(objects);

    let cam = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        aspect_ratio,
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
