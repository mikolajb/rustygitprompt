use crate::numbers::*;
use crate::colors::*;
use crate::settings::*;

pub fn analyze(repository: &git2::Repository) -> Result<Changes, git2::Error> {
    let mut options = git2::StatusOptions::new();
    options.include_untracked(true);
    options.show(git2::StatusShow::IndexAndWorkdir);
    let statuses = repository.statuses(Some(&mut options))?;

    let mut result = Changes {
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
            s if s.contains(git2::Status::INDEX_MODIFIED) => result
                .modifications_staged
                .replace(result.modifications_staged.unwrap_or_default() + 1),
            s if s.contains(git2::Status::WT_MODIFIED) => result
                .modifications
                .replace(result.modifications.unwrap_or_default() + 1),
            s if s.contains(git2::Status::INDEX_NEW) => result
                .new_files
                .replace(result.new_files.unwrap_or_default() + 1),
            s if s.contains(git2::Status::WT_NEW) => result
                .untracked
                .replace(result.untracked.unwrap_or_default() + 1),
            s if s.contains(git2::Status::INDEX_RENAMED) => result
                .renames_staged
                .replace(result.renames_staged.unwrap_or_default() + 1),
            s if s.contains(git2::Status::WT_RENAMED) => result
                .renames
                .replace(result.renames.unwrap_or_default() + 1),
            s if s.contains(git2::Status::INDEX_DELETED) => result
                .deletions_staged
                .replace(result.deletions_staged.unwrap_or_default() + 1),
            s if s.contains(git2::Status::WT_DELETED) => result
                .deletions
                .replace(result.deletions.unwrap_or_default() + 1),
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

impl std::fmt::Display for Changes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}",
            self.new_files
                .map(|i| { format(number(i), green("N".to_string())) })
                .unwrap_or_default(),
            self.modifications_staged
                .map(|i| { format(number(i), green("M".to_string())) })
                .unwrap_or_default(),
            self.renames_staged
                .map(|i| { format(number(i), green("R".to_string())) })
                .unwrap_or_default(),
            self.deletions_staged
                .map(|i| { format(number(i), green("D".to_string())) })
                .unwrap_or_default(),
            self.modifications
                .map(|i| { format(number(i), red("M".to_string())) })
                .unwrap_or_default(),
            self.renames
                .map(|i| { format(number(i), red("R".to_string())) })
                .unwrap_or_default(),
            self.deletions
                .map(|i| { format(number(i), red("D".to_string())) })
                .unwrap_or_default(),
            self.untracked
                .map(|i| { format(number(i), blue("U".to_string())) })
                .unwrap_or_default()
        )
    }
}

fn format(n: String, label: String) -> String {
    let settings = SETTINGS.lock().unwrap();
    match &settings.icon_list_format {
        IconListFormat::Digits => format!("{}{}", n, label),
        _ => format!("{}{}", label, n)
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
        expected.push_str(&blue("U".to_string()).to_string());
        assert_eq!(c.to_string(), expected);

        dir.close().expect("cannot close");
    }
}
