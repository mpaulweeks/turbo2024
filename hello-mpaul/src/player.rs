use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum PlayerId {
    P1,
    P2,
}

#[derive(Clone)]
pub struct Player {
    pub player_id: PlayerId,
    row_board: u8,
    row_hand: u8,
    pub board: Vec<Card>,
    pub hand: Vec<Card>,
    pub deck: Vec<Card>,
}

pub struct PositionedPlayer {
    phand: Vec<PositionedCard>,
    pboard: Vec<PositionedCard>,
}

pub fn create_player(index: PlayerId, deck: Deck) -> Player {
    return Player {
        row_board: if index == PlayerId::P1 { 2 } else { 1 },
        row_hand: if index == PlayerId::P1 { 3 } else { 0 },
        player_id: index,
        board: Vec::new(),
        hand: Vec::new(),
        deck,
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

pub fn click_action(p: Player) -> Option<Action> {
    let positioned = position_player(p.clone());
    let hovered: Vec<CardId> = positioned
        .phand
        .iter()
        .filter(|pcard| pcard.hover)
        .map(|pcard| pcard.card.card_id)
        .collect();
    if let Some(clicked) = hovered.first() {
        return Some(action_play_from_hand(p.player_id, *clicked));
    } else {
        return None;
    }
}

pub fn render_player(p: Player) {
    let positioned = position_player(p.clone());
    for pcard in positioned.phand.iter() {
        render_card(pcard.clone(), p.player_id == PlayerId::P1);
    }
    for pcard in positioned.pboard.iter() {
        render_card(pcard.clone(), p.player_id == PlayerId::P1);
    }
}
