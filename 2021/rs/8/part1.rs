use std::fs;
use std::io;
use std::collections;

fn main() -> io::Result<()> {
    let text = fs::read_to_string("input.txt")?;
    let lines = text.trim().split("\n");

    let mut easy_nums = 0;
    for line in lines {
        let mut split = line.split("|");
        let digits: Vec<&str> = split.next().unwrap().trim().split(" ").collect();
        let outputs: Vec<&str> = split.next().unwrap().trim().split(" ").collect();

        let mut seg_cnt_to_dgt = collections::HashMap::new();
        seg_cnt_to_dgt.insert(2, 1);
        seg_cnt_to_dgt.insert(4, 4);
        seg_cnt_to_dgt.insert(3, 7);
        seg_cnt_to_dgt.insert(7, 8);

        for output in outputs {
            easy_nums += match seg_cnt_to_dgt.get(&output.len()) {
                Some(_) => 1,
                _ => 0,
            }
        }
    }
    println!("{}", easy_nums);

    Ok(())
}
