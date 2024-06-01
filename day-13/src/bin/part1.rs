use itertools::Itertools;

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

struct Note {
    grid: Vec<Vec<char>>,
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

        dbg!(&horizontal_candidates);

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

        dbg!(&vertical_candidates);

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

    fn rows_are_equal(&self, y1: usize, y2: usize) -> bool {
        self.grid[y1] == self.grid[y2]
    }

    fn cols_are_equal(&self, x1: usize, x2: usize) -> bool {
        for row in &self.grid {
            if row[x1] != row[x2] {
                return false;
            }
        }
        true
    }
    
}

fn process(input: &str) -> String {
    input
        .split("\n\n")
        .map(|s| {
            let note = Note::new(s);
            note.mirror_score()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
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

        assert_eq!(result, "405");
    }
}
