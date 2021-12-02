use std::fs;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let br = io::BufReader::new(file);

    let mut cnt: i32 = -1;
    let mut last_depth: i32 = -1;
    for line in br.lines() {
        let depth: i32 = line
            .expect("Failed to read line.")
            .trim()
            .parse()
            .expect("Failed to parse line.");
        if depth > last_depth {
            cnt = cnt + 1;
        }
        last_depth = depth;
    }

    println!("{}", cnt);

    Ok(())
}
