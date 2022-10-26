use anyhow::Result;
use git2::{build::CheckoutBuilder, ObjectType};
use super::repository::{repo, RepoPath};

///
pub fn reset_stage(repo_path: &RepoPath, path: &str) -> Result<()> {
    let repo = repo(repo_path)?;
    let head = repo.head()?.target();

    log::trace!("reset_stage head : {:?}", head);
    if let Some(head_id) = head {
        let o = repo.find_object(head_id, Some(ObjectType::Commit))?;
        repo.reset_default(Some(&o), [path])?;
    } else {
        repo.reset_default(None, [path])?;
    }

    Ok(())
}

///
pub fn reset_workdir(repo_path: &RepoPath, path: &str) -> Result<()> {
    let repo = repo(repo_path)?;

    let mut checkout_opts = CheckoutBuilder::new();
    checkout_opts
        .update_index(true) // windows: needs this to be true WTF?!
        .remove_untracked(true)
        .force()
        .path(path);

    repo.checkout_index(None, Some(&mut checkout_opts))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_reset_stage() {
        // let mut repo_path = RepoPath::from("...");
        // reset_stage(&repo_path, "test.txt");
        // get_status()
    }
}
