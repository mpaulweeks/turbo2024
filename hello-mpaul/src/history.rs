use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct GameHistory {
    impulse_deck: Deck,
    p1deck: Deck,
    p2deck: Deck,
    pub action_ticks: f32,
    pub actions: Vec<Action>,
}

pub fn create_game() -> GameHistory {
    let mut starting_actions: Vec<Action> = Vec::new();
    for _ in 0..4 {
        starting_actions.push(action_draw_from_deck(PlayerId::P1));
        starting_actions.push(action_draw_from_deck(PlayerId::P2));
    }
    return GameHistory {
        impulse_deck: Vec::new(), // todo
        p1deck: create_deck(),
        p2deck: create_deck(),
        action_ticks: 0.0,
        actions: starting_actions,
    };
}

pub struct GameDelta {
    pub current: GameSnapshot,
    pub previous: GameSnapshot,
}

pub fn simulate_game(game: GameHistory) -> GameDelta {
    let mut current = GameSnapshot {
        p1: create_player(PlayerId::P1, game.p1deck),
        p2: create_player(PlayerId::P2, game.p2deck),
    };
    let mut previous = current.clone();

    for action in game.actions.iter() {
        previous = current.clone();
        current = apply_action(current, action.clone());
    }

    return GameDelta { current, previous };
}
