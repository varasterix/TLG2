mod color;
mod ray;
mod vec3;
use crate::vec3::Vec3;

fn main() {
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;
    const PATH: &str = "out/out.png";
    run(WIDTH, HEIGHT, PATH);
}

pub fn run(width: u32, height: u32, path: &str) {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = image_width / aspect_ratio as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = &(&(&origin - &(&horizontal / 2)) - &(&vertical / 2))
        - &Vec3::new(0.0, 0.0, focal_length);

    // Render
    let mut buffer = vec![];
    for y in 0..height {
        eprintln!("Scanlines remaining {}", height - y);
        for x in 0..width {
            let u: f64 = x as f64 / (image_width - 1) as f64;
            let v: f64 = y as f64 / (image_height - 1) as f64;
            let r = (x * 255 / width) as u8;
            let g = (y * 255 / height) as u8;
            let b = (width - x) as f32 / width as f32 * (height - y) as f32 / height as f32;
            let b = (b * 255.) as u8;
            buffer.extend([r, g, b]);
            let r: ray::Ray = ray::Ray::new(
                origin,
                &(&(&lower_left_corner + &(u * &horizontal)) + &(v * &vertical)) - &origin,
            );
            let pixel_color = ray_color(&r);
            color::write_color(pixel_color)
        }
    }

    image::save_buffer(path, &buffer, width, height, image::ColorType::Rgb8).unwrap();

    eprintln!("Done.")
}

fn ray_color(r: &ray::Ray) -> Vec3 {
    let unit_direction: vec3::Vec3 = vec3::Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return &((1.0 - t) * &Vec3::new(1.0, 1.0, 1.0)) + &(t * &Vec3::new(0.5, 0.7, 1.0));
}
