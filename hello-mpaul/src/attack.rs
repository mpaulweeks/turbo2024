use crate::*;

#[derive(Clone, Debug)]
pub struct AttackState {
    pub player_id: PlayerId,
    pub source: CardId,
    pub target: CardId,
}
