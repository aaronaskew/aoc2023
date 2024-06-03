use pathfinding::prelude::dijkstra;
use std::char::from_digit;

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn valid_next(&self, grid: &[Vec<usize>]) -> Vec<(Direction, Position)> {
        let mut next = Vec::new();

        if self.x > 0 {
            next.push((Direction::West, Position::new(self.x - 1, self.y)));
        }
        if self.x < grid[0].len() - 1 {
            next.push((Direction::East, Position::new(self.x + 1, self.y)));
        }
        if self.y > 0 {
            next.push((Direction::North, Position::new(self.x, self.y - 1)));
        }
        if self.y < grid.len() - 1 {
            next.push((Direction::South, Position::new(self.x, self.y + 1)));
        }

        next
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    position: Position,
    direction: Direction,
    direction_count: usize,
}

impl Node {
    fn new(position: Position, direction: Direction, direction_count: usize) -> Self {
        Self {
            position,
            direction,
            direction_count,
        }
    }
}

fn process(input: &str) -> String {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let goal_position = Position::new(grid[0].len() - 1, grid.len() - 1);

    let neighbors = |node: &Node| -> Vec<(Node, usize)> {
        let mut neighbors_and_costs = Vec::new();

        for (d, p) in node.position.valid_next(&grid) {
            let neighbor = if d != node.direction {
                if node.direction_count >= 4 {
                    match (d, node.direction) {
                        (Direction::North, Direction::South)
                        | (Direction::South, Direction::North)
                        | (Direction::East, Direction::West)
                        | (Direction::West, Direction::East) => None,
                        _ => Some(Node::new(p, d, 1)),
                    }
                } else {
                    None
                }
            } else if node.direction_count < 10 {
                Some(Node::new(p, d, node.direction_count + 1))
            } else {
                None
            };

            if let Some(neighbor) = neighbor {
                neighbors_and_costs.push((neighbor, grid[neighbor.position.y][neighbor.position.x]))
            }
        }

        // dbg!(&neighbors_and_costs);

        neighbors_and_costs
    };

    let success =
        |node: &Node| -> bool { node.position == goal_position && node.direction_count >= 4 };

    let start1 = Node::new(Position::new(0, 0), Direction::East, 0);
    let start2 = Node::new(Position::new(0, 0), Direction::South, 0);

    let mut results = Vec::new();

    if let Some((path1, min_heat_loss1)) = dijkstra(&start1, neighbors, success) {
        println!("path1");
        draw_path(&grid, path1);
        dbg!(&min_heat_loss1);
        results.push(min_heat_loss1);
        // return min_heat_loss1.to_string();
    }

    if let Some((path2, min_heat_loss2)) = dijkstra(&start2, neighbors, success) {
        println!("path2");
        draw_path(&grid, path2);
        dbg!(&min_heat_loss2);
        results.push(min_heat_loss2);
        // return min_heat_loss1.to_string();
    }

    results[0].min(results[1]).to_string()
}

fn draw_path(grid: &[Vec<usize>], path: Vec<Node>) {
    let mut new_grid: Vec<Vec<char>> = grid
        .iter()
        .map(|v| {
            v.iter()
                .map(|cost| from_digit((*cost).try_into().unwrap(), 10).unwrap())
                .collect()
        })
        .collect();

    for node in path {
        let x = node.position.x;
        let y = node.position.y;

        new_grid[y][x] = match node.direction {
            Direction::North => '^',
            Direction::South => 'V',
            Direction::East => '>',
            Direction::West => '<',
        }
    }

    for y in 0..new_grid.len() {
        for x in 0..new_grid[0].len() {
            print!("{}", new_grid[y][x]);
        }
        println!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        );

        assert_eq!(result, "94");
    }
    #[test]
    fn example2() {
        let result = process(
            "111111111111
999999999991
999999999991
999999999991
999999999991",
        );

        assert_eq!(result, "71");
    }
}
