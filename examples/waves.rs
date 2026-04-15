use braille::{BrailleCharUnOrdered, BrailleCharGridArray};


fn main() {
    println!("Hello, world!");

    const W: usize = 50;
    const H: usize = 8;
    let mut array: BrailleCharGridArray<BrailleCharUnOrdered, W, H> = BrailleCharGridArray::new();
    let w = W * 2;
    let h = H * 4;
    for x in 0..w {
        for y in 0..h {
            let t = x as f32 / 2.0;
            let c = t.cos() * 3.0;
            let s = t.cos() * 3.0 - 7.0;
            let y_ = y as f32 - h as f32 / 2.0;
            let b = if y_ > c {
                true
            } else if y_ < s {
                false
            } else {
                x % 2 - y % 2 == 0
            };

            array.set(x, y, b);
        }
    }

    for y in 0..H {
        for x in 0..W {
            print!("{}", array.get_char(x, y).unwrap().char());
        }
        println!();
    }
}

