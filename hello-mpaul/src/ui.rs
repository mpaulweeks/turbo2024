use crate::*;

pub fn get_atk_sprite(power: i32) -> String {
    let all = vec![
        "ATK_Sprites1".to_string(),
        "ATK_Sprites2".to_string(),
        "ATK_Sprites3".to_string(),
        "ATK_Sprites4".to_string(),
        "ATK_Sprites5".to_string(),
        "ATK_Sprites6".to_string(),
        "ATK_Sprites7".to_string(),
        "ATK_Sprites8".to_string(),
        "ATK_Sprites9".to_string(),
        "ATK_Sprites10".to_string(),
        "ATK_Sprites11".to_string(),
        "ATK_Sprites12".to_string(),
        "ATK_Sprites13".to_string(),
        "ATK_Sprites14".to_string(),
        "ATK_Sprites15".to_string(),
        "ATK_Sprites16".to_string(),
        "ATK_Sprites17".to_string(),
        "ATK_Sprites18".to_string(),
        "ATK_Sprites19".to_string(),
        "ATK_Sprites20".to_string(),
    ];
    return all.get(power as usize).unwrap().clone();
}

pub fn get_clock_sprite(impulse_turn: usize) -> String {
    let all = vec![
        "ATK_Sprites1".to_string(), // todo missing 0
        "TimeNumbers1".to_string(),
        "TimeNumbers2".to_string(),
        "TimeNumbers3".to_string(),
        "TimeNumbers4".to_string(),
        "TimeNumbers5".to_string(),
        "TimeNumbers6".to_string(),
        "TimeNumbers7".to_string(),
        "TimeNumbers8".to_string(),
        "TimeNumbers9".to_string(),
        "TimeNumbers10".to_string(),
        "TimeNumbers11".to_string(),
        "TimeNumbers12".to_string(),
    ];
    return all.get(impulse_turn).unwrap().clone();
}

pub fn get_mana_sprite(impulse_type: ImpulseType) -> String {
    return match impulse_type {
        ImpulseType::Blue => "Mana_Sprites1".to_string(),
        ImpulseType::Red => "Mana_Sprites2".to_string(),
        ImpulseType::Green => "Mana_Sprites3".to_string(),
        // ImpulseType::Blue => "Mana_Sprites4".to_string(),
    };
}
