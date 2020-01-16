use std::str::FromStr;
use std::sync::Mutex;

// emulated global variable for our settings
lazy_static! {
    pub static ref SETTINGS: Mutex<Settings> = Mutex::new(Settings{
        icon_list_format: IconListFormat::Digits,
        shell: Shell::None,
    });
}

#[derive(Debug)]
pub struct Settings {
    pub icon_list_format: IconListFormat,
    pub shell: Shell,
}

#[derive(Debug)]
pub enum IconListFormat {
    Superscript,
    Subscript,
    Digits,
}

impl FromStr for IconListFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<IconListFormat, ()> {
        match s.to_lowercase().as_str() {
            "superscript" => Ok(IconListFormat::Superscript),
            "subscript" => Ok(IconListFormat::Subscript),
            "digits" => Ok(IconListFormat::Digits),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Shell {
    None,
    Zsh,
    ANSI,
}

impl FromStr for Shell {
    type Err = ();

    fn from_str(s: &str) -> Result<Shell, ()> {
        match s.to_lowercase().as_str() {
            "zsh" => Ok(Shell::Zsh),
            "ansi" => Ok(Shell::ANSI),
            "none" => Ok(Shell::None),
            _ => Err(()),
        }
    }
}
