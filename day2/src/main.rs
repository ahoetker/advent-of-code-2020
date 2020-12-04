use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct CorporatePassword {
    first_number: u32,
    second_number: u32,
    character: char,
    text: String,
}

impl CorporatePassword {
    fn new(entry: &str) -> Self {
        let substrings: Vec<&str> = entry.split(' ').collect();
        let (repetitions, character, text) =
            (substrings[0], substrings[1], substrings[2].to_owned());
        let repetitions: Vec<u32> = repetitions
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        Self {
            first_number: repetitions[0],
            second_number: repetitions[1],
            character: character.chars().nth(0).unwrap(),
            text: text,
        }
    }

    fn validate_part_1(&self) -> bool {
        let num = self.text.chars().filter(|&c| c == self.character).count() as u32;
        num >= self.first_number && num <= self.second_number
    }

    fn validate_part_2(&self) -> bool {
        let first_char = self
            .text
            .chars()
            .nth(self.first_number as usize - 1)
            .unwrap();
        let second_char = self
            .text
            .chars()
            .nth(self.second_number as usize - 1)
            .unwrap();
        (first_char == self.character && second_char != self.character)
            || (first_char != self.character && second_char == self.character)
    }
}

fn read_input() -> std::io::Result<impl Iterator<Item = CorporatePassword>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|line| CorporatePassword::new(&line.unwrap())))
}

fn main() {
    // Part 1
    let num_valid = read_input()
        .unwrap()
        .filter(|password| password.validate_part_1())
        .count();
    println!("{}", num_valid);

    // Part 2
    let num_valid = read_input()
        .unwrap()
        .filter(|password| password.validate_part_2())
        .count();
    println!("{}", num_valid);
}
