use std::io::{self, BufRead};

fn get_number_of_trees(right: usize, down: usize, board: &[Vec<char>]) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    while x < board.len() {
        let line = &board[x];
        if line[y] == '#' {
            trees = trees + 1;
        }

        x += down;
        y = (y + right) % line.len();
    }

    trees
}

fn main() {
    let stdin = io::stdin();
    let board: Vec<Vec<_>> = stdin.lock().lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut product = 1u64;

    {
        let result = get_number_of_trees(1, 1, &board);
        println!("{}", result);
        product = product * (result as u64);
    }

    {
        let result = get_number_of_trees(3, 1, &board);
        println!("{}", result);
        product = product * (result as u64);
    }

    {
        let result = get_number_of_trees(5, 1, &board);
        println!("{}", result);
        product = product * (result as u64);
    }

    {
        let result = get_number_of_trees(7, 1, &board);
        println!("{}", result);
        product = product * (result as u64);
    }

    {
        let result = get_number_of_trees(1, 2, &board);
        println!("{}", result);
        product = product * (result as u64);
    }

    println!("{}", product);
}
