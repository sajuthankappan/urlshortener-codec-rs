extern crate urlshortener_converter;

use urlshortener_converter as converter;

#[test]
fn encode_for_zero_returns_a() {
    let alias = converter::encode(0);
    assert_eq!("a", alias);
}

#[test]
fn encode_for_1_returns_b() {
    let alias = converter::encode(1);
    assert_eq!("b", alias);
}

#[test]
fn encode_for_1000_returns_qi() {
    let alias = converter::encode(1000);
    assert_eq!("qi", alias);
}

#[test]
fn decode_for_qi_returns_1000() {
    let i = converter::decode("qi".to_string());
    assert_eq!(1000, i);
}

#[test]
fn decode_for_a_returns_0() {
    let i = converter::decode("a".to_string());
    assert_eq!(0, i);
}
