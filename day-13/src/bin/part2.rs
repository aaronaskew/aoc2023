use std::fmt;

use itertools::Itertools;

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

struct Note {
    grid: Vec<Vec<char>>,
}

impl fmt::Debug for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl Note {
    fn new(input: &str) -> Self {
        Self {
            grid: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn mirror_score(&self) -> usize {
        // check horizontal
        let horizontal_candidates = (0..self.grid.len())
            .tuple_windows()
            .filter(|(y1, y2)| self.rows_are_equal(*y1, *y2))
            .collect_vec();

        let ymax = self.grid.len() - 1;

        // dbg!(&horizontal_candidates);

        for (starty1, starty2) in horizontal_candidates {
            let (mut y1, mut y2) = (starty1, starty2);

            loop {
                if y1 == 0 || y2 == ymax {
                    return 100 * (starty1 + 1);
                } else {
                    y1 -= 1;
                    y2 += 1;
                    if !self.rows_are_equal(y1, y2) {
                        break;
                    }
                }
            }
        }

        // check vertical
        let vertical_candidates = (0..self.grid[0].len())
            .tuple_windows()
            .filter(|(x1, x2)| self.cols_are_equal(*x1, *x2))
            .collect_vec();

        let xmax = self.grid[0].len() - 1;

        // dbg!(&vertical_candidates);

        for (startx1, startx2) in vertical_candidates {
            let (mut x1, mut x2) = (startx1, startx2);

            loop {
                if x1 == 0 || x2 == xmax {
                    return startx1 + 1;
                } else {
                    x1 -= 1;
                    x2 += 1;
                    if !self.cols_are_equal(x1, x2) {
                        break;
                    }
                }
            }
        }

        0
    }

    fn smudged_mirror_score(&self) -> usize {
        // check horizontal

        let horizontal_candidates = (0..self.grid.len())
            .tuple_windows()
            .filter(|(y1, y2)| {
                let delta = self.rows_diff(*y1, *y2);
                matches!(delta, 0 | 1)
            })
            .collect_vec();

        let ymax = self.grid.len() - 1;

        // dbg!(&horizontal_candidates);

        for (starty1, starty2) in horizontal_candidates {
            let (mut y1, mut y2) = (starty1, starty2);
            let mut horiz_delta = self.rows_diff(y1, y2);

            loop {
                if y1 == 0 || y2 == ymax {
                    if horiz_delta == 1 {
                        return 100 * (starty1 + 1);
                    } else {
                        break;
                    }
                } else {
                    y1 -= 1;
                    y2 += 1;
                    horiz_delta += self.rows_diff(y1, y2);
                }
            }
        }

        // check vertical
        let vertical_candidates = (0..self.grid[0].len())
            .tuple_windows()
            .filter(|(x1, x2)| {
                let delta = self.cols_diff(*x1, *x2);
                matches!(delta, 0 | 1)
            })
            .collect_vec();

        let xmax = self.grid[0].len() - 1;

        // dbg!(&vertical_candidates);

        for (startx1, startx2) in vertical_candidates {
            let (mut x1, mut x2) = (startx1, startx2);
            let mut vert_delta = self.cols_diff(x1, x2);

            loop {
                if x1 == 0 || x2 == xmax {
                    if vert_delta == 1 {
                        return startx1 + 1;
                    } else {
                        break;
                    }
                } else {
                    x1 -= 1;
                    x2 += 1;
                    vert_delta += self.cols_diff(x1, x2);
                }
            }
        }

        0
    }

    fn rows_are_equal(&self, y1: usize, y2: usize) -> bool {
        self.grid[y1] == self.grid[y2]
    }

    fn rows_diff(&self, y1: usize, y2: usize) -> usize {
        let mut delta = 0;
        for x in 0..self.grid[0].len() {
            if self.grid[y1][x] != self.grid[y2][x] {
                delta += 1;
            }
        }

        delta
    }

    fn cols_are_equal(&self, x1: usize, x2: usize) -> bool {
        for row in &self.grid {
            if row[x1] != row[x2] {
                return false;
            }
        }
        true
    }

    fn cols_diff(&self, x1: usize, x2: usize) -> usize {
        let mut delta = 0;
        for row in &self.grid {
            if row[x1] != row[x2] {
                delta += 1;
            }
        }
        delta
    }

    // fn flip(&self, x: usize, y: usize) -> Note {
    //     let mut flipped_grid = self.grid.clone();
    //     let ch = flipped_grid[y][x];
    //     match ch {
    //         '.' => flipped_grid[y][x] = '#',
    //         _ => flipped_grid[y][x] = '.',
    //     }
    //     Note { grid: flipped_grid }
    // }

    // fn check_flipped_grids(&self) -> usize {
    //     let original_score = self.mirror_score();

    //     for flip_y in 0..self.grid.len() {
    //         for flip_x in 0..self.grid[0].len() {
    //             let note = self.flip(flip_x, flip_y);
    //             let score = note.mirror_score();
    //             dbg!(&self, original_score, flip_x, flip_y, note, score);
    //             if score > 0 && score != original_score {
    //                 return score;
    //             }
    //         }
    //     }

    //     panic!("score should not be zero");

    //     0
    // }
}

fn process(input: &str) -> String {
    input
        .split("\n\n")
        .map(|s| {
            let note = Note::new(s);
            let smudged_score = note.smudged_mirror_score();
            dbg!(&note.mirror_score());
            dbg!(&smudged_score);
            smudged_score
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example1() {
        let result = process(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );

        assert_eq!(result, "400");
    }

    #[test]
    fn example2() {
        let result = process(
            "#...#..##.#..##..
..###.#.##..#..##
.....#...#####.##
.....#...#####.##
..###.#.##..#..##
#...#..##.#..##..
.##.#.###.###..##
.###.#.#..#.#.#.#
##...#.#.....#.#.
.#...#..##.##..#.
######.#..#..##.#
#######.##.....#.
####.##.##.....#.",
        );

        assert_ne!(result, "0");
    }
}
