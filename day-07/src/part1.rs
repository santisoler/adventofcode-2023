use crate::utils::read_file;
use counter::Counter;
use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    use crate::part1::Hand;

    #[test]
    fn test_hands_comparison_1() {
        let hand1 = Hand {
            cards: vec![2, 1, 2, 2, 3],
            bid: 5,
        };
        let hand2 = Hand {
            cards: vec![2, 2, 2, 2, 3],
            bid: 3,
        };
        assert!(hand2 > hand1);
        assert!(hand1 < hand2);
    }

    #[test]
    fn test_hands_comparison_2() {
        let hand1 = Hand {
            cards: vec![2, 1, 2, 2, 3],
            bid: 5,
        };
        let hand2 = Hand {
            cards: vec![13, 1, 13, 2, 13],
            bid: 3,
        };
        assert!(hand2 > hand1);
        assert!(hand1 < hand2);
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u8>,
    bid: u32,
}

impl Hand {
    fn get_cards_frequencies(&self) -> Vec<usize> {
        let mut card_counts: Vec<usize> = self
            .cards
            .iter()
            .collect::<Counter<_>>()
            .values()
            .map(|x| *x)
            .collect();
        card_counts.sort();
        card_counts.reverse();
        card_counts
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_freqs = self.get_cards_frequencies();
        let other_freqs = other.get_cards_frequencies();
        match self_freqs.cmp(&other_freqs) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.cards.cmp(&other.cards),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn card_to_int(card: &char) -> u8 {
    if card.is_numeric() {
        return card.to_digit(14).unwrap() as u8;
    } else if *card == 'A' {
        return 14;
    } else if *card == 'K' {
        return 13;
    } else if *card == 'Q' {
        return 12;
    } else if *card == 'J' {
        return 11;
    } else if *card == 'T' {
        return 10;
    } else {
        panic!("card not recognized");
    }
}

fn parse_line(line: &str) -> Hand {
    let mut parts = line.split(" ").into_iter();
    let cards = parts
        .next()
        .unwrap()
        .chars()
        .map(|c| card_to_int(&c))
        .collect();
    let bid = parts.next().unwrap().parse().unwrap();
    Hand { cards, bid }
}

pub fn solve_part1(fname: &String) -> u32 {
    let content = read_file(fname);
    let mut hands: Vec<Hand> = content.lines().map(|line| parse_line(&line)).collect();
    hands.sort();
    let mut result = 0;
    for (i, hand) in hands.iter().enumerate() {
        result += (i + 1) as u32 * hand.bid
    }
    result
}
