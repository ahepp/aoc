use std::fs;
use std::io;
use std::io::prelude::*;

const WINDOW: usize = 3;

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let br = io::BufReader::new(file);

    let mut i: usize = 0;
    let mut cnt: i32 = 0;
    let mut depths: [i32; WINDOW] = [0; WINDOW];
    for line in br.lines() {
        if i >= WINDOW {
            let depth: i32 = line
                .expect("Failed to read line.")
                .trim()
                .parse()
                .expect("Failed to parse line.");
            if depth > depths[i % WINDOW] {
                cnt = cnt + 1;
            }
            depths[i % WINDOW] = depth;
        }
        i = i + 1;
    }

    println!("{}", cnt);

    Ok(())
}
