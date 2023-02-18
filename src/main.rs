mod color;
mod ray;
mod vec3;
use crate::vec3::Vec3;

fn ray_color(r: &ray::Ray) -> Vec3 {
    let unit_direction: vec3::Vec3 = vec3::Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return &(&(1.0 - t) * &Vec3::new(1.0, 1.0, 1.0)) + &(&t * &Vec3::new(0.5, 0.7, 1.0));
}

fn main() {
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
    let lower_left_corner = &(&(&origin - &(&horizontal / &2)) - &(&vertical / &2))
        - &Vec3::new(0.0, 0.0, focal_length);

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height - 1).rev() {
        eprintln!("Scanlines remaining {}", j);
        for i in 0..image_width {
            let u = i / (image_width - 1);
            let v = j / (image_height - 1);
            let r: ray::Ray = ray::Ray::new(
                origin,
                &(&(&lower_left_corner + &(&u * &horizontal)) + &(&v * &vertical)) - &origin,
            );
            let pixel_color = ray_color(&r);
            color::write_color(pixel_color)
        }
    }

    eprintln!("Done.")
}
