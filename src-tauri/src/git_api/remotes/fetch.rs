//!

use crate::git_api::{
    cred::BasicAuthCredential,
    error::Result,
    repository::{repo_open, RepoPath},
    utils,
    RemoteProgress, ProgressPercent,
};
use super::callbacks::{Callbacks, Sender};
use super::proxy_auto;
use utils::bytes2string;
use git2::{BranchType, FetchOptions};

///
fn fetch_from_remote(
	repo_path: &RepoPath,
	remote: &str,
	basic_credential: Option<BasicAuthCredential>,
	progress_sender: Option<Sender<RemoteProgress>>,
) -> Result<()> {
	let repo = repo_open(repo_path)?;

	let mut remote = repo.find_remote(remote)?;

	let mut options = FetchOptions::new();
	let callbacks = Callbacks::new(progress_sender, basic_credential);
	options.prune(git2::FetchPrune::On);
	options.proxy_options(proxy_auto());
	options.download_tags(git2::AutotagOption::All);
	options.remote_callbacks(callbacks.callbacks());
	remote.fetch(&[] as &[&str], Some(&mut options), None)?;
	// fetch tags (also removing remotely deleted ones)
	remote.fetch(
		&["refs/tags/*:refs/tags/*"],
		Some(&mut options),
		None,
	)?;

	Ok(())
}

/// updates/prunes all branches from all remotes
pub fn fetch_all(
	repo_path: &RepoPath,
	basic_credential: &Option<BasicAuthCredential>,
	progress_sender: &Option<Sender<ProgressPercent>>,
) -> Result<()> {
	let repo = repo_open(repo_path)?;
	let remotes = repo
		.remotes()?
		.iter()
		.flatten()
		.map(String::from)
		.collect::<Vec<_>>();
	let remotes_count = remotes.len();

	for (idx, remote) in remotes.into_iter().enumerate() {
		fetch_from_remote(
			repo_path,
			&remote,
			basic_credential.clone(),
			None,
		)?;

		if let Some(sender) = progress_sender {
			let progress = ProgressPercent::new(idx, remotes_count);
			sender.send(progress)?;
		}
	}

	Ok(())
}

/// fetches from upstream/remote for local `branch`
pub(crate) fn fetch(
	repo_path: &RepoPath,
	branch: &str,
	basic_credential: Option<BasicAuthCredential>,
	progress_sender: Option<Sender<RemoteProgress>>,
) -> Result<usize> {
	let repo = repo_open(repo_path)?;
	let branch_ref = repo
		.find_branch(branch, BranchType::Local)?
		.into_reference();
	let branch_ref = bytes2string(branch_ref.name_bytes())?;
	let remote_name = repo.branch_upstream_remote(&branch_ref)?;
	let remote_name = bytes2string(&remote_name)?;
	let mut remote = repo.find_remote(&remote_name)?;

	let mut options = FetchOptions::new();
	options.download_tags(git2::AutotagOption::All);
	let callbacks = Callbacks::new(progress_sender, basic_credential);
	options.remote_callbacks(callbacks.callbacks());
	options.proxy_options(proxy_auto());

	remote.fetch(&[branch], Some(&mut options), None)?;

	Ok(remote.stats().received_bytes())
}
