use crate::*;

#[derive(Clone)]
pub struct AttackState {
    pub player_id: PlayerId,
    pub source: CardId,
    pub target: CardId,
}
