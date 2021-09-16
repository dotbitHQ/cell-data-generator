use clap::Clap;
use regex::Regex;
use std::fs;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Link Xie. <xieaolin@gmail.com>")]
struct Options {
    #[clap(
        short = 'i',
        long = "input",
        about = "Input directory for collecting reserved accounts."
    )]
    input: Option<String>,
    #[clap(
        short = 'o',
        long = "output",
        about = "Output file for generating a single file contains all reserved accounts."
    )]
    output: Option<String>,
}

fn main() {
    let options: Options = Options::parse();
    // println!("{:?}", options);

    let input = if options.input.is_some() {
        options.input.unwrap()
    } else {
        String::from("./raw-reserved-accounts")
    };
    let output = if options.output.is_some() {
        options.output.unwrap()
    } else {
        String::from("./data/reserved_accounts.txt")
    };

    let account_re = Regex::new(r"([^\s]+\.bit)").unwrap();
    let mut reserved_accounts = Vec::new();
    for item in fs::read_dir(&input).expect(format!("{} should be a directory.", input).as_str()) {
        if let Ok(file) = item {
            let data = fs::read_to_string(file.path())
                .expect(format!("{} should be a readable file.", file.path().to_str().unwrap()).as_str());
            let mut accounts = account_re
                .find_iter(&data)
                .map(|item| item.as_str().trim_end_matches(".bit").to_ascii_lowercase())
                .collect::<Vec<_>>();
            reserved_accounts.append(&mut accounts);
        }
    }

    reserved_accounts.sort();
    reserved_accounts.dedup();

    fs::write(&output, reserved_accounts.join("\n")).expect(format!("{} should be a writable file.", output).as_str());

    println!("Done ✅");
}
