use super::error::Result;
use super::repository::{repo_open, RepoPath};
use git2::{Commit, Error, Oid, Signature};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use unicode_truncate::UnicodeTruncateStr;

/// identifies a single commit
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct CommitId(#[serde_as(as = "DisplayFromStr")] Oid);

impl Default for CommitId {
    fn default() -> Self {
        Self(Oid::zero())
    }
}

impl CommitId {
    /// create new `CommitId`
    pub const fn new(id: Oid) -> Self {
        Self(id)
    }

    ///
    pub(crate) const fn get_oid(self) -> Oid {
        self.0
    }

    ///
    pub fn from_str(s: &str) -> Result<Self, Error> {
        let oid = Oid::from_str(s)?;
        Ok(Self::new(oid))
    }

    /// 7 chars short hash
    pub fn get_short_string(&self) -> String {
        self.to_string()[0..7].into()
    }
}

impl ToString for CommitId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<CommitId> for Oid {
    fn from(id: CommitId) -> Self {
        id.0
    }
}

impl From<Oid> for CommitId {
    fn from(id: Oid) -> Self {
        Self::new(id)
    }
}

///
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
pub struct CommitSignature {
    ///
    pub name: String,
    ///
    pub email: String,
    /// time in secs since Unix epoch
    pub time: i64,
}

impl CommitSignature {
    /// convert from git2-rs `Signature`
    pub fn from(s: &Signature<'_>) -> Self {
        Self {
            name: s.name().unwrap_or("").to_string(),
            email: s.email().unwrap_or("").to_string(),

            time: s.when().seconds(),
        }
    }
}

///
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct CommitMessage {
    /// first line
    pub subject: String,
    /// remaining lines if more than one
    pub body: Option<String>,
}

impl CommitMessage {
    ///
    pub fn from(s: &str) -> Self {
        let mut lines = s.lines();
        let subject = lines
            .next()
            .map_or_else(String::new, std::string::ToString::to_string);

        let body: Vec<String> = lines.map(std::string::ToString::to_string).collect();

        Self {
            subject,
            body: if body.is_empty() {
                None
            } else {
                Some(body.join("\n"))
            },
        }
    }

    ///
    pub fn combine(self) -> String {
        if let Some(body) = self.body {
            format!("{}\n{body}", self.subject)
        } else {
            self.subject
        }
    }
}

///
#[derive(Serialize, Deserialize, Debug)]
pub struct CommitInfo {
    ///
    pub id: CommitId,
    ///
    pub author: CommitSignature,
    /// committer when differs to `author` otherwise None
    pub committer: Option<CommitSignature>,
    ///
    pub time: i64,
    ///
    pub message: CommitMessage,
}

///
pub fn get_commits_info(
    repo_path: &RepoPath,
    ids: &[CommitId],
    message_length_limit: usize,
) -> Result<Vec<CommitInfo>> {
    let repo = repo_open(repo_path)?;

    let commits = ids
        .iter()
        .map(|id| repo.find_commit((*id).into()))
        .collect::<std::result::Result<Vec<Commit>, Error>>()?
        .into_iter();

    let res = commits
        .map(|c: Commit| {
            let author = CommitSignature::from(&c.author());
            let committer = CommitSignature::from(&c.committer());
            let committer = if author == committer {
                None
            } else {
                Some(committer)
            };
            let message = CommitMessage::from(get_message(&c, Some(message_length_limit)).as_str());
            CommitInfo {
                id: CommitId(c.id()),
                author: author,
                committer: committer,
                time: c.time().seconds(),
                message: message,
            }
        })
        .collect::<Vec<_>>();

    Ok(res)
}

///
pub fn get_commit_info(
    repo_path: &RepoPath,
    commit_id: CommitId,
) -> Result<CommitInfo> {
    let repo = repo_open(repo_path)?;

    let commit = repo.find_commit(commit_id.into())?;
    let author = CommitSignature::from(&commit.author());
    let committer = CommitSignature::from(&commit.committer());
    let committer = if author == committer {
        None
    } else {
        Some(committer)
    };
    let message = CommitMessage::from(get_message(&commit, None).as_str());

    Ok(CommitInfo {
        id: CommitId(commit.id()),
        author: author,
        committer: committer,
        time: commit.time().seconds(),
        message: message,
    })
}

/// if `message_limit` is set the message will be
/// limited to the first line and truncated to fit
pub fn get_message(
    c: &Commit,
    message_limit: Option<usize>,
) -> String {
    let msg = String::from_utf8_lossy(c.message_bytes());
    let msg = msg.trim();

    message_limit.map_or_else(
        || msg.to_string(),
        |limit| {
            let msg = msg.lines().next().unwrap_or_default();
            msg.unicode_truncate(limit).0.to_string()
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{get_commits_info, CommitId};
    use crate::git_api::error::Result;
    use crate::git_api::stage::stage_add_file;
    use crate::git_api::tests::{init_log, repo_init_empty};
    use crate::git_api::{commit::commit, RepoPath};
    use serde_json;
    use std::{fs::File, io::Write, path::Path};

    #[test]
    fn test_commit_id_serde() {
        init_log();
        let c1: CommitId = CommitId::from_str("12345678").unwrap();
        assert_eq!(c1.to_string()[0..8], "12345678".to_string());
        log::trace!("commit_id: {}", c1.to_string());

        let c1_serialized = serde_json::to_string(&c1).unwrap();
        log::trace!("commit_id_serial: {:?}", c1_serialized);

        let c2: CommitId = serde_json::from_str(c1_serialized.as_str()).unwrap();
        log::trace!("commit_id2: {}", c2.to_string());
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_log() -> Result<()> {
        init_log();
        let file_path = Path::new("foo");
        let (_td, repo) = repo_init_empty().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        File::create(&root.join(file_path))?.write_all(b"a")?;
        stage_add_file(repo_path, file_path).unwrap();
        let c1 = commit(repo_path, "commit1").unwrap();
        File::create(&root.join(file_path))?.write_all(b"a")?;
        stage_add_file(repo_path, file_path).unwrap();
        let c2 = commit(repo_path, "commit2").unwrap();

        let res = get_commits_info(repo_path, &[c2, c1], 50).unwrap();

        assert_eq!(res.len(), 2);
        assert_eq!(res[0].message.subject.as_str(), "commit2");
        assert_eq!(res[0].author.name.as_str(), "name");
        assert_eq!(res[1].message.subject.as_str(), "commit1");

        Ok(())
    }

    #[test]
    fn test_log_first_msg_line() -> Result<()> {
        let file_path = Path::new("foo");
        let (_td, repo) = repo_init_empty().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        File::create(&root.join(file_path))?.write_all(b"a")?;
        stage_add_file(repo_path, file_path).unwrap();
        let c1 = commit(repo_path, "subject\nbody").unwrap();

        let res = get_commits_info(repo_path, &[c1], 50).unwrap();

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].message.subject.as_str(), "subject");

        Ok(())
    }
}
