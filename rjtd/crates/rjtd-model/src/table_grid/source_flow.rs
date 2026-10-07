pub(crate) fn native_rule_text_supported(text: &str) -> bool {
    text.chars().all(native_rule_character_supported)
}

pub(crate) fn native_rule_character_supported(character: char) -> bool {
    character.is_ascii_graphic()
        || character == ' '
        || matches!(character, '\u{3000}'..='\u{30ff}' | '\u{3400}'..='\u{9fff}' | '\u{ff01}'..='\u{ffef}')
}
