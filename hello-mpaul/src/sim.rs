use crate::*;

#[derive(PartialEq, Clone)]
pub enum RoundPhase {
    Begin,
    Draw,
    Deploy,
    Plan,
    PreAttack,
    PostAttack,
}

#[derive(Clone)]
pub struct GameSim {
    pub is_replay: bool,
    pub action_ticks: f32,
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
                    self.round_phase = RoundPhase::PreAttack;
                    self.p1.ready = false;
                    self.p2.ready = false;
                    self.p1.animating_attack = true;
                    self.p2.animating_attack = true;
                    self.p1.show_board();
                    self.p2.show_board();
                }
            }
            RoundPhase::PreAttack => {
                if self.is_replay || self.action_ticks >= MAX_ACTION_TICKS * 3.0 {
                    self.round_phase = RoundPhase::PostAttack;
                    self.p1.animating_attack = false;
                    self.p2.animating_attack = false;
                }
            }
            RoundPhase::PostAttack => {
                // execute attacks
                // todo weave these
                let attacks = self.weave_attacks();
                for attack in attacks.iter() {
                    self.execute_attack(attack.clone());
                }
                // clear attacks
                self.p1.attacks = Vec::new();
                self.p2.attacks = Vec::new();
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

    fn weave_attacks(&self) -> Vec<AttackState> {
        let active = if self.impulse.board.len() % 2 == 0 {
            PlayerId::P2
        } else {
            PlayerId::P1
        };
        let (t1, t2) = match active {
            PlayerId::P1 => (&self.p1, &self.p2),
            PlayerId::P2 => (&self.p2, &self.p1),
        };
        let mut a1 = t1.attacks.clone();
        let mut a2 = t2.attacks.clone();
        let mut out: Vec<AttackState> = Vec::new();
        while a1.len() > 0 || a2.len() > 0 {
            if a1.len() > 0 {
                out.push(a1.remove(0))
            }
            if a2.len() > 0 {
                out.push(a2.remove(0))
            }
        }
        return out;
    }

    fn execute_attack(&mut self, attack: AttackState) {
        let (attacker, defender) = match attack.player_id {
            PlayerId::P1 => (&mut self.p1, &mut self.p2),
            PlayerId::P2 => (&mut self.p2, &mut self.p1),
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
                if du.power > 0 {
                    au.power = (attack_power - du.power).clamp(0, 99);
                    du.power = (du.power - attack_power).clamp(0, 99);
                } else {
                    defender.health = (defender.health - attack_power);
                }
            } else {
                defender.health = (defender.health - attack_power);
            }
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
            PlayerId::P1 => self.p1 = player,
            PlayerId::P2 => self.p2 = player,
        }
    }

    pub fn check_click(&self, player_id: PlayerId) -> Option<Action> {
        let clicker = match player_id {
            PlayerId::P1 => self.p1.clone(),
            PlayerId::P2 => self.p2.clone(),
        };
        if let Some(action) = click_action(self.clone(), self.p1.clone(), clicker.clone()) {
            return Some(action);
        }
        if let Some(action) = click_action(self.clone(), self.p2.clone(), clicker.clone()) {
            return Some(action);
        }
        return None;
    }
}

pub fn render_round(state: GameSim) {
    let message = match state.round_phase {
        RoundPhase::Begin => "Begin",
        RoundPhase::Draw => "Draw Phase",
        RoundPhase::Deploy => "Deploy Phase",
        RoundPhase::Plan => "Planing Phase",
        RoundPhase::PreAttack => "Attack Phase",
        RoundPhase::PostAttack => "PostAttack Phase",
    };
    let res = resolution();
    let screen_width = res[0] as f32;
    let screen_height = res[1] as f32;
    text!(
        message,
        x = screen_width * 0.05,
        y = screen_height / 2.0,
        font = Font::L
    );
}
