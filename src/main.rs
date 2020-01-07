use git2::Error;
use git2::Repository;
use std::env;

fn main() -> std::io::Result<()> {
    let mut path = env::current_dir()?;

    let repo = loop {
        let repo = Repository::init(path.as_path());
        if repo.is_err() {
            if !path.pop() {
                return Ok(());
            }
        } else {
            break repo.unwrap();
        }
    };

    let r = MyRepository { repository: repo };

    match r.branch() {
        Ok(branch) => print!("{}", branch.to_string()),
        Err(e) => panic!("failed to analyze branch: {}", e),
    }

    match r.status() {
        Ok(status) => print!("{}", status.to_string()),
        Err(e) => panic!("failed to get status: {}", e),
    }

    Ok(())
}

struct MyRepository {
    repository: Repository,
}

impl MyRepository {
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

    fn branch(&self) -> Result<BranchStatus, Error> {
        let head = match self.repository.head() {
            Ok(h) => h,
            Err(_e) => return Ok(BranchStatus {
                name: "detached".to_string(),
                local: None,
                upstream: None,
            }),
        };
        let mut name = head.name().unwrap();

        if name == "refs/heads/master" {
            name = "🅼"
        }

        let local = self
            .repository
            .find_branch("master", git2::BranchType::Local)
            .and_then(|master: git2::Branch| {
                self.repository
                    .graph_ahead_behind(head.target().unwrap(), master.get().target().unwrap())
            })
            .ok();

        let upstream = self
            .repository
            .find_branch(name, git2::BranchType::Local)
            .and_then(|u: git2::Branch| {
                self.repository.graph_ahead_behind(
                    head.target().unwrap(),
                    u.into_reference().target().unwrap(),
                )
            })
            .ok();

        Ok(BranchStatus {
            name: name.to_string(),
            local,
            upstream,
        })
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

        result.push_str(&self.new_files.map(|i| { format!("{}N", i) }).unwrap_or("".to_string()));
        result.push_str(&self.modifications_staged.map(|i| { format!("{}M", i) }).unwrap_or("".to_string()));
        result.push_str(&self.renames_staged.map(|i| { format!("{}R", i) }).unwrap_or("".to_string()));
        result.push_str(&self.deletions_staged.map(|i| { format!("{}D", i) }).unwrap_or("".to_string()));
        result.push_str(&self.modifications.map(|i| { format!("{}M", i) }).unwrap_or("".to_string()));
        result.push_str(&self.renames.map(|i| { format!("{}R", i) }).unwrap_or("".to_string()));
        result.push_str(&self.deletions.map(|i| { format!("{}D", i) }).unwrap_or("".to_string()));
        result.push_str(&self.untracked.map(|i| { format!("{}U", i) }).unwrap_or("".to_string()));

        result
    }
}

struct BranchStatus {
    name: String,
    local: Option<(usize, usize)>,
    upstream: Option<(usize, usize)>,
}

impl BranchStatus {
    fn upstream(&self) -> Option<String> {
        match self.upstream {
            Some((a, b)) if a > 0 && b > 0 => Some(format!("⇵ {}{}", a, b)),
            Some((a, _b)) if a > 0 => Some(format!("↓ {}", a)),
            Some((_a, b)) if b > 0 => Some(format!("↑ {}", b)),
            _ => None,
        }
    }

    fn local(&self) -> Option<String> {
        match self.local {
            Some((a, b)) if a > 0 && b > 0 => Some(format!("m ↔ {}{}", a, b)),
            Some((a, _b)) if a > 0 => Some(format!("m ← {}", a)),
            Some((_a, b)) if b > 0 => Some(format!("m → {}", b)),
            _ => None,
        }
    }

    fn to_string(&self) -> String {
        let mut result = String::new();

        result.push_str(&self.name);
        result.push_str(&self.local().unwrap_or("".to_string()));
        result.push_str(&self.upstream().unwrap_or("⚡".to_string()));

        result
    }
}
