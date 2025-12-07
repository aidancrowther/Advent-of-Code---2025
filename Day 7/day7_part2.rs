use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {

    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut beam_indeces: Vec<i32> = Vec::new();
    let mut adjacencies: Vec<Vec<i32>> = vec![];
    let mut id = 0;

    for line_result in reader.lines() {

        let line = line_result?;
        let mut current_indeces: Vec<i32> = line.chars().map(|c| if c == 'S' || c == '|'{
            1
        } else if c == '^' {
            id += 1;
            -1 * id
        } else {
            0
        }).collect();

        if beam_indeces.is_empty() {
            beam_indeces = current_indeces.clone();
            adjacencies.push(current_indeces.clone());
            continue;
        }

        for (idx, &val) in beam_indeces.iter().enumerate() {
            if val > 0 {
                if current_indeces[idx] == 0 { 
                    current_indeces[idx] = 1; 
                } else if current_indeces[idx] < 0 {
                    if idx > 0 { current_indeces[idx-1] = 1; }
                    if idx < beam_indeces.len()-1 { current_indeces[idx+1] = 1; }
                }
            }
        }

        beam_indeces = current_indeces.clone();
        adjacencies.push(current_indeces.clone());

    }

    let mut cache: Vec<Vec<Option<u64>>> = vec![vec![None; adjacencies[0].len()]; adjacencies.len()];
    let count = find_neighbours(&adjacencies, &mut cache, adjacencies[0].iter().position(|&p| p == 1).unwrap(), 1);
    println!("{}", count);

    Ok(())

}

fn find_neighbours(adjacencies: &Vec<Vec<i32>>, cache: &mut Vec<Vec<Option<u64>>>, index: usize, depth: usize) -> u64 {

    if depth >= adjacencies.len() { return 1; }

    let mut sum = 0;

    if let Some(cached) = cache[depth][index] {
        return cached;
    }

    if index > 0 && adjacencies[depth][index] < 0 {
        sum += find_neighbours(adjacencies, cache, index-1, depth+1);
    }
    
    if index < adjacencies[0].len()-1 && adjacencies[depth][index] < 0 {
        sum += find_neighbours(adjacencies, cache, index+1, depth+1);
    }

    if adjacencies[depth][index] == 1 {
        sum += find_neighbours(adjacencies, cache, index, depth+1);
    }

    cache[depth][index] = Some(sum);
    return sum;
}