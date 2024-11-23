use crate::*;

#[derive(PartialEq, Clone)]
pub enum RoundPhase {
    Begin,
    Draw,
    Deploy,
    Plan,
    Attack,
}

#[derive(Clone)]
pub struct GameSim {
    pub round_phase: RoundPhase,
    pub impulse: ImpulseState,
    pub p1: PlayerState,
    pub p2: PlayerState,
}

impl GameSim {
    pub fn draw_impulse(&mut self) {
        let card = self.impulse.deck.pop();
        match card {
            // The division was valid
            Some(c) => self.impulse.board.push(c),
            // The division was invalid
            None => self::println!("Empty deck!"),
        }
    }
    pub fn draw_player(&mut self, player_id: PlayerId) {
        let mut player = match player_id {
            PlayerId::P1 => self.p1.clone(),
            PlayerId::P2 => self.p2.clone(),
        };
        let card = player.deck.pop();
        match card {
            // The division was valid
            Some(c) => player.hand.push(c),
            // The division was invalid
            None => self::println!("Empty deck!"),
        }
        match player_id {
            PlayerId::P1 => self.p1 = player,
            PlayerId::P2 => self.p2 = player,
        }
    }
    pub fn clear_dead(&mut self, player_id: PlayerId) {
        let mut player = match player_id {
            PlayerId::P1 => self.p1.clone(),
            PlayerId::P2 => self.p2.clone(),
        };
        let mut new_board: Vec<UnitCard> = Vec::new();
        for unit in player.board.iter() {
            let mut new_unit = unit.clone();
            new_unit.attacking = false;
            if (new_unit.power > 0) {
                new_board.push(new_unit);
            }
        }
        player.board = new_board;
        match player_id {
            PlayerId::P1 => self.p1 = player,
            PlayerId::P2 => self.p2 = player,
        }
    }
    pub fn advance(&mut self) {
        let start_phase = self.round_phase.clone();
        match start_phase {
            RoundPhase::Begin => {
                // todo test
                self.draw_impulse();
                self.draw_impulse();
                self.draw_impulse();
                self.draw_impulse();
                self.draw_impulse();
                for _ in 0..3 {
                    self.draw_player(PlayerId::P1);
                    self.draw_player(PlayerId::P2);
                }
                self.round_phase = RoundPhase::Draw;
            }
            RoundPhase::Draw => {
                self.draw_impulse();
                self.draw_player(PlayerId::P1);
                self.draw_player(PlayerId::P2);
                self.draw_player(PlayerId::P1);
                self.draw_player(PlayerId::P2);
                self.round_phase = RoundPhase::Deploy;
                self.p1.ready = false;
                self.p2.ready = false;
            }
            RoundPhase::Deploy => {
                if self.p1.ready && self.p2.ready {
                    self.round_phase = RoundPhase::Plan;
                    self.p1.ready = false;
                    self.p2.ready = false;
                }
            }
            RoundPhase::Plan => {
                if self.p1.ready && self.p2.ready {
                    self.round_phase = RoundPhase::Attack;
                    self.p1.ready = false;
                    self.p2.ready = false;
                }
            }
            RoundPhase::Attack => {
                // execute attacks
                // clear up dead and reset attacking
                self.clear_dead(PlayerId::P1);
                self.clear_dead(PlayerId::P2);
                self.round_phase = RoundPhase::Draw;
            }
        }
        if start_phase != self.round_phase {
            return self.advance();
        }
    }

    pub fn apply_action(&mut self, action: Action) {
        let mut player = match action.player_id {
            PlayerId::P1 => self.p1.clone(),
            PlayerId::P2 => self.p2.clone(),
        };
        match action.action_type {
            ActionType::Ready => {
                player.ready = true;
            }
            ActionType::PlayFromHand => {
                let mut new_board = player.board.to_vec();
                let mut new_hand: Vec<UnitCard> = Vec::new();
                for unit in player.hand.iter() {
                    let is_hover = unit.card.card_id == action.card_id;
                    if is_hover {
                        new_board.push(unit.clone());
                    } else {
                        new_hand.push(unit.clone());
                    }
                }
                player.board = new_board;
                player.hand = new_hand;
            }
            ActionType::AttackTarget => {
                player.attacks.push(Attack {
                    source: action.card_id,
                    target: action.enemy_card_id,
                });
                for i in 0..player.board.len() {
                    let mut unit = player.board[i].clone();
                    if action.card_id == unit.card.card_id {
                        unit.attacking = true;
                        player.board[i] = unit;
                    }
                }
            }
        }
        match action.player_id {
            PlayerId::P1 => self.p1 = player,
            PlayerId::P2 => self.p2 = player,
        }
    }
}

pub fn render_round(state: GameSim) {
    let message = match state.round_phase {
        RoundPhase::Begin => "Begin",
        RoundPhase::Draw => "Draw",
        RoundPhase::Deploy => "Deploy",
        RoundPhase::Plan => "Plan",
        RoundPhase::Attack => "Attack",
    };
    text!(message)
}
