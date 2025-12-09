use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {

    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut points: Vec<Vec<i64>> = vec![];

    for line_result in reader.lines() {
        let line = line_result?;
        points.push(line.split(",").map(|p| p.parse::<i64>().unwrap()).collect());
    }

    let num_points = points.len();

    let mut largest: (i64, usize, usize) = (0, 0, 0);

    for x in 0..num_points {
        for y in x+1..num_points {
            let first = &points[x];
            let second = &points[y];

            let width = (first[0] - second[0]).abs() + 1;
            let height = (first[1] - second[1]).abs() + 1;
            let area = width * height;

            if area > largest.0 {
                largest.0 = area;
                largest.1 = x;
                largest.2 = y;
            }
        }
    }

    println!("Largest area is {}, using points {:?} and {:?}", largest.0, points[largest.1], points[largest.2]);

    Ok(())

}