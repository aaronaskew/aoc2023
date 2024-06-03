
use glam::IVec2;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{anychar, multispace0, space1, u8},
    combinator::map_res,
    multi::many1,
    sequence::tuple,
    IResult,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct PlanLine {
    direction: char,
    length: u8,
    color: Color,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn parse_line(input: &str) -> IResult<&str, PlanLine> {
    let (input, direction) = anychar(input)?;
    let (input, _) = space1(input)?;
    let (input, length) = u8(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, color) = hex_color(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = multispace0(input)?;

    Ok((
        input,
        PlanLine {
            direction,
            length,
            color,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<PlanLine>> {
    many1(parse_line)(input)
}

fn print_grid(grid: &[Vec<char>]) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
    println!();
}

fn dig(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut updated_grid = grid.to_vec();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '.' && is_inside(grid, x, y) {
                updated_grid[y][x] = '#';
            }
        }
    }

    updated_grid
}

fn is_inside(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    // check if on edges, as the edges can never be inside
    if x == 0 || x == grid[0].len() - 1 || y == 0 || y == grid.len() - 1 {
        return false;
    }

    // cast ray to left and count number of times it crosses the boundaries
    // odd number = inside, even number = outside
    let mut crossings = 0;
    let mut last_char = '.';
    let mut crossing_vertical = 0;
    for x in (0..=x - 1).rev() {
        let current_char = grid[y][x];

        if current_char == '#' && last_char == '.' {
            crossing_vertical = 0;
        }

        if current_char == '#' {
            if grid[y - 1][x] == '#' {
                crossing_vertical -= 1;
            }
            if grid[y + 1][x] == '#' {
                crossing_vertical += 1;
            }
        }

        if (current_char == '.' && last_char == '#' || current_char == '#' && x == 0)
            && crossing_vertical == 0
        {
            crossings += 1;
        }

        last_char = current_char;
    }

    let is_inside = crossings % 2 == 1;

    let mut marked_grid = grid.to_vec();
    marked_grid[y][x] = 'X';
    // print_grid(&marked_grid);
    // dbg!(&x, &y, &crossings, &is_inside);

    is_inside
}

fn process(input: &str) -> String {
    let (_, plan_lines) = parse(input).unwrap();

    let mut dug_points = vec![IVec2::new(0, 0)];

    let mut curr_point = dug_points[0];

    for plan_line in plan_lines {
        for _ in 0..plan_line.length {
            curr_point += match plan_line.direction {
                'U' => IVec2::new(0, -1),
                'D' => IVec2::new(0, 1),
                'R' => IVec2::new(1, 0),
                'L' => IVec2::new(-1, 0),
                c => panic!("{} isn't a direction", c),
            };
            dug_points.push(curr_point);
        }
    }

    // normalize the dug_points so the leftmost and topmost values are zero
    // dbg!(&dug_points);
    let min_x = dug_points.iter().map(|p| p.x).min().unwrap();

    let min_y = dug_points.iter().map(|p| p.y).min().unwrap();

    let max_x = dug_points.iter().map(|p| p.x).max().unwrap();

    let max_y = dug_points.iter().map(|p| p.y).max().unwrap();

    // dbg!(&min_x, &max_x, &min_y, &max_y);

    let mut grid = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

    for point in dug_points {
        grid[(point.y - min_y) as usize][(point.x - min_x) as usize] = '#';
    }

    print_grid(&grid);

    let grid = dig(&grid);

    print_grid(&grid);

    grid.iter()
        .flat_map(|v| v.iter())
        .filter(|c| **c == '#')
        .count()
        .to_string()
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

        assert_eq!(result, "62");
    }
}
