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
    pub board: Vec<UnitCard>,
    pub hand: Vec<UnitCard>,
    pub deck: Vec<UnitCard>,
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

pub fn position_player(p: PlayerState, game: GameSnapshot) -> Vec<PositionedUnit> {
    let mut out: Vec<PositionedUnit> = Vec::new();
    for (i, c) in p.board.iter().enumerate() {
        out.push(PositionedUnit {
            unit: c.clone(),
            pcard: position_card(c.card.clone(), p.row_board, i, None),
        });
    }
    for (i, c) in p.hand.iter().enumerate() {
        let can_play = impulse_check(c.clone(), game.impulse.clone());
        let action = if can_play {
            Some(create_action_play_from_hand(
                p.player_id.clone(),
                c.card.card_id,
            ))
        } else {
            None
        };
        out.push(PositionedUnit {
            unit: c.clone(),
            pcard: position_card(c.card.clone(), p.row_hand, i, action),
        });
    }
    return out;
}

pub fn click_action(p: PlayerState, game: GameSnapshot) -> Option<Action> {
    let positioned = position_player(p.clone(), game);
    let hovered: Vec<&PositionedUnit> = positioned.iter().filter(|unit| unit.pcard.hover).collect();
    if let Some(clicked) = hovered.first() {
        return clicked.pcard.action.clone();
    } else {
        return None;
    }
}

fn units_to_map(cards: Vec<PositionedUnit>) -> BTreeMap<CardId, PositionedUnit> {
    return cards
        .into_iter()
        .map(|x| (x.unit.card.card_id.clone(), x))
        .collect::<BTreeMap<_, _>>();
}

fn tween_player(
    current: PlayerState,
    previous: PlayerState,
    percent: f32,
    game: GameSnapshot,
) -> Vec<PositionedUnit> {
    let current_cards = position_player(current, game.clone());
    let previous_cards = units_to_map(position_player(previous, game.clone()));
    return current_cards
        .iter()
        .map(|curr_unit| {
            if let Some(prev_unit) = previous_cards.get(&curr_unit.unit.card.card_id) {
                return PositionedUnit {
                    unit: curr_unit.unit.clone(),
                    pcard: tween_card(curr_unit.pcard.clone(), prev_unit.pcard.clone(), percent),
                };
            } else {
                return curr_unit.clone();
            }
        })
        .collect();
}

pub fn render_player(
    current: PlayerState,
    previous: PlayerState,
    percent: f32,
    game: GameSnapshot,
) {
    let positioned = tween_player(current.clone(), previous, percent, game);
    for pcard in positioned.iter() {
        render_unit(pcard.clone(), current.player_id == PlayerId::P1);
    }
}
