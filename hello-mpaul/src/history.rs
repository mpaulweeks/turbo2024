use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct GameHistory {
    impulse_deck: Vec<ImpulseCard>,
    local: Option<PlayerId>,
    p1deck: Deck,
    p2deck: Deck,
    pub action_ticks: f32,
    pub actions: Vec<Action>,
}

pub fn create_game() -> GameHistory {
    return GameHistory {
        impulse_deck: create_impulse_deck(),
        // local: Some(PlayerId::P1),
        local: Some(PlayerId::P2),
        // local: None,
        p1deck: create_deck(),
        p2deck: create_deck(),
        action_ticks: 0.0,
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
            player_local: create_player(PlayerId::P1, game.p1deck, true, Position::Bottom),
            player_remote: create_player(PlayerId::P2, game.p2deck, true, Position::Top),
        },
        Some(PlayerId::P1) => GameSim {
            round_phase: RoundPhase::Begin,
            impulse: create_impulse_state(game.impulse_deck),
            player_local: create_player(PlayerId::P1, game.p1deck, true, Position::Bottom),
            player_remote: create_player(PlayerId::P2, game.p2deck, false, Position::Top),
        },
        Some(PlayerId::P2) => GameSim {
            round_phase: RoundPhase::Begin,
            impulse: create_impulse_state(game.impulse_deck),
            player_local: create_player(PlayerId::P2, game.p2deck, true, Position::Bottom),
            player_remote: create_player(PlayerId::P1, game.p1deck, false, Position::Top),
        },
    };
    let mut previous = current.clone();
    current.advance();

    for action in game.actions.iter() {
        previous = current.clone();
        current.advance();
        current.apply_action(action.clone());
        current.advance();
    }

    return GameDelta { current, previous };
}
