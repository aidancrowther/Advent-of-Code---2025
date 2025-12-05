use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {

    let file_path = "database.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut ranges:Vec<Vec<u64>> = vec![];
    let mut ids:Vec<u64> = Vec::new();
    let mut range_input:bool = true;

    for line_result in reader.lines() {

        let line = line_result?;
        
        if line == "" {
            range_input = false;
        } else if range_input {
            ranges.push(line.split("-").map(|s| s.parse::<u64>().unwrap()).collect());
        } else {
            ids.push(line.parse::<u64>().unwrap());
        }

    }

    let mut fresh = 0;

    ranges.sort_by(|a, b| a[1].cmp(&b[1]));
    ids.sort();

    for id in ids {
        for range in &ranges {
            if range[0] <= id && id <= range[1] {
                fresh += 1;
                break;
            }
        }
    }

    println!("{fresh}");

    Ok(())

}