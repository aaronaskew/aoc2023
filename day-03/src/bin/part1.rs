fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let cols = input.lines().next().unwrap().len() as i32;
    let rows = input.lines().count() as i32;

    dbg!(cols, rows);

    let mut digits = String::new();
    let mut coords = Vec::<Coord>::new();
    let mut numbers = Vec::<Number>::new();

    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            if c.is_ascii_digit() {
                digits.push(c);
                coords.push(Coord {
                    col: col as i32,
                    row: row as i32,
                });
            } else if !digits.is_empty() {
                let value: u32 = digits.parse().unwrap();
                numbers.push(Number {
                    value,
                    coords: coords.clone(),
                });
                digits = String::new();
                coords = Vec::new();
            }
        })
    });

    dbg!(&numbers);

    numbers.iter().for_each(|number| {
        let mut same_row = true;
        let current_row = number.coords[0].row;

        number.coords.iter().for_each(|coord| {
            if coord.row != current_row {
                same_row = false;
            }
        });

        assert!(same_row);
    });

    let test_coord_offsets: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    numbers
        .iter()
        .filter(|number| {
            let mut symbol_found = false;

            number.coords.iter().for_each(|coord| {
                test_coord_offsets.iter().for_each(|offset| {
                    let test_coord = Coord {
                        col: coord.col + offset.0,
                        row: coord.row + offset.1,
                    };

                    if test_coord.col >= 0
                        && test_coord.col < cols
                        && test_coord.row >= 0
                        && test_coord.row < rows
                        && is_symbol(char_at(
                            input,
                            test_coord.col as usize,
                            test_coord.row as usize,
                        ))
                    {
                        symbol_found = true;
                    }
                })
            });

            symbol_found
        })
        .fold(0, |acc, number| acc + number.value)
        .to_string()

    // todo!();
}

#[derive(Debug)]
struct Number {
    value: u32,
    coords: Vec<Coord>,
}

#[derive(Debug, Clone)]
struct Coord {
    col: i32,
    row: i32,
}

fn is_symbol(c: char) -> bool {
    !matches!(c, '0'..='9' | '.')
}

fn char_at(input: &str, col: usize, row: usize) -> char {
    input.lines().nth(row).unwrap().chars().nth(col).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );

        assert_eq!(result, "4361");
    }
}
