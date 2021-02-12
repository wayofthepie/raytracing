use std::{error::Error, fs::OpenOptions, io::Write};

fn main() -> Result<(), Box<dyn Error>> {
    let image_width: usize = 256;
    let image_height: usize = 256;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("riaw.ppm")?;
    let mut buf = String::new();
    buf.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;
            let val = 255.99;
            let ir = (val * r) as usize;
            let ig = (val * g) as usize;
            let ib = (val * b) as usize;
            buf.push_str(&format!("{} {} {}\n", ir, ig, ib));
        }
    }
    file.write_all(buf.as_bytes())?;
    file.flush()?;
    Ok(())
}

#[derive(Debug, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        let Self { x, y, z } = self;
        Self {
            x: -x,
            y: -y,
            z: -z,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Vec3;

    #[test]
    fn negation_should_negate_all_fields() {
        let x = 0.1;
        let y = 0.2;
        let z = 0.3;
        let vec = Vec3 { x, y, z };
        assert_eq!(
            -vec,
            Vec3 {
                x: -x,
                y: -y,
                z: -z
            }
        );
    }
}
