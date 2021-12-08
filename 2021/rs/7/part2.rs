use std::cmp;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let text = fs::read_to_string("input.txt")?;
    let vals: Vec<u32> = text.trim().split(",").map(|v| v.parse().unwrap()).collect();
    println!("vals:\t{:?}", vals);

    let mean_idx_floor: u32 = ((vals.iter().sum::<u32>() as f64) / (vals.len() as f64)) as u32;
    let min_sum = cmp::min(find_dist_sum(&vals, mean_idx_floor), find_dist_sum(&vals, mean_idx_floor + 1));
    println!("{}", mean_idx_floor);
    println!("{}", min_sum);

    Ok(())
}

fn find_dist_sum(vals: &Vec<u32>, median: u32) -> u32 {
    let mut sum = 0;
    for val in vals {
        let mut n = 0;
        if median > *val {
            n = median - *val; 
        } else {
            n = *val - median;
        }
        sum += n * (n + 1) / 2;
    }
    return sum;
}
