use clap::Parser;
use settings::*;
use std::env;

#[macro_use]
extern crate lazy_static;

mod branch;
mod changes;
mod colors;
mod numbers;
mod settings;

#[derive(clap::Parser)]
#[clap(name = "rustygitprompt", about, author, version)]
struct Prompt {
    #[clap(
        arg_enum,
        short,
        long,
        help = "Sets the format for icon list",
        default_value = "digits"
    )]
    icon_list_format: IconListFormat,
    #[clap(
        arg_enum,
        short,
        long,
        help = "Sets a color: zsh, ansi or none (default)",
        default_value = "none"
    )]
    color: Shell,
    #[clap(
        short,
        long,
        help = "Sets a master/main branch label",
        default_value = "m"
    )]
    master_branch_label: String,
}

fn main() -> std::io::Result<()> {
    let app = Prompt::parse();

    {
        let mut settings = SETTINGS.lock().unwrap();
        settings.icon_list_format = app.icon_list_format;
        settings.shell = app.color;
        settings.master_branch_label = app.master_branch_label;
    }
    let repo = match git2::Repository::discover(env::current_dir()?.as_path()) {
        Ok(r) => r,
        Err(_r) => return Ok(()),
    };

    print!(" ");

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
