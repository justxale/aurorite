pub fn get_modification(value: i16) -> i16 {
    if value <= 11 {
        (value - 11) / 2
    } else {
        (value - 10) / 2
    }
}

pub fn get_proficiency_bonus(level: i16) -> i16 {
    2 + ((level - 1) / 4)
}
