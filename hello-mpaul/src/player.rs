use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Player {
    pub index: u8,
    pub hand: Vec<Card>,
    pub deck: Vec<Card>,
}

pub fn create_player(index: u8) -> Player {
    return Player {
        index: index,
        hand: Vec::new(),
        deck: create_deck(),
    };
}
