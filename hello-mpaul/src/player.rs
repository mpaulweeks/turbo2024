use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Player {
    pub index: u8,
    pub hand: Vec<Card>,
    pub deck: Vec<Card>,
}

pub fn create_player(index: u8) -> Player {
    let mut deck = create_deck();
    let mut hand: Vec<Card> = Vec::new();
    for n in 0..5 {
        let card = deck.pop();
        match card {
            // The division was valid
            Some(c) => hand.push(c),
            // The division was invalid
            None => self::println!("Empty deck!"),
        }
    }
    return Player {
        index: index,
        hand: hand,
        deck: deck,
    };
}
