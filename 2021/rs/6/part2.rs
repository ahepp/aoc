use std::fs;
use std::io;

const RESET_PERIOD: usize = 6;
const FULL_PERIOD: usize = 8;
const SIMULATION_DURATION: u64 = 256;

fn main() -> io::Result<()> {
    let text = fs::read_to_string("input.txt")?;
    let mut timers: Vec<u64> = vec![0; FULL_PERIOD + 1];
    for t in text.trim().split(",") {
        let t: usize = t.parse().unwrap();
        timers[t] = timers[t] + 1;
    }

    println!("0:\t{:?}", timers);

    for t in 1..SIMULATION_DURATION+1 {
        let expired = timers[0];
        for i in 1..FULL_PERIOD + 1 {
            timers[i - 1] = timers[i];
        }
        timers[RESET_PERIOD] = timers[RESET_PERIOD] + expired;
        timers[FULL_PERIOD] = expired;

        println!("{}:\t{:?}", t, timers);
    }

    println!("{}", timers.iter().sum::<u64>());
    Ok(())
}
