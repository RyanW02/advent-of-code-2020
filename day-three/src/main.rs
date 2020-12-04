use std::fs;

fn main() {
    println!("Part one solution: {}", get_trees(3, 1));
    println!("Part two solution: {}", get_part_two_solution());
}

fn get_part_two_solution() -> usize {
    let offsets = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let mut total = 1;
    offsets.iter()
        .map(|&(x,y)| get_trees(x, y))
        .for_each(|trees| total *= trees);

    total
}

fn get_trees(x_offset: usize, y_offset: usize) -> usize {
    let rows = parse_input();

    let mut x_position = 0;
    rows.iter().skip(y_offset).enumerate()
        .filter(|&(i, _)| i % y_offset == 0) // Get only the rows we hit
        .filter(|&(_, row)| { // Get the rows on which we hit a tree
            x_position += x_offset;

            if x_position >= row.len() {
                x_position -= row.len();
            }

            row[x_position] == Cell::Tree
        })
        .count()
}

// Returns a Vec of rows of cells
fn parse_input() -> Vec<Vec<Cell>> {
    fs::read_to_string("input.txt").unwrap().lines()
        .map(|line| line.chars())
        .map(|chars| chars.map(Cell::from_char).collect())
        .collect()
}

#[derive(Debug, PartialEq)]
enum Cell {
    Tree,
    Empty,
}

impl Cell {
    pub fn from_char(c: char) -> Cell {
        match c {
            '.' => Cell::Empty,
            '#' => Cell::Tree,
            _ => panic!("Invalid character: {}", c)
        }
    }
}

