use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {

    let mut sum:u64 = 0;

    let file_path = "batteries.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {

        let line = line_result?;
        let bank:Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut to_remove = bank.len() - 12;
        let mut array:Vec<u32> = Vec::with_capacity(bank.len());

        for &val in bank.iter() {

            while to_remove > 0 && !array.is_empty() && *array.last().unwrap() < val {
                array.pop();
                to_remove -= 1;
            }

            array.push(val);

        }

        while array.len() > 12 {
            array.pop();
        }

        for i in 0..12{
            sum += (array.pop().unwrap() as u64) * 10_u64.pow(i);
        }
        
    }

    println!("{sum}");
    Ok(())

}