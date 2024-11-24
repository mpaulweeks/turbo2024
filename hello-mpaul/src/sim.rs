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
    pub player_local: PlayerState,
    pub player_remote: PlayerState,
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
            PlayerId::P1 => self.player_local.clone(),
            PlayerId::P2 => self.player_remote.clone(),
        };
        let card = player.deck.pop();
        match card {
            // The division was valid
            Some(c) => player.hand.push(c),
            // The division was invalid
            None => self::println!("Empty deck!"),
        }
        match player_id {
            PlayerId::P1 => self.player_local = player,
            PlayerId::P2 => self.player_remote = player,
        }
    }
    pub fn clear_dead(&mut self, player_id: PlayerId) {
        let mut player = match player_id {
            PlayerId::P1 => self.player_local.clone(),
            PlayerId::P2 => self.player_remote.clone(),
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
            PlayerId::P1 => self.player_local = player,
            PlayerId::P2 => self.player_remote = player,
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
                self.player_local.ready = false;
                self.player_remote.ready = false;
            }
            RoundPhase::Deploy => {
                if self.player_local.ready && self.player_remote.ready {
                    self.round_phase = RoundPhase::Plan;
                    self.player_local.ready = false;
                    self.player_remote.ready = false;
                }
            }
            RoundPhase::Plan => {
                if self.player_local.ready && self.player_remote.ready {
                    self.round_phase = RoundPhase::Attack;
                    self.player_local.ready = false;
                    self.player_remote.ready = false;
                }
            }
            RoundPhase::Attack => {
                // execute attacks
                // todo weave these
                let mut attacks = self.player_local.attacks.clone();
                attacks.append(&mut self.player_remote.attacks.clone());
                for attack in attacks.iter() {
                    self.execute_attack(attack.clone());
                }
                // clear attacks
                self.player_local.attacks = Vec::new();
                self.player_remote.attacks = Vec::new();
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

    fn execute_attack(&mut self, attack: AttackState) {
        let mut attacker = match attack.player_id {
            PlayerId::P1 => self.player_local.clone(),
            PlayerId::P2 => self.player_remote.clone(),
        };
        let mut defender = match attack.player_id {
            PlayerId::P1 => self.player_remote.clone(),
            PlayerId::P2 => self.player_local.clone(),
        };
        let attack_unit = attacker
            .board
            .iter_mut()
            .find(|unit| unit.card.card_id == attack.source);
        let defend_unit = defender
            .board
            .iter_mut()
            .find(|unit| unit.card.card_id == attack.target);
        if let Some(au) = attack_unit {
            let attack_power = au.power;
            if let Some(du) = defend_unit {
                au.power = (attack_power - du.power).clamp(0, 99);
                du.power = (du.power - attack_power).clamp(0, 99);
            } else {
                defender.health = (defender.health - attack_power).clamp(0, 99);
            }
        }
        match attacker.player_id {
            PlayerId::P1 => {
                self.player_local = attacker;
                self.player_remote = defender;
            }
            PlayerId::P2 => {
                self.player_local = defender;
                self.player_remote = attacker;
            }
        }
    }

    pub fn apply_action(&mut self, action: Action) {
        let mut player = match action.player_id {
            PlayerId::P1 => self.player_local.clone(),
            PlayerId::P2 => self.player_remote.clone(),
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
            ActionType::Targeting => {
                player.targeting = Some(action.card_id);
            }
            ActionType::DeclareAttack => {
                player.targeting = None;
                player.attacks.push(AttackState {
                    player_id: action.player_id.clone(),
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
            PlayerId::P1 => self.player_local = player,
            PlayerId::P2 => self.player_remote = player,
        }
    }

    pub fn check_click(&self, player_id: PlayerId) -> Option<Action> {
        let clicker = match player_id {
            PlayerId::P1 => self.player_local.clone(),
            PlayerId::P2 => self.player_remote.clone(),
        };
        if let Some(action) = click_action(self.clone(), self.player_local.clone(), clicker.clone())
        {
            return Some(action);
        }
        if let Some(action) =
            click_action(self.clone(), self.player_remote.clone(), clicker.clone())
        {
            return Some(action);
        }
        return None;
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
    let res = resolution();
    let screen_width = res[0] as f32;
    let screen_height = res[1] as f32;
    text!(
        message,
        x = screen_width * 0.05,
        y = screen_height / 2.0,
        font = Font::L
    )
}
