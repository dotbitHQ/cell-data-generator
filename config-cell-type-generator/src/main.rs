use bloom_filter::BloomFilter;
use ckb_hash::blake2b_256;
use das_types::{constants::*, packed::*, prelude::*, util as das_util};
use faster_hex::hex_string;
use util::{gen_char_set, gen_price_config};

mod bloom_filter;
mod util;

fn gen_config_cell_main() -> String {
    // ⚠️ Do not modify the following lines of type_id_table,
    // it will be use for search and replace in deploy scripts.
    let type_id_table = TypeIdTable::new_builder()
        .apply_register_cell(Hash::from([
            151, 232, 245, 67, 207, 94, 104, 93, 140, 106, 205, 34, 195, 137, 22, 177, 84, 83, 203,
            107, 108, 24, 91, 62, 60, 58, 67, 141, 47, 29, 151, 181,
        ]))
        .pre_account_cell(Hash::from([
            29, 90, 249, 224, 198, 31, 27, 67, 244, 147, 134, 69, 50, 108, 55, 158, 74, 19, 4, 131,
            14, 248, 123, 43, 73, 239, 244, 182, 175, 78, 212, 172,
        ]))
        .proposal_cell(Hash::from([
            119, 173, 236, 90, 225, 19, 113, 49, 87, 37, 216, 42, 51, 151, 65, 218, 170, 138, 146,
            40, 52, 144, 69, 173, 254, 36, 56, 114, 166, 127, 235, 224,
        ]))
        .ref_cell(Hash::from([
            109, 86, 167, 36, 157, 34, 188, 249, 232, 156, 100, 62, 83, 246, 90, 24, 130, 56, 62,
            45, 251, 165, 182, 81, 15, 195, 66, 87, 217, 255, 74, 190,
        ]))
        .account_cell(Hash::from([
            44, 37, 101, 241, 35, 174, 206, 48, 63, 50, 153, 35, 255, 208, 103, 110, 202, 175, 100,
            219, 33, 73, 77, 15, 15, 18, 212, 36, 252, 249, 183, 186,
        ]))
        .on_sale_cell(Hash::default())
        .bidding_cell(Hash::default())
        .primary_market_cell(Hash::default())
        .wallet_cell(Hash::from([
            150, 217, 93, 216, 230, 127, 183, 61, 104, 248, 138, 192, 3, 209, 216, 227, 118, 196,
            36, 152, 21, 116, 247, 242, 48, 71, 115, 63, 53, 150, 202, 136,
        ]))
        .build();

    let entity = ConfigCellMain::new_builder()
        .account_expiration_grace_period(Uint32::from(2_592_000)) // 30 days
        .min_ttl(Uint32::from(300))
        .type_id_table(type_id_table)
        .build();

    let config_id = (ConfigID::ConfigCellMain as u32).to_le_bytes();
    let cell_data = Bytes::from(blake2b_256(entity.as_slice()).to_vec());
    let action_witness = das_util::wrap_action_witness("config", None);
    let cell_witness = das_util::wrap_entity_witness(DataType::ConfigCellMain, entity);

    format!(
        "0x{} 0x{} 0x{} 0x{}",
        hex_string(&config_id).unwrap(),
        hex_string(cell_data.as_reader().raw_data()).unwrap(),
        hex_string(action_witness.as_reader().raw_data()).unwrap(),
        hex_string(cell_witness.as_reader().raw_data()).unwrap(),
    )
}

fn gen_config_cell_register() -> String {
    let price_config = PriceConfigList::new_builder()
        .push(gen_price_config(1, 12_000_000, 1_200_000))
        .push(gen_price_config(2, 11_000_000, 1_100_000))
        .push(gen_price_config(3, 10_000_000, 1_000_000))
        .push(gen_price_config(4, 9_000_000, 900_000))
        .push(gen_price_config(5, 8_000_000, 800_000))
        .push(gen_price_config(6, 7_000_000, 700_000))
        .push(gen_price_config(7, 6_000_000, 600_000))
        .push(gen_price_config(8, 5_000_000, 500_000))
        .build();

    let char_sets = CharSetList::new_builder()
        .push(gen_char_set(CharSetType::Emoji, 1, vec!["😂", "👍", "✨"]))
        .push(gen_char_set(
            CharSetType::Digit,
            1,
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
        ))
        .push(gen_char_set(
            CharSetType::En,
            0,
            vec![
                "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
                "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F",
                "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V",
                "W", "X", "Y", "Z",
            ],
        ))
        .build();

    let profit_config = ProfitConfig::new_builder()
        .profit_rate_of_channel(Uint32::from(1000))
        .profit_rate_of_inviter(Uint32::from(1000))
        .profit_rate_of_das(Uint32::from(8000))
        .build();

    let entity = ConfigCellRegister::new_builder()
        .apply_min_waiting_block_number(Uint32::from(4))
        .apply_max_waiting_block_number(Uint32::from(5760))
        .account_max_length(Uint32::from(1000))
        .char_sets(char_sets)
        .price_configs(price_config)
        .proposal_min_confirm_interval(Uint8::from(8))
        .proposal_min_extend_interval(Uint8::from(4))
        .proposal_min_recycle_interval(Uint8::from(16))
        .proposal_max_account_affect(Uint32::from(50))
        .proposal_max_pre_account_contain(Uint32::from(50))
        .profit(profit_config)
        .build();

    let config_id = (ConfigID::ConfigCellRegister as u32).to_le_bytes();
    let cell_data = Bytes::from(blake2b_256(entity.as_slice()).to_vec());
    let action_witness = das_util::wrap_action_witness("config", None);
    let cell_witness = das_util::wrap_entity_witness(DataType::ConfigCellRegister, entity);

    format!(
        "0x{} 0x{} 0x{} 0x{}",
        hex_string(&config_id).unwrap(),
        hex_string(cell_data.as_reader().raw_data()).unwrap(),
        hex_string(action_witness.as_reader().raw_data()).unwrap(),
        hex_string(cell_witness.as_reader().raw_data()).unwrap(),
    )
}

fn gen_config_cell_bloom_filter() -> String {
    let mut bf = BloomFilter::new(1438, 10);
    bf.insert(b"google");
    bf.insert(b"apple");
    bf.insert(b"microsoft");
    bf.insert(b"qq");
    bf.insert(b"ali");
    bf.insert(b"baidu");
    bf.insert(b"das00001");
    bf.insert(b"das00002");
    bf.insert(b"das00003");
    bf.insert(b"das");
    let mut entity = bf.export_bit_u8();

    let mut length = (entity.len() as u32 + 4).to_le_bytes().to_vec();
    length.extend(entity);
    entity = length;

    let config_id = (ConfigID::ConfigCellBloomFilter as u32).to_le_bytes();
    let cell_data = Bytes::from(blake2b_256(entity.as_slice()).to_vec());
    let action_witness = das_util::wrap_action_witness("config", None);
    let cell_witness = das_util::wrap_raw_witness(DataType::ConfigCellBloomFilter, entity);

    format!(
        "0x{} 0x{} 0x{} 0x{}",
        hex_string(&config_id).unwrap(),
        hex_string(cell_data.as_reader().raw_data()).unwrap(),
        hex_string(action_witness.as_reader().raw_data()).unwrap(),
        hex_string(cell_witness.as_reader().raw_data()).unwrap(),
    )
}

fn gen_config_cell_market() -> String {
    let primary_market_config = MarketConfig::new_builder()
        .max_auction_waiting(Uint32::from(86400))
        .min_auction_raise_rate(Uint32::from(1000))
        .build();

    let secondary_market_config = MarketConfig::new_builder()
        .max_auction_time(Uint32::from(2_592_000))
        .max_auction_waiting(Uint32::from(86400))
        .max_selling_time(Uint32::from(2_592_000))
        .min_auction_raise_rate(Uint32::from(1000))
        .build();

    let entity = ConfigCellMarket::new_builder()
        .primary_market(primary_market_config)
        .secondary_market(secondary_market_config)
        .build();

    let config_id = (ConfigID::ConfigCellMarket as u32).to_le_bytes();
    let cell_data = Bytes::from(blake2b_256(entity.as_slice()).to_vec());
    let action_witness = das_util::wrap_action_witness("config", None);
    let cell_witness = das_util::wrap_entity_witness(DataType::ConfigCellMarket, entity);

    format!(
        "0x{} 0x{} 0x{} 0x{}",
        hex_string(&config_id).unwrap(),
        hex_string(cell_data.as_reader().raw_data()).unwrap(),
        hex_string(action_witness.as_reader().raw_data()).unwrap(),
        hex_string(cell_witness.as_reader().raw_data()).unwrap(),
    )
}

fn main() {
    let config1 = gen_config_cell_main();
    let config2 = gen_config_cell_register();
    let config3 = gen_config_cell_bloom_filter();
    let config4 = gen_config_cell_market();

    println!("{},{},{},{}", config1, config2, config3, config4);
}
