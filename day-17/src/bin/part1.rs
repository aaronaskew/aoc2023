use std::collections::{BinaryHeap, HashMap};

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

/// The state we'll track in our priority queue. We need to track the node above
/// and the cost to get there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    node: Node,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We are using a min heap, so we are doing this backwards
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbors(node: &Node, grid: &[Vec<usize>]) -> Vec<Node> {
    let mut neighbors = Vec::new();

    for (d, p) in node.position.valid_next(grid) {
        if d != node.direction {
            match (d, node.direction) {
                (Direction::North, Direction::South)
                | (Direction::South, Direction::North)
                | (Direction::East, Direction::West)
                | (Direction::West, Direction::East) => {}
                _ => neighbors.push(Node::new(p, d, 1)),
            }
        } else if node.direction_count < 3 {
            neighbors.push(Node::new(p, d, node.direction_count + 1));
        }
    }

    neighbors
}

fn dijkstra<F>(
    grid: &[Vec<usize>],
    start: &Position,
    goal: &Position,
    neighbor_fn: F,
) -> Option<usize>
where
    F: Fn(&Node, &[Vec<usize>]) -> Vec<Node>,
{
    // Track our min distances at each node. In our specific case,
    // we have multiple because we could be coming from South or East at
    // the start.
    let mut distances = HashMap::new();
    distances.insert(Node::new(*start, Direction::East, 0), 0);
    distances.insert(Node::new(*start, Direction::South, 0), 0);

    // Track paths we want to visit. We have two again.
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        node: Node::new(*start, Direction::East, 0),
        cost: 0,
    });
    frontier.push(State {
        node: Node::new(*start, Direction::South, 0),
        cost: 0,
    });

    // Grab the next node from the frontier
    while let Some(State { node, cost }) = frontier.pop() {
        // If we are at the goal, we are done
        if node.position == *goal {
            return Some(cost);
        }

        // Otherwise, check our neighbors
        for neighbor in neighbor_fn(&node, grid) {
            // If we've already visited this node and it was cheaper,
            // we don't need to keep checking this way.
            let new_cost = cost + grid[neighbor.position.y][neighbor.position.x];
            if let Some(&best) = distances.get(&neighbor) {
                if new_cost >= best {
                    continue;
                }
            }

            // Otherwise, add it to our distances and frontier.
            distances.insert(neighbor, new_cost);
            frontier.push(State {
                node: neighbor,
                cost: new_cost,
            })
        }
    }

    None
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

    let start = Position::new(0, 0);
    let goal = Position::new(grid[0].len() - 1, grid.len() - 1);
    let now = std::time::Instant::now();

    let min_distance = dijkstra(&grid, &start, &goal, neighbors).unwrap();
    dbg!(&min_distance, now.elapsed());

    min_distance.to_string()
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

        assert_eq!(result, "102");
    }
}
