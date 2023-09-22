pub use crate::osx::setup_osx;
pub use crate::archlinux::setup_arch_linux;
pub mod osx;
pub mod archlinux;
pub mod windows;
pub mod ubuntu;


#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

use argh::FromArgs;
use windows::setup_windows;
use ubuntu::setup_ubuntu;

#[derive(FromArgs)]

/// Franky is a tool to setup your system
struct Cli {
    #[argh(subcommand)]
    setup: Setup,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Setup {
    Osx(Osx),
    Archlinux(Archlinux),
    Windows(Windows),
    Ubuntu(Ubuntu),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "osx")]
/// setup Mac OSX

struct Osx {
    #[argh(switch)]
        /// Whether to run the setup.
        #[argh(description = "whether to run the setup.")]
        setup: bool,
    }

    #[derive(FromArgs)]
    #[argh(subcommand, name = "archlinux")]
    /// setup Arch Linux
    #[argh(description = "Setup Arch Linux.")]
    struct Archlinux {
        #[argh(switch)]
        #[argh(description = "whether to run the setup.")]
        setup: bool,
    }

    #[derive(FromArgs)]
    #[argh(subcommand, name = "windows")]
    /// setup Windows
    #[argh(description = "Setup Windows.")]
    struct Windows {
        #[argh(switch)]
        #[argh(description = "whether to run the setup.")]
        setup: bool,
    }

    #[derive(FromArgs)]
    #[argh(subcommand, name = "ubuntu")]
    /// setup Ubuntu
    #[argh(description = "Setup Ubuntu.")]
    struct Ubuntu {
        #[argh(switch)]
        #[argh(description = "whether to run the setup.")]
        setup: bool,
    }


#[tokio::main]
async fn main() {

    match start().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

}

// check os and run the appropriate setup function
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    println!("Franky is a tool to setup your system");
    let cli: Cli = argh::from_env();
    match cli.setup {
        Setup::Osx(osx) => {
            if osx.setup {
                setup_osx().await;
            }
        }
        Setup::Archlinux(archlinux) => {
            if archlinux.setup {
                setup_arch_linux().await;
            }
        }
        Setup::Windows(windows) => {
            if windows.setup {
                setup_windows().await;
            }
        }
        Setup::Ubuntu(ubuntu) => {
            if ubuntu.setup {
                println!("Setting up Ubuntu");
                setup_ubuntu().await;
            }
        }
    }
    Ok(())
}