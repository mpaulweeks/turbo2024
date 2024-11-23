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
    pub attacks: Vec<Attack>,
    pub board: Vec<UnitCard>,
    pub hand: Vec<UnitCard>,
    pub deck: Vec<UnitCard>,
    pub ready: bool,
}

pub fn create_player(index: PlayerId, deck: Deck) -> PlayerState {
    return PlayerState {
        row_board: if index == PlayerId::P1 { 3 } else { 1 },
        row_hand: if index == PlayerId::P1 { 4 } else { 0 },
        player_id: index,
        attacks: Vec::new(),
        board: Vec::new(),
        hand: Vec::new(),
        deck,
        ready: false,
    };
}

pub fn position_player(p: PlayerState, game: GameSim) -> Vec<PositionedUnit> {
    let mut out: Vec<PositionedUnit> = Vec::new();

    let ready_action = if p.ready {
        None
    } else {
        Some(create_action_ready(p.player_id.clone()))
    };
    out.push(PositionedUnit {
        unit: create_ready_unit(),
        pos: position_card(
            (p.row_hand as f32 + p.row_board as f32) / 2.0,
            -1.5,
            ready_action,
        ),
    });

    for (i, c) in p.board.iter().enumerate() {
        let can_attack = game.round_phase == RoundPhase::Attack && !c.attacking;
        let unit_action = if can_attack {
            Some(create_action_attack(
                p.player_id.clone(),
                c.card.card_id,
                EMPTY_CARD_ID,
            ))
        } else {
            None
        };
        out.push(PositionedUnit {
            unit: c.clone(),
            pos: position_card(p.row_board as f32, i as f32, unit_action),
        });
    }
    for (i, c) in p.hand.iter().enumerate() {
        let can_play = game.round_phase == RoundPhase::Deploy
            && impulse_check(c.clone(), game.impulse.clone());
        let unit_action = if can_play {
            Some(create_action_play_from_hand(
                p.player_id.clone(),
                c.card.card_id,
            ))
        } else {
            None
        };
        out.push(PositionedUnit {
            unit: c.clone(),
            pos: position_card(p.row_hand as f32, i as f32, unit_action),
        });
    }
    return out;
}

pub fn click_action(p: PlayerState, game: GameSim) -> Option<Action> {
    let positioned = position_player(p.clone(), game);
    let hovered: Vec<&PositionedUnit> = positioned.iter().filter(|unit| unit.pos.hover).collect();
    if let Some(clicked) = hovered.first() {
        return clicked.pos.action.clone();
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
    game: GameSim,
) -> Vec<PositionedUnit> {
    let current_cards = position_player(current, game.clone());
    let previous_cards = units_to_map(position_player(previous, game.clone()));
    return current_cards
        .iter()
        .map(|curr_unit| {
            if let Some(prev_unit) = previous_cards.get(&curr_unit.unit.card.card_id) {
                return PositionedUnit {
                    unit: curr_unit.unit.clone(),
                    pos: tween_card(curr_unit.pos.clone(), prev_unit.pos.clone(), percent),
                };
            } else {
                return curr_unit.clone();
            }
        })
        .collect();
}

pub fn render_player(current: PlayerState, previous: PlayerState, percent: f32, game: GameSim) {
    let positioned = tween_player(current.clone(), previous, percent, game);
    for pcard in positioned.iter() {
        // todo forcing visible=true for local testing
        render_unit(pcard.clone(), true);
    }
}
