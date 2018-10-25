use super::*;

#[test]
fn hide_payload() {
    let payload_path = "./src/tests/texts/small.txt";
    let carrier_path = "./src/tests/texts/carrier_sonnet.txt";

    let text = hide(payload_path, carrier_path);
    assert_eq!(text, "W\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}h\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}o\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}so list to hunt, I know where is an hind,\nBut as for me, hélas, I may no more.\nThe vain travail hath wearied me so sore,\nI am of them that farthest cometh behind.\n\nYet may I by no means my wearied mind\nDraw from the deer, but as she fleeth afore\nFainting I follow. I leave off therefore,\nSithens in a net I seek to hold the wind.\n\nWho list her hunt, I put him out of doubt,\nAs well as I may spend his time in vain.\nAnd graven with diamonds in letters plain\nThere is written, her fair neck round about:\n\nNoli me tangere, for Caesar\'s I am,\nAnd wild for to hold, though I seem tame.");
}

#[test]
fn reveal_hidden_text() {
    let carrier_path = "./src/tests/hidden.txt";

    let text = reveal(carrier_path);
    assert_eq!(text, "revealed");
}
