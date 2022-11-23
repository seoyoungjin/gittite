use super::error::Result;
use super::repository::{repo_open, RepoPath};
use git2::{build::CheckoutBuilder, ObjectType};

///
pub fn reset_stage(
    repo_path: &RepoPath,
    path: &str,
) -> Result<()> {
    let repo = repo_open(repo_path)?;
    if repo.head().is_ok() {
        let head = repo.head()?.target();
        log::trace!("reset_stage head : {:?}", head);
        if let Some(head_id) = head {
            let o = repo.find_object(head_id, Some(ObjectType::Commit))?;
            repo.reset_default(Some(&o), [path])?;
        }
    } else {
        log::trace!("reset_stage empty head");
        repo.reset_default(None, [path])?;
    }

    Ok(())
}

///
pub fn reset_workdir(
    repo_path: &RepoPath,
    path: &str,
) -> Result<()> {
    let repo = repo_open(repo_path)?;

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
    use crate::git_api::addremove::stage_add_file;
    use crate::git_api::repository::RepoPath;
    use crate::git_api::tests::{get_statuses, repo_init_empty};
    use std::{fs::File, io::Write, path::Path};

    #[test]
    fn test_unstage_in_empty_repo() {
        let file_path = Path::new("foo.txt");
        let (_td, repo) = repo_init_empty().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        File::create(&root.join(file_path))
            .unwrap()
            .write_all(b"test\nfoo")
            .unwrap();
        assert_eq!(get_statuses(repo_path), (1, 0));

        stage_add_file(repo_path, file_path).unwrap();
        assert_eq!(get_statuses(repo_path), (0, 1));

        reset_stage(repo_path, file_path.to_str().unwrap()).unwrap();
        assert_eq!(get_statuses(repo_path), (1, 0));
    }
}
