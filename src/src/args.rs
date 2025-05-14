use clap::{Parser, Subcommand};

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

    #[command(bin_name = "axctl", name = "change-hostname", aliases = ["hostname"], about = "Change the device hostname")]
    ChangeHostName(ChangeHostName),

    #[command(bin_name = "axctl", name = "net", aliases = ["network"], about = "Basic networking commands")]
    Network(SimpleNetworking),
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

#[derive(Default, Debug, Clone, Parser)]
pub struct ChangeHostName {
    #[arg(help = "Change the hostname")]
    pub hostname: String,
}

#[derive(Debug, Clone, Subcommand)]
pub enum NetworkOperation {
    #[command(about = "Show IP and interface status")]
    Status,

    #[command(about = "Restart networking services")]
    Restart,

    #[command(about = "Run a basic network diagnostic")]
    Test,
}

#[derive(Debug, Clone, Parser)]
pub struct SimpleNetworking {
    #[command(subcommand)]
    pub action: NetworkOperation,
}
