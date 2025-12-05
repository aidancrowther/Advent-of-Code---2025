use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {

    let file_path = "database.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut ranges:Vec<Vec<u64>> = vec![];

    for line_result in reader.lines() {

        let line = line_result?;
        
        if line == "" {
            break;
        } else {
            ranges.push(line.split("-").map(|s| s.parse::<u64>().unwrap()).collect());
        }

    }

    ranges.sort_by(|a, b| a[1].cmp(&b[1]));

    let mut range = ranges.pop().unwrap();
    let (mut min, mut max) = (range[0], range[1]);
    let mut fresh_ids:u64 = 0;

    while !ranges.is_empty() {
        
        range = ranges.pop().unwrap();
        let (curr_min, curr_max) = (range[0], range[1]);

        if curr_max >= min {
            if curr_min < min { min = curr_min; }
        } else {
            fresh_ids += max - min + 1;
            (max, min) = (curr_max, curr_min);
        }

        if ranges.is_empty() { fresh_ids += max - min + 1; }
    }

    println!("{fresh_ids}");

    Ok(())

}