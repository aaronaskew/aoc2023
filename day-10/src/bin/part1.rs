use std::{collections::HashMap, thread::current};

use nom::{
    branch::alt,
    character::complete::{char, multispace0},
    multi::many1,
    sequence::terminated,
    IResult, Parser,
};
use nom_locate::{position, LocatedSpan};

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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    StartingPosition,
    Ground,
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    pipe_type: PipeType,
    position: Pos,
    distance_from_start: Option<u32>,
}

type Span<'a> = LocatedSpan<&'a str>;

use Direction::*;
use PipeType::*;

fn parse_cell(input: Span) -> IResult<Span, Cell> {
    let (input, pipe_type) = alt((
        char('|').map(|_| Vertical),
        char('-').map(|_| Horizontal),
        char('L').map(|_| NorthEast),
        char('J').map(|_| NorthWest),
        char('7').map(|_| SouthWest),
        char('F').map(|_| SouthEast),
        char('.').map(|_| Ground),
        char('S').map(|_| StartingPosition),
    ))(input)?;
    let (input, position) = position(input)?;
    Ok((
        input,
        Cell {
            pipe_type,
            position: (position.location_line() as usize, position.get_column() - 1),
            distance_from_start: None,
        },
    ))
}

fn parse_grid(input: Span) -> IResult<Span, HashMap<Pos, Cell>> {
    let (input, cells) = many1(terminated(parse_cell, multispace0))(input)?;

    let cells = cells.iter().map(|cell| (cell.position, *cell)).collect();

    Ok((input, cells))
}

fn can_go(grid: &HashMap<(usize, usize), Cell>, pos: (usize, usize), dir: Direction) -> bool {
    // dbg!(&pos, &dir);

    if let Some(destination_cell) = grid.get(&match dir {
        Direction::North => (pos.0 - 1, pos.1),
        Direction::South => (pos.0 + 1, pos.1),
        Direction::East => (pos.0, pos.1 + 1),
        Direction::West => (pos.0, pos.1 - 1),
    }) {
        // dbg!(destination_cell);
        // dbg!(dir, destination_cell.pipe_type);

        matches!(
            (dir, destination_cell.pipe_type),
            (North, Vertical | SouthEast | SouthWest)
                | (South, Vertical | NorthEast | NorthWest)
                | (East, Horizontal | NorthWest | SouthWest)
                | (West, Horizontal | NorthEast | SouthEast)
        )
    } else {
        false
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

/// Position = (usize, usize) `(x, y)` `(col, row)`
type Pos = (usize, usize);

fn process(input: &str) -> String {
    println!("{}", &input);
    let input = Span::new(input);
    let (_, mut grid) = parse_grid(input).expect("should parse grid");
    //dbg!(&grid);

    let (&start_position:Pos, _) = grid
        .iter()
        .find(|(_, cell)| cell.pipe_type == StartingPosition)
        .unwrap();
    grid.get_mut(&start_position).unwrap().distance_from_start = Some(0);

    // dbg!(&grid);

    let mut current_position = start_position;
    let mut distance = 0_u32;

    loop {
        let previous_position = current_position;

        // dbg!(&current_position);
        // dbg!(can_go(&grid, current_position, North));
        // dbg!(can_go(&grid, current_position, East));
        // dbg!(can_go(&grid, current_position, South));
        // dbg!(can_go(&grid, current_position, West));

        if can_go(&grid, current_position, North)
            && previous_position != (current_position.0 - 1, current_position.1)
        {
            current_position = (current_position.0 - 1, current_position.1);
        } else if can_go(&grid, current_position, East)
            && previous_position != (current_position.0, current_position.1 + 1)
        {
            current_position = (current_position.0, current_position.1 + 1);
        } else if can_go(&grid, current_position, South)
            && previous_position != (current_position.0 + 1, current_position.1)
        {
            current_position = (current_position.0 + 1, current_position.1);
        } else if can_go(&grid, current_position, West)
            && previous_position != (current_position.0, current_position.1 - 1)
        {
            current_position = (current_position.0, current_position.1 - 1);
        }

        dbg!(&current_position);
    }

    todo!("be quite!")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );

        assert_eq!(result, "4");

        let result = process(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );

        assert_eq!(result, "8");
    }
}
