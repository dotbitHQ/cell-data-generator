use ckb_hash::blake2b_256;
use das_types_std::{constants::*, packed::*, prelude::*, util as das_util};
use faster_hex::hex_string;
use std::convert::TryFrom;

fn main() {
    let id: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let next: Vec<u8> = vec![
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ];
    let account = AccountChars::default();
    let expired_at = u64::MAX.to_le_bytes();

    let entity = AccountCellData::new_builder()
        .id(AccountId::try_from(id.clone()).unwrap())
        .account(account)
        .registered_at(Uint64::from(0))
        .status(Uint8::from(0))
        .build();

    let hash = Hash::try_from(blake2b_256(entity.as_slice()).to_vec()).unwrap();
    // The merkle root of DAS team's members' messages.
    let message: Vec<u8> = vec![
        30, 252, 99, 64, 227, 240, 82, 13, 3, 164, 175, 111, 13, 217, 225, 82, 69, 52, 130, 85, 131, 88, 229, 199, 155,
        146, 129, 38, 84, 19, 109, 8, 184, 180, 171, 63, 69, 73, 42, 137, 66, 186, 6, 89, 56, 213, 240, 101, 226, 149,
        151, 174, 178, 137, 65, 239, 230, 110, 44, 13, 66, 13, 26, 66, 23, 72, 163, 142, 230, 249, 32, 7, 107, 168,
        127, 35, 95, 124, 151, 19, 126, 76, 244, 91, 224, 175, 88, 181, 245, 83, 85, 188, 9, 150, 231, 208,
    ];
    let cell_data = [
        hash.as_reader().raw_data(),
        id.as_slice(),
        next.as_slice(),
        &expired_at[..],
        message.as_slice(),
    ]
    .concat();
    let action_witness = das_util::wrap_action_witness("init_account_chain", None);
    let cell_witness = das_util::wrap_data_witness::<AccountCellData, AccountCellData, AccountCellData>(
        DataType::AccountCellData,
        Some((2, 0, entity)),
        None,
        None,
    );

    println!(
        "0x 0x{} 0x{} 0x{}",
        hex_string(cell_data.as_slice()).unwrap(),
        hex_string(action_witness.as_reader().raw_data()).unwrap(),
        hex_string(cell_witness.as_reader().raw_data()).unwrap(),
    );
}
