use das_types::util as das_util;
use faster_hex::hex_string;

fn main() {
    let action_witness = das_util::wrap_action_witness("create_wallet", None);

    println!(
        "0xb7526803f67ebe70aba6 0x{}  ",
        hex_string(action_witness.as_reader().raw_data()).unwrap()
    );
}
