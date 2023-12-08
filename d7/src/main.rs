use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd, Reverse};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
enum Card {
    // associate a rank with each card
    J = 0,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

impl Card {
    pub fn from(s: char) -> Card {
        match s {
            '2' => Card::N2,
            '3' => Card::N3,
            '4' => Card::N4,
            '5' => Card::N5,
            '6' => Card::N6,
            '7' => Card::N7,
            '8' => Card::N8,
            '9' => Card::N9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("unknown card"),
        }
    }
}

struct CardBucket {
    count: Vec<(Card, u32)>,
    jokers: u32,
}

impl CardBucket {
    pub fn from(vs: &Vec<Card>) -> Self {
        let mut jokers = 0;
        let mut count = vec![0; 13];
        for card in vs {
            if let Card::J = card {
                jokers += 1;
            } else {
                let k: u8 = (*card).into();
                count[k as usize] += 1;
            }
        }
        let mut count: Vec<(Card, u32)> = count
            .iter()
            .enumerate()
            .map(|(k, count)| (Card::try_from(k as u8).unwrap(), *count))
            .collect();

        count.sort_by_key(|(_, count)| Reverse(*count));

        Self { count, jokers }
    }

    pub fn nth(&self, k: usize) -> (Card, u32) {
        self.count[k]
    }

    pub fn jokers(&self) -> u32 {
        self.jokers
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
enum HandType {
    HighCard = 0,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn from_str(s: &str) -> Self {
        assert_eq!(s.len(), 5);

        let cards = s.chars().map(|c| Card::from(c)).collect();
        Self { cards }
    }

    pub fn compare_card_by_card(&self, other: &Hand) -> Ordering {
        for (lhs, rhs) in std::iter::zip(self.cards.iter(), other.cards.iter()) {
            if lhs > rhs {
                return Ordering::Greater;
            } else if lhs < rhs {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }

    pub fn get_hand_type(&self) -> HandType {
        let bucket = CardBucket::from(&self.cards);

        let ht = match (bucket.nth(0), bucket.nth(1)) {
            ((_, 5), _) => HandType::FiveOfAKind,
            ((_, 4), _) => HandType::FourOfAKind,
            ((_, 3), (_, 2)) => HandType::FullHouse,
            ((_, 3), _) => HandType::ThreeOfAKind,
            ((_, 2), (_, 2)) => HandType::TwoPairs,
            ((_, 2), _) => HandType::OnePair,
            _ => HandType::HighCard,
        };

        match (ht, bucket.jokers()) {
            (HandType::FiveOfAKind, _) => HandType::FiveOfAKind,
            (HandType::FourOfAKind, 0) => HandType::FourOfAKind,
            (HandType::FourOfAKind, _) => HandType::FiveOfAKind,
            (HandType::FullHouse, _) => HandType::FullHouse,
            (HandType::ThreeOfAKind, 0) => HandType::ThreeOfAKind,
            (HandType::ThreeOfAKind, 1) => HandType::FourOfAKind,
            (HandType::ThreeOfAKind, _) => HandType::FiveOfAKind,
            (HandType::TwoPairs, 0) => HandType::TwoPairs,
            (HandType::TwoPairs, _) => HandType::FullHouse,
            (HandType::OnePair, 0) => HandType::OnePair,
            (HandType::OnePair, 1) => HandType::ThreeOfAKind,
            (HandType::OnePair, 2) => HandType::FourOfAKind,
            (HandType::OnePair, _) => HandType::FiveOfAKind,
            (HandType::HighCard, 0) => HandType::HighCard,
            (HandType::HighCard, 1) => HandType::OnePair,
            (HandType::HighCard, 2) => HandType::ThreeOfAKind,
            (HandType::HighCard, 3) => HandType::FourOfAKind,
            (HandType::HighCard, _) => HandType::FiveOfAKind,
        }
    }

    pub fn compare(&self, other: &Hand) -> Ordering {
        // get both hand types
        let sht = self.get_hand_type();
        let oht = other.get_hand_type();

        match sht.cmp(&oht) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => self.compare_card_by_card(other),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        std::iter::zip(self.cards.iter(), other.cards.iter()).all(|(lhs, rhs)| lhs == rhs)
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        self.compare(other)
    }
}

fn main() {
    /*
    let data = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";
    */

    let mut file = std::fs::File::open("input").unwrap();
    let mut data = String::new();
    use std::io::Read;
    file.read_to_string(&mut data).unwrap();

    // -- quick testing
    assert!(Card::N2 < Card::N3);
    assert!(Card::N2 < Card::A);
    assert!(Card::N8 == Card::N8);

    assert!(Hand::from_str("22222") == Hand::from_str("22222"));
    assert!(Hand::from_str("33333") > Hand::from_str("22222"));
    assert!(Hand::from_str("22222") > Hand::from_str("22223"));

    let mut hands = vec![];
    for line in data.split('\n') {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        let mut line = line.split(' ');

        // read hand
        let hand = line.next().unwrap();
        let hand = Hand::from_str(hand);
        println!("{hand:?}");
        let ht = hand.get_hand_type();
        println!("{ht:?}");

        // read hand score
        let score = line.next().unwrap();
        let score = u32::from_str_radix(score, 10).unwrap();

        hands.push((hand, score));
    }

    // rank hands
    hands.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));

    let sum: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, score))| (i as u32 + 1) * score)
        .sum();

    println!("{sum:?}");
}
