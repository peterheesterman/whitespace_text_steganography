extern crate regex;

use std::char;

fn is_zero_width_whitespace_character(candidate: char) -> bool {
    candidate == '​'
}

fn get_whitespace_groups(carrier: String) -> Vec<u32> {
    let mut whitespace_groups: Vec<u32> = Vec::new();

    let mut carrier = carrier.chars();
    carrier.next();

    let mut character = carrier.next().unwrap();
    while is_zero_width_whitespace_character(character) {
        let mut count: u32 = 0;
        while is_zero_width_whitespace_character(character) {
            count = count + 1;
            character = carrier.next().unwrap();
        }
        whitespace_groups.push(count);
        character = carrier.next().unwrap();
    }

    println!("{:?}", whitespace_groups);
    whitespace_groups
}

pub fn extract(carrier: String) -> String {
    let mut payload = String::new();

    for group in get_whitespace_groups(carrier).iter() {
        payload.push(char::from_u32(*group).unwrap());
    }

    payload.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_identify_zero_width_whitespace() {
        assert!(is_zero_width_whitespace_character('​'))
    }

    #[test]
    fn can_group_white_space() {
        let whitespace_infested_text = String::from("W​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​h​​​​​​​​​​o​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​​so list to hunt");
        let whitespace_groups_expected: Vec<u32> = vec![49, 10, 50];
        assert_eq!(
            get_whitespace_groups(whitespace_infested_text),
            whitespace_groups_expected
        );
    }
}
