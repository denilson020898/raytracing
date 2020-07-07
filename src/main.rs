mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

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
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.0, std::f32::MAX, &mut rec) {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
fn main() {
    let aspect_ratio = 16.0 / 9.0f32;
    let max_depth = 50;
    let image_width = 200;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples = 100;

    println!("P3\n{} {}\n255\n", image_width, image_height);

    let mut world = HittableList::default();
    let s1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    world.push(Box::new(s1));
    world.push(Box::new(s2));

    let cam = Camera::new();

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
