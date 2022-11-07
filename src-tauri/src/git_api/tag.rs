// TODO
// #![deny(warnings)]

use super::error::{Error, Result};
use super::{CommitId, RepoPath};
use crate::git_api::repository::repo_open;
use crate::git_api::commit_info::{
    get_commit_info, CommitSignature, CommitMessage
};
use crate::git_api::utils::bytes2string;
use std::{ffi::OsString, ops::Not};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use structopt::StructOpt;
use structopt::clap::AppSettings;

///
#[derive(StructOpt, Debug)]
#[structopt(setting(AppSettings::NoBinaryName))]
struct TagCommand {
    #[structopt(subcommand)]
    cmd: SubCommand
}

#[derive(structopt::StructOpt, Debug, PartialEq)]
enum SubCommand {
    #[structopt(name = "add")]
    Add {
        tagname: String,
        object: Option<String>,
        #[structopt(short, long)]
        /// replace an existing tag with the given name
        force: bool,
        #[structopt(short, long)]
        /// message for a new tag
        message: Option<String>,
    },
    #[structopt(name = "list")]
    List {
        pattern: Option<String>,
        // #[structopt(name = "n", short)]
        // specify number of lines from the annotation to print
        // flag_n: Option<u32>,
    },
    #[structopt(name = "delete")]
    Delete {
        tagname: String,
    },
}

///
pub fn tag<I>(repo_path: &RepoPath, args: I) -> Result<Value>
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone
{
    let opt = TagCommand::from_iter_safe(args)?;
    log::trace!("tag command: {:?}", opt);

    let res = match opt.cmd {
        SubCommand::Add { tagname, force, message, object }  =>  {
            let msg = message.as_ref().map(String::as_str);
            let res = tag_add(repo_path, tagname, object, message, force)?;
            serde_json::to_value(res)
        },
        SubCommand::List { pattern } => {
            let res = get_tags(repo_path, pattern)?;
            serde_json::to_value(res)
        },
        SubCommand::Delete { tagname } => {
            let res = tag_delete(repo_path, tagname.as_str())?;
            serde_json::to_value(res)
        },
        _ => {
            return Err(Error::Generic("invalid tag command".to_string()))
        },
    };

    match res {
        Ok(v) => Ok(v),
        Err(e) => Err(Error::Generic(e.to_string()))
    }
}

///
pub fn tag_add(
    repo_path: &RepoPath,
    tagname: String,
    object: Option<String>,
    message: Option<String>,
    force: bool
 ) -> Result<()> {
    let repo = repo_open(&repo_path)?;
    let target = object.as_ref().map(|s| &s[..]).unwrap_or("HEAD");
    let obj = repo.revparse_single(target)?;

    if let Some(ref message) = message {
        let sig = repo.signature()?;
        repo.tag(tagname.as_str(), &obj, &sig, message, force)?;
    } else {
        repo.tag_lightweight(tagname.as_str(), &obj, force)?;
    }

    Ok(())
}

///
#[derive(Serialize, Deserialize)]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Tag {
	/// tag name
	pub name: String,
	/// tag annotation
	pub annotation: Option<String>,
}

impl Tag {
	///
	pub fn new(name: &str) -> Self {
		Self {
			name: name.into(),
			annotation: None,
		}
	}
}

/// returns `Tags` type filled with all tags found in repo
pub fn get_tags(
    repo_path: &RepoPath,
    pattern: Option<String>
) -> Result<Vec<Tag>> {
    let repo = repo_open(&repo_path)?;
    let pattern = pattern.as_ref().map(|s| &s[..]).unwrap_or("*");

    let mut res: Vec<Tag> = Vec::new();
    for name in repo.tag_names(Some(pattern))?.iter() {
        let name = name.unwrap();
        let obj = repo.revparse_single(name)?;
        if let Some(tag) = obj.as_tag() {
            res.push(Tag {
                name: tag.name().unwrap().to_owned(),
                annotation: tag.message().as_deref().map(String::from)
                // annotation: Some(tag.message().unwrap().to_owned())
            });
        } else if let Some(commit) = obj.as_commit() {
            res.push(Tag {
                name: name.to_string(),
                annotation: commit.message().as_deref().map(String::from)
            });
        } else {
            res.push(Tag { name: name.to_string(), annotation: None });
        }
    }

    Ok(res)
}

///
pub struct TagWithMetadata {
	pub name: String,
	pub author: CommitSignature,
	pub time: i64,
	pub message: CommitMessage,
	pub commit_id: CommitId,
	pub annotation: Option<String>,
}

///
pub fn get_tag_metadata(
    repo_path: &RepoPath,
    tag_name: &str
) -> Result<TagWithMetadata> {
    let repo = repo_open(&repo_path)?;
    let obj = repo.revparse_single(tag_name)?;
    let id = obj.id();
    let commit = repo
        .find_tag(id)
        .and_then(|tag| tag.target())
        .and_then(|target| target.peel_to_commit())
        .map_or_else(
            |_| {
                if repo.find_commit(id).is_ok() {
                    Some(CommitId::new(id))
                } else {
                    None
                }
            },
            |commit| Some(CommitId::new(commit.id())),
        );
    let annotation = repo
        .find_tag(id)
        .ok()
        .as_ref()
        .and_then(git2::Tag::message_bytes)
        .and_then(|msg| {
            msg.is_empty()
                .not()
                .then(|| bytes2string(msg).ok())
                .flatten()
        });

    if let Some(commit_id) = commit {
        log::trace!("commit_id {:?}", commit);
        let commit_info = get_commit_info(repo_path, commit_id)?;
        let tag_meta = TagWithMetadata {
	        name: String::from(tag_name),
	        author: commit_info.author.clone(),
	        time: commit_info.time,
	        message: commit_info.message.clone(),
	        commit_id: commit_id,
	        annotation: annotation.map(String::from),
        };
        return Ok(tag_meta);
    }

    Err(Error::Generic("no commit_id".to_string()))
}

///
pub fn tag_delete(
    repo_path: &RepoPath,
    tag_name: &str
) -> Result<()> {
	let repo = repo_open(repo_path)?;
	repo.tag_delete(tag_name)?;

	Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_api::tests::repo_init;
    use git2::ObjectType;

    #[test]
    fn test_smoke() {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        assert_eq!(get_tags(repo_path, None).unwrap().is_empty(), true);
    }

    #[test]
    fn test_multitags() {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();
        // let repo_path = &RepoPath::from("...");
	    // let repo = repo_open(repo_path).unwrap();

        let sig = repo.signature().unwrap();
        let head_id = repo.head().unwrap().target().unwrap();
        let target = repo
            .find_object(
                repo.head().unwrap().target().unwrap(),
                Some(ObjectType::Commit),
            )
            .unwrap();

        repo.tag("a", &target, &sig, "", false).unwrap();
        repo.tag("b", &target, &sig, "", false).unwrap();

        let tags = get_tags(repo_path, None).unwrap();
        log::trace!("get_tage() {:?}", tags);
        assert_eq!(tags.iter().map(|t| &t.name).collect::<Vec<_>>(),
            vec!["a", "b"]
        );

        let tag0 = get_tag_metadata(repo_path, "a").unwrap();
        assert_eq!(tag0.name, "a");
        assert_eq!(tag0.message.subject, "initial");
        let tag1 = get_tag_metadata(repo_path, "b").unwrap();
        assert_eq!(tag1.name, "b");
        assert_eq!(tag1.message.subject, "initial");
        assert_eq!(tag0.commit_id, tag1.commit_id);

        tag_delete(repo_path, "a").unwrap();
        let tags = get_tags(repo_path, None).unwrap();
        assert_eq!(tags.len(), 1);

        tag_delete(repo_path, "b").unwrap();
        let tags = get_tags(repo_path, None).unwrap();
        assert_eq!(tags.len(), 0);
    }

    #[test]
    fn test_tag_subcommand() {
        let opt = TagCommand::from_iter(["add", "tagname"]);
        assert_eq!(opt.cmd,
            SubCommand::Add {
                force: false,
                message: None,
                tagname: "tagname".to_string(),
                object: None,
            }
        );
        let opt = TagCommand::from_iter(["add", "-m", "message", "tagname"]);
        assert_eq!(opt.cmd,
            SubCommand::Add {
                force: false,
                message: Some("message".to_string()),
                tagname: "tagname".to_string(),
                object: None,
            }
        );

        let opt = TagCommand::from_iter(["list"]);
        assert_eq!(opt.cmd, SubCommand::List { pattern: None });
        let opt = TagCommand::from_iter(["list", "pattern"]);
        assert_eq!(opt.cmd,
            SubCommand::List { pattern: Some("pattern".to_string()) }
        );

        let opt = TagCommand::from_iter(["delete", "tagname"]);
        assert_eq!(opt.cmd,
            SubCommand::Delete { tagname: "tagname".to_string() }
        );
    }
}
