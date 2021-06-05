use crate::colors::*;
use crate::settings::*;

pub fn analyze(repository: &git2::Repository) -> Result<BranchStatus, git2::Error> {
    let head = match repository.head() {
        Ok(h) => h,
        Err(_e) => {
            return Ok(BranchStatus {
                name: "detached".to_string(),
                local: None,
                upstream: None,
            })
        }
    };

    let branch_name = head.name().unwrap();

    let settings = SETTINGS.lock().unwrap();
    let hr_name = match branch_name {
        "refs/heads/master" => settings.master_branch_label.clone(),
        "refs/heads/main" => settings.master_branch_label.clone(),
        _ => head.shorthand().unwrap().to_string(),
    };

    let local = repository
        .find_branch("master", git2::BranchType::Local)
        .and_then(|master: git2::Branch| {
            repository.graph_ahead_behind(head.target().unwrap(), master.get().target().unwrap())
        })
        .ok();

    let upstream = repository
        .branch_upstream_name(branch_name)
        .ok()
        .and_then(|bname_buf: git2::Buf| bname_buf.as_str().map(|s| s.to_string()))
        .and_then(|bname| {
            repository
                .graph_ahead_behind(
                    head.target().unwrap(),
                    repository.refname_to_id(&bname).unwrap(),
                )
                .ok()
        });

    Ok(BranchStatus {
        name: hr_name,
        local,
        upstream,
    })
}

pub struct BranchStatus {
    name: String,
    local: Option<(usize, usize)>,
    upstream: Option<(usize, usize)>,
}

impl BranchStatus {
    fn upstream(&self) -> Option<String> {
        match self.upstream {
            Some((a, b)) if a > 0 && b > 0 => Some(format!(
                "{}{}{}",
                red("⇵".to_string()),
                yellow(b.to_string()),
                green(a.to_string())
            )),
            Some((a, 0)) if a > 0 => Some(format!("{}{}", green("↑".to_string()), a)),
            Some((0, b)) if b > 0 => Some(format!("{}{}", yellow("↓".to_string()), b)),
            Some((0, 0)) => Some("".to_string()),
            _ => Some(red("✗".to_string()).to_string()),
        }
    }

    fn local(&self) -> Option<String> {
        match self.local {
            Some((a, b)) if a > 0 && b > 0 => {
                Some(format!("m{}{}{}", magenta("↔".to_string()), a, b))
            }
            Some((a, 0)) if a > 0 => Some(format!("m{}{}", magenta("←".to_string()), a)),
            Some((0, b)) if b > 0 => Some(format!("m{}{}", magenta("→".to_string()), b)),
            _ => None,
        }
    }
}

impl std::fmt::Display for BranchStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.local().unwrap_or_default(),
            self.upstream().unwrap_or_default(),
            self.name
        )
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

        let repo = git2::Repository::discover(dir.path()).expect("cannot open repository");
        let b = analyze(&repo).expect("failed to analize branch");

        assert_eq!(b.name, "detached");
        assert_eq!(b.upstream, None);
        assert_eq!(b.local, None);

        dir.close().expect("cannot close");
    }
}
