use super::{CommitId, RepoPath};
use crate::git_api::repository::repo_open;
use anyhow::{anyhow, Result};
use std::ffi::OsString;
use git2::{
    build::CheckoutBuilder, Oid, Repository, StashApplyOptions,
    StashFlags,
};
use serde_json::Value;
use structopt::StructOpt;
use structopt::clap::AppSettings;

#[derive(StructOpt, Debug)]
#[structopt(setting(AppSettings::NoBinaryName))]
struct Stash {
    #[structopt(subcommand)]
    cmd: SubCommand
}

#[derive(structopt::StructOpt, Debug, PartialEq)]
enum SubCommand {
    #[structopt(name = "save")]
    Save {
        #[structopt(name = "message")]
        message: Option<String>,
    },
    #[structopt(name = "list")]
    List,
    #[structopt(name = "show")]
    Show {
        #[structopt(short)]
        patch: bool,
        stash: String,
    },
    #[structopt(name = "drop")]
    Drop { stash: String },
    #[structopt(name = "pop")]
    Pop { stash: String },
    #[structopt(name = "apply")]
    Apply {
        stash: String,
        #[structopt(long)]
        index: bool
    },
    // #[structopt(name = "branch")]
    // Branch {
    //     branch: String,
    //     stash: String
    // },
    // #[structopt(name = "clear")]
    Clear,
}

///
pub fn get_stashes(repo_path: &RepoPath) -> Result<Vec<CommitId>> {
    let mut repo = repo_open(repo_path)?;
    let mut list = Vec::new();

    repo.stash_foreach(|_index, _msg, id| {
        list.push((*id).into());
        true
    })?;

    Ok(list)
}

/// checks whether a given commit is a stash commit.
pub fn is_stash_commit(
    repo_path: &RepoPath,
    id: &CommitId,
) -> Result<bool> {
    let stashes = get_stashes(repo_path)?;
    Ok(stashes.contains(id))
}

///
pub fn stash_drop(
    repo_path: &RepoPath,
    stash_id: CommitId,
) -> Result<()> {
    let mut repo = repo_open(repo_path)?;
    let index = get_stash_index(&mut repo, stash_id.into())?;

    repo.stash_drop(index)?;

    Ok(())
}

///
pub fn stash_pop(
    repo_path: &RepoPath,
    stash_id: CommitId,
) -> Result<()> {
    let mut repo = repo_open(repo_path)?;
    let index = get_stash_index(&mut repo, stash_id.into())?;

    repo.stash_pop(index, None)?;

    Ok(())
}

///
pub fn stash_apply(
    repo_path: &RepoPath,
    stash_id: CommitId,
    allow_conflicts: bool,
) -> Result<()> {
    let mut repo = repo_open(repo_path)?;
    let index = get_stash_index(&mut repo, stash_id.get_oid())?;

    let mut checkout = CheckoutBuilder::new();
    checkout.allow_conflicts(allow_conflicts);

    let mut opt = StashApplyOptions::default();
    opt.checkout_options(checkout);
    repo.stash_apply(index, Some(&mut opt))?;

    Ok(())
}

fn get_stash_index(
    repo: &mut Repository,
    stash_id: Oid,
) -> Result<usize> {
    let mut idx = None;

    repo.stash_foreach(|index, _msg, id| {
        if *id == stash_id {
            idx = Some(index);
            false
        } else {
            true
        }
    })?;

    idx.ok_or_else(|| {
        anyhow!("stash commit not found".to_string())
    })
}

///
pub fn stash_save(
    repo_path: &RepoPath,
    message: Option<&str>,
    include_untracked: bool,
    keep_index: bool,
) -> Result<CommitId> {
    let mut repo = repo_open(repo_path)?;
    let sig = repo.signature()?;
    let mut options = StashFlags::DEFAULT;

    if include_untracked {
        options.insert(StashFlags::INCLUDE_UNTRACKED);
    }
    if keep_index {
        options.insert(StashFlags::KEEP_INDEX);
    }
    let id = repo.stash_save2(&sig, message, Some(options))?;

    Ok(CommitId::new(id))
}

///
pub fn stash<I>(repo_path: &RepoPath, args: I) -> Result<Value>
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone
{
    let opt = Stash::from_iter_safe(args)?;
    log::trace!("stash: {:?}", opt);

    let res = match opt.cmd {
        /*
        SubCommand::List  => get_stashes(repo_path),
        SubCommand::Save{ message }  =>  {
            // TODO include_untracked, keep_index
            let msg = message.as_ref().map(String::as_str);
            stash_save(repo_path, msg, false, false)
        },
        SubCommand::Drop{ stash } => {
            let stashid = CommitId::from(stash);
            stash_drop(repo_path, stash)
        },
        SubCommand::Pop{ stash }  => stash_pop(repo_path, stash),
        // TODO
        SubCommand::Apply{ stash, index }  => stash_apply(repo_path, false, index),
        */
        // SubCommand::Branch{ branch, stash }  => println!("branch {} {}", branch, stash),
        // SubCommand::Clear  => println!("{:?}", "clear"),
        _ => (),
    };

    match serde_json::to_value(res) {
        Ok(v) => Ok(v),
        Err(e) => Err(anyhow!("Error serializing {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_api::tests::{
        debug_cmd_print,
        get_statuses,
        repo_init,
        write_commit_file,
    };
    use crate::git_api::{
        commit::commit,
        commit_info::get_commits_info,
        commit_files::get_commit_files,
        addremove::stage_add_file,
        utils::{repo_read_file, repo_write_file},
    };
    use std::{fs::File, io::Write, path::Path};

    #[test]
    fn test_smoke() {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        assert_eq!(stash_save(repo_path, None, true, false).is_ok(), false);
        assert_eq!(get_stashes(repo_path).unwrap().is_empty(), true);
    }

    #[test]
    fn test_stashing() -> Result<()> {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        File::create(&root.join("foo.txt"))?.write_all(b"test\nfoo")?;
        assert_eq!(get_statuses(repo_path), (1, 0));

        stash_save(repo_path, None, true, false)?;
        assert_eq!(get_statuses(repo_path), (0, 0));

        Ok(())
    }

    #[test]
    fn test_stashes() -> Result<()> {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        File::create(&root.join("foo.txt"))?.write_all(b"test\nfoo")?;

        stash_save(repo_path, Some("foo"), true, false)?;
        let res = get_stashes(repo_path)?;
        assert_eq!(res.len(), 1);

        let infos = get_commits_info(repo_path, &[res[0]], 100).unwrap();
        assert_eq!(infos[0].message.subject, "On master: foo");

        Ok(())
    }

    #[test]
    fn test_stash_nothing_untracked() -> Result<()> {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        File::create(&root.join("foo.txt"))?.write_all(b"test\nfoo")?;
        assert!(stash_save(repo_path, Some("foo"), false, false).is_err());

        Ok(())
    }

    #[test]
    fn test_stash_without_2nd_parent() -> Result<()> {
        let file_path1 = Path::new("file1.txt");
        let (_td, repo) = repo_init()?;
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        File::create(&root.join(file_path1))?.write_all(b"test")?;
        stage_add_file(repo_path, file_path1)?;
        commit(repo_path, "c1")?;

        File::create(&root.join(file_path1))?.write_all(b"modified")?;

        //NOTE: apparently `libgit2` works differently to git stash in
        //always creating the third parent for untracked files while the
        //cli skips that step when no new files exist
        debug_cmd_print(repo_path, "git stash");

        let stash = get_stashes(repo_path)?[0];
        let diff = get_commit_files(repo_path, stash, None)?;
        assert_eq!(diff.len(), 1);

        Ok(())
    }

    #[test]
    fn test_stash_apply_conflict() {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        repo_write_file(&repo, "test.txt", "test").unwrap();
        let id = stash_save(repo_path, Some("foo"), true, false).unwrap();
        repo_write_file(&repo, "test.txt", "foo").unwrap();

        let res = stash_apply(repo_path, id, false);
        assert!(res.is_err());
    }

    #[test]
    fn test_stash_apply_conflict2() {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        write_commit_file(&repo, "test.txt", "test", "c1");
        repo_write_file(&repo, "test.txt", "test2").unwrap();

        let id = stash_save(repo_path, Some("foo"), true, false).unwrap();
        repo_write_file(&repo, "test.txt", "test3").unwrap();
        let res = stash_apply(repo_path, id, false);

        assert!(res.is_err());
    }

    #[test]
    fn test_stash_apply_creating_conflict() {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        write_commit_file(&repo, "test.txt", "test", "c1");
        repo_write_file(&repo, "test.txt", "test2").unwrap();
        let id = stash_save(repo_path, Some("foo"), true, false).unwrap();
        repo_write_file(&repo, "test.txt", "test3").unwrap();
        let res = stash_apply(repo_path, id, false);
        assert!(res.is_err());

        let res = stash_apply(repo_path, id, true);
        assert!(res.is_ok());
    }

    #[test]
    fn test_stash_pop_no_conflict() {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        write_commit_file(&repo, "test.txt", "test", "c1");
        repo_write_file(&repo, "test.txt", "test2").unwrap();

        let id = stash_save(repo_path, Some("foo"), true, false).unwrap();
        let res = stash_pop(repo_path, id);

        assert!(res.is_ok());
        assert_eq!(repo_read_file(&repo, "test.txt").unwrap(), "test2");
    }

    #[test]
    fn test_stash_pop_conflict() {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        repo_write_file(&repo, "test.txt", "test").unwrap();
        let id = stash_save(repo_path, Some("foo"), true, false).unwrap();

        repo_write_file(&repo, "test.txt", "test2").unwrap();
        let res = stash_pop(repo_path, id);

        assert!(res.is_err());
        assert_eq!(repo_read_file(&repo, "test.txt").unwrap(), "test2");
    }

    #[test]
    fn test_stash_pop_conflict_after_commit() {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        write_commit_file(&repo, "test.txt", "test", "c1");

        repo_write_file(&repo, "test.txt", "test2").unwrap();
        let id = stash_save(repo_path, Some("foo"), true, false).unwrap();

        repo_write_file(&repo, "test.txt", "test3").unwrap();
        let res = stash_pop(repo_path, id);

        assert!(res.is_err());
        assert_eq!( repo_read_file(&repo, "test.txt").unwrap(), "test3");
    }

    #[test]
    fn test_stash_subcommand() {
        let opt = Stash::from_iter(["save"]);
        assert_eq!(opt.cmd, SubCommand::Save{message: None});
        let opt = Stash::from_iter(["save", "message"]);
        assert_eq!(opt.cmd, SubCommand::Save{message: Some("message".to_string())});

        let opt = Stash::from_iter(["list"]);
        assert_eq!(opt.cmd, SubCommand::List);

        let opt = Stash::from_iter_safe(["drop"]);
        assert_eq!(opt.is_err(), true);
        let opt = Stash::from_iter(["drop", "stash"]);
        assert_eq!(opt.cmd, SubCommand::Drop{ stash : "stash".to_string() });

        let opt = Stash::from_iter(["apply", "stash"]);
        assert_eq!(opt.cmd, SubCommand::Apply{ stash : "stash".to_string() , index: false});
        let opt = Stash::from_iter(["apply", "stash", "--index"]);
        assert_eq!(opt.cmd, SubCommand::Apply{ stash : "stash".to_string() , index: true});
    }
}
