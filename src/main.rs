mod ray;
mod vec3;
use ray::{ray_color, Ray};
use std::{error::Error, io::Write};
use vec3::Vec3;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;

const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = std::io::stdout();
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);
    stdout.write_all(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let color = ray_color(&ray);
            write_color(&stdout, color)?;
        }
    }
    stdout.flush()?;
    Ok(())
}

fn write_color(mut stdout: &std::io::Stdout, color: Vec3) -> Result<(), Box<dyn Error>> {
    let c = 255.99;
    stdout.write_all(
        &format!(
            "{} {} {}\n",
            (c * color.x) as usize,
            (c * color.y) as usize,
            (c * color.z) as usize
        )
        .as_bytes(),
    )?;
    Ok(())
}
