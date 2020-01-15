use std::env;

mod branch;
mod changes;

fn main() -> std::io::Result<()> {
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
