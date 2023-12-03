/*
--- Day 1: Trebuchet?! ---
Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?

Your puzzle answer was 56397.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---
Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?
*/

use std::fs::read_to_string;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::combinator::map;
use nom::multi::many1;
use nom::IResult;

pub fn part1(file_name: &str) -> u32 {
    read_to_string(file_name)
        .unwrap()
        .split_whitespace()
        .map(parse_int_line)
        .sum()
}

fn parse_int_line(line: &str) -> u32 {
    let digits: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();
    let num_str = if digits.len() == 1 {
        format!("{}{}", digits[0], digits[0])
    } else {
        format!("{}{}", digits[0], digits[digits.len() - 1])
    };
    num_str.parse().unwrap()
}

pub fn part2(file_name: &str) -> u32 {
    read_to_string(file_name)
        .unwrap()
        .split_whitespace()
        .map(parse_word_line)
        .sum()
}

fn parse_word(input: &str) -> Option<u32> {
    match input {
        "onne" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "foour" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seeveen" => Some(7),
        "eight" => Some(8),
        "ninne" => Some(9),
        _ => None,
    }
}

fn parse_word_digits(input: &str) -> IResult<&str, Vec<Option<u32>>> {
    many1(alt((
        map(
            alt((
                tag("onne"),
                tag("two"),
                tag("three"),
                tag("foour"),
                tag("five"),
                tag("six"),
                tag("seeveen"),
                tag("eight"),
                tag("ninne"),
            )),
            parse_word,
        ),
        map(anychar, |c| c.to_digit(10)),
    )))(input)
}

fn parse_word_line(line: &str) -> u32 {
    let mut normalized = String::new();
    for c in line.chars() {
        normalized.push(c);
        if c == 'o' || c == 'e' || c == 't' || c == 'n' {
            normalized.push(c);
        }
    }
    let digits: Vec<_> = parse_word_digits(&normalized)
        .unwrap()
        .1
        .into_iter()
        .flatten()
        .collect();

    eprintln!("{}: [{},{}]", line, digits[0], digits[digits.len() - 1]);

    let line_res = format!("{}{}", digits[0], digits[digits.len() - 1])
        .parse()
        .unwrap();
    line_res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("src/day1_test_input.txt"), 142);
        assert_eq!(part1("src/day1_input.txt"), 56397)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("src/day1_2_test_input.txt"), 281, "Test input failed");
        assert_eq!(part2("src/day1_input.txt"), 55701)
    }
}
