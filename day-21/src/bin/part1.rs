use std::{
    collections::{HashSet, VecDeque},
    iter::Inspect,
};

use glam::IVec2;
use itertools::Position;

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input, 64);
    dbg!(output);
}

enum Direction {
    N,
    S,
    E,
    W,
}

fn can_go(grid: &[Vec<char>], position: IVec2, direction: Direction) -> bool {
    let new_position = match direction {
        Direction::N => position + IVec2::NEG_Y,
        Direction::S => position + IVec2::Y,
        Direction::E => position + IVec2::X,
        Direction::W => position + IVec2::NEG_X,
    };

    if new_position.x < 0 || new_position.x >= grid[0].len() as i32 {
        return false;
    }

    if new_position.y < 0 || new_position.y >= grid.len() as i32 {
        return false;
    }

    if grid[new_position.y as usize][new_position.x as usize] == '#' {
        return false;
    }

    true
}

fn move_one(grid: &[Vec<char>], position: IVec2) -> Vec<IVec2> {
    let mut new_positions = vec![];

    if can_go(grid, position, Direction::N) {
        new_positions.push(position + IVec2::NEG_Y);
    }

    if can_go(grid, position, Direction::S) {
        new_positions.push(position + IVec2::Y);
    }

    if can_go(grid, position, Direction::E) {
        new_positions.push(position + IVec2::X);
    }

    if can_go(grid, position, Direction::W) {
        new_positions.push(position + IVec2::NEG_X);
    }

    new_positions
}

fn process(input: &str, steps: u8) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // get start pos
    let mut start_pos = IVec2::ZERO;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                start_pos = IVec2::new(x as i32, y as i32);
            }
        }
    }

    dbg!(&start_pos);

    let mut possible_positions = HashSet::new();

    possible_positions.insert(start_pos);

    for count in 1..=steps {
        let mut next_possible_positions = HashSet::new();

        possible_positions.iter().for_each(|position| {
            move_one(&grid, *position).iter().for_each(|new_pos| {
                next_possible_positions.insert(*new_pos);
            });
        });

        possible_positions = next_possible_positions;

        dbg!(&count, &possible_positions, possible_positions.len());
    }

    possible_positions.len().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
            6,
        );

        assert_eq!(result, "16");
    }
}
