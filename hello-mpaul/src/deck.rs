use crate::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_deck() -> Vec<Card> {
    let mut deck = Vec::new();
    for _ in 0..10 {
        deck.push(Card {
            instance_id: deck.len() as u32 + 1,
            sprite: "VICardForward_Front".to_string(),
        });
    }
    let shuffled = vec.shuffle(&mut thread_rng());
    return shuffled;
}
