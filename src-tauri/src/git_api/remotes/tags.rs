//!

use crate::git_api::{
    error::Result,
    cred::BasicAuthCredential,
    remotes::proxy_auto,
    repository::repo_open,
    RepoPath,
};
// use crossbeam_channel::Sender;
use super::callbacks::{Callbacks, Sender};
use git2::{Direction, PushOptions};
use std::collections::HashSet;

///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PushTagsProgress {
    /// fetching tags from remote to check which local tags need pushing
    CheckRemote,
    /// pushing local tags that are missing remote
    Push {
        ///
        pushed: usize,
        ///
        total: usize,
    },
    /// done
    Done,
}

/// lists the remotes tags
fn remote_tag_refs(
    repo_path: &RepoPath,
    remote: &str,
    basic_credential: Option<BasicAuthCredential>,
) -> Result<Vec<String>> {
    let repo = repo_open(repo_path)?;
    let mut remote = repo.find_remote(remote)?;
    let callbacks = Callbacks::new(None, basic_credential);
    let conn = remote.connect_auth(
        Direction::Fetch,
        Some(callbacks.callbacks()),
        Some(proxy_auto()),
    )?;

    let remote_heads = conn.list()?;
    let remote_tags = remote_heads
        .iter()
        .map(|s| s.name().to_string())
        .filter(|name| {
            name.starts_with("refs/tags/") && !name.ends_with("^{}")
        })
        .collect::<Vec<_>>();

    Ok(remote_tags)
}

/// lists the remotes tags missing
pub fn tags_missing_remote(
    repo_path: &RepoPath,
    remote: &str,
    basic_credential: Option<BasicAuthCredential>,
) -> Result<Vec<String>> {
    let repo = repo_open(repo_path)?;
    let tags = repo.tag_names(None)?;

    let mut local_tags = tags
        .iter()
        .filter_map(|tag| tag.map(|tag| format!("refs/tags/{tag}")))
        .collect::<HashSet<_>>();
    let remote_tags =
        remote_tag_refs(repo_path, remote, basic_credential)?;

    for t in remote_tags {
        local_tags.remove(&t);
    }

    Ok(local_tags.into_iter().collect())
}

///
pub fn push_tags(
    repo_path: &RepoPath,
    remote: &str,
    basic_credential: Option<BasicAuthCredential>,
    progress_sender: Option<Sender<PushTagsProgress>>,
) -> Result<()> {
    progress_sender
        .as_ref()
        .map(|sender| sender.send(PushTagsProgress::CheckRemote));

    let tags_missing = tags_missing_remote(
        repo_path,
        remote,
        basic_credential.clone(),
    )?;

    let repo = repo_open(repo_path)?;
    let mut remote = repo.find_remote(remote)?;

    let total = tags_missing.len();

    progress_sender.as_ref().map(|sender| {
        sender.send(PushTagsProgress::Push { pushed: 0, total })
    });

    for (idx, tag) in tags_missing.into_iter().enumerate() {
        let mut options = PushOptions::new();
        let callbacks = Callbacks::new(None, basic_credential.clone());
        options.remote_callbacks(callbacks.callbacks());
        options.packbuilder_parallelism(0);
        options.proxy_options(proxy_auto());
        remote.push(&[tag.as_str()], Some(&mut options))?;

        progress_sender.as_ref().map(|sender| {
            sender.send(PushTagsProgress::Push {
                pushed: idx + 1,
                total,
            })
        });
    }

    drop(basic_credential);

    progress_sender.map(|sender| {
        sender.send(PushTagsProgress::Done);
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        git_api::{
            self, tag, commit,
            remotes::{
                fetch::{fetch, fetch_all},
                push::{push_branch, push_raw},
            },
            tests::{repo_clone, repo_init_bare},
            remotes::push::PushType,
        },
    };
    // use pretty_assertions::assert_eq;
    use git_api::tests::write_commit_file;

    #[test]
    fn test_push_pull_tags() {
        let (r1_dir, _repo) = repo_init_bare().unwrap();
        let r1_dir = r1_dir.path().to_str().unwrap();

        let (clone1_dir, clone1) = repo_clone(r1_dir).unwrap();

        let clone1_dir: &RepoPath =
            &clone1_dir.path().to_str().unwrap().into();

        let (clone2_dir, clone2) = repo_clone(r1_dir).unwrap();

        let clone2_dir: &RepoPath =
            &clone2_dir.path().to_str().unwrap().into();

        // clone1
        let commit1 =
            write_commit_file(&clone1, "test.txt", "test", "commit1");
        commit::tag_commit(clone1_dir, &commit1, "tag1", None).unwrap();

        push_branch(
            clone1_dir, "origin", "master", false, false, None, None,
        )
        .unwrap();
        push_tags(clone1_dir, "origin", None, None).unwrap();

        // clone2
        let _commit2 = write_commit_file(
            &clone2,
            "test2.txt",
            "test",
            "commit2",
        );

        assert_eq!(tag::get_tags(clone2_dir, None).unwrap().len(), 0);

        //lets fetch from origin
        let bytes = fetch(clone2_dir, "master", None, None).unwrap();
        assert!(bytes > 0);

        /* TODO
        tag::merge_upstream_commit(clone2_dir, "master").unwrap();
        assert_eq!(tag::get_tags(clone2_dir, None).unwrap().len(), 1);
        */
    }

    #[test]
    fn test_get_remote_tags() {
        let (r1_dir, _repo) = repo_init_bare().unwrap();
        let r1_dir = r1_dir.path().to_str().unwrap();

        let (clone1_dir, clone1) = repo_clone(r1_dir).unwrap();

        let clone1_dir: &RepoPath =
            &clone1_dir.path().to_str().unwrap().into();

        let (clone2_dir, _clone2) = repo_clone(r1_dir).unwrap();

        let clone2_dir: &RepoPath =
            &clone2_dir.path().to_str().unwrap().into();

        // clone1
        let commit1 =
            write_commit_file(&clone1, "test.txt", "test", "commit1");
        commit::tag_commit(clone1_dir, &commit1, "tag1", None).unwrap();

        push_branch(
            clone1_dir, "origin", "master", false, false, None, None,
        )
        .unwrap();
        push_tags(clone1_dir, "origin", None, None).unwrap();

        // clone2
        let tags = remote_tag_refs(clone2_dir, "origin", None).unwrap();

        assert_eq!(
            tags.as_slice(),
            &[String::from("refs/tags/tag1")]
        );
    }

    #[test]
    fn test_tags_missing_remote() {
        let (r1_dir, _repo) = repo_init_bare().unwrap();
        let r1_dir = r1_dir.path().to_str().unwrap();

        let (clone1_dir, clone1) = repo_clone(r1_dir).unwrap();

        let clone1_dir: &RepoPath =
            &clone1_dir.path().to_str().unwrap().into();

        // clone1
        let commit1 =
            write_commit_file(&clone1, "test.txt", "test", "commit1");
        commit::tag_commit(clone1_dir, &commit1, "tag1", None).unwrap();

        push_branch(
            clone1_dir, "origin", "master", false, false, None, None,
        )
        .unwrap();

        let tags_missing =
            tags_missing_remote(clone1_dir, "origin", None).unwrap();

        assert_eq!(
            tags_missing.as_slice(),
            &[String::from("refs/tags/tag1")]
        );
        push_tags(clone1_dir, "origin", None, None).unwrap();
        let tags_missing =
            tags_missing_remote(clone1_dir, "origin", None).unwrap();
        assert!(tags_missing.is_empty());
    }

    #[test]
    fn test_tags_fetch() {
        let (r1_dir, _repo) = repo_init_bare().unwrap();
        let r1_dir = r1_dir.path().to_str().unwrap();

        let (clone1_dir, clone1) = repo_clone(r1_dir).unwrap();
        let clone1_dir: &RepoPath =
            &clone1_dir.path().to_str().unwrap().into();

        let commit1 =
            write_commit_file(&clone1, "test.txt", "test", "commit1");
        push_branch(
            clone1_dir, "origin", "master", false, false, None, None,
        )
        .unwrap();

        let (clone2_dir, _clone2) = repo_clone(r1_dir).unwrap();
        let clone2_dir: &RepoPath =
            &clone2_dir.path().to_str().unwrap().into();

        // clone1 - creates tag
        commit::tag_commit(clone1_dir, &commit1, "tag1", None).unwrap();
        let tags1 = tag::get_tags(clone1_dir, None).unwrap();

        push_tags(clone1_dir, "origin", None, None).unwrap();
        let tags_missing =
            tags_missing_remote(clone1_dir, "origin", None).unwrap();
        assert!(tags_missing.is_empty());

        // clone 2 - pull
        fetch(clone2_dir, "master", None, None).unwrap();

        let tags2 = tag::get_tags(clone2_dir, None).unwrap();

        assert_eq!(tags1, tags2);
    }

    #[test]
    fn test_tags_fetch_all() {
        let (r1_dir, _repo) = repo_init_bare().unwrap();
        let r1_dir = r1_dir.path().to_str().unwrap();

        let (clone1_dir, clone1) = repo_clone(r1_dir).unwrap();
        let clone1_dir: &RepoPath =
            &clone1_dir.path().to_str().unwrap().into();

        let commit1 =
            write_commit_file(&clone1, "test.txt", "test", "commit1");
        push_branch(
            clone1_dir, "origin", "master", false, false, None, None,
        )
        .unwrap();

        let (clone2_dir, _clone2) = repo_clone(r1_dir).unwrap();
        let clone2_dir: &RepoPath =
            &clone2_dir.path().to_str().unwrap().into();

        // clone1 - creates tag
        commit::tag_commit(clone1_dir, &commit1, "tag1", None).unwrap();
        let tags1 = tag::get_tags(clone1_dir, None).unwrap();

        push_tags(clone1_dir, "origin", None, None).unwrap();
        let tags_missing =
            tags_missing_remote(clone1_dir, "origin", None).unwrap();
        assert!(tags_missing.is_empty());

        // clone 2 - pull
        fetch_all(clone2_dir, &None, &None).unwrap();
        let tags2 = tag::get_tags(clone2_dir, None).unwrap();
        assert_eq!(tags1, tags2);
    }

    #[test]
    fn test_tags_delete_remote() {
        let (r1_dir, _repo) = repo_init_bare().unwrap();
        let r1_dir = r1_dir.path().to_str().unwrap();

        let (clone1_dir, clone1) = repo_clone(r1_dir).unwrap();
        let clone1_dir: &RepoPath =
            &clone1_dir.path().to_str().unwrap().into();

        let commit1 =
            write_commit_file(&clone1, "test.txt", "test", "commit1");
        push_branch(
            clone1_dir, "origin", "master", false, false, None, None,
        )
        .unwrap();

        let (clone2_dir, _clone2) = repo_clone(r1_dir).unwrap();
        let clone2_dir: &RepoPath =
            &clone2_dir.path().to_str().unwrap().into();

        // clone1 - creates tag
        commit::tag_commit(clone1_dir, &commit1, "tag1", None).unwrap();
        push_tags(clone1_dir, "origin", None, None).unwrap();

        // clone 2 - pull
        fetch_all(clone2_dir, &None, &None).unwrap();
        assert_eq!(tag::get_tags(clone2_dir, None).unwrap().len(), 1);

        // delete on clone 1
        tag::tag_delete(clone1_dir, "tag1").unwrap();
        push_raw(
            clone1_dir,
            "origin",
            "tag1",
            PushType::Tag,
            false,
            true,
            None,
            None,
        )
        .unwrap();

        push_tags(clone1_dir, "origin", None, None).unwrap();

        // clone 2
        fetch_all(clone2_dir, &None, &None).unwrap();
        assert_eq!(tag::get_tags(clone2_dir, None).unwrap().len(), 0);
    }
}
