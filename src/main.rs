#[macro_use]
extern crate impl_ops;

mod camera;
mod ray;
mod sphere;
mod vec3;
use camera::Camera;
use rand::Rng;
use ray::ray_color;
use sphere::{Hittables, Sphere};
use std::{error::Error, io::Write};
use vec3::Vec3;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: u16 = 50;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = std::io::stdout();
    // World
    let mut world = Hittables::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::new();
    let mut rng = rand::thread_rng();

    stdout.write_all(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f32;
                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, &world, MAX_DEPTH, &mut rng);
            }
            write_color(&stdout, color, SAMPLES_PER_PIXEL)?;
        }
    }
    stdout.flush()?;
    Ok(())
}

fn write_color(
    mut stdout: &std::io::Stdout,
    color: Vec3,
    samples_per_pixel: usize,
) -> Result<(), Box<dyn Error>> {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;
    let scale = 1.0 / samples_per_pixel as f32;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let c = 256.0;
    stdout.write_all(
        &format!(
            "{} {} {}\n",
            (c * r.clamp(0.0, 0.999)) as usize,
            (c * g.clamp(0.0, 0.999)) as usize,
            (c * b.clamp(0.0, 0.999)) as usize,
        )
        .as_bytes(),
    )?;
    Ok(())
}
