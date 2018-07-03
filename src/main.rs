extern crate image;
extern crate rand;

use rand::{thread_rng, Rng};
use std::f64;
use std::env;


fn surflet(grid_x : f64, grid_y : f64, x : f64, y : f64, perms : &mut Vec<usize>, dirs : &mut Vec<(f64, f64)>, per : usize) -> f64 {
    let dist_x : f64 = (x - grid_x).abs();
    let dist_y : f64 = (y - grid_y).abs();

    let poly_x : f64 = 1.0_f64 - 6.0_f64 * dist_x.powf(5.0_f64) + 15.0_f64 * dist_x.powf(4.0_f64) - 10.0_f64 * dist_x.powf(3.0_f64);
    let poly_y : f64 = 1.0_f64 - 6.0_f64 * dist_y.powf(5.0_f64) + 15.0_f64 * dist_y.powf(4.0_f64) - 10.0_f64 * dist_y.powf(3.0_f64);

    let hashed : usize = perms[perms[(grid_x.floor() as usize) % per] + (grid_y.floor() as usize) % per];
    let grad : f64 = (x - grid_x) * dirs[hashed].0 + (y - grid_y) * dirs[hashed].1;

    poly_x * poly_y * grad
}


fn noise(x : f64, y : f64, perms : &mut Vec<usize>, dirs : &mut Vec<(f64, f64)>, per : usize) -> f64 {
    let int_x = x.floor();
    let int_y = y.floor();
    surflet(int_x, int_y, x, y, perms, dirs, per) + surflet(int_x + 1.0, int_y, x, y, perms, dirs, per)
     + surflet(int_x, int_y + 1.0, x, y, perms, dirs, per)  + surflet(int_x + 1.0, int_y + 1.0, x, y, perms, dirs, per)
}

fn f_bm(x : f64, y : f64, perms : &mut Vec<usize>, dirs : &mut Vec<(f64, f64)>, per : usize, octs : usize) -> f64 {
    let mut val = 0_f64;
    for o in 0..octs {
        val += 0.5_f64.powf(o as f64) * noise(x * 2.0_f64.powf(o as f64), y * 2.0_f64.powf(o as f64), perms, dirs, (per * 2_usize.pow(o as u32)) as usize)
    }
    return val
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let size : u32;
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
        },
        _ => {
            size = 128;
        }
    }

    let mut imgbuf = image::GrayImage::new(size, size);

    let mut perm: Vec<usize> = (0..256).collect();
    thread_rng().shuffle(&mut perm);

    let perm_clone = perm.clone();
    perm.extend(perm_clone);

    let pi2 = 2.0 * f64::consts::PI / 256.0;
    let mut directions = (0..256).map(|x| ((pi2 * (x as f64)).cos(), (pi2 * (x as f64)).sin())).collect::<Vec<_>>();
    let octs = 5;
    let per : usize = 4;
    let freq : f64 = 1.0_f64/(size as f64 / per as f64);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut i = (f_bm(x as f64 * freq, y as f64 * freq, &mut perm, &mut directions, per, octs) + 1.0) / 2.0;
        *pixel = image::Luma([(i * 255.0) as u8]);
    }
    imgbuf.save(format!("noise{0}x{0}.png", size)).unwrap();
}
