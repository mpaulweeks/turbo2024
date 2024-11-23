use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum ActionType {
    PlayFromHand,
    Ready,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Action {
    pub action_type: ActionType,
    pub player_id: PlayerId,
    pub card_id: u32,
}

pub fn create_action_play_from_hand(player_id: PlayerId, card_id: CardId) -> Action {
    return Action {
        action_type: ActionType::PlayFromHand,
        player_id,
        card_id,
    };
}

pub fn create_action_ready(player_id: PlayerId) -> Action {
    return Action {
        action_type: ActionType::Ready,
        player_id,
        card_id: 0,
    };
}
