use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Player {
    index: u8,
    row_board: u8,
    row_hand: u8,
    board: Vec<Card>,
    hand: Vec<Card>,
    deck: Vec<Card>,
}

pub struct PositionedPlayer {
    phand: Vec<PositionedCard>,
    pboard: Vec<PositionedCard>,
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
        row_board: if index == 0 { 2 } else { 1 },
        row_hand: if index == 0 { 3 } else { 0 },
        board: Vec::new(),
        hand: hand,
        deck: deck,
    };
}

pub fn position_player(p: Player) -> PositionedPlayer {
    let mut pboard: Vec<PositionedCard> = Vec::new();
    let mut phand: Vec<PositionedCard> = Vec::new();
    for (i, c) in p.board.iter().enumerate() {
        pboard.push(position_card(c.clone(), p.row_board, i));
    }
    for (i, c) in p.hand.iter().enumerate() {
        phand.push(position_card(c.clone(), p.row_hand, i));
    }
    return PositionedPlayer { pboard, phand };
}

pub fn process_click(p: Player) -> Player {
    let positioned = position_player(p.clone());
    let hovered: Vec<u32> = positioned
        .phand
        .iter()
        .filter(|pcard| pcard.hover)
        .map(|pcard| pcard.card.instance_id)
        .collect();
    let mut new_board = p.board.to_vec();
    let mut new_hand: Vec<Card> = Vec::new();
    for card in p.hand.iter() {
        let is_hover = hovered.contains(&card.instance_id);
        if is_hover {
            new_board.push(card.clone());
        } else {
            new_hand.push(card.clone());
        }
    }
    return Player {
        index: p.index,
        row_hand: p.row_hand,
        row_board: p.row_board,
        board: new_board,
        hand: new_hand,
        deck: p.deck,
    };
}

pub fn render_player(p: Player) {
    let positioned = position_player(p.clone());
    for pcard in positioned.phand.iter() {
        render_card(pcard.clone(), p.index == 0);
    }
    for pcard in positioned.pboard.iter() {
        render_card(pcard.clone(), p.index == 0);
    }
}
