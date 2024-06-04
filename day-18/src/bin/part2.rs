use glam::{I64Vec2, IVec2};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until1, take_while1, take_while_m_n},
    character::complete::{anychar, multispace0, space1, u8},
    combinator::map_res,
    multi::many1,
    sequence::tuple,
    Err, IResult,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct PlanLine {
    direction: I64Vec2,
    length: i64,
}

fn from_hex(input: &str) -> Result<i64, std::num::ParseIntError> {
    i64::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_length(input: &str) -> IResult<&str, i64> {
    let (input, _) = tag("#")(input)?;

    map_res(take_while_m_n(5, 5, is_hex_digit), from_hex)(input)
}

fn hex_direction(input: &str) -> IResult<&str, I64Vec2> {
    let (input, direction) = take_while1(is_hex_digit)(input)?;
    Ok((
        input,
        match direction {
            "0" => I64Vec2::X,
            "1" => I64Vec2::Y,
            "2" => I64Vec2::NEG_X,
            "3" => I64Vec2::NEG_Y,
            _ => panic!("not a valid direction"),
        },
    ))
}

fn parse_line(input: &str) -> IResult<&str, PlanLine> {
    let (input, _) = take_until1("(")(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, length) = hex_length(input)?;
    let (input, direction) = hex_direction(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, PlanLine { direction, length }))
}

fn parse(input: &str) -> IResult<&str, Vec<PlanLine>> {
    many1(parse_line)(input)
}

fn process(input: &str) -> String {
    let (_, plan_lines) = parse(input).unwrap();

    // dbg!(&plan_lines);

    let vertices: Vec<I64Vec2> = plan_lines
        .iter()
        .scan(I64Vec2::ZERO, |state, p| {
            *state += p.direction * p.length;

            Some(*state)
        })
        .collect();

    dbg!(&vertices);

    let area = (vertices
        .iter()
        .tuple_windows()
        .fold(0, |acc, (a, b)| acc + (a.x * b.y - a.y * b.x))
        / 2)
    .abs();

    let perimeter = (vertices.iter().tuple_windows().fold(0, |acc, (a, b)| {
        let distance = (*a - *b).abs();

        acc + distance.x + distance.y
    }) + {
        let a = vertices.first().unwrap();
        let b = vertices.last().unwrap();
        let distance = (*a - *b).abs();

        dbg!(distance.x + distance.y)
    }) / 2;

    dbg!(&area, &perimeter, area + perimeter + 1);

    (area + perimeter + 1).to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        );

        assert_eq!(result, "952408144115");
    }
}
