use clap::{ArgAction, Parser, Subcommand, ValueHint};

static VERSION : &str = concat!(env!("CARGO_PKG_VERSION"));

#[derive(Debug, Clone, Parser)]
#[command(bin_name = "axctl", version = VERSION, about = "System control utility for AxOS", infer_subcommands = true)]
pub struct Args {
    
    #[command(subcommand)]
    pub subcommand: Option<Operation>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Operation {
    
    #[command(bin_name = "axctl", name = "toggle-splash-screen", aliases = ["splash"], about = "Toggle the boot splash screen on or off")]
    ToggleSplashScreen,

    #[command(bin_name = "axctl", name = "toggle-boot-menu", aliases = ["boot-menu"], about = "Toggle the boot menu on or off")]
    ToggleBootMenu,
}

#[derive(Default, Debug, Clone, Parser)]
pub struct ToggleSplashScreenArgs {
    
    #[arg(help = "Enable or disable the splash screen")]
    pub enable: Option<bool>,
}

#[derive(Default, Debug, Clone, Parser)]
pub struct ToggleBootMenuArgs {
    
    #[arg(help = "Enable or disable the boot menu")]
    pub enable: Option<bool>,
}

