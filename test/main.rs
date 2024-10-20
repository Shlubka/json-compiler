use rand::Rng;
use std::collections::VecDeque;
use std::{
    collections::HashSet,
    io::{self, Write},
};

fn read_from_keyboard() -> u32 {
    io::stdout().flush().expect("flush error");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    if let Ok(i) = trimmed.parse::<u32>() {
        return i;
    } else {
        println!("\x1b[31m!!!No number in input, установлено значение по умолчанию(10)!!!\x1b[0m");
        return 10;
    }
}

fn create_graph(size: usize) -> Vec<Vec<u32>> {
    let mut rng = rand::thread_rng();
    let mut graf = vec![vec![0u32; size]; size];

    for i in 0..size {
        for j in i..size {
            if i == j {
                continue;
            }
            graf[i][j] = rng.gen::<u32>() % 2;
            graf[j][i] = graf[i][j];
        }
    }

    return graf;
}

fn visit_rec(graf: &Vec<Vec<u32>>, visited: &mut HashSet<u32>, node: u32, size: u32) {
    if visited.insert(node) {
        print!("{} ", node + 1);
        for j in 0..size {
            if graf[node as usize][j as usize] == 1 {
                visit_rec(graf, visited, j, size);
            }
        }
    }
}

fn visit_no_rec(graf: &Vec<Vec<u32>>, visited: &mut HashSet<u32>, size: u32) {
    let mut stack = VecDeque::new();
    stack.push_back(0);

    while let Some(node) = stack.pop_back() {
        if visited.contains(&node) {
            continue;
        }

        print!("{} ", node + 1);
        visited.insert(node);

        for j in (0..size).rev() {
            if graf[node as usize][j as usize] == 1 && !visited.contains(&j) {
                stack.push_back(j);
            }
        }
    }
}

fn create_adjacency_list(graf: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let size = graf.len();
    let mut adj_list = vec![vec![]; size];

    for i in 0..size {
        for j in 0..size {
            if graf[i][j] == 1 {
                adj_list[i].push(j as u32);
            }
        }
    }

    adj_list
}

fn visit_rec_adj_list(adj_list: &Vec<Vec<u32>>, visited: &mut HashSet<u32>, node: u32) {
    if visited.insert(node) {
        print!("{} ", node + 1);
        for &neighbor in &adj_list[node as usize] {
            visit_rec_adj_list(adj_list, visited, neighbor);
        }
    }
}

fn visit_no_rec_adj_list(adj_list: &Vec<Vec<u32>>, visited: &mut HashSet<u32>, node: u32) {
    let mut stack = VecDeque::new();
    stack.push_back(node);
    while let Some(node) = stack.pop_back() {
        if visited.contains(&node) {
            continue;
        }

        print!("{} ", node + 1);
        visited.insert(node);

        for j in adj_list[node as usize].iter().rev() {
            if !visited.contains(&j) {
                stack.push_back(*j);
            }
        }
    }
}

fn main() {
    let mut visited = HashSet::<u32>::new();
    print!("Enter graph size, more than 0 (one number for line and column) > ");
    let size = read_from_keyboard();
    let graf = create_graph(size as usize);

    println!("Graph:");
    for row in &graf {
        println!("{row:?}");
    }

    println!("Recursive:");
    visit_rec(&graf, &mut visited, 0, size);

    visited.clear();

    println!("\nNon-recursive:");
    visit_no_rec(&graf, &mut visited, size);
    println!();

    visited.clear();
    let adj_list = create_adjacency_list(&graf);
    println!("Adjacency list");
    for row in &adj_list {
        println!("{row:?}");
    }
    println!("\nRecursive (adjacency list):");
    visit_rec_adj_list(&adj_list, &mut visited, 0);
    println!();

    visited.clear();
    println!("\nNon-recursive (adjacency list):");
    visit_no_rec_adj_list(&adj_list, &mut visited, 0);
    println!();
}
