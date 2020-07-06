mod vec3;

use vec3::{Color, Vec3};

use std::io::{self, Write};

fn write_color(pixel_color: Color) {
    let ir = (255.999 * pixel_color.x()) as i32;
    let ig = (255.999 * pixel_color.y()) as i32;
    let ib = (255.999 * pixel_color.z()) as i32;
    println!("{} {} {}\n", ir, ig, ib)
}
fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(1.0, 2.0, 3.0);
    v1 += v2;

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScalines remaining: {}", j);
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let r = i as f32 / (image_width as f32 - 1.0);
            let g = j as f32 / (image_height as f32 - 1.0);
            let b = 0.25f32;
            let pixel_color = Color::new(r, g, b);
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone.\n");
}
