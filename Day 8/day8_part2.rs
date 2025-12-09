use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

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
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();

    for x in 0..num_points {
        for y in x+1..num_points {
            edges.push((get_distance(&points[x], &points[y]), x, y));
        }
    }

    edges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut connections: Vec<HashSet<usize>> = Vec::new();

    while !edges.is_empty() {

        let mut node: HashSet<usize> = HashSet::new();
        let edge = edges.pop().unwrap();
        
        node.insert(edge.1);
        node.insert(edge.2);
        connections.push(node);
        
    }

    let mut union_sets: Vec<HashSet<usize>> = Vec::new();

    while !connections.is_empty() {
        let mut i = 0;
        let mut set = connections.pop().unwrap();
        let curr_set = set.clone();

        while i < union_sets.len(){
            if !union_sets[i].is_disjoint(&set) {
                let merge = union_sets.swap_remove(i);
                set = set.union(&merge).copied().collect();
            } else { i += 1; }
        }

        union_sets.push(set.clone());

        if set.len() == num_points {
            
            let mut total = 1;

            for point in curr_set {
                total *= points[point][0];
            }

            println!("{}", total);
            break;
        }
    }

    Ok(())

}

fn get_distance(point_1: &Vec<i64>, point_2: &Vec<i64>) -> i64 {
    
    let mut distance: i64 = 0;

    for i in 0..point_1.len() {
        distance += (point_1[i] - point_2[i]).pow(2);
    }

    return distance;

}