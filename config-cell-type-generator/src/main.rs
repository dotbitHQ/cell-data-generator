use ckb_hash::blake2b_256;
use das_types_std::{constants::*, packed::*, prelude::*, util as das_util};
use faster_hex::hex_string;
use util::{gen_price_config, gen_timestamp, prepend_molecule_like_length, read_lines};

mod constants;
mod util;
use constants::*;
use hex;

macro_rules! out_point {
    ($tx_hash:expr, $index:expr) => {
        OutPoint::new_builder()
            .tx_hash(Hash::from($tx_hash))
            .index(Uint32::from($index))
            .build()
    };
}

macro_rules! gen_return_from_entity {
    ( $config_type:expr, $entity:expr ) => {{
        let config_type = ($config_type as u32).to_le_bytes();
        let cell_data = Bytes::from(blake2b_256($entity.as_slice()).to_vec());
        let action_witness = das_util::wrap_action_witness("config", None);

        let cell_witness = das_util::wrap_entity_witness($config_type, $entity);

        // println!(
        //     "size of {:?}: {}",
        //     $config_type,
        //     cell_witness.as_slice().len()
        // );
        if cell_witness.as_slice().len() > WITNESS_SIZE_LIMIT {
            panic!(
                "The size of {:?} is more than {} bytes, this needs to modify das-contracts to support.",
                $config_type, WITNESS_SIZE_LIMIT
            )
        }

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

        // println!(
        //     "size of {:?}: {}",
        //     $config_type,
        //     cell_witness.as_slice().len()
        // );
        if cell_witness.as_slice().len() > WITNESS_SIZE_LIMIT {
            panic!(
                "The size of {:?} is more than {} bytes, this needs to modify das-contracts to support.",
                $config_type, WITNESS_SIZE_LIMIT
            )
        }

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
        .max_length(Uint32::from(42))
        // The basic_capacity contains 1 CKB for kinds of fees
        .basic_capacity(Uint64::from(20_600_000_000))
        .prepared_fee_capacity(Uint64::from(100_000_000))
        .expiration_grace_period(Uint32::from(2_592_000))
        .record_min_ttl(Uint32::from(300))
        .record_size_limit(Uint32::from(5000))
        .transfer_account_fee(Uint64::from(10_000))
        .edit_manager_fee(Uint64::from(10_000))
        .edit_records_fee(Uint64::from(10_000))
        .common_fee(Uint64::from(10_000))
        .transfer_account_throttle(Uint32::from(300))
        .edit_manager_throttle(Uint32::from(300))
        .edit_records_throttle(Uint32::from(300))
        .common_throttle(Uint32::from(300))
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

fn gen_config_cell_income() -> String {
    let entity = ConfigCellIncome::new_builder()
        .basic_capacity(Uint64::from(20_000_000_000))
        .max_records(Uint32::from(50))
        .min_transfer_capacity(Uint64::from(12_000_000_000))
        .build();

    gen_return_from_entity!(DataType::ConfigCellIncome, entity)
}

fn gen_config_cell_main() -> String {
    // ⚠️ Do not modify the following lines of type_id_table,
    // it will be use for search and replace in deploy scripts.
    /* CAREFUL do not commit any changes for these configs below ⬇️ */
    let type_id_table = TypeIdTable::new_builder()
        .account_cell(Hash::from([17, 6, 217, 234, 204, 222, 9, 149, 167, 224, 126, 128, 221, 12, 231, 80, 159, 33, 117, 37, 56, 223, 221, 30, 226, 82, 109, 36, 87, 72, 70, 177]))
        .account_sale_cell(Hash::from([148, 56, 18, 74, 189, 244, 203, 191, 214, 16, 101, 232, 182, 69, 35, 23, 43, 239, 94, 239, 226, 124, 183, 105, 196, 10, 202, 240, 54, 170, 137, 194]))
        // .account_auction_cell(Hash::from([]))
        .apply_register_cell(Hash::from([15, 191, 248, 113, 221, 5, 174, 225, 253, 162, 190, 56, 120, 106, 210, 29, 82, 162, 118, 92, 96, 37, 209, 239, 105, 39, 215, 97, 213, 26, 60, 209]))
        .balance_cell(Hash::from([79, 245, 143, 44, 118, 180, 172, 38, 253, 246, 117, 170, 130, 84, 30, 2, 228, 207, 137, 98, 121, 198, 214, 152, 45, 23, 185, 89, 120, 139, 47, 12]))
        .income_cell(Hash::from([8, 209, 205, 198, 171, 146, 217, 202, 190, 0, 150, 162, 199, 100, 47, 115, 208, 239, 27, 36, 201, 76, 67, 242, 28, 108, 58, 50, 255, 224, 187, 94]))
        .offer_cell(Hash::from([26, 63, 2, 170, 137, 101, 26, 24, 17, 47, 12, 33, 208, 174, 55, 10, 134, 225, 63, 106, 6, 12, 55, 129, 132, 205, 133, 154, 123, 182, 82, 3]))
        .pre_account_cell(Hash::from([108, 132, 65, 35, 63, 0, 116, 25, 85, 246, 94, 71, 103, 33, 161, 165, 65, 121, 151, 193, 228, 54, 136, 1, 201, 156, 127, 97, 127, 139, 117, 68]))
        .proposal_cell(Hash::from([103, 212, 140, 9, 17, 228, 6, 81, 141, 226, 17, 107, 217, 28, 106, 243, 124, 5, 241, 219, 35, 51, 76, 168, 41, 210, 175, 48, 66, 66, 126, 68]))
        .reverse_record_cell(Hash::from([97, 113, 20, 22, 70, 143, 165, 33, 30, 173, 95, 36, 198, 243, 239, 173, 251, 188, 51, 34, 116, 197, 212, 14, 80, 198, 254, 173, 203, 95, 150, 6]))
        .build();

    let das_lock_out_point_table = DasLockOutPointTable::new_builder()
        .ckb_signall(out_point!([32, 155, 53, 32, 141, 167, 210, 13, 136, 47, 8, 113, 243, 151, 156, 104, 197, 57, 129, 188, 196, 202, 167, 18, 116, 192, 53, 68, 144, 116, 208, 130], 0))
        // .ckb_multisign(out_point!([], 0))
        // .ckb_anyone_can_pay(out_point!([], 0))
        .eth(out_point!([143, 250, 64, 155, 160, 125, 116, 240, 143, 99, 192, 63, 130, 183, 66, 141, 54, 40, 95, 231, 91, 33, 115, 252, 36, 118, 192, 247, 184, 12, 112, 122], 0))
        .tron(out_point!([5, 67, 234, 58, 146, 251, 66, 127, 238, 92, 207, 179, 64, 103, 12, 29, 36, 235, 167, 141, 171, 253, 14, 189, 86, 115, 141, 252, 49, 36, 64, 94], 0))
        .ed25519(out_point!([167, 6, 244, 110, 88, 227, 85, 166, 210, 157, 115, 19, 245, 72, 173, 210, 27, 135, 86, 57, 234, 112, 96, 93, 24, 246, 130, 193, 160, 135, 64, 214], 0))
        .build();
    /* CAREFUL do not commit any changes for these configs above ⬆️ */

    let entity = ConfigCellMain::new_builder()
        .status(Uint8::from(SystemStatus::On as u8))
        .type_id_table(type_id_table)
        .das_lock_out_point_table(das_lock_out_point_table)
        .build();

    gen_return_from_entity!(DataType::ConfigCellMain, entity)
}

fn gen_config_cell_price() -> String {
    let discount = DiscountConfig::new_builder()
        .invited_discount(Uint32::from(500))
        .build();

    #[cfg(feature = "mainnet")]
    let prices = PriceConfigList::new_builder()
        .push(gen_price_config(1, 1024_000_000, 1024_000_000))
        .push(gen_price_config(2, 1024_000_000, 1024_000_000))
        .push(gen_price_config(3, 660_000_000, 660_000_000))
        .push(gen_price_config(4, 160_000_000, 160_000_000))
        .push(gen_price_config(5, 5_000_000, 5_000_000))
        .push(gen_price_config(6, 5_000_000, 5_000_000))
        .push(gen_price_config(7, 5_000_000, 5_000_000))
        .push(gen_price_config(8, 5_000_000, 5_000_000))
        .build();

    #[cfg(not(feature = "mainnet"))]
    let prices = PriceConfigList::new_builder()
        .push(gen_price_config(1, u64::MAX, u64::MAX))
        .push(gen_price_config(2, 30_000_000, 30_000_000))
        .push(gen_price_config(3, 20_000_000, 20_000_000))
        .push(gen_price_config(4, 10_000_000, 10_000_000))
        .push(gen_price_config(5, 5_000_000, 5_000_000))
        .push(gen_price_config(6, 5_000_000, 5_000_000))
        .push(gen_price_config(7, 5_000_000, 5_000_000))
        .push(gen_price_config(8, 5_000_000, 5_000_000))
        .build();

    let entity = ConfigCellPrice::new_builder().discount(discount).prices(prices).build();

    gen_return_from_entity!(DataType::ConfigCellPrice, entity)
}

fn gen_config_cell_proposal() -> String {
    let entity = ConfigCellProposal::new_builder()
        .proposal_min_confirm_interval(Uint8::from(0))
        .proposal_min_extend_interval(Uint8::from(1))
        .proposal_min_recycle_interval(Uint8::from(8))
        .proposal_max_account_affect(Uint32::from(50))
        .proposal_max_pre_account_contain(Uint32::from(50))
        .build();

    gen_return_from_entity!(DataType::ConfigCellProposal, entity)
}

fn gen_config_cell_profit_rate() -> String {
    let entity = ConfigCellProfitRate::new_builder()
        .channel(Uint32::from(1000))
        .inviter(Uint32::from(1000))
        .proposal_create(Uint32::from(200))
        .proposal_confirm(Uint32::from(0))
        .income_consolidate(Uint32::from(500))
        .sale_buyer_inviter(Uint32::from(100))
        .sale_buyer_channel(Uint32::from(150))
        .sale_das(Uint32::from(100))
        .auction_bidder_inviter(Uint32::from(100))
        .auction_bidder_channel(Uint32::from(100))
        .auction_das(Uint32::from(100))
        .auction_prev_bidder(Uint32::from(4700))
        .build();

    gen_return_from_entity!(DataType::ConfigCellProfitRate, entity)
}

fn gen_config_cell_record_key_namespace() -> String {
    let mut record_key_namespace = Vec::new();
    let lines = read_lines("record_key_namespace.txt").expect("Expect file ./data/record_key_namespace.txt exist.");
    for line in lines {
        if let Ok(key) = line {
            record_key_namespace.push(key);
        }
    }
    record_key_namespace.sort();
    // println!("record_key_namespace: \n{}", record_key_namespace.join("\n"));

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
    // Load and group preserved accounts
    let mut preserved_accounts_groups: Vec<Vec<Vec<u8>>> = vec![Vec::new(); PRESERVED_ACCOUNT_CELL_COUNT as usize];
    let lines = read_lines("reserved_accounts.txt").expect("Expect file ./data/reserved_accounts.txt exist.");
    for line in lines {
        if let Ok(account) = line {
            let account_hash = blake2b_256(account.as_bytes())
                .get(..ACCOUNT_ID_LENGTH)
                .unwrap()
                .to_vec();
            let index = (account_hash[0] % PRESERVED_ACCOUNT_CELL_COUNT) as usize;

            preserved_accounts_groups[index].push(account_hash);
        }
    }

    let mut output = String::new();
    let mut comma = "";
    for (_i, mut group) in preserved_accounts_groups.into_iter().enumerate() {
        // println!("Preserved account group[{}] count: {}", _i, group.len());
        if group.len() > PRESERVED_ACCOUNT_LIMIT_PER_CELL {
            panic!("Some ConfigCell of preserved accounts has broke the predict limitation.")
        }

        group.sort();
        let mut raw = group.into_iter().flatten().collect::<Vec<u8>>();
        raw = prepend_molecule_like_length(raw);

        let data_type = das_util::preserved_accounts_group_to_data_type(_i);
        output += comma;
        output += gen_return_from_raw!(data_type, raw).as_str();
        comma = ",";
    }

    output
}

fn gen_config_cell_char_set() -> String {
    let settings: Vec<(DataType, &str, u8)> = vec![
        (DataType::ConfigCellCharSetEmoji, "char_set_emoji.txt", 1),
        (DataType::ConfigCellCharSetDigit, "char_set_digit.txt", 1),
        (DataType::ConfigCellCharSetEn, "char_set_en.txt", 0),
        // (DataType::ConfigCellCharSetZhHans, "char_set_zh_hans.txt", 0),
        // (DataType::ConfigCellCharSetZhHant, "char_set_zh_hant.txt", 0),
    ];

    let mut output = String::new();
    let mut comma = "";
    for (_i, setting) in settings.iter().enumerate() {
        let mut charsets = Vec::new();
        let lines = read_lines(setting.1).expect(format!("Expect file ./data/{} exist.", setting.1).as_str());
        for line in lines {
            if let Ok(char) = line {
                charsets.push(char);
            }
        }

        // println!("Character count of {:?}: {}", setting.0, charsets.len());

        // Join all record keys with 0x00 byte as entity.
        let mut raw: Vec<u8> = Vec::new();
        raw.push(setting.2); // global status
        for key in charsets {
            raw.extend(key.as_bytes());
            raw.extend(&[0u8]);
        }
        let raw = prepend_molecule_like_length(raw);

        output += comma;
        output += gen_return_from_raw!(setting.0, raw).as_str();
        comma = ",";
    }

    output
}

fn gen_config_cell_release() -> String {
    // release to 40% = 1717986918
    // release to 45% = 1932735282
    // release to 50% = 2147483647
    // release to 55% = 2362232012
    // release to 60% = 2576980377

    #[cfg(feature = "mainnet")]
    let lucky_number = 1503238553;

    #[cfg(not(feature = "mainnet"))]
    let lucky_number = 2147483647;

    let entity = ConfigCellRelease::new_builder()
        .lucky_number(Uint32::from(lucky_number))
        .build();

    gen_return_from_entity!(DataType::ConfigCellRelease, entity)
}

fn gen_config_cell_secondary_market() -> String {
    // CAREFUL The minimum price should contains the basic_capacity of AccountCell.
    let entity = ConfigCellSecondaryMarket::new_builder()
        .common_fee(Uint64::from(10_000))
        // sale
        .sale_min_price(Uint64::from(20_000_000_000))
        .sale_expiration_limit(Uint32::from(86400 * 30))
        .sale_description_bytes_limit(Uint32::from(5000))
        .sale_cell_basic_capacity(Uint64::from(20_000_000_000))
        .sale_cell_prepared_fee_capacity(Uint64::from(100_000_000))
        // offser
        .offer_cell_basic_capacity(Uint64::from(20_000_000_000))
        .offer_cell_prepared_fee_capacity(Uint64::from(100_000_000))
        .offer_min_price(Uint64::from(100_000_000_000))
        .offer_message_bytes_limit(Uint32::from(5000))
        // auction
        .auction_max_extendable_duration(Uint32::from(86400 * 7))
        .auction_duration_increment_each_bid(Uint32::from(600))
        .auction_min_opening_price(Uint64::from(200_000_000_000))
        .auction_min_increment_rate_each_bid(Uint32::from(1000))
        .auction_description_bytes_limit(Uint32::from(5000))
        .auction_cell_basic_capacity(Uint64::from(20_000_000_000))
        .auction_cell_prepared_fee_capacity(Uint64::from(100_000_000))
        .build();

    gen_return_from_entity!(DataType::ConfigCellSecondaryMarket, entity)
}

fn gen_config_cell_reverse_resolution() -> String {
    let entity = ConfigCellReverseResolution::new_builder()
        .record_basic_capacity(Uint64::from(20_000_000_000))
        .record_prepared_fee_capacity(Uint64::from(100_000_000))
        .common_fee(Uint64::from(10_000))
        .build();

    gen_return_from_entity!(DataType::ConfigCellReverseResolution, entity)
}

// fn calc_config_cells_need_update() {
//     use std::collections::HashSet;
//
//     // Load and group preserved accounts
//     let lines =
//         read_lines("new_to_update.txt").expect("Expect file ./data/new_to_updated.txt exist.");
//
//     let mut id_set = HashSet::new();
//
//     for line in lines {
//         if let Ok(account) = line {
//             let account_hash = blake2b_256(account.as_bytes())
//                 .get(..ACCOUNT_ID_LENGTH)
//                 .unwrap()
//                 .to_vec();
//             let index = (account_hash[0] % PRESERVED_ACCOUNT_CELL_COUNT) as usize;
//             let key = hex_string(((10000 + index) as u32).to_le_bytes().as_ref()).unwrap();
//             println!("Because {} need to update 0x{}", account, key);
//
//             id_set.insert(key);
//         }
//     }
//
//     let mut id_vec = id_set.into_iter().collect::<Vec<_>>();
//     id_vec.sort();
//
//     println!();
//     println!("All ConfigCells which need to be updated:");
//     println!();
//     for key in id_vec {
//         println!("0x{}", key)
//     }
// }

/**
this function is nearly the same as the function in template_generator.rs under das-contracts repo.
**/
fn gen_config_cell_unavailable_account() -> String {
    let mut unavailable_account_hashes = Vec::new();
    let lines = util::read_lines("unavailable_account_hashes.txt")
        .expect("Expect file ./data/unavailable_account_hashes.txt exist.");

    for line in lines {
        if let Ok(account_hash_string) = line {
            let account_hash: Vec<u8> = hex::decode(account_hash_string).unwrap();
            unavailable_account_hashes.push(account_hash.get(..ACCOUNT_ID_LENGTH).unwrap().to_vec());
        }
    }

    unavailable_account_hashes.sort(); // todo: maybe we don't need to sort, traverse is just enough

    let mut raw = Vec::new();

    for account_hash in unavailable_account_hashes {
        raw.extend(account_hash);
    }
    let raw = util::prepend_molecule_like_length(raw);

    gen_return_from_raw!(DataType::ConfigCellUnAvailableAccount, raw)
}

fn main() {
    print!("{},", gen_config_cell_account());
    print!("{},", gen_config_cell_apply());
    print!("{},", gen_config_cell_income());
    print!("{},", gen_config_cell_main());
    print!("{},", gen_config_cell_price());
    print!("{},", gen_config_cell_proposal());
    print!("{},", gen_config_cell_profit_rate());
    print!("{},", gen_config_cell_record_key_namespace());
    print!("{},", gen_config_cell_release());
    print!("{},", gen_config_cell_secondary_market());
    print!("{},", gen_config_cell_reverse_resolution());
    print!("{},", gen_config_cell_reserved_account());
    print!("{},", gen_config_cell_unavailable_account());
    print!("{}", gen_config_cell_char_set());
    print!("\n");
}
