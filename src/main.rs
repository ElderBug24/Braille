use braille::BrailleChar;

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

    println!("{:08b}", 0b1100_1011);

    let a = BrailleChar::UnOrdered(0b_11001011);
    let a = BrailleChar::Ordered(a.ordered());
    let a = BrailleChar::UnOrdered(a.unordered());
    let c = a.char();

    println!("b: |{}|", c);

    'main: loop {
        // let input = input("03\n14\n25\n67\n  01234567\n> ").unwrap();
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
