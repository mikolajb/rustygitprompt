use clap::ArgEnum;
use std::sync::Mutex;

// emulated global variable for our settings
lazy_static! {
    pub static ref SETTINGS: Mutex<Settings> = Mutex::new(Settings {
        icon_list_format: IconListFormat::Digits,
        shell: Shell::None,
        master_branch_label: String::from("m"),
    });
}

#[derive(Debug)]
pub struct Settings {
    pub icon_list_format: IconListFormat,
    pub shell: Shell,
    pub master_branch_label: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum IconListFormat {
    Superscript,
    Subscript,
    Digits,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Shell {
    None,
    Zsh,
    ANSI,
}
