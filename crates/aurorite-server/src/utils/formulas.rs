pub fn get_modification(value: u8) -> i8 {
    if value <= 11 {
        (value as i8 - 11) / 2
    } else {
        (value as i8 - 10) / 2
    }
}

pub fn get_proficiency_bonus(level: u8) -> u8 {
    2 + ((level - 1) / 4)
}
