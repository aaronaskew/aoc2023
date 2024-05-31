use std::collections::HashSet;

use itertools::{repeat_n, Itertools};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Puzzle<'a> {
    data: &'a str,
    damage_groups: Vec<usize>,
}

impl<'a> Puzzle<'a> {
    fn generate_combinations(&self) -> HashSet<String> {
        let num_unknowns = self.data.chars().filter(|c| c == &'?').count();
        let total_damaged: usize = self.damage_groups.iter().sum();
        let known_damaged = self.data.chars().filter(|c| c == &'#').count();

        println!("data: {}", self.data);
        println!("damage_groups: {:?}", self.damage_groups);
        println!(
            "total damaged: {} unknowns: {} known damaged: {}",
            total_damaged, num_unknowns, known_damaged
        );

        let remaining_damaged = total_damaged - known_damaged;

        // let mut pool: Vec<char> = Vec::new();

        // for i in 0..num_unknowns {
        //     if i < remaining_damaged {
        //         pool.push('#');
        //     } else {
        //         pool.push('.');
        //     }
        // }

        // println!("pool {:?}", pool);

        let pool_permutations = repeat_n(["#", "."].into_iter(), num_unknowns)
            .multi_cartesian_product()
            .filter(|v| v.iter().filter(|s| **s == "#").count() == remaining_damaged)
            .map(|v| v.join(""))
            // .filter(|v| self.is_valid_combination(v.clone()))
            .collect::<Vec<String>>();
        // dbg!(pool_permutations.len());
        // dbg!(&pool_permutations);

        // let permutation_set: HashSet<Vec<&char>> = HashSet::new();

        // for permutation in pool_permutations {
        //     permutation_set.insert(permutation);
        // }

        // dbg!(permutation_set.len());

        let mut combinations: HashSet<String> = HashSet::new();

        for perm in pool_permutations {
            let mut text = String::new();
            let mut current_permutation = perm.clone();
            for c in self.data.chars() {
                if c == '?' {
                    text.push(current_permutation.pop().unwrap())
                } else {
                    text.push(c);
                }
            }
            if self.is_valid_combination(text.clone()) {
                combinations.insert(text);
            }
        }

        println!("combinations.len: {:#?}", combinations.len());

        combinations
    }

    // fn valid_combination_count(&self, combinations: HashSet<String>) -> usize {
    //     combinations
    //         .iter()
    //         .filter(|combination| {
    //             let mut combination_damage_groups: Vec<usize> = Vec::new();

    //             let mut curr_damage_group_length = 0_usize;

    //             for c in combination.chars() {
    //                 match c {
    //                     '#' => curr_damage_group_length += 1,
    //                     _ => {
    //                         if curr_damage_group_length > 0 {
    //                             combination_damage_groups.push(curr_damage_group_length);
    //                             curr_damage_group_length = 0;
    //                         }
    //                     }
    //                 }
    //             }

    //             if curr_damage_group_length > 0 {
    //                 combination_damage_groups.push(curr_damage_group_length);
    //             }

    //             // println!("self.damage_groups: {:?}", self.damage_groups);
    //             // println!("comb damage_groups: {:?}", combination_damage_groups);

    //             combination_damage_groups == self.damage_groups
    //         })
    //         .count()
    // }

    fn is_valid_combination(&self, combination: String) -> bool {
        let mut combination_damage_groups: Vec<usize> = Vec::new();

        let mut curr_damage_group_length = 0_usize;

        for c in combination.chars() {
            match c {
                '#' => curr_damage_group_length += 1,
                _ => {
                    if curr_damage_group_length > 0 {
                        combination_damage_groups.push(curr_damage_group_length);
                        curr_damage_group_length = 0;
                    }
                }
            }
        }

        if curr_damage_group_length > 0 {
            combination_damage_groups.push(curr_damage_group_length);
        }

        // dbg!(&combination);
        // dbg!(&self.damage_groups);
        // dbg!(&combination_damage_groups);

        combination_damage_groups == self.damage_groups
    }
}

fn process(input: &str) -> String {
    let total_combinations: usize = input
        .lines()
        .map(|line| {
            let data = line.split_ascii_whitespace().next().unwrap();
            let groups = line.split_ascii_whitespace().last().unwrap();
            let group_vec: Vec<usize> = groups
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            let puzzle = Puzzle {
                data,
                damage_groups: group_vec,
            };

            let combinations = puzzle.generate_combinations();
            // let count = puzzle.valid_combination_count(combinations);

            // println!("count: {count}");

            // count

            combinations.len()
        })
        .sum();

    total_combinations.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            ".???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );

        assert_eq!(result, "21");
    }
}
