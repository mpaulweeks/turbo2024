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
