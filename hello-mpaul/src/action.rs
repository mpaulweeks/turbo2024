use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
enum ActionType {
    DrawFromDeck,
    PlayFromHand,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Action {
    action_type: ActionType,
    player_id: PlayerId,
    card_id: u32,
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
    pub p1: Player,
    pub p2: Player,
}

pub fn apply_action(snapshot: GameSnapshot, action: Action) -> GameSnapshot {
    let mut player = if action.player_id == PlayerId::P1 {
        snapshot.p1.clone()
    } else {
        snapshot.p2.clone()
    };
    match action.action_type {
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
            let mut new_hand: Vec<Card> = Vec::new();
            for card in player.hand.iter() {
                let is_hover = card.card_id == action.card_id;
                if is_hover {
                    new_board.push(card.clone());
                } else {
                    new_hand.push(card.clone());
                }
            }
            player.board = new_board;
            player.hand = new_hand;
        }
    }
    let mut out = GameSnapshot {
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
