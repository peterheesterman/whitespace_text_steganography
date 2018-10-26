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

pub fn reveal(carrier_path: &str) -> String {
    let carrier = file_helpers::get_file_string(carrier_path);

    revealer::extract(carrier)
}

