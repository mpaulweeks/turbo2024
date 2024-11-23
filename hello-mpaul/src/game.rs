use crate::*;

pub struct GameHistory {
    p1deck: Deck,
    p2deck: Deck,
    actions: Vec<Action>,
}

pub fn create_game() -> GameHistory {
    return GameHistory {
        p1deck: create_deck(),
        p2deck: create_deck(),
        actions: Vec::new(),
    };
}

pub fn simulate_game(game: GameHistory) -> GameSnapshot {
    let mut snapshot = GameSnapshot {
        p1: create_player(PlayerId::P1),
        p2: create_player(PlayerId::P2),
    };

    for action in game.actions.iter() {
        snapshot = apply_action(snapshot, action.clone());
    }

    return snapshot;
}
