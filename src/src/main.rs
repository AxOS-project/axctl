use clap::{Parser};

use internal::utils;
use crate::args::Operation;

mod args;
mod internal;
mod operations;

#[tokio::main]
async fn main() {

    let args = args::Args::parse();

    match args.subcommand {
        Some(Operation::ToggleSplashScreen) => {
            utils::run_with_root(|| {
                operations::splash_screen_toggle()
            });
        }
        Some(Operation::ToggleBootMenu) => {
            utils::run_with_root(|| {
                operations::grub_menu_toggle()
            });
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
        Some(Operation::UpdateReleases) => {
            utils::run_with_root(|| {
                operations::update_releases()
            });
        }
        Some(Operation::Dpms(args)) => {
            operations::dpms_control(&args);
        }
        None => {
            println!("No subcommand provided. Use --help for more information.");
        }
    }
}