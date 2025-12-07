use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {

    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut beam_indeces: Vec<u8> = Vec::new();
    let mut total_splits = 0;

    for line_result in reader.lines() {

        let line = line_result?;
        let mut current_indeces: Vec<u8> = line.chars().map(|c| if c == 'S' || c == '|'{
            1
        } else if c == '^' {
            2
        } else {
            0
        }).collect();

        if beam_indeces.is_empty() {
            beam_indeces = current_indeces.clone();
            continue;
        }

        for (idx, &val) in beam_indeces.iter().enumerate() {
            if val == 1 {
                if current_indeces[idx] == 0 { 
                    current_indeces[idx] = 1; 
                } else if current_indeces[idx] == 2 {
                    if idx > 0 { current_indeces[idx-1] = 1; }
                    if idx < beam_indeces.len()-1 { current_indeces[idx+1] = 1; }

                    total_splits += 1;
                }
            }
        }

        beam_indeces = current_indeces.clone();

    }

    println!("{}", total_splits);

    Ok(())

}