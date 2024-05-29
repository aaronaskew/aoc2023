use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

#[derive(PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

type Position = (usize, usize);

#[derive(Debug)]
enum Pipe {
    NorthSouth, // | is a vertical pipe connecting north and south.
    EastWest,   // - is a horizontal pipe connecting east and west.
    NorthEast,  // L is a 90-degree bend connecting north and east.
    NorthWest,  // J is a 90-degree bend connecting north and west.
    SouthWest,  // 7 is a 90-degree bend connecting south and west.
    SouthEast,  // F is a 90-degree bend connecting south and east.
    Nothing,
    // . is ground; there is no pipe in this tile.
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
}

impl Pipe {
    fn can_go(&self, dir: Direction) -> bool {
        matches!(
            (dir, self),
            (Direction::North, Pipe::NorthSouth)
                | (Direction::North, Pipe::NorthEast)
                | (Direction::North, Pipe::NorthWest)
                | (Direction::South, Pipe::NorthSouth)
                | (Direction::South, Pipe::SouthWest)
                | (Direction::South, Pipe::SouthEast)
                | (Direction::East, Pipe::EastWest)
                | (Direction::East, Pipe::NorthEast)
                | (Direction::East, Pipe::SouthEast)
                | (Direction::West, Pipe::EastWest)
                | (Direction::West, Pipe::NorthWest)
                | (Direction::West, Pipe::SouthWest)
        )
    }
}

fn process(input: &str) -> String {
    let mut grid = HashMap::<Position, Pipe>::new();
    let size = (input.lines().next().unwrap().len(), input.lines().count());
    println!("size: {size:?}");
    let mut start_pos: Position = (0, 0);

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid.insert(
                (col, row),
                match c {
                    '|' => Pipe::NorthSouth,
                    '-' => Pipe::EastWest,
                    'L' => Pipe::NorthEast,
                    'J' => Pipe::NorthWest,
                    '7' => Pipe::SouthWest,
                    'F' => Pipe::SouthEast,
                    'S' => {
                        start_pos = (col, row);
                        Pipe::Nothing
                    }
                    _ => Pipe::Nothing,
                },
            );
        }
    }

    println!("maze: {:?}", grid);

    // Determine start pipe

    let n = if start_pos.1 != 0 {
        grid.get(&(start_pos.0, &start_pos.1 - 1))
            .unwrap()
            .can_go(Direction::South)
    } else {
        false
    };
    let s = grid
        .get(&(start_pos.0, &start_pos.1 + 1))
        .unwrap()
        .can_go(Direction::North);
    let e = grid
        .get(&(start_pos.0 + 1, start_pos.1))
        .unwrap()
        .can_go(Direction::West);
    let w = if start_pos.0 != 0 {
        grid.get(&(start_pos.0 - 1, start_pos.1))
            .unwrap()
            .can_go(Direction::East)
    } else {
        false
    };

    println!("n: {n}");
    println!("s: {s}");
    println!("e: {e}");
    println!("w: {w}");

    if let Some(start_pipe) = grid.get_mut(&start_pos) {
        *start_pipe = match (n, s, e, w) {
            (true, true, false, false) => Pipe::NorthSouth,
            (true, false, true, false) => Pipe::NorthEast,
            (true, false, false, true) => Pipe::NorthWest,
            (false, true, true, false) => Pipe::SouthEast,
            (false, true, false, true) => Pipe::SouthWest,
            (false, false, true, true) => Pipe::EastWest,
            _ => panic!("can't determine start pipe type"),
        };
    }

    println!("start_pipe: {:?}", grid.get(&start_pos).unwrap());

    // Crawl the maze
    let mut steps = 0;
    let mut curr_pos = start_pos;
    let mut last_pos = start_pos;

    let mut loop_positions = HashSet::<Position>::new();

    loop {
        loop_positions.insert(curr_pos);

        let curr_pipe = grid.get(&curr_pos).unwrap();

        let next_pos = if curr_pipe.can_go(Direction::North)
            && (curr_pos.0, curr_pos.1 - 1) != last_pos
        {
            (curr_pos.0, curr_pos.1 - 1)
        } else if curr_pipe.can_go(Direction::South) && (curr_pos.0, curr_pos.1 + 1) != last_pos {
            (curr_pos.0, curr_pos.1 + 1)
        } else if curr_pipe.can_go(Direction::East) && (curr_pos.0 + 1, curr_pos.1) != last_pos {
            (curr_pos.0 + 1, curr_pos.1)
        } else if curr_pipe.can_go(Direction::West) && (curr_pos.0 - 1, curr_pos.1) != last_pos {
            (curr_pos.0 - 1, curr_pos.1)
        } else {
            panic!("can't find valid next pipe")
        };

        last_pos = curr_pos;
        curr_pos = next_pos;
        steps += 1;

        if curr_pos == start_pos {
            break;
        }
    }

    println!("steps: {steps} half: {}", steps / 2);

    for row in 0..size.1 {
        for col in 0..size.0 {
            if loop_positions.contains(&(col, row)) {
                print!(
                    "{}",
                    match grid.get(&(col, row)).unwrap() {
                        Pipe::NorthSouth => "|",
                        Pipe::EastWest => "-",
                        Pipe::NorthEast => "L",
                        Pipe::NorthWest => "J",
                        Pipe::SouthWest => "7",
                        Pipe::SouthEast => "F",
                        Pipe::Nothing => "#",
                    }
                );
            } else {
                print!(".")
            }
        }
        println!();
    }

    let mut contained_area = 0;
    let mut is_inside_loop;

    for row in 0..size.1 {
        for col in 0..size.0 {
            if loop_positions.contains(&(col, row)) {
                continue;
            } else {
                is_inside_loop = false;
                let mut bends: Vec<Direction> = Vec::new();

                // count the boundary crossings to the left
                if col == 0 {
                    continue;
                } else {
                    let i = col - 1;
                    for x in (0..=i).rev() {
                        // println!("pos: {:?}", (x, row));
                        let pipe = grid.get(&(x, row)).unwrap();
                        if loop_positions.contains(&(x, row)) {
                            match *pipe {
                                Pipe::NorthSouth => {
                                    is_inside_loop = !is_inside_loop;
                                }
                                Pipe::NorthEast | Pipe::NorthWest => {
                                    if bends.is_empty() {
                                        bends.push(Direction::North);
                                    } else if *bends.last().unwrap() == Direction::North {
                                        bends.pop();
                                    } else if *bends.last().unwrap() == Direction::South {
                                        bends.pop();
                                        is_inside_loop = !is_inside_loop;
                                    }
                                }
                                Pipe::SouthEast | Pipe::SouthWest => {
                                    if bends.is_empty() {
                                        bends.push(Direction::South);
                                    } else if *bends.last().unwrap() == Direction::South {
                                        bends.pop();
                                    } else if *bends.last().unwrap() == Direction::North {
                                        bends.pop();
                                        is_inside_loop = !is_inside_loop;
                                    }
                                }
                                Pipe::EastWest => {}
                                _ => panic!("invalid pipe found"),
                            }
                        }
                    }
                    if is_inside_loop {
                        contained_area += 1;
                        println!("+1 (col, row) {:?}", (col, row));
                    }
                }
            }
        }
    }

    println!("contained_area: {contained_area}");

    format!("{}", contained_area)
    // todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );

        assert_eq!(result, "4");
    }

    #[test]
    fn example2() {
        let result = process(".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...");

        assert_eq!(result, "8");
    }

    #[test]
    fn example3() {
        let result = process("FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L");

        assert_eq!(result, "10");
    }
}
