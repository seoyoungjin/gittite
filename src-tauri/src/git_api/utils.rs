//! git_api various methods

use super::CommitId;
use git2::{Error, ErrorClass, ErrorCode, Repository};

///
pub fn get_head_repo(repo: &Repository) -> Result<CommitId, Error> {
    let head = repo.head()?.target();
    head.map_or(
        Err(Error::new(
            ErrorCode::NotFound,
            ErrorClass::Object,
            "No head".to_string()
        )),
        |head_id| Ok(head_id.into()),
    )
}
