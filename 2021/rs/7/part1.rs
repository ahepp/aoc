use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let text = fs::read_to_string("input.txt")?;
    let vals: Vec<u32> = text.trim().split(",").map(|v| v.parse().unwrap()).collect();
    println!("vals:\t{:?}", vals);

    let median = find_median(&vals);
    println!("median:\t{}", median);

    let dist_sum = find_dist_sum(&vals, median);
    println!("{}", dist_sum);

    Ok(())
}

fn find_median(vals: &Vec<u32>) -> u32 {
    let mut max = 0;
    for val in vals {
        if *val > max {
            max = *val;
        }
    }
    println!("max:\t{}", max);

    let mut freqs = vec![0; (max as usize) + 1];
    for val in vals {
        freqs[*val as usize] += 1;
    }
    println!("freqs:\t{:?}", freqs);

    let target = vals.len() / 2;
    let mut cnt = 0;
    println!("target:\t{}", target);

    for (i, freq) in freqs.iter().enumerate() {
        cnt += freq;
        if cnt >= target {
            return i as u32;
        }
    }

    println!("no median found");
    return 0;
}

fn find_dist_sum(vals: &Vec<u32>, median: u32) -> u32 {
    let mut sum = 0;
    for val in vals {
        if median > *val {
            sum += median - *val;
        } else {
            sum += *val - median;
        }
    }
    return sum;
}
