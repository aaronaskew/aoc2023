// Used https://www.youtube.com/watch?v=GJq_Hza8nSk

use std::collections::HashMap;

use itertools::repeat_n;

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl Record {
    fn new(springs: Vec<Spring>, groups: Vec<usize>) -> Self {
        Self { springs, groups }
    }

    fn parse_all(input: &str) -> Vec<Record> {
        input
            .lines()
            .map(|line| {
                let springs_str = line.split_ascii_whitespace().next().unwrap();
                let groups_str = line.split_ascii_whitespace().last().unwrap();

                let springs: Vec<Spring> = springs_str
                    .chars()
                    .map(|c| match c {
                        '.' => Spring::Operational,
                        '#' => Spring::Damaged,
                        _ => Spring::Unknown,
                    })
                    .collect();

                let unfolded_springs = [
                    &springs[..],
                    &springs[..],
                    &springs[..],
                    &springs[..],
                    &springs[..],
                ]
                .join(&Spring::Unknown);

                // dbg!(&unfolded_springs);

                let groups: Vec<usize> = groups_str
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect();

                let unfolded_groups = repeat_n(groups, 5).flatten().collect();

                Record::new(unfolded_springs, unfolded_groups)
            })
            .collect()
    }
}

fn possible_solutions(memo: &mut HashMap<Record, usize>, record: &Record) -> usize {
    // Check to see if we have already calculated this record
    if let Some(&v) = memo.get(record) {
        // *memo_hits().lock().unwrap() += 1;
        return v;
    }

    // If we have no groups left, we have a solution IF there are no other
    // damaged springs in our list. Otherwise, it can't be valid.
    if record.groups.is_empty() {
        let v = match record.springs.iter().any(|s| *s == Spring::Damaged) {
            true => 0,
            false => 1,
        };
        memo.insert(record.clone(), v);
        return v;
    }

    // At this point, we have some number of groups left, so make sure we have enough springs
    // left to fill them.
    if record.springs.len() < record.groups.iter().sum::<usize>() + record.groups.len() - 1 {
        memo.insert(record.clone(), 0);
        return 0;
    }

    // We can't do anything with operational springs, so just skip them.
    if record.springs[0] == Spring::Operational {
        let solutions = possible_solutions(
            memo,
            &Record::new(record.springs[1..].to_vec(), record.groups.clone()),
        );
        memo.insert(record.clone(), solutions);
        return solutions;
    }

    // At this point, we know we are at the beginning of a possible position for the current group. We
    // can check if that's possible, and if it is, we can see how many valid solutions we'd get if we did.
    let mut solutions = 0;
    let cur = record.groups[0];
    let all_non_operational = record.springs[0..cur]
        .iter()
        .all(|c| *c != Spring::Operational);
    let end = (cur + 1).min(record.springs.len());
    if all_non_operational
        && ((record.springs.len() > cur && record.springs[cur] != Spring::Damaged)
            || record.springs.len() <= cur)
    {
        solutions = possible_solutions(
            memo,
            &Record::new(record.springs[end..].to_vec(), record.groups[1..].to_vec()),
        );
    }

    // If our current position is Unknown, we could also choose not to use it, so include those possibilities.
    if record.springs[0] == Spring::Unknown {
        solutions += possible_solutions(
            memo,
            &Record::new(record.springs[1..].to_vec(), record.groups.clone()),
        );
    }

    memo.insert(record.clone(), solutions);
    solutions
}

fn process(input: &str) -> String {
    let records = Record::parse_all(input);
    dbg!(&records);

    let mut memo = HashMap::new();
    let solutions = records
        .iter()
        .map(|record| possible_solutions(&mut memo, record))
        .sum::<usize>();

    dbg!(&solutions);
    solutions.to_string()
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

        assert_eq!(result, "525152");
    }
}
