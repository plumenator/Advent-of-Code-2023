/*
our puzzle answer was 538046.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---
The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

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
In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?

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
            let yes = (s.index >= self.start - 1 && s.index <= self.end + 1)
                || (s.index - width >= self.start - 1 && s.index - width <= self.end + 1)
                || (s.index + width >= self.start - 1 && s.index + width <= self.end + 1);
            if yes {
                eprintln!("{:?}, {:?}", s, self);
            }
            yes
        })
    }
}

pub fn part1(file_name: &str) -> u64 {
    let input = read_to_string(file_name).unwrap();
    let width = input.lines().next().unwrap().len();
    let (numbers, symbols) = parse_schematic(&input);
    numbers
        .iter()
        .filter(|n| n.part(&symbols, width))
        .map(|n| n.value)
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
                    end: curr + orig_len - remaining.len() - 1,
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
        assert_eq!(super::part1("src/day3_input.txt"), 538046)
    }
}
