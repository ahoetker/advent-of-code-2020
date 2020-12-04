use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Route {
    position: (usize, usize),
    max_height: usize,
    rise: usize,
    run: usize,
}

impl Route {
    fn new(max_height: usize, rise: usize, run: usize) -> Self {
        Route {
            position: (0, 0),
            max_height,
            rise,
            run,
        }
    }
}

impl Iterator for Route {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.position.0 += self.run;
        self.position.1 += self.rise;

        if self.position.1 < self.max_height {
            Some(self.position)
        } else {
            None
        }
    }
}

struct Field {
    rows: Vec<Vec<char>>,
    height: usize,
    pattern_width: usize,
}

impl Field {
    fn new(input_file: &str) -> Self {
        let input = File::open(input_file).unwrap();
        let reader = BufReader::new(input);
        let rows: Vec<Vec<char>> = reader
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect();
        let height = rows.len();
        let pattern_width = rows[0].len();
        Field {
            rows,
            height,
            pattern_width,
        }
    }

    fn get_square(&self, x: usize, y: usize) -> char {
        self.rows[y][x % self.pattern_width]
    }
}

fn main() {
    let field = Field::new("input.txt");

    // Part 1
    let route = Route::new(field.height, 1, 3);
    let num_trees = route
        .into_iter()
        .map(|(x, y)| field.get_square(x, y))
        .filter(|&c| c == '#')
        .count();
    println!("{}", num_trees);

    // Part 2
    let mut num_trees = 1;
    for (run, rise) in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let route = Route::new(field.height, rise, run);
        num_trees *= route
            .into_iter()
            .map(|(x, y)| field.get_square(x, y))
            .filter(|&c| c == '#')
            .count();
    }
    println!("{}", num_trees);
}
