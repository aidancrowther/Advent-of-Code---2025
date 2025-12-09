use std::fs::File;
use std::cmp;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {

    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut points: Vec<Vec<u32>> = vec![];
    let mut max_dimension: usize = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        points.push(line.split(",").map(|p| p.parse::<u32>().unwrap()).collect());

        let x = points.last().unwrap()[0] as usize;
        let y = points.last().unwrap()[1] as usize;

        if x > max_dimension { max_dimension = x; }
        if y > max_dimension { max_dimension = y; }
    }

    let num_points = points.len();
    let mut grid: Vec<Vec<u8>> = vec![vec![0; max_dimension+1]; max_dimension+1];
    let mut last_point = points[0].clone();
    points.push(last_point.clone());

    for idx in 1..points.len() {
        let point = &points[idx];

        let curr_x = point[0];
        let curr_y = point[1];
        let last_x = last_point[0];
        let last_y = last_point[1];

        if last_x == curr_x {
            let (start, end) = (cmp::min(curr_y, last_y), cmp::max(curr_y, last_y));
            for y in start..=end {
                grid[y as usize][curr_x as usize] = 1;
            }
        } else {
            let (start, end) = (cmp::min(curr_x, last_x), cmp::max(curr_x, last_x));
            for x in start..=end {
                grid[curr_y as usize][x as usize] = 1;
            }
        }
        last_point = point.to_vec();
    }

    flood_fill(&mut grid, 0, 0);
    for line in &mut grid { line.iter_mut().for_each(|v| if *v == 0 { *v = 1; }); }
    for line in &mut grid { line.iter_mut().for_each(|v| if *v == 2 { *v = 0; }); }

    let mut integral_matrix: Vec<Vec<u32>> = vec![vec![0; grid.len()+1]; grid.len()+1];

    for y in 0..grid.len() {
        let mut sum = 0;
        for x in 0..grid.len() {
            sum += grid[y][x] as u32;
            integral_matrix[y+1][x+1] = integral_matrix[y][x+1] + sum;
        }
    }
        let mut largest: (u32, usize, usize) = (0, 0, 0);
        for i in 0..num_points {
            for j in i+1..num_points {
                let first = &points[i];
                let second = &points[j];
                
                let width = cmp::max(first[0], second[0]) - cmp::min(first[0], second[0]) + 1;
                let height = cmp::max(first[1], second[1]) - cmp::min(first[1], second[1]) + 1;
                let area = width * height;

                if area <= largest.0 { continue; }

                let start_x = cmp::min(first[0], second[0]) as usize;
                let start_y = cmp::min(first[1], second[1]) as usize;
                let end_x = cmp::max(first[0], second[0]) as usize;
                let end_y = cmp::max(first[1], second[1]) as usize;

                let sum: u32 = integral_matrix[end_y + 1][end_x + 1] - 
                integral_matrix[end_y + 1][start_x] -
                integral_matrix[start_y][end_x + 1] +
                integral_matrix[start_y][start_x];

                if sum == area {
                    largest.0 = area;
                    largest.1 = i;
                    largest.2 = j;
                }
            }
        }
    println!("Largest area is {} between points {:?} {:?}", largest.0, points[largest.1], points[largest.2]);
    Ok(())

}

fn flood_fill(matrix: &mut Vec<Vec<u8>>, start_x: usize, start_y: usize) {
    
    let dim = matrix.len()-1;
    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push((start_x, start_y));

    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();

        if x > dim || y > dim { continue; }
        if matrix[y][x] != 0 { continue; }

        matrix[y][x] = 2;

        if x > 0 { stack.push((x-1, y)); }
        if x < dim { stack.push((x+1, y)); }
        if y > 0 { stack.push((x, y-1)); }
        if y < dim { stack.push((x, y+1)); }
    }
}