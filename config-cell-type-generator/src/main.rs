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
            162, 195, 162, 177, 141, 168, 151, 189, 36, 57, 26, 146, 25, 86, 228, 93, 36, 91, 70,
            22, 157, 106, 204, 154, 6, 99, 49, 109, 21, 181, 28, 177,
        ]))
        .pre_account_cell(Hash::from([
            146, 214, 169, 82, 91, 154, 5, 66, 34, 152, 42, 180, 116, 11, 230, 254, 66, 129, 230,
            95, 255, 82, 171, 37, 46, 125, 175, 147, 6, 225, 46, 63,
        ]))
        .proposal_cell(Hash::from([
            65, 84, 181, 249, 17, 75, 141, 45, 216, 50, 62, 234, 213, 213, 231, 29, 9, 89, 162,
            220, 115, 240, 103, 46, 130, 154, 228, 218, 191, 253, 178, 216,
        ]))
        .ref_cell(Hash::from([
            231, 153, 83, 240, 36, 85, 46, 97, 48, 34, 10, 3, 210, 73, 125, 199, 194, 247, 132,
            244, 41, 124, 105, 186, 33, 208, 196, 35, 145, 83, 80, 229,
        ]))
        .account_cell(Hash::from([
            39, 71, 117, 228, 117, 193, 37, 43, 83, 51, 194, 14, 21, 18, 183, 177, 41, 108, 76, 91,
            82, 162, 90, 162, 235, 214, 228, 31, 88, 148, 196, 31,
        ]))
        .on_sale_cell(Hash::default())
        .bidding_cell(Hash::default())
        .primary_market_cell(Hash::default())
        .wallet_cell(Hash::from([
            152, 120, 178, 38, 223, 148, 101, 194, 21, 253, 60, 148, 220, 159, 155, 246, 100, 141,
            91, 234, 72, 162, 69, 121, 207, 131, 39, 79, 225, 56, 1, 210,
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
                "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
            ],
        ))
        .build();

    let profit_config = ProfitConfig::new_builder()
        .profit_rate_of_channel(Uint32::from(1000))
        .profit_rate_of_inviter(Uint32::from(1000))
        .profit_rate_of_das(Uint32::from(8000))
        .profit_rate_of_proposal_create(Uint32::from(400))
        .profit_rate_of_proposal_confirm(Uint32::from(100))
        .build();

    let discount_config = DiscountConfig::new_builder()
        .invited_discount(Uint32::from(500))
        .build();

    let entity = ConfigCellRegister::new_builder()
        .apply_min_waiting_block_number(Uint32::from(1))
        .apply_max_waiting_block_number(Uint32::from(5760))
        .account_max_length(Uint32::from(1000))
        .char_sets(char_sets)
        .price_configs(price_config)
        .proposal_min_confirm_interval(Uint8::from(2))
        .proposal_min_extend_interval(Uint8::from(1))
        .proposal_min_recycle_interval(Uint8::from(4))
        .proposal_max_account_affect(Uint32::from(50))
        .proposal_max_pre_account_contain(Uint32::from(50))
        .profit(profit_config)
        .discount(discount_config)
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
