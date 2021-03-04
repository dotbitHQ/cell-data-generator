use ckb_hash::blake2b_256;
use das_types::{constants::*, packed::*, prelude::*, util as das_util};
use faster_hex::hex_string;
use std::convert::TryFrom;

fn main() {
    let id: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let next: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    let account = AccountChars::default();
    let expired_at = u64::MAX.to_le_bytes();

    let owner_lock = Script::default();
    let manager_lock = Script::default();

    let entity = AccountCellData::new_builder()
        .id(AccountId::try_from(id.clone()).unwrap())
        .account(account)
        .owner_lock(owner_lock)
        .manager_lock(manager_lock)
        .registered_at(Uint64::from(0))
        .status(Uint8::from(0))
        .build();

    let hash = Hash::try_from(blake2b_256(entity.as_slice()).to_vec()).unwrap();
    let cell_data = [
        hash.as_reader().raw_data(),
        id.as_slice(),
        next.as_slice(),
        &expired_at[..],
        &[0],
    ]
    .concat();
    let action_witness = das_util::wrap_action_witness("init-account-chain", None);
    let cell_witness =
        das_util::wrap_data_witness(DataType::AccountCellData, Some((1, 0, entity)), None, None);

    println!(
        "0x 0x{} 0x{} 0x{}",
        hex_string(cell_data.as_slice()).unwrap(),
        hex_string(action_witness.as_reader().raw_data()).unwrap(),
        hex_string(cell_witness.as_reader().raw_data()).unwrap(),
    );
}
