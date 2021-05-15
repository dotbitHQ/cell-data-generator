use das_types::{constants::CharSetType, packed::*, prelude::*};

pub fn gen_price_config(length: u8, new_price: u64, renew_price: u64) -> PriceConfig {
    PriceConfig::new_builder()
        .length(Uint8::from(length))
        .new(Uint64::from(new_price))
        .renew(Uint64::from(renew_price))
        .build()
}

pub fn gen_char_set(name: CharSetType, global: u8, chars: Vec<&str>) -> CharSet {
    let mut builder = CharSet::new_builder()
        .name(Uint32::from(name as u32))
        .global(Uint8::from(global));

    let mut chars_builder = Chars::new_builder();
    for char in chars {
        chars_builder = chars_builder.push(Bytes::from(char.as_bytes()));
    }
    builder = builder.chars(chars_builder.build());

    builder.build()
}

pub fn prepend_molecule_like_length(raw: Vec<u8>) -> Vec<u8> {
    // Prepend length of bytes to raw data, include the bytes of length itself.
    let mut entity = (raw.len() as u32 + 4).to_le_bytes().to_vec();
    entity.extend(raw);

    entity
}
