use git2::Error;
use std::env;
use ansi_term::Colour::*;

mod branch;

fn main() -> std::io::Result<()> {
    let repo = match git2::Repository::discover(env::current_dir()?.as_path()) {
        Ok(r) => r,
        Err(_r) => return Ok(()),
    };

    let r = Repository { repository: &repo };

    print!(" ");

    match r.status() {
        Ok(status) => print!("{}", status.to_string()),
        Err(e) => panic!("failed to get status: {}", e),
    }

    match crate::branch::analyze(repo) {
        Ok(branch) => print!("{}", branch.to_string()),
        Err(e) => panic!("failed to analyze branch: {}", e),
    }

    Ok(())
}

struct Repository<'a> {
    repository: &'a git2::Repository,
}

impl Repository<'_> {
    fn status(&self) -> Result<RepoStatus, Error> {
        let mut options = git2::StatusOptions::new();
        options.include_untracked(true);
        options.show(git2::StatusShow::IndexAndWorkdir);
        let statuses = self.repository.statuses(Some(&mut options))?;

        let mut result = RepoStatus{
            new_files: None,
            modifications_staged: None,
            modifications: None,
            untracked: None,
            renames_staged: None,
            renames: None,
            deletions_staged: None,
            deletions: None,
        };

        for entry in statuses.iter() {
            match entry.status() {
                s if s.contains(git2::Status::INDEX_MODIFIED) => result.modifications_staged.replace(result.modifications_staged.unwrap_or(0) + 1),
                s if s.contains(git2::Status::WT_MODIFIED) => result.modifications.replace(result.modifications.unwrap_or(0) + 1),
                s if s.contains(git2::Status::INDEX_NEW) => result.new_files.replace(result.new_files.unwrap_or(0) + 1),
                s if s.contains(git2::Status::WT_NEW) => result.untracked.replace(result.untracked.unwrap_or(0) + 1),
                s if s.contains(git2::Status::INDEX_RENAMED) => result.renames_staged.replace(result.renames_staged.unwrap_or(0) + 1),
                s if s.contains(git2::Status::WT_RENAMED) => result.renames.replace(result.renames.unwrap_or(0) + 1),
                s if s.contains(git2::Status::INDEX_DELETED) => result.deletions_staged.replace(result.deletions_staged.unwrap_or(0) + 1),
                s if s.contains(git2::Status::WT_DELETED) => result.deletions.replace(result.deletions.unwrap_or(0) + 1),
                // s if s.contains(git2::Status::CONFLICTED) => match entry.head_to_index().unwrap().status() {

                // }
                _ => continue,
            };
        }

        Ok(result)
    }

}

struct RepoStatus {
    modifications_staged: Option<usize>,
    modifications: Option<usize>,
    new_files: Option<usize>,
    untracked: Option<usize>,
    renames_staged: Option<usize>,
    renames: Option<usize>,
    deletions_staged: Option<usize>,
    deletions: Option<usize>,
}

impl RepoStatus {
    fn to_string(&self) -> String {
        let mut result = String::new();

        result.push_str(&self.new_files.map(|i| { format!("{}{}", i, Green.paint("N")) }).unwrap_or("".to_string()));
        result.push_str(&self.modifications_staged.map(|i| { format!("{}{}", i, Green.paint("M")) }).unwrap_or("".to_string()));
        result.push_str(&self.renames_staged.map(|i| { format!("{}{}", i, Green.paint("R")) }).unwrap_or("".to_string()));
        result.push_str(&self.deletions_staged.map(|i| { format!("{}{}", i, Green.paint("D")) }).unwrap_or("".to_string()));
        result.push_str(&self.modifications.map(|i| { format!("{}{}", i, Red.paint("M")) }).unwrap_or("".to_string()));
        result.push_str(&self.renames.map(|i| { format!("{}{}", i, Red.paint("R")) }).unwrap_or("".to_string()));
        result.push_str(&self.deletions.map(|i| { format!("{}{}", i, Red.paint("D")) }).unwrap_or("".to_string()));
        result.push_str(&self.untracked.map(|i| { format!("{}{}", i, Blue.paint("U")) }).unwrap_or("".to_string()));

        result
    }
}
