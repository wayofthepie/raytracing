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
use material::{Lambertian, Material, Metal};
use rand::{distributions::Uniform, prelude::Distribution};
use ray::ray_color;
use sphere::Sphere;
use std::{cell::RefCell, error::Error, io::Write, rc::Rc};
use vec3::Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: u16 = 50;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = std::io::stdout();

    let mut lamberian_one = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let material_ground = package_material(&mut lamberian_one);

    let mut lamberian_two = Lambertian::new(Vec3::new(0.7, 0.3, 0.3));
    let material_center = package_material(&mut lamberian_two);

    let mut metal_one = Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3);
    let material_left = package_material(&mut metal_one);

    let mut metal_two = Metal::new(Vec3::new(0.8, 0.6, 0.3), 1.0);
    let material_right = package_material(&mut metal_two);

    let mut rng = rand::thread_rng();
    let between = Uniform::new(0.0, 1.0);

    // World
    let mut world = Hittables::new();
    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    let camera = Camera::new();

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

fn package_material(material: &mut dyn Material) -> Rc<RefCell<&mut dyn Material>> {
    let metal: RefCell<&mut dyn Material> = RefCell::new(material);
    Rc::new(metal)
}
