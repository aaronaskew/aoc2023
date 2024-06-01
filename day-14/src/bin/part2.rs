use std::{collections::HashSet, time::SystemTime};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Eq)]
struct State {
    step: usize,
    grid: Vec<Vec<char>>,
}

impl State {
    fn new(step: usize, grid: Vec<Vec<char>>) -> Self {
        Self { step, grid }
    }
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.grid.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
    }
}

struct Telescope {
    grid: Vec<Vec<char>>,
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
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self { grid }
    }

    fn round_positions(&self) -> Vec<(usize, usize)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, v)| {
                v.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == 'O')
                    .map(move |(x, _)| (x, y))
            })
            .collect()
    }

    fn tilt(&mut self, direction: Direction) {
        let mut round_positions = self.round_positions();

        // dbg!(&direction, &round_positions);

        match direction {
            Direction::North => round_positions.sort_by(|(_ax, ay), (_bx, by)| ay.cmp(by)),
            Direction::South => round_positions.sort_by(|(_ax, ay), (_bx, by)| by.cmp(ay)),
            Direction::East => round_positions.sort_by(|(ax, _ay), (bx, _by)| bx.cmp(ax)),
            Direction::West => round_positions.sort_by(|(ax, _ay), (bx, _by)| ax.cmp(bx)),
        }

        // dbg!(&round_positions);

        for (x, y) in round_positions {
            let (newx, newy) = match direction {
                Direction::North => self.farthest_clear_pos_north(x, y),
                Direction::South => self.farthest_clear_pos_south(x, y),
                Direction::East => self.farthest_clear_pos_east(x, y),
                Direction::West => self.farthest_clear_pos_west(x, y),
            };

            if (newx, newy) != (x, y) {
                self.grid[newy][newx] = 'O';
                self.grid[y][x] = '.';
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

    fn farthest_clear_pos_south(&self, x: usize, y: usize) -> (usize, usize) {
        let mut farthest_y = y;

        loop {
            if farthest_y == self.grid.len() - 1
                || matches!(self.grid[farthest_y + 1][x], '#' | 'O')
            {
                break;
            } else {
                farthest_y += 1;
            }
        }

        (x, farthest_y)
    }

    fn farthest_clear_pos_west(&self, x: usize, y: usize) -> (usize, usize) {
        let mut farthest_x = x;

        loop {
            if farthest_x == 0 || matches!(self.grid[y][farthest_x - 1], '#' | 'O') {
                break;
            } else {
                farthest_x -= 1;
            }
        }

        (farthest_x, y)
    }

    fn farthest_clear_pos_east(&self, x: usize, y: usize) -> (usize, usize) {
        let mut farthest_x = x;

        loop {
            if farthest_x == self.grid[0].len() - 1
                || matches!(self.grid[y][farthest_x + 1], '#' | 'O')
            {
                break;
            } else {
                farthest_x += 1;
            }
        }

        (farthest_x, y)
    }
}

fn process(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut ts = Telescope::new(grid);

    dbg!(&ts);

    let timer = SystemTime::now();

    let mut seen: HashSet<State> = HashSet::new();

    for i in 0..1_000_000 {
        seen.insert(State::new(i, ts.grid.clone()));

        ts.tilt(Direction::North);
        ts.tilt(Direction::West);
        ts.tilt(Direction::South);
        ts.tilt(Direction::East);

        // See if we've seen this state before.
        if let Some(state) = seen.get(&State::new(0, ts.grid.clone())) {
            // If we have, we are in a cycle. We can calculate the cycle length and fast-forward to the
            // end.
            let cycle_len = i + 1 - state.step;
            let remaining = 1_000_000_000 - i - 1;
            let remaining = remaining % cycle_len;

            for _ in 0..remaining {
                ts.tilt(Direction::North);
                ts.tilt(Direction::West);
                ts.tilt(Direction::South);
                ts.tilt(Direction::East);
            }

            dbg!(timer.elapsed().unwrap());

            return ts.load_north().to_string();
        }
    }

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

        assert_eq!(result, "64");
    }
}
