use crate::*;

pub const MIN_ACTION_TICKS: f32 = 0.0;
pub const MAX_ACTION_TICKS: f32 = 60.0;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct GameHistory {
    pub local: Option<PlayerId>,
    impulse_deck: Vec<ImpulseCard>,
    p1deck: Deck,
    p2deck: Deck,
    pub action_index: i32,
    pub action_ticks: f32,
    pub actions: Vec<Action>,
}

pub fn create_game(rands: &mut Rands) -> GameHistory {
    return GameHistory {
        // local: Some(PlayerId::P1),
        // local: Some(PlayerId::P2),
        local: None,
        impulse_deck: create_impulse_deck(rands),
        p1deck: create_deck(rands),
        p2deck: create_deck(rands),
        action_index: 0,
        action_ticks: MAX_ACTION_TICKS,
        actions: Vec::new(),
    };
}

pub struct GameDelta {
    pub current: GameSim,
    pub previous: GameSim,
}

pub fn simulate_game(game: GameHistory) -> GameDelta {
    let mut current = match game.local {
        None => GameSim {
            round_phase: RoundPhase::Begin,
            impulse: create_impulse_state(game.impulse_deck),
            p1: create_player(PlayerId::P1, game.p1deck, true, Position::Bottom),
            p2: create_player(PlayerId::P2, game.p2deck, true, Position::Top),
        },
        Some(PlayerId::P1) => GameSim {
            round_phase: RoundPhase::Begin,
            impulse: create_impulse_state(game.impulse_deck),
            p1: create_player(PlayerId::P1, game.p1deck, true, Position::Bottom),
            p2: create_player(PlayerId::P2, game.p2deck, false, Position::Top),
        },
        Some(PlayerId::P2) => GameSim {
            round_phase: RoundPhase::Begin,
            impulse: create_impulse_state(game.impulse_deck),
            p1: create_player(PlayerId::P1, game.p1deck, false, Position::Top),
            p2: create_player(PlayerId::P2, game.p2deck, true, Position::Bottom),
        },
    };
    let mut previous = current.clone();
    current.advance();

    for (index, action) in game.actions.iter().enumerate() {
        if (index as i32) < game.action_index {
            previous = current.clone();
            current.advance();
            current.apply_action(action.clone());
            current.advance();
        }
    }

    return GameDelta { current, previous };
}
