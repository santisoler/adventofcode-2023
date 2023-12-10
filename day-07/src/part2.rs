use crate::utils::read_file;
use counter::Counter;
use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    use crate::part2::Hand;

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
        // Get card counter
        let mut card_counter = self.cards.iter().collect::<Counter<_>>();
        // Replace jokers to get the best hand
        let n_jokers = match card_counter.get_mut(&1_u8) {
            Some(x) => x.to_owned(),
            None => 0,
        };
        if n_jokers > 0 && n_jokers < 5 {
            let most_freq_card = get_most_frequent_card(&card_counter);
            if let Some(x) = card_counter.get_mut(&most_freq_card) {
                *x += n_jokers;
            }
            card_counter.remove(&1_u8).unwrap();
        }
        // Sort out amount of cards of same value
        let mut card_counts: Vec<usize> = card_counter.values().map(|x| *x).collect();
        card_counts.sort();
        card_counts.reverse();
        card_counts
    }
}

fn get_most_frequent_card(card_counter: &Counter<&u8>) -> u8 {
    let mut max_freq = 0;
    let mut most_freq_card: u8 = 0;
    for (card, freq) in card_counter.iter() {
        if *freq > max_freq {
            most_freq_card = **card;
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
