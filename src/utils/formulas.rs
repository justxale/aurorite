pub fn get_modification(value: i16) -> i16 {
    if value <= 11 {
        (value - 11) / 2
    } else {
        (value - 10) / 2
    }
}

