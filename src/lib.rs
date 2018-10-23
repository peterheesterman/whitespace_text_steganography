

#[cfg(test)]
mod tests;

mod hider;
mod revealer;

// Same size and type
// Copy all the payload or the limit if the carrier size.
pub fn hide<'a>(_payload_path: &str, _carrier_path: &'a str) -> &'a str {
    "hidden"
}

pub fn reveal(_carrier_path: &str) -> &str {
    "revealed"
}