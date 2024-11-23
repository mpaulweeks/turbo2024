use std::collections::BTreeMap;

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum PlayerId {
    P1,
    P2,
}

#[derive(Clone)]
pub struct PlayerState {
    pub player_id: PlayerId,
    row_board: u8,
    row_hand: u8,
    pub board: Vec<Card>,
    pub hand: Vec<Card>,
    pub deck: Vec<Card>,
}

pub fn create_player(index: PlayerId, deck: Deck) -> PlayerState {
    return PlayerState {
        row_board: if index == PlayerId::P1 { 3 } else { 1 },
        row_hand: if index == PlayerId::P1 { 4 } else { 0 },
        player_id: index,
        board: Vec::new(),
        hand: Vec::new(),
        deck,
    };
}

pub fn position_player(p: PlayerState) -> Vec<PositionedCard> {
    let mut out: Vec<PositionedCard> = Vec::new();
    for (i, c) in p.board.iter().enumerate() {
        out.push(position_card(c.clone(), p.row_board, i, None));
    }
    for (i, c) in p.hand.iter().enumerate() {
        out.push(position_card(
            c.clone(),
            p.row_hand,
            i,
            Some(action_play_from_hand(p.player_id.clone(), c.card_id)),
        ));
    }
    return out;
}

pub fn click_action(p: PlayerState) -> Option<Action> {
    let positioned = position_player(p.clone());
    let hovered: Vec<&PositionedCard> = positioned.iter().filter(|pcard| pcard.hover).collect();
    if let Some(clicked) = hovered.first() {
        return clicked.action.clone();
    } else {
        return None;
    }
}

fn cards_to_map(cards: Vec<PositionedCard>) -> BTreeMap<CardId, PositionedCard> {
    return cards
        .into_iter()
        .map(|x| (x.card.card_id.clone(), x))
        .collect::<BTreeMap<_, _>>();
}

fn tween_player(current: PlayerState, previous: PlayerState, percent: f32) -> Vec<PositionedCard> {
    let current_cards = position_player(current);
    let previous_cards = cards_to_map(position_player(previous));
    return current_cards
        .iter()
        .map(|current_card| {
            if let Some(p_card) = previous_cards.get(&current_card.card.card_id) {
                return tween_card(current_card.clone(), p_card.clone(), percent);
            } else {
                return current_card.clone();
            }
        })
        .collect();
}

pub fn render_player(current: PlayerState, previous: PlayerState, percent: f32) {
    let positioned = tween_player(current.clone(), previous, percent);
    for pcard in positioned.iter() {
        render_card(pcard.clone(), current.player_id == PlayerId::P1);
    }
}
