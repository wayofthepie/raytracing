use std::{error::Error, io::Write};
use vec3::Vec3;
mod vec3;

fn main() -> Result<(), Box<dyn Error>> {
    let image_width: usize = 256;
    let image_height: usize = 256;
    let mut stdout = std::io::stdout();
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let color = Vec3 {
                x: (i as f32 / (image_width - 1) as f32),
                y: (j as f32 / (image_height - 1) as f32),
                z: 0.25,
            };
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
