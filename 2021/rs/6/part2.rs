use std::fs;
use std::io;

const RESET_PERIOD: usize = 6;
const NEW_PERIOD: usize = 8;
const SIMULATION_DURATION: u64 = 256;

fn main() -> io::Result<()> {
    let text = fs::read_to_string("input.txt")?;
    let mut timers: [u64; NEW_PERIOD + 1] = [0; NEW_PERIOD + 1];
    for t in text.trim().split(",") {
        let t: usize = t.parse().unwrap();
        timers[t] += 1;
    }

    println!("0:\t{:?}", timers);
    for t in 0..SIMULATION_DURATION {
        timers.rotate_left(1);
        timers[RESET_PERIOD] += timers[NEW_PERIOD];
        println!("{}:\t{:?}", t + 1, timers);
    }

    println!("{}", timers.iter().sum::<u64>());
    Ok(())
}
