use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {

    let mut sum = 0;

    let file_path = "batteries.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {

        let line = line_result?;
        let bank:Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut ones_max = 0;
        let mut tens_max = 0;
        let mut max_right = 0;

        for (idx, val) in bank.iter().rev().enumerate() {    
            if idx > 0 {
                if 10 * *val + max_right > 10*tens_max+ones_max {
                    tens_max = *val;
                    ones_max = max_right;
                }
            }

            if idx == 0 || *val > max_right {
                max_right = *val;
            }
        }

        sum += tens_max*10+ones_max;
        
    }

    println!("{sum}");
    Ok(())

}