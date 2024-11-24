use std::collections::BTreeMap;

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum PlayerId {
    P1,
    P2,
}

#[derive(PartialEq)]
pub enum Position {
    Top,
    Bottom,
}

#[derive(Clone)]
pub struct PlayerState {
    pub player_id: PlayerId,
    pub health: i32,
    row_board: u8,
    row_hand: u8,
    pub attacks: Vec<AttackState>,
    pub board: Vec<UnitCard>,
    pub hand: Vec<UnitCard>,
    pub deck: Vec<UnitCard>,
    pub ready: bool,
    pub targeting: Option<CardId>,
    pub visible: bool,
}

pub fn create_player(
    index: PlayerId,
    deck: Deck,
    visible: bool,
    position: Position,
) -> PlayerState {
    return PlayerState {
        row_board: if position == Position::Bottom { 3 } else { 1 },
        row_hand: if position == Position::Bottom { 4 } else { 0 },
        health: 20,
        player_id: index,
        attacks: Vec::new(),
        board: Vec::new(),
        hand: Vec::new(),
        deck,
        ready: false,
        targeting: None,
        visible,
    };
}

pub fn position_player(game: GameSim, p: PlayerState, clicker: PlayerState) -> Vec<PositionedUnit> {
    let mut out: Vec<PositionedUnit> = Vec::new();
    let clicker_can_attack = game.round_phase == RoundPhase::Plan
        && p.player_id != clicker.player_id
        && clicker.targeting.is_some();

    let ready_action = if !p.ready && p.targeting.is_none() && p.player_id == clicker.player_id {
        Some(create_action_ready(p.player_id.clone()))
    } else if clicker_can_attack {
        Some(create_action_attack(
            clicker.player_id.clone(),
            clicker.targeting.unwrap(),
            EMPTY_CARD_ID,
        ))
    } else {
        None
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
        let can_target = game.round_phase == RoundPhase::Plan
            && p.player_id == clicker.player_id
            && p.targeting.is_none()
            && !c.attacking;
        let unit_action = if can_target {
            Some(create_action_target(p.player_id.clone(), c.card.card_id))
        } else if clicker_can_attack {
            Some(create_action_attack(
                clicker.player_id.clone(),
                clicker.targeting.unwrap(),
                c.card.card_id,
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
            && p.player_id == clicker.player_id
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

pub fn click_action(game: GameSim, player: PlayerState, clicker: PlayerState) -> Option<Action> {
    let positioned = position_player(game, player.clone(), clicker);
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
    let current_cards = position_player(game.clone(), current.clone(), current);
    let previous_cards = units_to_map(position_player(game.clone(), previous.clone(), previous));
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

impl PlayerState {
    pub fn render_player(
        &self,
        previous: PlayerState,
        percent: f32,
        game: GameSim,
    ) -> Vec<PositionedUnit> {
        let positioned = tween_player(self.clone(), previous, percent, game.clone());
        for pcard in positioned.iter() {
            // todo forcing visible=true for local testing
            render_unit(self.clone(), pcard.clone(), self.visible);
        }

        // health
        let res = resolution();
        let screen_width = res[0] as f32;
        let screen_height = res[1] as f32;
        let grid_width = screen_width * 0.8;
        let panel_width = screen_width - grid_width;
        text!(
            &self.health.to_string(),
            x = panel_width / 2.0,
            y = match self.player_id {
                PlayerId::P1 => screen_height * 0.65,
                PlayerId::P2 => screen_height * 0.35,
            },
            font = Font::L,
        );

        return positioned;
    }

    pub fn render_target(&self, positioned: Vec<PositionedUnit>) {
        if let Some(attacker) = self.targeting {
            for pcard in positioned.iter() {
                if pcard.unit.card.card_id == attacker {
                    pcard.render_target();
                }
            }
        }
    }
}
