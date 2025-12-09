use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

fn main() -> io::Result<()> {

    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut next_net = 1;
    let num_to_check = 1000;
    let num_to_collect = 3;
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

    let mut connection_matrix: Vec<Vec<Option<i64>>> = vec![vec![None; num_points]; num_points];

    for edge in 0..num_to_check {
        if !connection_matrix[edges[edge].1][edges[edge].2].is_some() {
            connection_matrix[edges[edge].1][edges[edge].2] = Some(next_net);
            connection_matrix[edges[edge].2][edges[edge].1] = Some(next_net);

            next_net += 1;
        }
    }

    let mut connected_nodes: Vec<HashSet<usize>> = Vec::new();

    for x in 0..num_points {
        for y in x+1..num_points {
            if connection_matrix[x][y].is_some() {

                let mut index: Option<usize> = None;

                for i in 0..connected_nodes.len() {
                    if connected_nodes[i].contains(&x) || connected_nodes[i].contains(&y) {
                        connected_nodes[i].insert(x);
                        connected_nodes[i].insert(y);
                        index = Some(i);
                    }
                }

                if index.is_none() {
                    let mut new_set = HashSet::new();
                    new_set.insert(x);
                    new_set.insert(y);
                    connected_nodes.push(new_set);
                }

            }
        }
    }

    let mut union_sets: Vec<HashSet<usize>> = Vec::new();

    while !connected_nodes.is_empty() {
        let mut i = 0;
        let mut set = connected_nodes.pop().unwrap();

        while i < union_sets.len(){
            if !union_sets[i].is_disjoint(&set) {
                let merge = union_sets.swap_remove(i);
                set = set.union(&merge).copied().collect();
            } else { i += 1; }
        }

        union_sets.push(set);
    }

    union_sets.sort_by_key(|v| v.len());

    let mut total = union_sets.pop().unwrap().len();

    for _ in 1..num_to_collect { total = total * union_sets.pop().unwrap().len(); }

    println!("{}", total);

    Ok(())

}

fn get_distance(point_1: &Vec<i64>, point_2: &Vec<i64>) -> i64 {
    
    let mut distance: i64 = 0;

    for i in 0..point_1.len() {
        distance += (point_1[i] - point_2[i]).pow(2);
    }

    return distance;

}