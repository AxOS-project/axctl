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

    #[command(bin_name = "axctl", name = "device-info", aliases = ["info"], about = "Display device information")]
    DeviceInfo,

    #[command(bin_name = "axctl", name = "load-style-config", aliases = ["load-style"], about = "Load the style configurations (this may overwrite existing configurations)")]
    LoadStyleConfig,

    #[command(bin_name = "axctl", name = "load-sleex-user-config", aliases = ["load-sleex-uc"], about = "Load the configurations for sleex (this may overwrite existing configurations)")]
    LoadSleexUserConfig,

    #[command(bin_name = "axctl", name = "update-releases", about = "Updates the OS version in /etc/lsb-release and /etc/os-release")]
    UpdateRelases
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

#[derive(Debug, Clone, Parser)]
pub struct DeviceInfo;