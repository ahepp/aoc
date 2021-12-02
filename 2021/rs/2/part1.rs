use std::fs;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let br = io::BufReader::new(file);

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for line in br.lines() {
        let unwrapped = line.expect("Failed to read line.");
        let mut split = unwrapped.split(" ");
        let cmd = split.next().expect("Failed to get command token.");
        let mag: i32 = split
            .next()
            .expect("Failed to get magnitude token.")
            .parse()
            .expect("Failed to parse magnitude.");
        match cmd {
            "forward" => x += mag,
            "up" => y -= mag,
            "down" => y += mag,
            _ => panic!("Unknown command"),
        }
    }

    println!("x: {}\ty: {}\n{}", x, y, x * y);

    Ok(())
}
