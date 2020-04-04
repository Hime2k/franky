extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Franky")
        .version("0.1.0")
        .author("Rupa Ghimire <rupaghimire14@gmail.com>")
        .about("Setup things for you")
        .arg(
            Arg::with_name("setup")
                .short("s")
                .long("setup")
                .value_name("PROGRAM")
                .help("Sets a custom program")
                .takes_value(true),
        )
        .get_matches();
}
