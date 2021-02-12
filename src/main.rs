use std::{error::Error, fs::OpenOptions, io::Write};

fn main() -> Result<(), Box<dyn Error>> {
    let image_width: usize = 256;
    let image_height: usize = 256;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("riaw.ppm")?;
    let mut buf = String::new();
    buf.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));

    for j in (0..image_height).rev() {
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
