extern crate clap;
use clap::{crate_authors, crate_version, App, Arg};
pub use crate::osx::{ setup_osx };
pub use crate::archlinux::{ setup_arch_linux };
pub mod osx;
pub mod archlinux;

fn main() {
    let matches = App::new("Franky")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Setup things for you")
        .arg(
            Arg::with_name("setup")
                .short("s")
                .long("setup")
                .value_name("PROGRAM")
                .help("Sets a custom program")
                .required(true)
                .default_value("osx")
                .takes_value(true),
        )
        .get_matches();
        let setup = matches.value_of("setup").unwrap();
        match setup {
                "osx" => setup_osx(),
                "archlinux" => setup_arch_linux(),
                _ => println!("Not Supported")
        }

}
