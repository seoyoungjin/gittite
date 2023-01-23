// TODO
// #![deny(warnings)]

use super::commit_info::{get_commits_info, CommitMessage, CommitSignature};
use super::error::Result;
use super::repository::repo_open;
use super::utils::bytes2string;
use super::{CommitId, RepoPath};
use serde::Serialize;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ffi::OsString,
    ops::Not,
};
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Serialize)]
pub enum TagResponse {
    TagAdd(()),
    TagList(Vec<Tag>),
    TagDelete(()),
}

#[derive(StructOpt, Debug)]
#[structopt(setting(AppSettings::NoBinaryName))]
struct TagCommand {
    #[structopt(subcommand)]
    cmd: SubCommand,
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
    Delete { tagname: String },
}

///
pub fn tag<I>(
    repo_path: &RepoPath,
    args: I,
) -> Result<TagResponse>
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone,
{
    let opt = TagCommand::from_iter_safe(args)?;
    log::trace!("tag command: {:?}", opt);

    let res = match opt.cmd {
        SubCommand::Add {
            tagname,
            force,
            message,
            object,
        } => {
            // let msg = message.as_ref().map(String::as_str);
            tag_add(repo_path, tagname, object, message, force)?;
            TagResponse::TagAdd(())
        }
        SubCommand::List { pattern } => {
            let res = tag_list(repo_path, pattern)?;
            TagResponse::TagList(res)
        }
        SubCommand::Delete { tagname } => {
            tag_delete(repo_path, tagname.as_str())?;
            TagResponse::TagDelete(())
        } // _ => return Err(Error::Generic("invalid tag command".to_string())),
    };

    Ok(res)
}

///
#[derive(Serialize, Clone, Hash, PartialEq, Eq, Debug)]
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

/// hashmap of tag target commit hash to tag names
pub type Tags = BTreeMap<CommitId, Vec<Tag>>;

/// returns `Tags` type filled with all tags found in repo
pub fn get_tags(repo_path: &RepoPath) -> Result<Tags> {
    let mut res = Tags::new();
    let mut adder = |key, value: Tag| {
        if let Some(key) = res.get_mut(&key) {
            key.push(value);
        } else {
            res.insert(key, vec![value]);
        }
    };

    let repo = repo_open(repo_path)?;

    repo.tag_foreach(|id, name| {
        if let Ok(name) =
            // skip the `refs/tags/` part
            String::from_utf8(name[10..name.len()].into())
        {
            //NOTE: find_tag (using underlying git_tag_lookup) only
            // works on annotated tags lightweight tags `id` already
            // points to the target commit
            // see https://github.com/libgit2/libgit2/issues/5586
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

            if let Some(commit) = commit {
                adder(commit, Tag { name, annotation });
            }

            return true;
        }
        false
    })?;

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

static MAX_MESSAGE_WIDTH: usize = 100;

///
pub fn get_tags_with_metadata(repo_path: &RepoPath) -> Result<Vec<TagWithMetadata>> {
    let tags_grouped_by_commit_id = get_tags(repo_path)?;
    let tags_with_commit_id: Vec<(&str, Option<&str>, &CommitId)> = tags_grouped_by_commit_id
        .iter()
        .flat_map(|(commit_id, tags)| {
            tags.iter()
                .map(|tag| (tag.name.as_ref(), tag.annotation.as_deref(), commit_id))
                .collect::<Vec<_>>()
        })
        .collect();

    let unique_commit_ids: HashSet<_> = tags_with_commit_id
        .iter()
        .copied()
        .map(|(_, _, &commit_id)| commit_id)
        .collect();
    let mut commit_ids = Vec::with_capacity(unique_commit_ids.len());
    commit_ids.extend(unique_commit_ids);

    let commit_infos = get_commits_info(repo_path, &commit_ids, MAX_MESSAGE_WIDTH)?;
    let unique_commit_infos: HashMap<_, _> = commit_infos
        .iter()
        .map(|commit_info| (commit_info.id, commit_info))
        .collect();

    let mut tags: Vec<TagWithMetadata> = tags_with_commit_id
        .into_iter()
        .filter_map(|(tag, annotation, commit_id)| {
            unique_commit_infos
                .get(commit_id)
                .map(|commit_info| TagWithMetadata {
                    name: String::from(tag),
                    author: commit_info.author.clone(),
                    time: commit_info.time,
                    message: commit_info.message.clone(),
                    commit_id: *commit_id,
                    annotation: annotation.map(String::from),
                })
        })
        .collect();

    tags.sort_unstable_by(|a, b| b.time.cmp(&a.time));

    Ok(tags)
}

///
pub fn tag_delete(
    repo_path: &RepoPath,
    tag_name: &str,
) -> Result<()> {
    let repo = repo_open(repo_path)?;
    repo.tag_delete(tag_name)?;

    Ok(())
}

///
pub fn tag_add(
    repo_path: &RepoPath,
    tagname: String,
    object: Option<String>,
    message: Option<String>,
    force: bool,
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
pub fn tag_list(
    repo_path: &RepoPath,
    pattern: Option<String>,
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
                annotation: tag.message().as_deref().map(String::from),
            });
        } else if let Some(commit) = obj.as_commit() {
            res.push(Tag {
                name: name.to_string(),
                annotation: commit.message().as_deref().map(String::from),
            });
        } else {
            res.push(Tag {
                name: name.to_string(),
                annotation: None,
            });
        }
    }

    Ok(res)
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

        assert_eq!(get_tags(repo_path).unwrap().is_empty(), true);
    }

    #[test]
    fn test_multitags() {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

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

        assert_eq!(
            get_tags(repo_path).unwrap()[&CommitId::new(head_id)]
                .iter()
                .map(|t| &t.name)
                .collect::<Vec<_>>(),
            vec!["a", "b"]
        );

        let tags = get_tags_with_metadata(repo_path).unwrap();

        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].name, "a");
        assert_eq!(tags[0].message.subject, "initial");
        assert_eq!(tags[1].name, "b");
        assert_eq!(tags[1].message.subject, "initial");
        assert_eq!(tags[0].commit_id, tags[1].commit_id);

        tag_delete(repo_path, "a").unwrap();
        let tags = get_tags(repo_path).unwrap();

        assert_eq!(tags.len(), 1);

        tag_delete(repo_path, "b").unwrap();

        let tags = get_tags(repo_path).unwrap();
        assert_eq!(tags.len(), 0);
    }

    #[test]
    fn test_tag_subcommand() {
        let opt = TagCommand::from_iter(["add", "tagname"]);
        assert_eq!(
            opt.cmd,
            SubCommand::Add {
                force: false,
                message: None,
                tagname: "tagname".to_string(),
                object: None,
            }
        );
        let opt = TagCommand::from_iter(["add", "-m", "message", "tagname"]);
        assert_eq!(
            opt.cmd,
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
        assert_eq!(
            opt.cmd,
            SubCommand::List {
                pattern: Some("pattern".to_string())
            }
        );

        let opt = TagCommand::from_iter(["delete", "tagname"]);
        assert_eq!(
            opt.cmd,
            SubCommand::Delete {
                tagname: "tagname".to_string()
            }
        );
    }
}
