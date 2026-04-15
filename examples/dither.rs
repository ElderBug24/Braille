use std::env;

use braille::{BrailleCharUnOrdered, BrailleCharGridVector};

use glam::Vec3;


struct Buffer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec3>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        return Self {
            width,
            height,
            data: vec![Vec3::ZERO; width * height]
        };
    }

    pub fn dimensions(&self) -> (usize, usize) {
        return (self.width, self.height);
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Vec3 {
        return self.data[y * self.width + x];
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, value: Vec3) {
        self.data[y * self.width + x] = value;
    }

    #[inline]
    fn get_mut(&mut self, x: usize, y: usize) -> &mut Vec3 {
        return &mut self.data[y * self.width + x];
    }

    pub fn from_file(path: &str) -> Self {
        let img = image::open(path).unwrap().to_rgb8();
        let (w, h) = img.dimensions();

        let mut buf = Self::new(w as usize, h as usize);

        for y in 0..h as usize {
            for x in 0..w as usize {
                let p = img.get_pixel(x as u32, y as u32).0;
                buf.set(x, y, Vec3::new(p[0] as f32, p[1] as f32, p[2] as f32));
            }
        }

        return buf;
    }
}

fn dither_img(file: &str) {
    let mut img = Buffer::from_file(file);
    let (w, h) = img.dimensions();

    let mut array: BrailleCharGridVector<BrailleCharUnOrdered> = BrailleCharGridVector::new(w/2, h/4);

    for y in 0..h {
        for x in 0..w {
            let pixel = img.get(x, y);
            let l = pixel.element_sum() /  3.0;
            img.set(x, y, Vec3::splat(l));
        }
    }

    for y in 0..(h/4*4) {
        for x in 0..(w/2*2) {
            let oldpixel = img.get(x, y).clamp(Vec3::ZERO, Vec3::splat(255.0));

            let (b, nl) = match oldpixel.x {
                0.0..127.0 => (false, 0.0),
                _ => (true, 255.0)
            };
            let newpixel = Vec3::splat(nl);

            array.set(x, y, b);

            let quant_error = oldpixel - newpixel;

            let right = x + 1 < w;
            let down  = y + 1 < h;
            let left  = x > 0;

            if right {
                *img.get_mut(x+1, y) += quant_error * 7.0/ 16.0;
            }
            if down {
                if left {
                    *img.get_mut(x-1, y+1) += quant_error * 3.0/ 16.0;
                }
                *img.get_mut(x, y+1) += quant_error * 5.0 / 16.0;
                if right {
                    *img.get_mut(x+1, y+1) += quant_error * 1.0/ 16.0;
                }
            }
        }
    }

    for y in 0..(h/4) {
        for x in 0..(w/2) {
            print!("{}", array.get_char(x, y).unwrap().char());
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let values = if args.is_empty() {
        vec![
            String::from("assets/cat.png"),
            String::from("assets/car.png"),
            String::from("assets/bunny.png")
        ]
    } else {
        args
    };

    for arg in values {
        dither_img(&arg);
    }
}

