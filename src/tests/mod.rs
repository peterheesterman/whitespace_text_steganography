
use super::*;

#[test]
fn hide_payload() {
    let payload_path = "./src/tests/payload.txt";
    let carrier_path = "./src/tests/carrier.txt";

    let text = hide(payload_path, carrier_path);
    assert_eq!(text, "hidden");
    //println!("{}", text);
}

#[test]
fn reveal_hidden_text() {
    let carrier_path = "./src/tests/hidden.txt";
    
    let text = reveal(carrier_path);
    assert_eq!(text, "revealed");
}

