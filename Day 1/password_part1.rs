use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {

    let mut idx = 50;
    let mut pass = 0;

    let file_path = "directions.txt"; // Replace with your file name

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?; // Handle potential errors for each line
        let (direction, turns) = line.split_at(1);
        rotate(&mut idx, &mut pass, if direction == "L" {true} else {false}, turns.parse::<i32>().unwrap());
    }

    println!("{}", pass);

    Ok(())

}

//left = true
//right = false
fn rotate(idx: &mut i32, pass: &mut i32, direction: bool, turns: i32){
    
    if direction {
        *idx = (((*idx - turns) % 100) + 100) % 100;
    } else {
        *idx = (((*idx + turns) % 100) + 100) % 100;
    }

    if *idx == 0 { *pass += 1; }

}