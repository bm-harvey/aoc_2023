use crate::solution::Solution;
pub struct Day7;

impl Solution for Day7 {
    fn solution(raw_data: &str) -> (Box<dyn std::fmt::Display>, Box<dyn std::fmt::Display>) {
        let mut hand_bids = raw_data
            .lines()
            .map(|l| {
                let mut chrs = l.chars();
                let cards = [
                    chrs.next().map(|c| Card::from_char(&c)).unwrap(),
                    chrs.next().map(|c| Card::from_char(&c)).unwrap(),
                    chrs.next().map(|c| Card::from_char(&c)).unwrap(),
                    chrs.next().map(|c| Card::from_char(&c)).unwrap(),
                    chrs.next().map(|c| Card::from_char(&c)).unwrap(),
                ];

                let hand = Hand::new(cards);

                let bid = chrs
                    .filter(|&c| c != ' ')
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();

                (hand, bid)
            })
            .collect::<Vec<_>>();

        hand_bids.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));

        let sol_1 = hand_bids
            .iter()
            .map(|(_, r)| r)
            .enumerate()
            .map(|(idx, &rank)| (idx as u32 + 1) * rank)
            .sum::<u32>();

        let mut hand_bids = raw_data
            .lines()
            .map(|l| {
                let mut chrs = l.chars();
                let cards = [
                    chrs.next().map(|c| JokerCard::from_char(&c)).unwrap(),
                    chrs.next().map(|c| JokerCard::from_char(&c)).unwrap(),
                    chrs.next().map(|c| JokerCard::from_char(&c)).unwrap(),
                    chrs.next().map(|c| JokerCard::from_char(&c)).unwrap(),
                    chrs.next().map(|c| JokerCard::from_char(&c)).unwrap(),
                ];

                let hand = JokerHand::new(cards);

                let bid = chrs
                    .filter(|&c| c != ' ')
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();

                (hand, bid)
            })
            .collect::<Vec<_>>();

        hand_bids.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));

        let sol_2 = hand_bids
            .iter()
            .map(|(_, r)| r)
            .enumerate()
            .map(|(idx, &rank)| (idx as u32 + 1) * rank)
            .sum::<u32>();

        (Box::new(sol_1), Box::new(sol_2))
    }
}

#[derive(PartialOrd, PartialEq, Default, Eq, Ord, Debug)]
struct Card {
    value: u8,
}

impl Card {
    fn from_char(c: &char) -> Self {
        let value = match c {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Card value not valid"),
        };

        Card { value }
    }

    fn value(&self) -> u8 {
        self.value
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn new(cards: [Card; 5]) -> Self {
        Hand { cards }
    }

    fn value(&self) -> u8 {
        let mut values = Vec::<u8>::new();
        let mut mults = Vec::<u8>::new();

        for card in self.cards.iter() {
            let pair = values
                .iter()
                .enumerate()
                .find(|(_, &value)| card.value == value);

            match pair {
                Some((idx, _)) => {
                    mults[idx] += 1;
                }
                None => {
                    values.push(card.value);
                    mults.push(1);
                }
            }
        }

        if Hand::five_of_a_kind(&mults) {
            return 6;
        }
        if Hand::four_of_a_kind(&mults) {
            return 5;
        }
        if Hand::full_house(&mults) {
            return 4;
        }
        if Hand::three_of_a_kind(&mults) {
            return 3;
        }
        if Hand::two_pair(&mults) {
            return 2;
        }
        if Hand::one_pair(&mults) {
            return 1;
        }
        0
    }

    fn five_of_a_kind(mults: &[u8]) -> bool {
        *mults.iter().max().unwrap() == 5
    }
    fn four_of_a_kind(mults: &[u8]) -> bool {
        *mults.iter().max().unwrap() == 4
    }
    fn full_house(mults: &[u8]) -> bool {
        *mults.iter().max().unwrap() == 3 && mults.len() == 2
    }
    fn three_of_a_kind(mults: &[u8]) -> bool {
        *mults.iter().max().unwrap() == 3 && mults.len() == 3
    }
    fn two_pair(mults: &[u8]) -> bool {
        *mults.iter().max().unwrap() == 2 && mults.len() == 3
    }
    fn one_pair(mults: &[u8]) -> bool {
        *mults.iter().max().unwrap() == 2 && mults.len() == 4
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let value_self = self.value();
        let value_other = other.value();
        if value_self == value_other {
            for idx in 0..self.cards.len() {
                let c1_value = self.cards[idx].value();
                let c2_value = other.cards[idx].value();
                if c1_value != c2_value {
                    return Some(c1_value.cmp(&c2_value));
                }
            }
            return Some(std::cmp::Ordering::Equal);
        }
        self.value().partial_cmp(&other.value())
    }
}

impl Eq for Hand {}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(PartialOrd, PartialEq, Default, Eq, Ord, Debug)]
struct JokerCard {
    value: u8,
}

impl JokerCard {
    fn from_char(c: &char) -> Self {
        let value = match c {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 0,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Card value not valid"),
        };

        JokerCard { value }
    }

    fn value(&self) -> u8 {
        self.value
    }
}

#[derive(Debug)]
struct JokerHand {
    cards: [JokerCard; 5],
}

impl JokerHand {
    fn new(cards: [JokerCard; 5]) -> Self {
        JokerHand { cards }
    }

    fn jokers(&self) -> usize {
        self.cards.iter().filter(|c| c.value == 0).count()
    }

    fn value(&self) -> u8 {
        let mut values = Vec::<u8>::new();
        let mut mults = Vec::<u8>::new();

        for card in self.cards.iter().filter(|c| c.value() != 0) {
            let pair = values
                .iter()
                .enumerate()
                .find(|(_, &value)| card.value == value);

            match pair {
                Some((idx, _)) => {
                    mults[idx] += 1;
                }
                None => {
                    values.push(card.value);
                    mults.push(1);
                }
            }
        }

        let max_idx = (0..mults.len()).max_by(|&idx_1, &idx_2| mults[idx_1].cmp(&mults[idx_2]));

        match max_idx {
            None => mults = vec![self.jokers() as u8],
            Some(idx) => mults[idx] += self.jokers() as u8,
        }

        if JokerHand::five_of_a_kind(&mults) {
            return 6;
        }
        if JokerHand::four_of_a_kind(&mults) {
            return 5;
        }
        if JokerHand::full_house(&mults) {
            return 4;
        }
        if JokerHand::three_of_a_kind(&mults) {
            return 3;
        }
        if JokerHand::two_pair(&mults) {
            return 2;
        }
        if JokerHand::one_pair(&mults) {
            return 1;
        }
        0
    }

    fn five_of_a_kind(mults: &[u8]) -> bool {
        *mults.iter().max().unwrap() == 5
    }
    fn four_of_a_kind(mults: &[u8]) -> bool {
        *mults.iter().max().unwrap() == 4
    }
    fn full_house(mults: &[u8]) -> bool {
        *mults.iter().max().unwrap() == 3 && mults.len() == 2
    }
    fn three_of_a_kind(mults: &[u8]) -> bool {
        *mults.iter().max().unwrap() == 3 && mults.len() == 3
    }
    fn two_pair(mults: &[u8]) -> bool {
        *mults.iter().max().unwrap() == 2 && mults.len() == 3
    }
    fn one_pair(mults: &[u8]) -> bool {
        *mults.iter().max().unwrap() == 2 && mults.len() == 4
    }
}

impl PartialEq for JokerHand {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let value_self = self.value();
        let value_other = other.value();
        if value_self == value_other {
            for idx in 0..self.cards.len() {
                let c1_value = self.cards[idx].value();
                let c2_value = other.cards[idx].value();
                if c1_value != c2_value {
                    return Some(c1_value.cmp(&c2_value));
                }
            }
            return Some(std::cmp::Ordering::Equal);
        }
        self.value().partial_cmp(&other.value())
    }
}

impl Eq for JokerHand {}
impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
