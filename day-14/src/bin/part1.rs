use std::{collections::HashSet, time::SystemTime};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

struct Telescope {
    grid: Vec<Vec<char>>,
    _round_positions: HashSet<(usize, usize)>,
}

impl std::fmt::Debug for Telescope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("\n");

        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                s.push(self.grid[y][x]);
                if x == self.grid[0].len() - 1 {
                    s.push('\n');
                }
            }
        }

        write!(f, "{s}")
    }
}

impl Telescope {
    fn _new(grid: Vec<Vec<char>>) -> Self {
        Self {
            grid: grid.clone(),
            _round_positions: grid
                .iter()
                .enumerate()
                .flat_map(|(y, v)| {
                    v.iter()
                        .enumerate()
                        .filter(|(_, c)| **c == 'O')
                        .map(move |(x, _)| (x, y))
                })
                .collect(),
        }
    }

    fn tilt_north(&mut self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                if self.grid[y][x] == 'O' {
                    // dbg!(x, y, self.farthest_clear_pos_north(x, y));
                    let (newx, newy) = self.farthest_clear_pos_north(x, y);
                    if (newx, newy) != (x, y) {
                        self.grid[newy][newx] = 'O';
                        self.grid[y][x] = '.';
                    }
                }
            }
        }
    }

    

    fn load_north(&self) -> usize {
        let loads: Vec<usize> = (1..=self.grid.len()).rev().collect();

        //dbg!(&loads);

        self.grid
            .iter()
            .enumerate()
            .map(|(y, v)| {
                v.iter()
                    .map(|c| match c {
                        'O' => loads[y],
                        _ => 0,
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }

    fn farthest_clear_pos_north(&self, x: usize, y: usize) -> (usize, usize) {
        let mut farthest_y = y;

        loop {
            if farthest_y == 0 || matches!(self.grid[farthest_y - 1][x], '#' | 'O') {
                break;
            } else {
                farthest_y -= 1;
            }
        }

        (x, farthest_y)
    }
}

fn process(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut ts = Telescope {
        grid,
        _round_positions: HashSet::new(),
    };

    dbg!(&ts);

    let timer = SystemTime::now();

    for _ in 0..1000 {
        ts.tilt_north();
    }

    dbg!(timer.elapsed().unwrap());

    //dbg!(&ts);

    ts.load_north().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );

        assert_eq!(result, "136");
    }
}
