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
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let card1 = Card::Four;
    let card2 = Card::Three;

    dbg!(card2 > card1);

    let (_, mut hands) = parse_hands(input).unwrap();

    dbg!(&hands);

    use HandValue::*;

    for hand in &mut hands {
        hand.value = match hand
            .cards
            .iter()
            .counts()
            .values()
            .sorted()
            .join("")
            .deref()
        {
            "11111" => Some(HighCard),
            "1112" => Some(OnePair),
            "122" => Some(TwoPair),
            "113" => Some(ThreeOfAKind),
            "23" => Some(FullHouse),
            "14" => Some(FourOfAKind),
            "5" => Some(FiveOfAKind),
            value => panic!("expected hand value string, got {value}"),
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
        }).enumerate().fold(0, |acc, (i, hand)| {
            acc + (i as u32 + 1) * hand.bid

        }).to_string()

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
                '2' => Two,
                '3' => Three,
                '4' => Four,
                '5' => Five,
                '6' => Six,
                '7' => Seven,
                '8' => Eight,
                '9' => Nine,
                'T' => Ten,
                'J' => Jack,
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
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
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
        let result = part1(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );

        assert_eq!(result, "6440");
    }
}
