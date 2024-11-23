use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
enum ActionType {
    DrawImpulse,
    DrawFromDeck,
    PlayFromHand,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Action {
    action_type: ActionType,
    player_id: PlayerId,
    card_id: u32,
}

pub fn action_draw_impulse() -> Action {
    return Action {
        action_type: ActionType::DrawImpulse,
        player_id: PlayerId::P1,
        card_id: 0,
    };
}

pub fn action_draw_from_deck(player_id: PlayerId) -> Action {
    return Action {
        action_type: ActionType::DrawFromDeck,
        player_id,
        card_id: 0,
    };
}

pub fn action_play_from_hand(player_id: PlayerId, card_id: CardId) -> Action {
    return Action {
        action_type: ActionType::PlayFromHand,
        player_id,
        card_id,
    };
}

#[derive(Clone)]
pub struct GameSnapshot {
    pub impulse: ImpulseState,
    pub p1: PlayerState,
    pub p2: PlayerState,
}

pub fn apply_action(snapshot: GameSnapshot, action: Action) -> GameSnapshot {
    let mut impulse = snapshot.impulse.clone();
    let mut player = if action.player_id == PlayerId::P1 {
        snapshot.p1.clone()
    } else {
        snapshot.p2.clone()
    };
    match action.action_type {
        ActionType::DrawImpulse => {
            let card = impulse.deck.pop();
            match card {
                // The division was valid
                Some(c) => impulse.board.push(c),
                // The division was invalid
                None => self::println!("Empty deck!"),
            }
        }
        ActionType::DrawFromDeck => {
            let card = player.deck.pop();
            match card {
                // The division was valid
                Some(c) => player.hand.push(c),
                // The division was invalid
                None => self::println!("Empty deck!"),
            }
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
    }
    let mut out = GameSnapshot {
        impulse,
        p1: snapshot.p1.clone(),
        p2: snapshot.p2.clone(),
    };
    if action.player_id == PlayerId::P1 {
        out.p1 = player;
    } else {
        out.p2 = player;
    };
    return out;
}
