use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Player {
    pub index: u8,
    pub row_hand: u8,
    pub row_board: u8,
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
        row_hand: if index == 0 { 3 } else { 0 },
        row_board: if index == 0 { 2 } else { 1 },
        hand: hand,
        deck: deck,
    };
}

pub fn get_positioned_cards(p: Player) -> Vec<PositionedCard> {
    let mut out: Vec<PositionedCard> = Vec::new();
    for (i, c) in p.hand.iter().enumerate() {
        out.push(position_card(c.clone(), p.row_hand, i));
    }
    return out;
}

pub fn render_player(p: Player) {
    let positioned = get_positioned_cards(p.clone());
    for pcard in positioned.iter() {
        render_card(pcard.clone(), p.index == 0);
    }
}
