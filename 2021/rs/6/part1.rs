use std::fs;
use std::io;

const PERIOD: u32 = 7;
const DURATION: u32 = 80;

fn main() -> io::Result<()> {
    let text = fs::read_to_string("input.txt")?;
    let mut timers: Vec<u32> = text.trim().split(",")
        .map(|t|t.parse().unwrap())
        .collect();

    println!("{:?}", timers);

    for t in 0..DURATION {
        let mut expired = 0;
        for timer in timers.iter_mut() {
            if *timer == 0 {
                *timer = PERIOD;
                expired = expired + 1;
            }
            *timer = *timer - 1;
        }
        let mut generated = vec![PERIOD + 1; expired];
        timers.append(&mut generated);
        println!("{}:\t{:?}",t, timers);
    }
    println!("{}", timers.len());

    Ok(())
}
