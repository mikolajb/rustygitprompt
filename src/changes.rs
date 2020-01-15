use ansi_term::Color::*;

pub fn analyze(repository: &git2::Repository) -> Result<Changes, git2::Error> {
        let mut options = git2::StatusOptions::new();
        options.include_untracked(true);
        options.show(git2::StatusShow::IndexAndWorkdir);
        let statuses = repository.statuses(Some(&mut options))?;

        let mut result = Changes{
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

pub struct Changes {
    modifications_staged: Option<usize>,
    modifications: Option<usize>,
    new_files: Option<usize>,
    untracked: Option<usize>,
    renames_staged: Option<usize>,
    renames: Option<usize>,
    deletions_staged: Option<usize>,
    deletions: Option<usize>,
}

impl Changes {
    pub fn to_string(&self) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_open_repo() {
        let dir = tempfile::Builder::new()
            .prefix("rustygitprompt")
            .tempdir()
            .expect("cannot create temporary file");

        Command::new("git")
            .arg("init")
            .current_dir(dir.path())
            .output()
            .expect("failed to create git repository");

        Command::new("touch")
            .arg("abc")
            .current_dir(dir.path())
            .output()
            .expect("failed to create git repository");


        let repo = git2::Repository::discover(dir.path()).expect("cannot open repository");
        let c = analyze(&repo).expect("failed to analize branch");

        assert_eq!(c.untracked.expect("new files expected"), 1);

        let mut expected = "1".to_string();
        expected.push_str(&Blue.paint("U").to_string());
        assert_eq!(c.to_string(), expected);

        dir.close().expect("cannot close");
    }
}
