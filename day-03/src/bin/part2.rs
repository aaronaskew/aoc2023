use std::collections::HashSet;

fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
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

    //dbg!(&numbers);

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

    let mut gears = Vec::<Gear>::new();

    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            if matches!(c, '*') {
                gears.push(Gear {
                    coord: Coord {
                        col: col as i32,
                        row: row as i32,
                    },
                    adjacent_nums: HashSet::new(),
                });
            }
        })
    });

    dbg!(&gears);

    gears.iter_mut().for_each(|gear| {
        test_coord_offsets.iter().for_each(|offset| {
            let test_coord = Coord {
                col: gear.coord.col + offset.0,
                row: gear.coord.row + offset.1,
            };

            if test_coord.col >= 0
                && test_coord.col < cols
                && test_coord.row >= 0
                && test_coord.row < rows
                && char_at(input, test_coord.col as usize, test_coord.row as usize).is_ascii_digit()
            {
                numbers.iter().for_each(|number| {
                    if number.coords.contains(&test_coord) {
                        gear.adjacent_nums.insert(number.clone());
                    }
                })
            }
        });
    });

    dbg!(&gears);
    dbg!(&gears.len());
    

    gears
        .iter()
        .filter(|gear| gear.adjacent_nums.len() == 2)
        .fold(0, |acc, gear| {
            acc + gear
                .adjacent_nums
                .iter()
                .fold(1, |acc, num| dbg!(dbg!(acc) * dbg!(num.value)))
        })
        .to_string()

    // todo!();
}

#[derive(Debug)]
struct Gear {
    coord: Coord,
    adjacent_nums: HashSet<Number>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Number {
    value: u32,
    coords: Vec<Coord>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Coord {
    col: i32,
    row: i32,
}

fn char_at(input: &str, col: usize, row: usize) -> char {
    input.lines().nth(row).unwrap().chars().nth(col).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part2(
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

        assert_eq!(result, "467835");
    }
}
