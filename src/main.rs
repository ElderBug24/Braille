use braille::{BrailleChar, BrailleCharGridArray};

use std::io::{self, Write};


fn input(prompt: &str) -> Result<String, &str> {
    let mut s = String::new();
    print!("{prompt}");
    let _ = io::stdout().flush();
    match io::stdin().read_line(&mut s) {
        Ok(_) => {
            if let Some('\n') = s.chars().next_back() { s.pop(); }
            if let Some('\r') = s.chars().next_back() { s.pop(); }
            return Ok(s)
    },
        Err(_) => return Err("Error: Failed to read line")
    }
}

fn main() {
    println!("Hello, world!");

    const W: usize = 10;
    const H: usize = 10;
    let mut array: BrailleCharGridArray<W, H> = BrailleCharGridArray::new();
    let w = W * 2;
    let h = H * 4;
    for x in 0..w {
        for y in 0..h {
            let d2 = (x - W).pow(2) + (y - H * 2).pow(2) / 4;
            if d2 < (W / 2).pow(2) {
                array.set(x, y, true);
            }
        }
    }

    for y in 0..W {
        for x in 0..H {
            print!("{}", array.get_char(x, y).char());
        }
        println!();
    }

    'main: loop {
        let input = input("01\n23\n45\n67\n  01234567\n> ").unwrap();

        if input.len() != 8 {
            break 'main;
        }

        let mut byte = 0u8;
        for (i, char) in input.chars().enumerate() {
            if char == '0' {}
            else if char == '1' {
                byte += 0b_1000_0000 >> i;
            } else { break 'main; }
        }
        let char = BrailleChar::UnOrdered(byte);

        println!("\n{}\n", char.char());
    }
}
