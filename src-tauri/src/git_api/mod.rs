//! Git API

pub mod clone;
pub mod cred;
pub mod error;
pub mod init;
pub mod repository;

pub mod commit;
pub mod commit_files;
pub mod commit_info;

// TODO mv
pub mod addremove;
// TODO reset_workdir
pub mod reset;

pub mod diff;
pub mod revlog;
// spec revspec
// pub mod rev_parse;
// show grep
pub mod status;

// merge rebase reset switch
pub mod blame;
pub mod branch;
pub mod stash;
pub mod tag;

// TODO push
pub mod remotes;

// sig tree blob
// pub mod cat_file;
mod ignore;
pub mod progress;
pub mod utils;

pub use blame::FileBlame;
pub use branch::{BranchCompare, BranchInfo};
pub use commit_info::{CommitId, CommitInfo};
pub use diff::FileDiff;
pub use error::{Error, Result};
pub use progress::{ProgressPercent, RemoteProgress};
pub use remotes::push::ProgressNotification;
pub use repository::RepoPath;
pub use revlog::CommitData;
pub use status::{StatusItem, StatusItemType};

#[cfg(test)]
mod tests {
    use super::error::Result;
    use super::utils::repo_write_file;
    use super::{addremove, commit, revlog, status};
    use super::{CommitId, RepoPath};

    use git2::{Repository, StatusShow};
    use std::{path::Path, process::Command};
    use tempfile::TempDir;

    // init log
    pub fn init_log() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Trace)
            .try_init();
    }

    /// Calling `set_search_path` with an empty directory makes sure that there
    /// is no git config interfering with our tests (for example user-local
    /// `.gitconfig`).
    #[allow(unsafe_code)]
    pub fn sandbox_config_files() {
        use git2::{opts::set_search_path, ConfigLevel};
        use std::sync::Once;

        static INIT: Once = Once::new();

        // Adapted from https://github.com/rust-lang/cargo/pull/9035
        INIT.call_once(|| unsafe {
            let temp_dir = TempDir::new().unwrap();
            let path = temp_dir.path();

            set_search_path(ConfigLevel::System, path).unwrap();
            set_search_path(ConfigLevel::Global, path).unwrap();
            set_search_path(ConfigLevel::XDG, path).unwrap();
            set_search_path(ConfigLevel::ProgramData, path).unwrap();
        });
    }

    ///
    pub fn repo_init_empty() -> Result<(TempDir, Repository)> {
        init_log();
        sandbox_config_files();

        let td = TempDir::new()?;
        let repo = Repository::init(td.path())?;
        {
            let mut config = repo.config()?;
            config.set_str("user.name", "name")?;
            config.set_str("user.email", "email")?;
        }
        Ok((td, repo))
    }

    /// Same as `repo_init`, but the repo is a bare repo (--bare)
    pub fn repo_init_bare() -> Result<(TempDir, Repository)> {
        init_log();

        let tmp_repo_dir = TempDir::new()?;
        let bare_repo = Repository::init_bare(tmp_repo_dir.path())?;
        Ok((tmp_repo_dir, bare_repo))
    }

    ///
    pub fn repo_init() -> Result<(TempDir, Repository)> {
        init_log();
        sandbox_config_files();

        let td = TempDir::new()?;
        let repo = Repository::init(td.path())?;
        {
            let mut config = repo.config()?;
            config.set_str("user.name", "name")?;
            config.set_str("user.email", "email")?;

            let mut index = repo.index()?;
            let id = index.write_tree()?;

            let tree = repo.find_tree(id)?;
            let sig = repo.signature()?;
            repo.commit(Some("HEAD"), &sig, &sig, "initial", &tree, &[])?;
        }
        Ok((td, repo))
    }

    ///
    pub fn repo_clone(p: &str) -> Result<(TempDir, Repository)> {
        sandbox_config_files();

        let td = TempDir::new()?;
        let td_path = td.path().as_os_str().to_str().unwrap();
        let repo = Repository::clone(p, td_path).unwrap();

        let mut config = repo.config()?;
        config.set_str("user.name", "name")?;
        config.set_str("user.email", "email")?;

        Ok((td, repo))
    }

    /// helper returning amount of files with changes in the (wd,stage)
    pub fn get_statuses(repo_path: &RepoPath) -> (usize, usize) {
        (
            status::get_status(repo_path, StatusShow::Workdir)
                .unwrap()
                .len(),
            status::get_status(repo_path, StatusShow::Index)
                .unwrap()
                .len(),
        )
    }

    /// helper to fetch commmit details using log walker
    pub fn get_commit_ids(
        repo_path: &RepoPath,
        max_count: usize,
    ) -> Vec<CommitId> {
        let args = ["-n".to_string(), max_count.to_string()];
        let commit_data = revlog::get_commits(repo_path, &args).unwrap();
        commit_data.iter().map(|t| t.commit_id).collect::<Vec<_>>()
    }

    ///
    pub fn debug_cmd_print(
        path: &RepoPath,
        cmd: &str,
    ) {
        let cmd = debug_cmd(path, cmd);
        eprintln!("\n----\n{}", cmd);
    }

    fn debug_cmd(
        path: &RepoPath,
        cmd: &str,
    ) -> String {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", cmd])
                .current_dir(path.gitpath())
                .output()
                .unwrap()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .current_dir(path.gitpath())
                .output()
                .unwrap()
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        format!(
            "{}{}",
            if stdout.is_empty() {
                String::new()
            } else {
                format!("out:\n{}", stdout)
            },
            if stderr.is_empty() {
                String::new()
            } else {
                format!("err:\n{}", stderr)
            }
        )
    }

    /// write, stage and commit a file
    pub fn write_commit_file(
        repo: &Repository,
        file: &str,
        content: &str,
        commit_name: &str,
    ) -> CommitId {
        repo_write_file(repo, file, content).unwrap();

        addremove::stage_add_file(
            &repo.workdir().unwrap().to_str().unwrap().into(),
            Path::new(file),
        )
        .unwrap();

        commit::commit(
            &repo.workdir().unwrap().to_str().unwrap().into(),
            commit_name,
        )
        .unwrap()
    }
}
