use std::str::FromStr;
use std::sync::Mutex;

// emulated global variable for our settings
lazy_static! {
    pub static ref SETTINGS: Mutex<Settings> = Mutex::new(Settings{icon_list_format: IconListFormat::Digits});
}

#[derive(Debug)]
pub struct Settings {
    pub icon_list_format: IconListFormat
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
