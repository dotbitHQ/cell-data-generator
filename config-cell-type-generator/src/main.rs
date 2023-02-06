#![allow(dead_code)]

use ckb_hash::blake2b_256;
use das_types_std::{constants::*, packed::*, prelude::*, util as das_util};
use faster_hex::hex_string;
use util::{gen_price_config, prepend_molecule_like_length, read_lines};

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
            hex_string(&config_type),
            hex_string(cell_data.as_reader().raw_data()),
            hex_string(action_witness.as_reader().raw_data()),
            hex_string(cell_witness.as_reader().raw_data()),
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
            hex_string(&config_type),
            hex_string(cell_data.as_reader().raw_data()),
            hex_string(action_witness.as_reader().raw_data()),
            hex_string(cell_witness.as_reader().raw_data()),
        )
    }};
}

fn gen_config_cell_account() -> String {
    let entity = ConfigCellAccount::new_builder()
        .max_length(Uint32::from(42))
        // The basic_capacity contains 1 CKB for kinds of fees
        .basic_capacity(Uint64::from(20_600_000_000))
        .prepared_fee_capacity(Uint64::from(100_000_000))
        .expiration_grace_period(Uint32::from(7_776_000))
        .record_min_ttl(Uint32::from(300))
        .record_size_limit(Uint32::from(5000))
        .transfer_account_fee(Uint64::from(20_000))
        .edit_manager_fee(Uint64::from(20_000))
        .edit_records_fee(Uint64::from(20_000))
        .common_fee(Uint64::from(20_000))
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
        .account_cell(Hash::from([79,23,10,4,129,152,64,143,79,77,54,189,188,221,206,190,122,10,232,82,68,211,171,8,253,64,168,12,191,199,9,24]))
        .account_sale_cell(Hash::from([128, 245,  32, 163, 121, 196,  28,  1, 154, 181, 106, 253,  66, 107,  83, 97, 117, 191, 249, 197, 116, 177, 117, 36, 218, 129, 210, 216,  47,  63, 183, 55]))
        // .account_auction_cell(Hash::from([]))
        .apply_register_cell(Hash::from([192, 36, 182, 239, 222, 141, 73, 175, 102, 91, 50, 69, 34, 58, 138, 168, 137, 227, 94, 222, 21, 188, 81, 3, 146, 167, 254, 162, 222, 192, 167, 88]))
        .balance_cell(Hash::from([235, 175, 193, 235, 233, 91, 136, 202, 196, 38, 249, 132, 237, 95, 206, 153, 128, 137, 236, 173, 12, 210, 248, 177, 119, 85, 201, 222, 76, 176, 33, 98]))
        .income_cell(Hash::from([108, 29, 105, 163, 88, 149, 79, 196, 113, 162, 255, 168, 42, 152, 174, 213, 164, 145, 46, 96, 2, 165, 231, 97, 82, 79, 35, 4, 171, 83, 191, 57]))
        .offer_cell(Hash::from([17, 0, 176, 13, 37, 221, 95, 25, 49, 139, 144, 52, 165, 226, 67, 150, 114, 232, 70, 2, 26, 209, 236, 11, 203, 25, 119, 83, 32, 253, 47, 33]))
        .pre_account_cell(Hash::from([24, 171, 135, 20, 126, 142, 129, 0, 10, 177, 185, 243, 25, 165, 120, 77, 76, 123, 108, 152, 169, 206, 201, 125, 115, 138, 92, 17, 246, 158, 114, 84]))
        .proposal_cell(Hash::from([97, 39, 164, 26, 208, 84, 158, 133, 116, 162, 91, 77, 135, 167, 65, 79, 30, 32, 87, 147, 6, 201, 67, 197, 63, 254, 125, 3, 243, 133, 155, 190]))
        .reverse_record_cell(Hash::from([235, 201, 225, 54, 88, 246, 223, 19, 89, 60, 245, 155, 126, 156, 209, 89, 96, 43, 108, 60, 125, 84, 177, 77, 234, 67, 186, 230, 0, 235, 174, 17]))
        .sub_account_cell(Hash::from([99, 81, 109, 232, 187, 81, 142, 209, 34, 94, 59, 99, 241, 56, 204, 190, 24, 228, 23, 147, 45, 36, 15, 19, 39, 200, 232, 107, 163, 39, 244, 180]))
        .eip712_lib(Hash::from([143, 130, 57, 130, 148, 121, 34, 122, 157, 213, 140, 199, 61, 163, 119, 191, 224, 135, 149, 137, 174, 178, 105, 205, 246, 23, 112, 147, 253, 179, 99, 137]))
        .build();

    let das_lock_out_point_table = DasLockOutPointTable::new_builder()
        .ckb_signall(out_point!([19, 115, 219, 137, 253, 44, 127, 241, 97, 125, 79, 214, 116, 14, 145, 97, 105, 99, 28, 90, 182, 201, 153, 87, 134, 100, 80, 113, 171, 25, 184, 34], 0))
        .ckb_multisign(out_point!([151, 2, 140, 83, 209, 127, 153, 145, 157, 150, 17, 36, 177, 104, 94, 120, 113, 80, 227, 206, 199, 53, 175, 209, 213, 230, 244, 248, 110, 194, 171, 34], 0))
        // .ckb_anyone_can_pay(out_point!([], 0))
        .eth(out_point!([57, 212, 171, 98, 2, 91, 34, 128, 89, 60, 134, 192, 251, 43, 126, 53, 124, 125, 71, 146, 148, 165, 37, 248, 242, 108, 2, 50, 155, 206, 133, 95], 0))
        .tron(out_point!([78, 108, 152, 20, 154, 24, 196, 215, 228, 82, 86, 6, 203, 47, 20, 171, 210, 178, 51, 190, 100, 52, 13, 42, 196, 131, 65, 93, 89, 91, 166, 131], 0))
        .ed25519(out_point!([151, 137, 4, 158, 212, 76, 218, 247, 96, 29, 26, 83, 162, 47, 106, 93, 176, 194, 212, 117, 3, 251, 154, 147, 0, 87, 21, 65, 210, 244, 161, 40], 0))
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
        .push(gen_price_config(1, 1_000_000, 1_000_000))
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
        .push(gen_price_config(1, 1_000_000, 1_000_000))
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
        (DataType::ConfigCellCharSetDigit, "char_set_digit_and_symbol.txt", 1),
        (DataType::ConfigCellCharSetEn, "char_set_en.txt", 0),
        // (DataType::ConfigCellCharSetZhHans, "char_set_zh_hans.txt", 0),
        // (DataType::ConfigCellCharSetZhHant, "char_set_zh_hant.txt", 0),
        (DataType::ConfigCellCharSetJa, "char_set_ja.txt", 0),
        (DataType::ConfigCellCharSetKo, "char_set_ko.txt", 0),
        // (DataType::ConfigCellCharSetRu, "char_set_ru.txt", 0),
        (DataType::ConfigCellCharSetTr, "char_set_tr.txt", 0),
        (DataType::ConfigCellCharSetTh, "char_set_th.txt", 0),
        (DataType::ConfigCellCharSetVi, "char_set_vi.txt", 0),

    ];

    let mut output = String::new();
    let mut comma = "";
    // let mut dedup_chars = Vec::new();
    for (_i, setting) in settings.iter().enumerate() {
        let mut charsets = Vec::new();
        let lines = read_lines(setting.1).expect(format!("Expect file ./data/{} exist.", setting.1).as_str());
        for line in lines {
            if let Ok(char) = line {
                let cleared_char = char.trim().to_string();
                if cleared_char.is_empty() {
                    continue;
                }
                if cleared_char.as_bytes().contains(&0u8) {
                    // CAREFUL! Characters which contains 0x00 are not allowed, so it exists warn the developer to review the config file.
                    panic!("File {} character {} contains 0x00 byte.", setting.1, cleared_char);
                }
                // if dedup_chars.contains(&char) {
                //     println!("{} find duplicated char: {} 0x{}", setting.1, char, hex::encode(char.as_bytes()));
                // } else {
                //     dedup_chars.push(char.clone());
                // }
                charsets.push(cleared_char);
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

///0x6d000000
fn gen_config_cell_release() -> String {
    // release to 40% = 1717986918
    // release to 45% = 1932735282
    // release to 50% = 2147483647
    // release to 55% = 2362232012
    // release to 60% = 2576980377

    #[cfg(feature = "mainnet")]
    let lucky_number = 2576980377;

    #[cfg(not(feature = "mainnet"))]
    let lucky_number = 2576980377;

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

fn gen_config_cell_sub_account() -> String {
    let entity = ConfigCellSubAccount::new_builder()
        .basic_capacity(Uint64::from(20_000_000_000))
        .prepared_fee_capacity(Uint64::from(2_000_000_000))
        .new_sub_account_price(Uint64::from(100_000_000))
        .renew_sub_account_price(Uint64::from(100_000_000))
        .new_sub_account_custom_price_das_profit_rate(Uint32::from(300))
        .renew_sub_account_custom_price_das_profit_rate(Uint32::from(300))
        .common_fee(Uint64::from(300_000))
        .create_fee(Uint64::from(0))
        .edit_fee(Uint64::from(0))
        .renew_fee(Uint64::from(0))
        .recycle_fee(Uint64::from(0))
        .build();

    gen_return_from_entity!(DataType::ConfigCellSubAccount, entity)
}

fn gen_config_cell_sub_account_beta_list() -> String {
    // If there is only the 0xd83bc404a35ee0c4c2055d5ac13a5c323aae494a is ConfigCell, it means the beta is end.
    let mut sub_account_beta_list = Vec::new();
    let lines = util::read_lines("sub_account_beta_list.txt")
        .expect("Expect file ./data/sub_account_beta_list.txt exist.");

    for line in lines {
        if let Ok(account) = line {
            let account_hash = blake2b_256(account.as_bytes())
                .get(..ACCOUNT_ID_LENGTH)
                .unwrap()
                .to_vec();

            sub_account_beta_list.push(account_hash);
        }
    }

    sub_account_beta_list.sort();

    let mut raw = sub_account_beta_list.into_iter().flatten().collect::<Vec<u8>>();
    raw = util::prepend_molecule_like_length(raw);

    gen_return_from_raw!(DataType::ConfigCellSubAccountBetaList, raw)
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

    let mut raw = unavailable_account_hashes.into_iter().flatten().collect::<Vec<u8>>();
    raw = util::prepend_molecule_like_length(raw);

    gen_return_from_raw!(DataType::ConfigCellUnAvailableAccount, raw)
    }

fn gen_config_cell_system_status() -> String {
    let entity = ConfigCellSystemStatus::new_builder()
        .apply_register_cell_type(ContractStatus::new(true, "1.1.0"))
        .pre_account_cell_type(ContractStatus::new(true, "1.4.0"))
        .proposal_cell_type(ContractStatus::new(true, "1.3.0"))
        .config_cell_type(ContractStatus::new(true, "1.1.2"))
        .account_cell_type(ContractStatus::new(true, "1.7.0"))
        .account_sale_cell_type(ContractStatus::new(true, "1.1.1"))
        .sub_account_cell_type(ContractStatus::new(true, "1.3.0"))
        .offer_cell_type(ContractStatus::new(true, "1.0.1"))
        .balance_cell_type(ContractStatus::new(true, "1.3.0"))
        .income_cell_type(ContractStatus::new(true, "1.2.1"))
        .reverse_record_cell_type(ContractStatus::new(true, "1.0.1"))
        .eip712_lib(ContractStatus::new(true, "1.0.0"))
        .build();

    gen_return_from_entity!(DataType::ConfigCellSystemStatus, entity)
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
    print!("{},", gen_config_cell_sub_account());
    print!("{},", gen_config_cell_sub_account_beta_list());
    print!("{},", gen_config_cell_reserved_account());
    print!("{},", gen_config_cell_unavailable_account());
    print!("{},", gen_config_cell_system_status());
    print!("{}", gen_config_cell_char_set());
    print!("\n");
}
