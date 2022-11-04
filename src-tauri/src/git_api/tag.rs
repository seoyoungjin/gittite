// TODO
// #![deny(warnings)]

use anyhow::Result;
use super::{CommitId, RepoPath};
use crate::git_api::repository::repo_open;
use git2::{Error};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    arg_tagname: Option<String>,
    arg_object: Option<String>,
    arg_pattern: Option<String>,
    #[structopt(name = "n", short)]
    /// specify number of lines from the annotation to print
    flag_n: Option<u32>,
    #[structopt(name = "force", short, long)]
    /// replace an existing tag with the given name
    flag_force: bool,
    #[structopt(name = "list", short, long)]
    /// list tags with names matching the pattern given
    flag_list: bool,
    #[structopt(name = "tag", short, long = "delete")]
    /// delete the tag specified
    flag_delete: Option<String>,
    #[structopt(name = "msg", short, long = "message")]
    /// message for a new tag
    flag_message: Option<String>,
}

fn tag(repo_path: &RepoPath, args: &Args) -> Result<(), Error> {
    let repo = repo_open(&repo_path)?;

    if let Some(ref name) = args.arg_tagname {
        let target = args.arg_object.as_ref().map(|s| &s[..]).unwrap_or("HEAD");
        let obj = repo.revparse_single(target)?;

        if let Some(ref message) = args.flag_message {
            let sig = repo.signature()?;
            repo.tag(name, &obj, &sig, message, args.flag_force)?;
        } else {
            repo.tag_lightweight(name, &obj, args.flag_force)?;
        }
    } else if let Some(ref name) = args.flag_delete {
        let obj = repo.revparse_single(name)?;
        let id = obj.short_id()?;
        repo.tag_delete(name)?;
        /* TODO
        println!(
            "Deleted tag '{}' (was {})",
            name,
            str::from_utf8(&*id).unwrap()
        );
        */
    } else if args.flag_list {
        let pattern = args.arg_pattern.as_ref().map(|s| &s[..]).unwrap_or("*");
        for name in repo.tag_names(Some(pattern))?.iter() {
            let name = name.unwrap();
            let obj = repo.revparse_single(name)?;

            if let Some(tag) = obj.as_tag() {
                // print_tag(tag, args);
            } else if let Some(commit) = obj.as_commit() {
                // print_commit(commit, name, args);
            } else {
                // print_name(name);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_api::tests::repo_init;
    use git2::ObjectType;

    #[test]
    fn test_smoke() {
        /* TODO
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        assert_eq!(get_tags(repo_path).unwrap().is_empty(), true);
        */
    }

    #[test]
    fn test_multitags() {
        /* TODO
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
        assert_eq!(tags[0].message, "initial");
        assert_eq!(tags[1].name, "b");
        assert_eq!(tags[1].message, "initial");
        assert_eq!(tags[0].commit_id, tags[1].commit_id);

        delete_tag(repo_path, "a").unwrap();
        let tags = get_tags(repo_path).unwrap();
        assert_eq!(tags.len(), 1);

        delete_tag(repo_path, "b").unwrap();
        let tags = get_tags(repo_path).unwrap();
        assert_eq!(tags.len(), 0);
        */
    }
}
