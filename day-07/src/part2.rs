use crate::utils::read_file;
use std::{cmp::Ordering, collections::HashMap};

#[cfg(test)]
mod tests {
    use crate::part2::{count_cards, Hand};
    use std::collections::HashMap;

    #[test]
    fn test_card_counts_1() {
        let cards: Vec<u8> = vec![2, 1, 2, 2, 3];
        let mut expected: HashMap<u8, usize> = HashMap::new();
        expected.insert(2, 3);
        expected.insert(1, 1);
        expected.insert(3, 1);
        assert_eq!(count_cards(&cards), expected);
    }

    #[test]
    fn test_hand_frequencies() {
        let hand = Hand {
            cards: vec![2, 1, 2, 2, 3],
            bid: 5,
        };
        assert_eq!(hand.get_cards_frequencies(), [4, 1]);
        let hand = Hand {
            cards: vec![1, 1, 1, 1, 1],
            bid: 5,
        };
        assert_eq!(hand.get_cards_frequencies(), [5]);
    }

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
            cards: vec![3, 2, 10, 3, 12],
            bid: 5,
        };
        let hand3 = Hand {
            cards: vec![10, 5, 5, 1, 5],
            bid: 3,
        };
        let hand2 = Hand {
            cards: vec![12, 12, 6, 7, 7],
            bid: 3,
        };
        let hand5 = Hand {
            cards: vec![12, 10, 1, 1, 10],
            bid: 3,
        };
        let hand4 = Hand {
            cards: vec![11, 11, 11, 1, 13],
            bid: 3,
        };
        assert!(hand1 < hand2);
        assert!(hand2 < hand3);
        assert!(hand3 < hand4);
        assert!(hand4 < hand5);
    }

    #[test]
    fn test_hands_comparison_3() {
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
        // Count cards
        let mut card_counter = count_cards(&self.cards);
        // Replace jokers to get the best hand
        let n_jokers = match card_counter.get_mut(&1) {
            Some(x) => *x,
            None => 0,
        };
        if n_jokers > 0 && n_jokers < 5 {
            let most_freq_card = get_most_frequent_card(&card_counter);
            if let Some(x) = card_counter.get_mut(&most_freq_card) {
                *x += n_jokers;
            }
            card_counter.remove(&1);
        };
        // Sort out amount of cards of same value
        let mut card_counts: Vec<usize> = card_counter.values().map(|x| *x).collect();
        card_counts.sort();
        card_counts.reverse();
        card_counts
    }
}

fn count_cards(cards: &Vec<u8>) -> HashMap<u8, usize> {
    let mut card_counts = HashMap::new();
    for card in cards.iter() {
        match card_counts.get_mut(card) {
            Some(count) => *count += 1,
            None => {
                card_counts.insert(*card, 1);
            }
        }
    }
    card_counts
}

fn get_most_frequent_card(card_counter: &HashMap<u8, usize>) -> u8 {
    let mut max_freq = 0;
    let mut most_freq_card: u8 = 0;
    for (card, freq) in card_counter.iter() {
        if *card != 1 && *freq > max_freq {
            most_freq_card = *card;
            max_freq = *freq;
        }
    }
    most_freq_card
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
    match card.is_numeric() {
        true => card.to_digit(13).unwrap() as u8,
        false => match card {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'T' => 10,
            'J' => 1,
            _ => panic!("card '{}' not recognized", card),
        },
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

pub fn solve_part2(fname: &String) -> u32 {
    let content = read_file(fname);
    let mut hands: Vec<Hand> = content.lines().map(|line| parse_line(&line)).collect();
    hands.sort();
    let mut result = 0;
    for (i, hand) in hands.iter().enumerate() {
        result += (i + 1) as u32 * hand.bid
    }
    result
}
