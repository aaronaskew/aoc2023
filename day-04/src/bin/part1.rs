use nom::character::complete::{space0, space1, u32};
use nom::{
    bytes::complete::{tag, take_until},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut cards = Vec::<Card>::new();

    dbg!(input.lines().count());
    input.lines().for_each(|line| {
        dbg!(line);
    });

    input.lines().for_each(|line| {
        cards.push(parse_card(line).unwrap().1);
    });

    dbg!(cards.len());

    // cards.iter().for_each(|card| {
    //     assert!(card.win_nums.len()==5);
    //     assert!(card.our_nums.len()==8);
    // });

    cards.iter_mut().for_each(|card| {
        let mut number_of_winning_nums = 0;
        card.our_nums.iter().for_each(|num| {
            if card.win_nums.contains(num) {
                number_of_winning_nums += 1;
            }
        });

        if number_of_winning_nums > 0 {
            card.prize = 2_u32.pow(number_of_winning_nums - 1);
        }
    });

    dbg!(&cards);

    cards.iter().fold(0,|acc,card| {
        acc + card.prize
    }).to_string()
    
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let mut card = Card {
        win_nums: Vec::new(),
        our_nums: Vec::new(),
        prize: 0,
    };

    let (input, _) = take_until(":")(input)?;
    let (input, _) = tag(": ")(input)?;
    dbg!(input);

    let (input, num_lists) = separated_pair(parse_number_list, tag("|"), parse_number_list)(input)?;

    dbg!(&num_lists);

    dbg!(num_lists.0.len(), num_lists.1.len());

    card.win_nums = num_lists.0;
    card.our_nums = num_lists.1;

    Ok((input, card))
}

fn parse_number_list(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = space0(input)?;
    let (input, num_list) = separated_list0(space1, u32)(input)?;
    let (input, _) = space0(input)?;
    Ok((input, num_list))
}

#[derive(Debug)]
struct Card {
    win_nums: Vec<u32>,
    our_nums: Vec<u32>,
    prize: u32,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 |  74 77 10 23 35 67 36 11",
        );

        assert_eq!(result, "13");
    }
}
