use std::iter;

pub fn interleave(payload: String, carrier: String) -> String {
    let zero_width_whitespace_character = '​';

    if carrier.len() < payload.len() + 1 {
        // TODO: add error passing and use into steg
        panic!("The carrier is too short!");
    }

    let mut carrier = carrier.chars();

    let mut package = String::new();
    package.push(carrier.next().unwrap());

    for character in payload.chars() {
        let number_of_zwwc_needed = character as u32;
        let space = iter::repeat(zero_width_whitespace_character)
            .take(number_of_zwwc_needed as usize)
            .collect::<String>();
        package.push_str(&space);
        package.push(carrier.next().unwrap());
    }

    package.push_str(&carrier.collect::<String>());

    package.to_string()
}
