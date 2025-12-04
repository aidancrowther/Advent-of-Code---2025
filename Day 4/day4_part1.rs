use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {

    let mut grid: Vec<Vec<u32>> = vec![];

    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {

        let line = line_result?;
        grid.push(line.chars().map(|c| {
            if c == '.' {
                0
            } else {
                1
            }
        }).collect());

    }

    let width = grid[0].len();
    let height = grid.len();
    let window_size = 1;
    let mut roll_count = 0;

    for y in 0..height {
        for x in 0..width {

            if grid[y][x] == 0 { continue; }
            
            let mut index_val = 0;

            let min_x = x.saturating_sub(window_size);
            let max_x = if x + window_size < width { x + window_size } else { width-1 };
            let min_y = y.saturating_sub(window_size);
            let max_y = if y + window_size < height { y + window_size } else { height-1 };

            for i in min_x..=max_x {
                for j in min_y..=max_y {
                    if i == x && j == y { continue; }
                    index_val += grid[j][i];
                }
            }

            if index_val < 4 { roll_count += 1; }

        }
    }

    println!("{roll_count}");

    Ok(())

}