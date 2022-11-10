// TODO
// #![deny(warnings)]

use super::{CommitId, RepoPath};
use super::repository::{repo_open};
use git2::{Commit, DiffOptions, ObjectType, Repository, Signature, Time};
use git2::{Error, Pathspec};
use std::ffi::OsString;
use serde::Serialize;
use structopt::clap::AppSettings;
use structopt::StructOpt;

// author name <email> date
//   author().name(), author().email(), author.when()
// js and rust time exchange format
// tree_id ,tree is used for diff

#[derive(Serialize)]
pub struct CommitData {
    pub commit_id: CommitId,
    summary: String,
    body: String,
    date: String,
    author: String,
    committer: String,
    parents: Vec<String>,
}

#[derive(StructOpt)]
#[structopt(setting(AppSettings::NoBinaryName))]
struct Args {
    #[structopt(name = "topo-order", long)]
    /// sort commits in topological order
    flag_topo_order: bool,
    #[structopt(name = "date-order", long)]
    /// sort commits in date order
    flag_date_order: bool,
    #[structopt(name = "reverse", long)]
    /// sort commits in reverse
    flag_reverse: bool,
    #[structopt(name = "author", long)]
    /// author to sort by
    flag_author: Option<String>,
    #[structopt(name = "committer", long)]
    /// committer to sort by
    flag_committer: Option<String>,
    #[structopt(name = "pat", long = "grep")]
    /// pattern to filter commit messages by
    flag_grep: Option<String>,
    #[structopt(name = "skip", long)]
    /// number of commits to skip
    flag_skip: Option<usize>,
    #[structopt(name = "max-count", short = "n", long)]
    /// maximum number of commits to show
    flag_max_count: Option<usize>,
    #[structopt(name = "merges", long)]
    /// only show merge commits
    flag_merges: bool,
    #[structopt(name = "no-merges", long)]
    /// don't show merge commits
    flag_no_merges: bool,
    #[structopt(name = "no-min-parents", long)]
    /// don't require a minimum number of parents
    flag_no_min_parents: bool,
    #[structopt(name = "no-max-parents", long)]
    /// don't require a maximum number of parents
    flag_no_max_parents: bool,
    #[structopt(name = "max-parents")]
    /// specify a maximum number of parents for a commit
    flag_max_parents: Option<usize>,
    #[structopt(name = "min-parents")]
    /// specify a minimum number of parents for a commit
    flag_min_parents: Option<usize>,
    #[structopt(name = "commit")]
    arg_commit: Vec<String>,
    #[structopt(name = "spec", last = true)]
    arg_spec: Vec<String>,
}

pub fn get_commits<I>(repo: &RepoPath, args: I) -> Result<Vec<CommitData>, Error>
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone
{
    let repo = repo_open(repo)?;
    let opts = Args::from_iter(args);

    let mut revwalk = repo.revwalk()?;
    // Prepare the revwalk based on CLI parameters
    let base = if opts.flag_reverse {
        git2::Sort::REVERSE
    } else {
        git2::Sort::NONE
    };
    revwalk.set_sorting(
        base | if opts.flag_topo_order {
            git2::Sort::TOPOLOGICAL
        } else if opts.flag_date_order {
            git2::Sort::TIME
        } else {
            git2::Sort::NONE
        },
    )?;
    for commit in &opts.arg_commit {
        if commit.starts_with('^') {
            let obj = repo.revparse_single(&commit[1..])?;
            revwalk.hide(obj.id())?;
            continue;
        }
        let revspec = repo.revparse(commit)?;
        if revspec.mode().contains(git2::RevparseMode::SINGLE) {
            revwalk.push(revspec.from().unwrap().id())?;
        } else {
            let from = revspec.from().unwrap().id();
            let to = revspec.to().unwrap().id();
            revwalk.push(to)?;
            if revspec.mode().contains(git2::RevparseMode::MERGE_BASE) {
                let base = repo.merge_base(from, to)?;
                let o = repo.find_object(base, Some(ObjectType::Commit))?;
                revwalk.push(o.id())?;
            }
            revwalk.hide(from)?;
        }
    }
    if opts.arg_commit.is_empty() {
        revwalk.push_head()?;
    }

    // Prepare our diff options and pathspec matcher
    let (mut diffopts, mut diffopts2) = (DiffOptions::new(), DiffOptions::new());
    for spec in &opts.arg_spec {
        diffopts.pathspec(spec);
        diffopts2.pathspec(spec);
    }
    let ps = Pathspec::new(opts.arg_spec.iter())?;

    // Filter our revwalk based on the CLI parameters
    macro_rules! filter_try {
        ($e:expr) => {
            match $e {
                Ok(t) => t,
                Err(e) => return Some(Err(e)),
            }
        };
    }
    let revwalk = revwalk
        .filter_map(|id| {
            let id = filter_try!(id);
            let commit = filter_try!(repo.find_commit(id));
            let parents = commit.parents().len();
            if parents < opts.min_parents() {
                return None;
            }
            if let Some(n) = opts.max_parents() {
                if parents >= n {
                    return None;
                }
            }
            if !opts.arg_spec.is_empty() {
                match commit.parents().len() {
                    0 => {
                        let tree = filter_try!(commit.tree());
                        let flags = git2::PathspecFlags::NO_MATCH_ERROR;
                        if ps.match_tree(&tree, flags).is_err() {
                            return None;
                        }
                    }
                    _ => {
                        let m = commit.parents().all(|parent| {
                            match_with_parent(&repo, &commit, &parent, &mut diffopts)
                                .unwrap_or(false)
                        });
                        if !m {
                            return None;
                        }
                    }
                }
            }
            if !sig_matches(&commit.author(), &opts.flag_author) {
                return None;
            }
            if !sig_matches(&commit.committer(), &opts.flag_committer) {
                return None;
            }
            if !log_message_matches(commit.message(), &opts.flag_grep) {
                return None;
            }
            Some(Ok(commit))
        })
        .skip(opts.flag_skip.unwrap_or(0))
        .take(opts.flag_max_count.unwrap_or(!0));

    // response
    let mut cv: Vec<CommitData> = Vec::new();
    for commit in revwalk {
        let commit = commit?;
        let mut cd = CommitData {
            commit_id: commit.id().into(),
            summary: commit.summary().unwrap().to_string(),
            body: match commit.body() {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },
            date: format_time(&commit.author().when()),
            author: commit.author().to_string(),
            committer: commit.committer().to_string(),
            parents: vec![],
        };

        if commit.parents().len() > 1 {
            for id in commit.parent_ids() {
                cd.parents.push(format!("{:.8}", id))
            }
        }

        cv.push(cd);
    }

    Ok(cv)
}

fn sig_matches(sig: &Signature, arg: &Option<String>) -> bool {
    match *arg {
        Some(ref s) => {
            sig.name().map(|n| n.contains(s)).unwrap_or(false)
                || sig.email().map(|n| n.contains(s)).unwrap_or(false)
        }
        None => true,
    }
}

fn log_message_matches(msg: Option<&str>, grep: &Option<String>) -> bool {
    match (grep, msg) {
        (&None, _) => true,
        (&Some(_), None) => false,
        (&Some(ref s), Some(msg)) => msg.contains(s),
    }
}

// TODO
fn format_time(time: &Time) -> String {
    let (offset, sign) = match time.offset_minutes() {
        n if n < 0 => (-n, '-'),
        n => (n, '+'),
    };
    let (hours, minutes) = (offset / 60, offset % 60);
    let ts = time::Timespec::new(time.seconds() + (time.offset_minutes() as i64) * 60, 0);
    let time = time::at(ts);

    return format!(
        "{} {}{:02}{:02}",
        time.strftime("%a %b %e %T %Y").unwrap(),
        sign,
        hours,
        minutes
    );
}

fn match_with_parent(
    repo: &Repository,
    commit: &Commit,
    parent: &Commit,
    opts: &mut DiffOptions,
) -> Result<bool, Error> {
    let a = parent.tree()?;
    let b = commit.tree()?;
    let diff = repo.diff_tree_to_tree(Some(&a), Some(&b), Some(opts))?;
    Ok(diff.deltas().len() > 0)
}

impl Args {
    fn min_parents(&self) -> usize {
        if self.flag_no_min_parents {
            return 0;
        }
        self.flag_min_parents
            .unwrap_or(if self.flag_merges { 2 } else { 0 })
    }

    fn max_parents(&self) -> Option<usize> {
        if self.flag_no_max_parents {
            return None;
        }
        self.flag_max_parents
            .or(if self.flag_no_merges { Some(1) } else { None })
    }
}
