use ckb_hash::blake2b_256;
use das_types::{constants::*, packed::*, prelude::*, util as das_util};
use faster_hex::hex_string;
use std::io::BufRead;
use std::path::PathBuf;
use std::{env, fs, io};
use util::{gen_char_set, gen_price_config, prepend_molecule_like_length};

mod charset;
mod util;

macro_rules! gen_return_from_entity {
    ( $config_type:expr, $entity:expr ) => {{
        let config_type = ($config_type as u32).to_le_bytes();
        let cell_data = Bytes::from(blake2b_256($entity.as_slice()).to_vec());
        let action_witness = das_util::wrap_action_witness("config", None);

        let cell_witness = das_util::wrap_entity_witness($config_type, $entity);

        format!(
            "0x{} 0x{} 0x{} 0x{}",
            hex_string(&config_type).unwrap(),
            hex_string(cell_data.as_reader().raw_data()).unwrap(),
            hex_string(action_witness.as_reader().raw_data()).unwrap(),
            hex_string(cell_witness.as_reader().raw_data()).unwrap(),
        )
    }};
}

macro_rules! gen_return_from_raw {
    ( $config_type:expr, $entity:expr ) => {{
        let config_type = ($config_type as u32).to_le_bytes();
        let cell_data = Bytes::from(blake2b_256($entity.as_slice()).to_vec());
        let action_witness = das_util::wrap_action_witness("config", None);

        let cell_witness = das_util::wrap_raw_witness($config_type, $entity);

        format!(
            "0x{} 0x{} 0x{} 0x{}",
            hex_string(&config_type).unwrap(),
            hex_string(cell_data.as_reader().raw_data()).unwrap(),
            hex_string(action_witness.as_reader().raw_data()).unwrap(),
            hex_string(cell_witness.as_reader().raw_data()).unwrap(),
        )
    }};
}

fn gen_config_cell_account() -> String {
    let entity = ConfigCellAccount::new_builder()
        .max_length(Uint32::from(1000))
        .basic_capacity(Uint64::from(20_000_000_000))
        .expiration_grace_period(Uint32::from(2_592_000))
        .record_min_ttl(Uint32::from(300))
        .build();

    gen_return_from_entity!(DataType::ConfigCellAccount, entity)
}

fn gen_config_cell_apply() -> String {
    let entity = ConfigCellApply::new_builder()
        .apply_min_waiting_block_number(Uint32::from(1))
        .apply_max_waiting_block_number(Uint32::from(5760))
        .build();

    gen_return_from_entity!(DataType::ConfigCellApply, entity)
}

fn gen_config_cell_char_set() -> String {
    let char_sets = CharSetList::new_builder()
        .push(gen_char_set(CharSetType::Emoji, 1, charset::emoji()))
        .push(gen_char_set(CharSetType::Digit, 1, charset::digit()))
        .push(gen_char_set(CharSetType::En, 0, charset::english()))
        .build();

    let entity = ConfigCellCharSet::new_builder()
        .char_sets(char_sets)
        .build();

    gen_return_from_entity!(DataType::ConfigCellCharSet, entity)
}

fn gen_config_cell_income() -> String {
    let entity = ConfigCellIncome::new_builder()
        .basic_capacity(Uint64::from(20_000_000_000))
        .max_records(Uint32::from(100))
        .build();

    gen_return_from_entity!(DataType::ConfigCellIncome, entity)
}

fn gen_config_cell_main() -> String {
    // ⚠️ Do not modify the following lines of type_id_table,
    // it will be use for search and replace in deploy scripts.
    let type_id_table = TypeIdTable::new_builder()
        .account_cell(Hash::from([
            213, 100, 26, 205, 166, 4, 225, 237, 52, 34, 251, 54, 22, 0, 127, 36, 226, 130, 102,
            196, 183, 111, 102, 7, 115, 130, 150, 200, 39, 140, 42, 79,
        ]))
        .apply_register_cell(Hash::from([
            15, 191, 248, 113, 221, 5, 174, 225, 253, 162, 190, 56, 120, 106, 210, 29, 82, 162,
            118, 92, 96, 37, 209, 239, 105, 39, 215, 97, 213, 26, 60, 209,
        ]))
        .bidding_cell(Hash::default())
        .income_cell(Hash::from([
            15, 191, 248, 113, 221, 5, 174, 225, 253, 162, 190, 56, 120, 106, 210, 29, 82, 162,
            118, 92, 96, 37, 209, 239, 105, 39, 215, 97, 213, 26, 60, 209,
        ]))
        .on_sale_cell(Hash::default())
        .pre_account_cell(Hash::from([
            108, 132, 65, 35, 63, 0, 116, 25, 85, 246, 94, 71, 103, 33, 161, 165, 65, 121, 151,
            193, 228, 54, 136, 1, 201, 156, 127, 97, 127, 139, 117, 68,
        ]))
        .proposal_cell(Hash::from([
            103, 212, 140, 9, 17, 228, 6, 81, 141, 226, 17, 107, 217, 28, 106, 243, 124, 5, 241,
            219, 35, 51, 76, 168, 41, 210, 175, 48, 66, 66, 126, 68,
        ]))
        .build();

    let entity = ConfigCellMain::new_builder()
        // .account_expiration_grace_period(Uint32::from(2_592_000)) // 30 days
        // .min_ttl(Uint32::from(300))
        .type_id_table(type_id_table)
        .build();

    gen_return_from_entity!(DataType::ConfigCellMain, entity)
}

fn gen_config_cell_price() -> String {
    let discount = DiscountConfig::new_builder()
        .invited_discount(Uint32::from(500))
        .build();

    let prices = PriceConfigList::new_builder()
        .push(gen_price_config(1, 12_000_000, 1_200_000))
        .push(gen_price_config(2, 11_000_000, 1_100_000))
        .push(gen_price_config(3, 10_000_000, 1_000_000))
        .push(gen_price_config(4, 9_000_000, 900_000))
        .push(gen_price_config(5, 8_000_000, 800_000))
        .push(gen_price_config(6, 7_000_000, 700_000))
        .push(gen_price_config(7, 6_000_000, 600_000))
        .push(gen_price_config(8, 5_000_000, 500_000))
        .build();

    let entity = ConfigCellPrice::new_builder()
        .discount(discount)
        .prices(prices)
        .build();

    gen_return_from_entity!(DataType::ConfigCellPrice, entity)
}

fn gen_config_cell_proposal() -> String {
    let entity = ConfigCellProposal::new_builder()
        .proposal_min_confirm_interval(Uint8::from(2))
        .proposal_min_extend_interval(Uint8::from(1))
        .proposal_min_recycle_interval(Uint8::from(8))
        .proposal_max_account_affect(Uint32::from(50))
        .proposal_max_pre_account_contain(Uint32::from(50))
        .build();

    gen_return_from_entity!(DataType::ConfigCellProposal, entity)
}

fn gen_config_cell_profit_rate() -> String {
    let entity = ConfigCellProfitRate::new_builder()
        .channel(Uint32::from(800))
        .inviter(Uint32::from(800))
        .das(Uint32::from(8000))
        .proposal_create(Uint32::from(400))
        .proposal_confirm(Uint32::from(0))
        .income_consolidate(Uint32::from(0))
        .build();

    gen_return_from_entity!(DataType::ConfigCellProfitRate, entity)
}

fn gen_config_cell_record_key_namespace() -> String {
    let dir = env::current_dir().unwrap();
    let mut file_path = PathBuf::new();
    file_path.push(dir);
    file_path.push("record_key_namespace.txt");

    // Read record keys from file, then sort them.
    let file = fs::File::open(file_path).expect("Expect file ./record_key_namespace.txt exist.");
    let lines = io::BufReader::new(file).lines();
    let mut record_key_namespace = Vec::new();
    for line in lines {
        if let Ok(key) = line {
            record_key_namespace.push(key);
        }
    }
    record_key_namespace.sort();

    // Join all record keys with 0x00 byte as entity.
    let mut raw = Vec::new();
    for key in record_key_namespace {
        raw.extend(key.as_bytes());
        raw.extend(&[0u8]);
    }
    let raw = prepend_molecule_like_length(raw);

    gen_return_from_raw!(DataType::ConfigCellRecordKeyNamespace, raw)
}

fn gen_config_cell_reserved_account() -> String {
    let dir = env::current_dir().unwrap();
    let mut file_path = PathBuf::new();
    file_path.push(dir);
    file_path.push("reserved_accounts.txt");

    // Read record keys from file, then sort them.
    let file = fs::File::open(file_path).expect("Expect file ./reserved_accounts.txt exist.");
    let lines = io::BufReader::new(file).lines();
    let mut account_hashes = Vec::new();
    for line in lines {
        if let Ok(account) = line {
            let account_hash = blake2b_256(account.as_bytes());
            account_hashes.push(account_hash.get(..10).unwrap().to_vec());
        }
    }
    account_hashes.sort();
    let mut raw = account_hashes.into_iter().flatten().collect::<Vec<u8>>();
    raw = prepend_molecule_like_length(raw);

    gen_return_from_raw!(DataType::ConfigCellRecordKeyNamespace, raw)
}

fn main() {
    println!(
        "{},{},{},{},{},{},{},{},{},{}",
        gen_config_cell_account(),
        gen_config_cell_apply(),
        gen_config_cell_char_set(),
        gen_config_cell_income(),
        gen_config_cell_main(),
        gen_config_cell_price(),
        gen_config_cell_proposal(),
        gen_config_cell_profit_rate(),
        gen_config_cell_record_key_namespace(),
        gen_config_cell_reserved_account()
    );
}