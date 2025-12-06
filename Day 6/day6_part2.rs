use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {

    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut problems: Vec<Vec<char>> = vec![];
    let mut operators: Vec<String> = Vec::new();
    let mut problem_length = 0;
    let mut total:u64 = 0;

    for (idx, line_result) in reader.lines().enumerate() {

        let line = line_result?;

        if line.chars().all(|c| c.is_ascii_digit() || c.is_whitespace()) {
            problems.push(line.chars().collect());
        } else {
            problem_length = idx;
            operators = line.split_whitespace().map(|w| w.to_string()).collect();
        }
    }

    while !operators.is_empty(){
        let operator = operators.pop().unwrap();
        let mut result = 0;
        let mut values: Vec<char> = Vec::new();
        values.resize(problem_length, '0');

        while !values.iter().all(|c| *c == ' ') && !problems.iter().all(|p| p.is_empty()){
            for i in 0..problem_length {
                values[i] = problems[i].pop().unwrap();
            }

            let number: String = values.iter().filter(|c| **c != ' ').collect::<String>();

            if number != "" {
                let parsed: u64 = number.parse().unwrap();
                if result == 0 {
                    result = parsed;
                } else if operator == "*" {
                    result = result * parsed;
                } else {
                    result = result + parsed;
                }
            }
        }
        total += result;
    }

    println!("{}", total);
    Ok(())

}