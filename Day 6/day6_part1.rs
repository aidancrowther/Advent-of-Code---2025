use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {

    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut problems: Vec<Vec<u64>> = vec![];
    let mut total:u64 = 0;

    for (idx, line_result) in reader.lines().enumerate() {

        let line = line_result?;
        let mut words: Vec<String> = line.split_whitespace().map(|w| w.to_string()).collect();

        if words[0].chars().all(|c| c.is_ascii_digit()) {
            problems.push(words.iter().map(|d| d.parse::<u64>().unwrap()).collect());
        } else {
            while !words.is_empty() {
                let operator = words.pop().unwrap();
                let mut result = problems[0].pop().unwrap();

                for i in 1..idx {
                    if operator == "*" {
                        result = result * problems[i].pop().unwrap();
                    } else if operator == "+" {
                        result = result + problems[i].pop().unwrap();
                    }
                }
                total += result;
            }
        }

    }

    println!("{}", total);
    Ok(())

}