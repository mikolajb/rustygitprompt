use std::env;
use settings::*;
use std::str::FromStr;
use std::process::exit;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate clap;

mod branch;
mod changes;
mod settings;


fn main() -> std::io::Result<()> {
    let mut app = clap_app!(rustygitprompt =>
        (version: "1.0")
        (author: "Mikołaj Baranowski <mikolajb@gmail.com>")
        (about: "A simple git prompt in rust")
        (@arg FORMAT: -i --icon_list_format +takes_value "Sets the format for icon list: superscript, subscript or digits (default)")
    );

    let format = app.clone().get_matches().value_of("FORMAT").map(|s| s.to_string());

    if let Some(c) = format {
        if let Ok(f) = IconListFormat::from_str(&c) {
            let mut settings = SETTINGS.lock().unwrap();
            settings.icon_list_format = f;
        } else {
            let _ = app.print_help();
            println!("");
            exit(1);
        }
    };

    let repo = match git2::Repository::discover(env::current_dir()?.as_path()) {
        Ok(r) => r,
        Err(_r) => return Ok(()),
    };

    match changes::analyze(&repo) {
        Ok(status) => print!("{}", status.to_string()),
        Err(e) => panic!("failed to analyze changes: {}", e),
    }

    match branch::analyze(&repo) {
        Ok(branch) => print!("{}", branch.to_string()),
        Err(e) => panic!("failed to analyze branch: {}", e),
    }

    Ok(())
}
