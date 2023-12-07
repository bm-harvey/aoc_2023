use crate::solution::Solution;
pub struct Day7;

impl Solution for Day7 {
    fn solution(raw_data: &str) -> (u32, u32) {
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
        //hand_bids.reverse();

        //for hb in hand_bids.iter() {
        //dbg!(&hb);
        //}

        let sol_1 = hand_bids
            .iter()
            .map(|(_, r)| r)
            .enumerate()
            .map(|(idx, &rank)| (idx as u32 + 1) * rank)
            .sum::<u32>();

        (sol_1, 0)
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
            for idx in 0..5 {
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
        let value_self = self.value();
        let value_other = other.value();
        if value_self == value_other {
            for idx in 0..5 {
                let c1_value = self.cards[idx].value();
                let c2_value = other.cards[idx].value();
                if c1_value != c2_value {
                    return c1_value.cmp(&c2_value);
                }
            }

            return std::cmp::Ordering::Equal;
        }
        self.value().cmp(&other.value())
    }
}
