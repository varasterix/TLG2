use image;
mod color;
mod ray;
mod vec3;
use crate::vec3::Vec3;
mod hittable;
mod sphere;

fn hit_sphere(center: &Vec3, radius: f64, r: &ray::Ray) -> f64 {
    let oc: Vec3 = r.origin() - center;
    let a = Vec3::norm(r.direction()).powi(2);
    let half_b = Vec3::dot(&oc, r.direction());
    let c = Vec3::norm(&oc).powi(2) - (radius * radius);
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}

fn ray_color(r: &ray::Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n: Vec3 = Vec3::normalize(&(&ray::Ray::at(r, t) - &Vec3::new(0.0, 0.0, -1.0)));
        return 0.5 * &Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    let unit_direction: vec3::Vec3 = vec3::Vec3::normalize(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return &((1.0 - t) * &Vec3::new(1.0, 1.0, 1.0)) + &(t * &Vec3::new(0.5, 0.7, 1.0));
}

fn ray_based() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        &(&(&origin - &(&horizontal / 2)) - &(&vertical / 2)) - &Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height - 1).rev() {
        eprintln!("Scanlines remaining {}", j);
        for i in 0..image_width {
            let u = (i as f64) / (image_width - 1) as f64;
            let v = (j as f64) / (image_height - 1) as f64;
            let r: ray::Ray = ray::Ray::new(
                origin,
                &(&(&lower_left_corner + &(u * &horizontal)) + &(v * &vertical)) - &origin,
            );
            let pixel_color = ray_color(&r);
            color::write_color(pixel_color)
        }
    }

    eprintln!("Done.")
}

fn pixel_based() {
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;
    const PATH: &str = "out/out.png";

    let mut buffer = vec![];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let r = (x * 255 / WIDTH) as u8;
            let g = (y * 255 / HEIGHT) as u8;
            let b = (WIDTH - x) as f32 / WIDTH as f32 * (HEIGHT - y) as f32 / HEIGHT as f32;
            let b = (b * 255.) as u8;
            buffer.extend([r, g, b]);
        }
    }

    image::save_buffer(PATH, &buffer, WIDTH, HEIGHT, image::ColorType::Rgb8).unwrap();
}

fn main() {
    // pixel_based();
    ray_based();
}
