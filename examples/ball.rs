use braille::{BrailleCharUnOrdered, BrailleCharGridArray};


fn main() {
    println!("Hello, world!");

    const W: usize = 20;
    const H: usize = 10;
    let mut array: BrailleCharGridArray<BrailleCharUnOrdered, W, H> = BrailleCharGridArray::new();
    let w = W * 2;
    let h = H * 4;
    for x in 0..w {
        for y in 0..h {
            let d2 = (x - W).pow(2) + (y - H * 2).pow(2);
            if d2 < (W / 2).pow(2) {
                array.set(x, y, x % 2 - y % 2 == 0)
            } else if d2 < W.pow(2) {
                array.set(x, y, true);
            }
        }
    }

    for y in 0..H {
        for x in 0..W {
            print!("{}", array.get_char(x, y).unwrap().char());
        }
        println!();
    }
}

