use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

type Position = (usize, usize);

#[derive(Debug)]
struct Universe {
    grid: Vec<Vec<char>>,
    size: (usize, usize),
    galaxies: Vec<(usize, usize)>,
    sum_of_distances: usize,
}

impl Universe {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<char>> = Vec::new();

        for line in input.lines() {
            let mut new_row: Vec<char> = Vec::new();
            for c in line.chars() {
                new_row.push(c);
            }
            grid.push(new_row.clone());

            // If row is empty space, insert it again
            if !new_row.contains(&'#') {
                grid.push(new_row);
            }
        }

        let mut empty_column_indicies = Vec::<usize>::new();
        for i in 0..grid[0].len() {
            let mut curr_column = Vec::<char>::new();
            #[allow(clippy::needless_range_loop)]
            for j in 0..grid.len() {
                curr_column.push(grid[j][i]);
            }
            if !curr_column.contains(&'#') {
                empty_column_indicies.push(i);
            }
        }

        println!("empty cols: {:#?}", empty_column_indicies);

        for col in empty_column_indicies.iter().rev() {
            for row in grid.iter_mut() {
                row.insert(*col, '.');
            }
        }

        let size = (grid[0].len(), grid.len());
        println!("size: {:?}", size);

        let mut galaxies = Vec::<(usize, usize)>::new();
        for j in 0..grid.len() {
            for i in 0..grid[0].len() {
                if grid[j][i] == '#' {
                    galaxies.push((i, j));
                }
            }
        }

        println!("galaxies: {:?}", galaxies);

        let mut pairs: HashSet<Vec<(usize, usize)>> = HashSet::new();

        for (i, galaxy_a) in galaxies.iter().enumerate() {
            for (j, galaxy_b) in galaxies.iter().enumerate() {
                if galaxy_a != galaxy_b {
                    let mut pair = vec![*galaxy_a, *galaxy_b];
                    pair.sort();
                    pairs.insert(pair);
                }
            }
        }

        println!("pairs len: {}", pairs.len());
        println!("{:#?}", pairs);

        let sum_of_distances = pairs
            .iter()
            .map(|pair| {
                assert!(pair.len() == 2);
                let a = pair[0];
                let b = pair[1];

                a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
            })
            .sum();

        Self {
            grid,
            size,
            galaxies,
            sum_of_distances,
        }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        for row in self.grid.clone() {
            text.extend(row.iter());
            text.extend(['\n'].iter());
        }

        f.write_str(&text)
    }
}

fn process(input: &str) -> String {
    let mut universe = Universe::new(input);

    println!("{}", universe);

    universe.sum_of_distances.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );

        assert_eq!(result, "374");
    }
}
