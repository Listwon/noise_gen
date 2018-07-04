extern crate image;
extern crate rand;
extern crate time;
use time::PreciseTime;

use rand::{thread_rng, Rng};
use std::f64;
use std::env;

#[derive(Debug)]
struct NoiseGen {
    perms : Vec<usize>,
    dirs : Vec<(f64, f64)>,
    per : usize,
    octs : u32,
    freq : f64
}

impl NoiseGen {
    fn surflet(&self, grid_x : f64, grid_y : f64, x : f64, y : f64, per : usize) -> f64 {
        let d_x : f64 = (x - grid_x).abs();
        let d_y : f64 = (y - grid_y).abs();

        let poly_x : f64 = 1.0_f64 - 6.0_f64 * d_x * d_x * d_x * d_x * d_x + 15.0_f64 * d_x * d_x * d_x * d_x - 10.0_f64 * d_x * d_x * d_x;
        let poly_y : f64 = 1.0_f64 - 6.0_f64 * d_y * d_y * d_y * d_y * d_y + 15.0_f64 * d_y * d_y * d_y * d_y - 10.0_f64 * d_y * d_y * d_y;

        let hashed : usize = self.perms[self.perms[(grid_x as usize) % per] + (grid_y as usize) % per];
        let grad : f64 = (x - grid_x) * self.dirs[hashed].0 + (y - grid_y) * self.dirs[hashed].1;

        poly_x * poly_y * grad
    }


    fn noise(&self, x : f64, y : f64, per : usize) -> f64 {
        let int_x = x.floor();
        let int_y = y.floor();
        self.surflet(int_x, int_y, x, y, per) + self.surflet(int_x + 1.0, int_y, x, y, per)
         + self.surflet(int_x, int_y + 1.0, x, y, per)  + self.surflet(int_x + 1.0, int_y + 1.0, x, y, per)
    }

    fn f_bm(&self, x : u32, y : u32) -> f64 {
        let mut val = 0_f64;
        for o in 0..self.octs {
            val += 1.0 / 2_usize.pow(o) as f64 * self.noise(x as f64 * self.freq * 2_usize.pow(o) as f64, y as f64 * self.freq * 2_i32.pow(o) as f64, self.per * 2_usize.pow(o))
        }
        return val * 0.5 + 0.5
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let size : u32;
    let per : usize;
    match args.len() {
        // one argument passed
        2 => {
            let num = &args[1];
            size = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: argument is not an integer");
                    return;
                },
            };
            per = 4;
        },
        3 => {
            let num = &args[1];
            size = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: first argument is not an integer");
                    return;
                },
            };
            let num_2 = &args[2];
            per = match num_2.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: second argument is not an integer");
                    return;
                },
            };
        },
        _ => {
            size = 256;
            per = 4;
        }
    }

    let start = PreciseTime::now();
    let mut imgbuf = image::GrayImage::new(size, size);

    let mut perm: Vec<usize> = (0..256).collect();
    thread_rng().shuffle(&mut perm);

    let perm_clone = perm.clone();
    perm.extend(perm_clone);

    let pi2 = 2.0 * f64::consts::PI / 256.0;
    let directions = (0..256).map(|x| ((pi2 * (x as f64)).cos(), (pi2 * (x as f64)).sin())).collect::<Vec<_>>();
    let octs = 5;
    let freq : f64 = 1.0_f64/(size as f64 / per as f64);

    let noise_gen = NoiseGen {
        perms: perm,
        dirs: directions,
        per: per,
        octs: octs,
        freq: freq
    };

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Luma([(noise_gen.f_bm(x, y) * 255.0) as u8]);
    }

    let end = PreciseTime::now();
    println!("{}", start.to(end));
    imgbuf.save(format!("noise{0}x{0}.png", size)).unwrap();
}
