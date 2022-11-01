

/// ditto
pub(crate) fn get_branch_name_repo(
    repo: &Repository,
) -> Result<String, anyhow::Error> {
    let iter = repo.branches(None)?;

    for b in iter {
        let b = b?;
        if b.0.is_head() {
            let name = b.0.name()?.unwrap_or("");
            return Ok(name.into());
        }
    }
    Err(anyhow!("git: no head found"))
}
