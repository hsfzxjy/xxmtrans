use encoding;
use encoding::{DecoderTrap, EncoderTrap, Encoding};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_xxmt(input: &str) -> String {
    use zhconv::converters::ZH_TO_HANT_CONVERTER as cvt;
    let x = cvt.convert(input);
    // let x = input;
    let x = encoding::all::BIG5_2003
        .encode(&x, EncoderTrap::Ignore)
        .unwrap();
    encoding::all::GBK.decode(&x, DecoderTrap::Ignore).unwrap()
}

#[test]
fn test_to_xxmt() {
    let input = "暗黑魔法师";
    assert_eq!("穞堵臸猭畍", to_xxmt(input));
}

