#[macro_use]
extern crate impl_ops;

mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec3;
use camera::Camera;
use hit::Hittables;
use material::{Dialectric, Lambertian, Material, Metal};
use rand::{distributions::Uniform, prelude::Distribution, Rng};
use ray::ray_color;
use sphere::Sphere;
use std::{cell::RefCell, error::Error, io::Write, rc::Rc};
use vec3::Vec3;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: usize = 1200;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 500;
const MAX_DEPTH: u16 = 50;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = std::io::stdout();
    let mut rng = rand::thread_rng();
    let between = Uniform::new(0.0, 1.0);

    // World
    let world = random_scene();

    // Camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vertical_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        vertical_up,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    stdout.write_all(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + between.sample(&mut rng)) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + between.sample(&mut rng)) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, &world, MAX_DEPTH);
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
    let scale = 1.0 / samples_per_pixel as f64;
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

fn random_scene() -> Hittables<Sphere<'static>> {
    let mut rng = rand::thread_rng();
    let mut world = Hittables::new();
    let material_ground = Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let material_ground = package_material(material_ground);
    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    ));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if choose_mat < 0.8 {
                    let albedo = Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
                    let material = Lambertian::new(albedo);
                    package_material(Box::new(material))
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let material = Metal::new(albedo, fuzz);
                    package_material(Box::new(material))
                } else {
                    package_material(Box::new(Dialectric::new(1.5)))
                };
                let sphere = Sphere::new(center, 0.2, material);
                world.add(sphere);
            }
        }
    }
    let material = package_material(Box::new(Dialectric::new(1.5)));
    let sphere = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material);
    world.add(sphere);
    let material = package_material(Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))));
    let sphere = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material);
    world.add(sphere);
    let material = package_material(Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)));
    let sphere = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material);
    world.add(sphere);
    world
}

fn package_material(material: Box<dyn Material>) -> Rc<RefCell<Box<dyn Material>>> {
    let metal: RefCell<Box<dyn Material>> = RefCell::new(material);
    Rc::new(metal)
}
