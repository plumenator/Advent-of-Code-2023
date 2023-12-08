/*
--- Day 8: Haunted Wasteland ---
You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about ghosts a few minutes ago.

One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of network of labeled nodes.

It seems like you're meant to use the left/right instructions to navigate the network. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!

After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where you are now, and you have to follow the left/right instructions until you reach ZZZ.

This format defines each node of the network individually. For example:

RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
Starting with AAA, you need to look up the next element based on the next left/right instruction in your input. In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC. Then, L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.

Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes 6 steps to reach ZZZ:

LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)

Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?
*/

use std::collections::HashMap;
use std::fs::read_to_string;

pub fn part1(file_name: &str) -> usize {
    let (instructions, first, graph) = parse_graph(read_to_string(file_name).unwrap().as_str());
    let mut count = 0;
    let mut current = first;
    for i in instructions.chars().cycle() {
        if current == "ZZZ" {
            break;
        }
        let (left, right) = &graph[&current];
        println!("{count}: {left} {right}");
        if i == 'L' {
            current = left.clone();
        } else {
            current = right.clone();
        }
        count += 1;
    }
    count
}

fn parse_graph(input: &str) -> (String, String, HashMap<String, (String, String)>) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let vertices: Vec<_> = lines
        .skip(1)
        .map(|line| {
            (
                line[0..3].to_owned(),
                (line[7..10].into(), line[12..15].into()),
            )
        })
        .collect();
    let first = vertices[0].0.clone();
    let graph = HashMap::from_iter(vertices.into_iter());
    (instructions.into(), first, graph)
}

#[cfg(test)]
mod test {
    #[test]
    fn part1_example() {
        assert_eq!(super::part1("src/day8_test_input.txt"), 2)
    }
    #[test]
    fn part1_example2() {
        assert_eq!(super::part1("src/day8_test_input2.txt"), 6)
    }
    #[test]
    fn part1_actual() {
        assert_eq!(super::part1("src/day8_input.txt"), 250602641)
    }
}
