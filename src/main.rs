mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Vec3};

use std::io::{self, Write};

fn write_color(pixel_color: Color) {
    let ir = (255.999 * pixel_color.x()) as i32;
    let ig = (255.999 * pixel_color.y()) as i32;
    let ib = (255.999 * pixel_color.z()) as i32;
    println!("{} {} {}\n", ir, ig, ib)
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> Option<f32> {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = Vec3::dot(&oc, &r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant >= 0.0 {
        return Some((-half_b - discriminant.sqrt()) / a);
    }
    None
}

fn ray_color(r: &Ray, world: &mut HittableList) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, std::f32::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
fn main() {
    let aspect_ratio = 16.0 / 9.0f32;
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    println!("P3\n{} {}\n255\n", image_width, image_height);

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut world = HittableList::default();
    let s1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    world.push(Box::new(s1));
    world.push(Box::new(s2));

    for j in (0..image_height).rev() {
        // eprint!("\rScalines remaining: {}", j);
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let u = i as f32 / (image_width as f32 - 1.0);
            let v = j as f32 / (image_height as f32 - 1.0);
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let pixel_color = ray_color(&r, &mut world);
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone.\n");
}
