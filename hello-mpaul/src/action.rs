use crate::*;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum ActionType {
    PlayFromHand,
    Ready,
    Targeting,
    DeclareAttack,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Action {
    pub action_type: ActionType,
    pub player_id: PlayerId,
    pub card_id: CardId,
    pub enemy_card_id: CardId,
}

pub fn create_action_play_from_hand(player_id: PlayerId, card_id: CardId) -> Action {
    return Action {
        action_type: ActionType::PlayFromHand,
        player_id,
        card_id,
        enemy_card_id: EMPTY_CARD_ID,
    };
}

pub fn create_action_ready(player_id: PlayerId) -> Action {
    return Action {
        action_type: ActionType::Ready,
        player_id,
        card_id: 0,
        enemy_card_id: EMPTY_CARD_ID,
    };
}

pub fn create_action_target(player_id: PlayerId, attacker: CardId) -> Action {
    return Action {
        action_type: ActionType::Targeting,
        player_id,
        card_id: attacker,
        enemy_card_id: EMPTY_CARD_ID,
    };
}

pub fn create_action_attack(player_id: PlayerId, attacker: CardId, target: CardId) -> Action {
    return Action {
        action_type: ActionType::DeclareAttack,
        player_id,
        card_id: attacker,
        enemy_card_id: target,
    };
}
