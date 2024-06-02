use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Beam {
    position: Position,
    direction: Direction,
}

impl Beam {
    fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self {
            position: Position::new(x, y),
            direction,
        }
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }
    }

    fn out_of_bounds(&self, grid: &Grid) -> bool {
        self.position.x < 0
            || self.position.x >= grid.grid[0].len() as i32
            || self.position.y < 0
            || self.position.y >= grid.grid.len() as i32
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self { grid }
    }

    fn energize(&self, initial_beam: Beam) -> i32 {
        let mut beam_queue = VecDeque::new();
        let mut seen: HashSet<Beam> = HashSet::new();
        let mut energized: HashSet<Position> = HashSet::new();

        beam_queue.push_back(initial_beam.clone());

        // dbg!(&beam_queue);
        // dbg!(&seen);

        loop {
            if beam_queue.is_empty() {
                break;
            }

            let mut beam = beam_queue.pop_front().unwrap();

            // dbg!(&beam);

            if beam.out_of_bounds(self) || seen.contains(&beam) {
                continue;
            }

            seen.insert(beam.clone());
            energized.insert(beam.position.clone());

            // dbg!(&energized.len());

            match (
                self.grid[beam.position.y as usize][beam.position.x as usize],
                beam.direction.clone(),
            ) {
                ('/', Direction::Up) => beam.direction = Direction::Right,
                ('/', Direction::Down) => beam.direction = Direction::Left,
                ('/', Direction::Left) => beam.direction = Direction::Down,
                ('/', Direction::Right) => beam.direction = Direction::Up,

                ('\\', Direction::Up) => beam.direction = Direction::Left,
                ('\\', Direction::Down) => beam.direction = Direction::Right,
                ('\\', Direction::Left) => beam.direction = Direction::Up,
                ('\\', Direction::Right) => beam.direction = Direction::Down,

                ('|', Direction::Left | Direction::Right) => {
                    beam.direction = Direction::Up;
                    let mut split_beam = beam.clone();
                    split_beam.direction = Direction::Down;
                    beam_queue.push_back(split_beam);
                }

                ('-', Direction::Up | Direction::Down) => {
                    beam.direction = Direction::Left;
                    let mut split_beam = beam.clone();
                    split_beam.direction = Direction::Right;
                    beam_queue.push_back(split_beam);
                }

                _ => {}
            }

            beam.move_forward();

            beam_queue.push_back(beam);
        }

        energized.len() as i32
    }

    fn max_energize(&self) -> i32 {
        let mut max_energy = 0;

        for x in 0..self.grid[0].len() {
            let initial_beam = Beam::new(x as i32, 0, Direction::Down);
            let energy = self.energize(initial_beam.clone());
            dbg!(&initial_beam, &energy);
            max_energy = max_energy.max(energy);
        }

        for x in 0..self.grid[0].len() {
            let initial_beam = Beam::new(x as i32, self.grid.len() as i32 - 1, Direction::Up);
            let energy = self.energize(initial_beam.clone());
            dbg!(&initial_beam, &energy);
            max_energy = max_energy.max(energy);
        }

        for y in 0..self.grid.len() {
            let initial_beam = Beam::new(0, y as i32, Direction::Right);
            let energy = self.energize(initial_beam.clone());
            dbg!(&initial_beam, &energy);
            max_energy = max_energy.max(energy);
        }

        for y in 0..self.grid.len() {
            let initial_beam = Beam::new(self.grid[0].len() as i32 - 1, y as i32, Direction::Left);
            let energy = self.energize(initial_beam.clone());
            dbg!(&initial_beam, &energy);
            max_energy = max_energy.max(energy);
        }

        max_energy
    }
}

fn process(input: &str) -> String {
    let  grid = Grid::new(input.lines().map(|line| line.chars().collect()).collect());

    // dbg!(&grid);

    grid.max_energize().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );

        assert_eq!(result, "51");
    }
}
