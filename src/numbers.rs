use crate::settings::*;

const SUPERSCRIPT: &'static [&'static str; 10] = &["⁰","¹","²","³","⁴","⁵","⁶","⁷","⁸","⁹"];
const SUBSCRIPT: &'static [&'static str; 10] = &["₀","₁","₂","₃","₄","₅","₆","₇","₈","₉"];
const DIGITS: &'static [&'static str; 10] = &["0","1","2","3","4","5","6","7","8","9"];

fn encode_base_10_number(n: usize, symbols: &[&str; 10]) -> String {
    n.to_string().chars().map(|c| symbols[c.to_digit(10).unwrap() as usize]).collect()
}

pub fn number(n: usize) -> String {
    let settings = SETTINGS.lock().unwrap();
    match &settings.icon_list_format {
        IconListFormat::Superscript => encode_base_10_number(n, SUPERSCRIPT),
        IconListFormat::Subscript => encode_base_10_number(n, SUBSCRIPT),
        IconListFormat::Digits => encode_base_10_number(n, DIGITS)
    }
}
