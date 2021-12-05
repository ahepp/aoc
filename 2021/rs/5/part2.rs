#![feature(mixed_integer_ops)]

use std::cmp;
use std::fs;
use std::io;

const ROWS: usize = 1000;
const COLS: usize = 1000;
const DANGER_THRESHOLD: usize = 2;

type Line = ((usize, usize), (usize, usize));

fn main() -> io::Result<()> {
    let text = fs::read_to_string("input.txt")?;

    let lines: Vec<Line> = text.trim().split("\n").map(|line| {
            let mut line_it = line.split(" -> ");
            let mut start_it = line_it.next().unwrap().split(",");
            let mut end_it = line_it.next().unwrap().split(",");
            let sx = start_it.next().unwrap().parse().unwrap();
            let sy = start_it.next().unwrap().parse().unwrap();
            let ex = end_it.next().unwrap().parse().unwrap();
            let ey = end_it.next().unwrap().parse().unwrap();
            ((ex, ey), (sx, sy))
        }).collect();

    let mut map: Vec<Vec<usize>> = vec![vec![0; COLS]; ROWS];

    println!("{:?}", lines);

    for line in lines {
        let ((sx, sy), (ex, ey)) = line;
        let dy: isize = ey as isize - sy as isize;
        let dx: isize = ex as isize - sx as isize;
        if (dx == 0) || (dy == 0) || (dx.abs() == dy.abs()) {
            draw_line(&mut map, &line);
        }
    }

    println!("\n{:?}", map);

    let mut sum = 0;
    for row in map {
        for col in row {
            if col >= DANGER_THRESHOLD {
                sum = sum + 1;
            }
        }
    }
    println!("dangerous areas: {}", sum);

    Ok(())
}

fn draw_line(map: &mut Vec<Vec<usize>>, line: &Line) {
    let ((sx, sy), (ex, ey)) = *line;
    let dy: isize = ey as isize - sy as isize;
    let dx: isize = ex as isize - sx as isize;
    let mag = cmp::max(dx.abs(), dy.abs());
    println!("\n{:?}", line);
    println!("dx: {}\tdy: {}\tmag: {}", dx, dy, mag);
    let xstep = dx / mag;
    let ystep = dy / mag;
    for i in 0..(mag+1) {
        let x: usize = sx.saturating_add_signed(xstep * i);
        let y: usize = sy.saturating_add_signed(ystep * i);
        print!("({}, {}), ", x, y);
        map[x][y] = map[x][y] + 1;
    }
    println!();
}
