extern crate regex;

use std::char;

pub fn extract(carrier: String) -> String {
    let mut whitespace_groups: Vec<u32> = Vec::new();

    let mut carrier = carrier.chars();
    carrier.next();

    let mut character = carrier.next().unwrap();
    while character == '​' {
        let mut count: u32 = 0;
        while character == '​' {
            if character == '​' {
                count = count + 1;
            }
            character = carrier.next().unwrap();
        }
        whitespace_groups.push(count);
        character = carrier.next().unwrap();
    }

    let mut payload = String::new();

    for group in whitespace_groups.iter() {
        payload.push(char::from_u32(*group).unwrap());
    }

    payload.to_string()
}
