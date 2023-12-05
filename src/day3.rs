/*
--- Day 3: Gear Ratios ---
You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

*/

use std::fs::read_to_string;

use nom::branch::alt;
use nom::character::complete::{anychar, char};
use nom::combinator::map;
use nom::error::Error;

#[derive(Debug)]
struct Number {
    value: u64,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Symbol {
    index: usize,
}

impl Number {
    fn part(&self, symbols: &Vec<Symbol>, width: usize) -> bool {
        symbols.iter().any(|s| {
            //eprintln!("{:?}, {:?}", s, self);
            (s.index >= self.start - 1 && s.index <= self.end + 1)
                || (s.index - width >= self.start - 1 && s.index - width <= self.end + 1)
                || (s.index + width >= self.start - 1 && s.index + width <= self.end + 1)
        })
    }
}

pub fn part1(file_name: &str) -> u64 {
    let input = read_to_string(file_name).unwrap();
    let width = input.lines().next().unwrap().len();
    let (numbers, symbols) = parse_schematic(&input);
    numbers
        .iter()
        .map(|g| if g.part(&symbols, width) { g.value } else { 0 })
        .sum()
}

#[derive(Debug)]
enum Token {
    Num(u64),
    Sym,
    Dot,
}

fn parse_schematic(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let input = input.replace("\n", "");
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let mut curr = 1;
    let mut input = input.as_str();
    loop {
        let orig_len = input.len();
        let (remaining, token) = alt::<_, _, Error<_>, _>((
            map(char('.'), |_| Token::Dot),
            map(nom::character::complete::u64, Token::Num),
            map(anychar, |_| Token::Sym),
        ))(input)
        .unwrap();
        //eprintln!("{}, {:?}", remaining, token);

        match token {
            Token::Num(value) => {
                numbers.push(Number {
                    value,
                    start: curr,
                    end: curr + orig_len - remaining.len(),
                });
            }
            Token::Sym => {
                symbols.push(Symbol { index: curr });
            }
            Token::Dot => {}
        }
        if remaining.is_empty() {
            break;
        }
        curr += orig_len - remaining.len();
        input = remaining;
    }
    (numbers, symbols)
}

#[cfg(test)]
mod test {
    #[test]
    fn part1_example() {
        assert_eq!(super::part1("src/day3_test_input.txt"), 4361);
    }
    #[test]
    fn part1_actual() {
        assert_eq!(super::part1("src/day3_input.txt"), 4361)
    }
}
