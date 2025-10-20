mod grub_menu_toggle;
mod splash_screen_toggle;
mod change_hostname;
mod networking;
mod device_info;
mod load_style_config;
mod apply_default_sleex_keybinds;
mod update_releases;

pub use grub_menu_toggle::grub_menu_toggle;
pub use splash_screen_toggle::splash_screen_toggle;
pub use change_hostname::change_hostname;
pub use networking::networking;
pub use device_info::device_info;
pub use load_style_config::load_style_config;
pub use apply_default_sleex_keybinds::apply_default_sleex_keybinds;
pub use update_releases::update_releases;