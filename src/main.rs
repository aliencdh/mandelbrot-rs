use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

type Complex = num::Complex<f32>;
const INFINITY: f32 = f32::MAX;
const ITER_LIMIT: u32 = 60;
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const ZOOM: f32 = 0.001;
const OFFSET_HOR: u32 = 700;
const OFFSET_VER: u32 = 0;

fn main() {
    let filename = std::env::args().skip(1).next().unwrap();
    let path = Path::new(&filename);
    let file = File::create(&path).unwrap();

    let mut writer = BufWriter::new(file);
    let mut data = format!("P3\n{} {}\n255\n", WIDTH, HEIGHT);

    for j in 0..HEIGHT {
        // logging
        std::io::stdout().flush().unwrap();
        println!("Scanlines Remaining: {}", j);

        for i in 0..WIDTH {
            let dist = check(Complex::new(
                (i as f32 - ((WIDTH + OFFSET_HOR) as f32) / 2.0) * ZOOM,
                (j as f32 - ((HEIGHT + OFFSET_VER) as f32) / 2.0) * ZOOM,
            ));
            let px_clr = if dist == 0 {
                Color::new(0, 0, 0)
            } else {
                Color::from_hsv(360 - (360 / dist) as u16, 100, 100)
            };

            data += format!("{} {} {}\n", px_clr.r, px_clr.g, px_clr.b).as_str();
        }
    }

    writer.write(data.as_bytes()).unwrap();
    println!("Done.");
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}
impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    fn from_hsv(h: u16, s: u8, v: u8) -> Self {
        // formula from rapidtables.com

        // calculate constants
        let c = ((v as f32) / 100.0) * ((s as f32) / 100.0);
        let x = c * (1.0 - ((h as f32 / 60.0) % 2.0 - 1.0).abs());
        let m = ((v as f32) / 100.0) - c;

        // calculate intermediate (R', G', B')
        let intermediate: (f32, f32, f32) = match h {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            300.. => (c, 0.0, x),
        };

        Color {
            r: ((intermediate.0 + m) * 255.0) as u8,
            g: ((intermediate.1 + m) * 255.0) as u8,
            b: ((intermediate.2 + m) * 255.0) as u8,
        }
    }
}

fn check(c: Complex) -> u32 {
    let mut x = Complex::new(0.0, 0.0);
    let mut iterations = 0;

    for _ in 0..ITER_LIMIT {
        iterations += 1;
        x = f(c, x);
        if x.norm() >= INFINITY {
            return iterations;
        }
    }

    0
}

fn f(c: Complex, x: Complex) -> Complex {
    x.powu(2) + c
}
