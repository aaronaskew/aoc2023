use std::ops::Deref;

use nom::{
    character::complete::space1,
    character::complete::{anychar, newline, u32},
    combinator::opt,
    multi::{many1, many_till},
    IResult,
};

use itertools::Itertools;

fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let card1 = Card::Four;
    let card2 = Card::Three;

    dbg!(card2 > card1);

    let (_, mut hands) = parse_hands(input).unwrap();

    dbg!(&hands);

    use HandValue::*;

    for hand in &mut hands {
        hand.value = {
            let hand_counts = hand.cards.iter().counts();

            dbg!(&hand_counts);
            let joker_count = hand_counts.get(&Card::Joker).unwrap_or(&0);
           

            match (hand_counts.values().sorted().join("").deref(), joker_count) {
                ("11111", 0) => Some(HighCard),
                ("11111", 1) => Some(OnePair),

                ("1112", 0) => Some(OnePair),
                ("1112", 1 | 2) => Some(ThreeOfAKind),

                ("122", 0) => Some(TwoPair),
                ("122", 1) => Some(FullHouse),
                ("122", 2) => Some(FourOfAKind),

                ("113", 0) => Some(ThreeOfAKind),
                ("113", 1 | 3) => Some(FourOfAKind),

                ("23", 0) => Some(FullHouse),
                ("23", 2 | 3) => Some(FiveOfAKind),

                ("14", 0) => Some(FourOfAKind),
                ("14", 1 | 4) => Some(FiveOfAKind),

                ("5", _) => Some(FiveOfAKind),
                value => panic!("expected hand value string, got ({}, {})", value.0, value.1),
            }
        }
    }

    dbg!(&hands);

    hands
        .iter()
        .sorted_by_key(|hand| {
            (
                hand.value.as_ref().unwrap(),
                hand.cards[0],
                hand.cards[1],
                hand.cards[2],
                hand.cards[3],
                hand.cards[4],
            )
        })
        .inspect(|hand| {
            println!("{:?} {:?}", hand.value.as_ref().unwrap(), hand.cards);
        })
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as u32 + 1) * hand.bid)
        .to_string()

    // todo!();
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = many1(parse_hand)(input)?;
    Ok((input, hands))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, hand) = many_till(anychar, space1)(input)?;
    let (input, bid) = u32(input)?;
    let (input, _) = opt(newline)(input)?;

    let cards = hand
        .0
        .iter()
        .map(|c| {
            use Card::*;

            match c {
                'J' => Joker,
                '2' => Two,
                '3' => Three,
                '4' => Four,
                '5' => Five,
                '6' => Six,
                '7' => Seven,
                '8' => Eight,
                '9' => Nine,
                'T' => Ten,
                'Q' => Queen,
                'K' => King,
                'A' => Ace,
                value => panic!("expected card char, got {value}"),
            }
        })
        .collect();

    Ok((
        input,
        Hand {
            cards,
            bid,
            value: None,
        },
    ))
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    value: Option<HandValue>,
}

#[derive(PartialEq, PartialOrd, Debug, Eq, Hash, Ord, Copy, Clone)]
enum Card {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(PartialEq, PartialOrd, Debug, Ord, Eq, Hash)]
enum HandValue {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part2(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );

        assert_eq!(result, "5905");
    }
}
