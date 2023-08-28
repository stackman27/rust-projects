pub fn is_light_on() -> bool {
    true
}

pub fn is_bedroom2_light_on() -> bool {
    use super::bedroom2;
    return bedroom2::is_light_on();
}
