#[cfg(test)]
mod tests;

mod file_helpers;
mod hider;
mod revealer;

pub fn hide<'a>(payload_path: &str, carrier_path: &'a str) -> String {
    let payload = file_helpers::get_file_string(payload_path);
    let carrier = file_helpers::get_file_string(carrier_path);

    hider::interleave(payload, carrier)
}

pub fn reveal(_carrier_path: &str) -> &str {
    //  use std::char;
    //  char::from_u32(intValue)
    "revealed"
}
