use std::process::exit;
use clap::{Parser};

use internal::utils;
use crate::args::Operation;

mod args;
mod internal;
mod operations;

#[tokio::main]
async fn main() {

    if !utils::is_run_with_root() {
        println!("This program must be run with root privileges.");
        exit(1)
    }

    let args = args::Args::parse();

    match args.subcommand {
        Some(Operation::ToggleSplashScreen) => {
            operations::splash_screen_toggle().await
        }
        Some(Operation::ToggleBootMenu) => {
            operations::grub_menu_toggle().await
        }
        Some(Operation::ChangeHostName(args)) => {
            // println!("{}", args.hostname); // debug
            operations::change_hostname(&args.hostname)
        }
        Some(Operation::Network(args)) => {
            operations::networking(&args)
        }
        Some(Operation::DeviceInfo) => {
            operations::device_info()
        }
        Some(Operation::LoadStyleConfig) => {
            operations::load_style_config()
        }
        Some(Operation::ApplyDefaultSleexKeybinds) => {
            operations::apply_default_sleex_keybinds();
        }
        Some(Operation::UpdateRelases) => {
            operations::update_releases();
        }
        None => {
            println!("No subcommand provided. Use --help for more information.");
        }
    }
}