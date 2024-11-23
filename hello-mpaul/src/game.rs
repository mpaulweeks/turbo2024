use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct GameHistory {
    p1deck: Deck,
    p2deck: Deck,
    pub actions: Vec<Action>,
}

pub fn create_game() -> GameHistory {
    let mut starting_actions: Vec<Action> = Vec::new();
    for _ in 0..4 {
        starting_actions.push(action_draw_from_deck(PlayerId::P1));
        starting_actions.push(action_draw_from_deck(PlayerId::P2));
    }
    return GameHistory {
        p1deck: create_deck(),
        p2deck: create_deck(),
        actions: starting_actions,
    };
}

pub fn simulate_game(game: GameHistory) -> GameSnapshot {
    let mut snapshot = GameSnapshot {
        p1: create_player(PlayerId::P1, game.p1deck),
        p2: create_player(PlayerId::P2, game.p2deck),
    };

    for action in game.actions.iter() {
        snapshot = apply_action(snapshot, action.clone());
    }

    return snapshot;
}
