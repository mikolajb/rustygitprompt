use crate::settings::*;
use ansi_term::Color::*;

pub fn red(s: String) -> String {
    let settings = SETTINGS.lock().unwrap();
    match &settings.shell {
        Shell::ANSI => Red.paint(s).to_string(),
        Shell::Zsh => format!("%F{{red}}{}%f", s),
        Shell::None => s,
    }
}

pub fn green(s: String) -> String {
    let settings = SETTINGS.lock().unwrap();
    match &settings.shell {
        Shell::ANSI => Green.paint(s).to_string(),
        Shell::Zsh => format!("%F{{green}}{}%f", s),
        Shell::None => s,
    }
}

pub fn blue(s: String) -> String {
    let settings = SETTINGS.lock().unwrap();
    match &settings.shell {
        Shell::ANSI => Blue.paint(s).to_string(),
        Shell::Zsh => format!("%F{{blue}}{}%f", s),
        Shell::None => s,
    }
}

pub fn yellow(s: String) -> String {
    let settings = SETTINGS.lock().unwrap();
    match &settings.shell {
        Shell::ANSI => Yellow.paint(s).to_string(),
        Shell::Zsh => format!("%F{{yellow}}{}%f", s),
        Shell::None => s,
    }
}

pub fn magenta(s: String) -> String {
    let settings = SETTINGS.lock().unwrap();
    match &settings.shell {
        Shell::ANSI => Purple.paint(s).to_string(),
        Shell::Zsh => format!("%F{{magenta}}{}%f", s),
        Shell::None => s,
    }
}
