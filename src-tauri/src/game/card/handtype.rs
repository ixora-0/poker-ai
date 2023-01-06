use std::cmp::Ordering;

use super::Hand;

type Ranking = u8;

#[derive(Clone, Copy, Eq)]
pub enum HandType {
    Flush,
    Straight,
    OnePair,
    HighCard,
}
impl HandType {
    /// All hand types in order
    pub const ALL_HAND_TYPES: [HandType; 4] = [
        HandType::Flush,
        HandType::Straight,
        HandType::OnePair,
        HandType::HighCard,
    ];
    pub fn check_hand(self, hand: Hand) -> bool {
        fn is_flush(hand: Hand) -> bool {
            // check if hand is a flush
            // cards' suits are the same with the first one
            hand.into_iter().all(|card| card.suit == hand[0].suit)
        }
        fn is_straight(hand: Hand) -> bool {
            // check if hand is straight
            if hand[4].rank == 1 {
                // check for 12345 and TJQKA straight
                let a = hand.get_ranks_array();
                if a == [2, 3, 4, 5, 1] || a == [10, 11, 12, 13, 1] {
                    return true;
                }
                return false;
            }
            // check if the ranks aren't consecutive
            for i in 1..5 {
                if hand[i].rank != hand[i - 1].rank + 1 {
                    return false;
                }
            }
            // hand is straight
            true
        }
        fn is_one_pair(hand: Hand) -> bool {
            // checks if hand has one pair
            // since hand is sorted, only compare to the next card
            for i in 0..4 {
                if hand[i].rank == hand[i + 1].rank {
                    return true;
                }
            }
            // no pairs
            false
        }
        fn is_high_card(hand: Hand) -> bool {
            // hand is high card if there's no flush, no straight and no pair
            !(hand.is_hand_type(HandType::Flush)
                || hand.is_hand_type(HandType::Straight)
                || hand.is_hand_type(HandType::OnePair))
        }
        match self {
            HandType::Flush => is_flush(hand),
            HandType::Straight => is_straight(hand),
            HandType::OnePair => is_one_pair(hand),
            HandType::HighCard => is_high_card(hand),
        }
    }
    pub fn get_ranking(self) -> Ranking {
        match self {
            HandType::Flush => 5,
            HandType::Straight => 6,
            HandType::OnePair => 9,
            HandType::HighCard => 10,
        }
    }
}
impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        self.get_ranking() == other.get_ranking()
    }
}
impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        // this hand type is greater if it's ranking is smaller and vice versa
        // TODO: break ties when ranking is equal
        self.get_ranking().cmp(&other.get_ranking()).reverse()
    }
}
