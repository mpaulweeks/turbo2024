use crate::*;

enum ActionType {
    PlayFromHand,
}
pub struct Action {
    action_type: ActionType,
    player_id: u8,
    card_id: u32,
}

pub fn action_play_from_hand(player_id: PlayerId, card_id: CardId) -> Action {
    return Action {
        action_type: ActionType::PlayFromHand,
        player_id,
        card_id,
    };
}
