/*
--- Day 10: Pipe Maze ---
You use the hang glider to ride the hot air from Desert Island all the way up to the floating metal island. This island is surprisingly cold and there definitely aren't any thermals to glide on, so you leave your hang glider behind.

You wander around for a while, but you don't find any people or animals. However, you do occasionally find signposts labeled "Hot Springs" pointing in a seemingly consistent direction; maybe you can find someone at the hot springs and ask them where the desert-machine parts are made.

The landscape here is alien; even the flowers and trees are made of metal. As you stop to admire some metal grass, you notice something metallic scurry away in your peripheral vision and jump into a big pipe! It didn't look like any animal you've ever seen; if you want a better look, you'll need to get ahead of it.

Scanning the area, you discover that the entire field you're standing on is densely packed with pipes; it was hard to tell at first because they're the same metallic silver color as the "ground". You make a quick sketch of all of the surface pipes you can see (your puzzle input).

The pipes are arranged in a two-dimensional grid of tiles:

| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
Based on the acoustics of the animal's scurrying, you're confident the pipe that contains the animal is one large, continuous loop.

For example, here is a square loop of pipe:

.....
.F-7.
.|.|.
.L-J.
.....
If the animal had entered this loop in the northwest corner, the sketch would instead look like this:

.....
.S-7.
.|.|.
.L-J.
.....
In the above diagram, the S tile is still a 90-degree F bend: you can tell because of how the adjacent pipes connect to it.

Unfortunately, there are also many pipes that aren't connected to the loop! This sketch shows the same loop as above:

-L|F7
7S-7|
L|7||
-L-J|
L|-JF
In the above diagram, you can still figure out which pipes form the main loop: they're the ones connected to S, pipes those pipes connect to, pipes those pipes connect to, and so on. Every pipe in the main loop connects to its two neighbors (including S, which will have exactly two pipes connecting to it, and which is assumed to connect back to those two pipes).

Here is a sketch that contains a slightly more complex main loop:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...
Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:

7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
If you want to get out ahead of the animal, you should find the tile in the loop that is farthest from the starting position. Because the animal is in the pipe, it doesn't make sense to measure this by direct distance. Instead, you need to find the tile that would take the longest number of steps along the loop to reach from the starting point - regardless of which way around the loop the animal went.

In the first example with the square loop:

.....
.S-7.
.|.|.
.L-J.
.....
You can count the distance each tile in the loop is from the starting point like this:

.....
.012.
.1.3.
.234.
.....
In this example, the farthest point from the start is 4 steps away.

Here's the more complex loop again:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...
Here are the distances for each tile on that loop:

..45.
.236.
01.78
14567
23...
Find the single giant loop starting at S. How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?

*/

use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    NoPipe,
}

struct Cell {
    x: isize,
    y: isize,
    pipe: Pipe,
}

pub fn part1(file_name: &str) -> u64 {
    let grid = parse_grid(file_name);
    let (sx, sy) = find_start(&grid);
    let sx = sx as isize;
    let sy = sy as isize;
    [(sx + 1, sy), (sx, sy + 1), (sx - 1, sy), (sx, sy - 1)]
        .iter()
        .filter_map(|&(x, y)| {
            if x < 0
                || y < 0
                || x >= grid[0].len() as isize
                || y >= grid.len() as isize
                || !connect(
                    &Cell {
                        x,
                        y,
                        pipe: grid[y as usize][x as usize],
                    },
                    sx,
                    sy,
                )
            {
                None
            } else {
                walk(x, y, &grid, &mut HashSet::new())
            }
        })
        .max()
        .unwrap()
        / 2
}

fn to_pipe(c: char) -> Pipe {
    use Pipe::*;
    match c {
        'S' => Start,
        '|' => Vertical,
        '-' => Horizontal,
        'L' => NorthEast,
        'J' => NorthWest,
        '7' => SouthWest,
        'F' => SouthEast,
        _ => NoPipe,
    }
}

fn walk(
    x: isize,
    y: isize,
    grid: &Vec<Vec<Pipe>>,
    visited: &mut HashSet<(isize, isize)>,
) -> Option<u64> {
    if visited.contains(&(x, y)) {
        return None;
    }
    visited.insert((x, y));
    if x < 0 || x >= grid[0].len() as isize || y < 0 || y >= grid.len() as isize {
        visited.remove(&(x, y));
        return None;
    }
    println!("{x}, {y}, {:?}", grid[y as usize][x as usize]);
    use Pipe::*;
    let ret = match grid[y as usize][x as usize] {
        Start => Some(1),
        Vertical => choose(walk(x, y + 1, grid, visited), walk(x, y - 1, grid, visited)),
        Horizontal => choose(walk(x + 1, y, grid, visited), walk(x - 1, y, grid, visited)),
        NorthEast => choose(walk(x, y - 1, grid, visited), walk(x + 1, y, grid, visited)),
        NorthWest => choose(walk(x, y - 1, grid, visited), walk(x - 1, y, grid, visited)),
        SouthWest => choose(walk(x, y + 1, grid, visited), walk(x - 1, y, grid, visited)),
        SouthEast => choose(walk(x, y + 1, grid, visited), walk(x + 1, y, grid, visited)),
        NoPipe => None,
    }
    .map(|e| {
        println!("{x}, {y}, e: {e}");
        e + 1
    });
    visited.remove(&(x, y));
    ret
}

fn connect(&Cell { x, y, pipe }: &Cell, sx: isize, sy: isize) -> bool {
    use Pipe::*;
    match pipe {
        Start => panic!("illegal pipe"),
        Vertical => y - sy == 1 || sy - y == 1,
        Horizontal => x - sx == 1 || sx - x == 1,
        NorthEast => sx - x == 1 || y - sy == 1,
        NorthWest => x - sx == 1 || y - sy == 1,
        SouthWest => x - sx == 1 || sy - y == 1,
        SouthEast => sx - x == 1 || sy - y == 1,
        NoPipe => false,
    }
}

fn choose(a: Option<u64>, b: Option<u64>) -> Option<u64> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn find_start(grid: &Vec<Vec<Pipe>>) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if &Pipe::Start == tile {
                return (x, y);
            }
        }
    }
    panic!("No start tile found");
}

fn parse_grid(file_name: &str) -> Vec<Vec<Pipe>> {
    read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| line.chars().map(to_pipe).collect())
        .collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn part1_example1() {
        assert_eq!(super::part1("src/day10_test_input.txt"), 4)
    }
    #[test]
    fn part1_example2() {
        assert_eq!(super::part1("src/day10_test_input2.txt"), 4)
    }
    #[test]
    fn part1_example3() {
        assert_eq!(super::part1("src/day10_test_input3.txt"), 8)
    }
    #[test]
    fn part1_example4() {
        assert_eq!(super::part1("src/day10_test_input4.txt"), 8)
    }
    #[test]
    fn part1_actual() {
        assert_eq!(super::part1("src/day10_input.txt"), 1861775706)
    }
}
