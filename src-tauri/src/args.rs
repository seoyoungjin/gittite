use crate::git_api::error::Result;
use crate::git_api::repository::RepoPath;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Set the git directory
    #[arg(long)]
    directory: Option<String>,

    /// Set the working directory
    #[arg(long)]
    workdir: Option<String>,
}

pub struct CliArgs {
    pub repo_path: RepoPath,
}

pub fn process_cmdline() -> Result<CliArgs> {
    let cli = Cli::parse();

    let workdir = cli.workdir.map(PathBuf::from);
    let gitdir = cli
        .directory
        .map_or_else(|| PathBuf::from("."), PathBuf::from);

    #[allow(clippy::option_if_let_else)]
    let repo_path = if let Some(wd) = workdir {
        RepoPath::Workdir {
            gitdir,
            workdir: wd,
        }
    } else {
        RepoPath::Path(gitdir)
    };

    Ok(CliArgs { repo_path })
}

#[test]
fn test_args() {
    use clap::CommandFactory;
    Cli::command().debug_assert();

    let cli = Cli::parse_from(["tite", "--directory", "foo"]);
    assert_eq!(cli.directory, Some("foo".to_string()));
    let cli = Cli::parse_from(["tite", "--directory=foo", "--workdir=bar"]);
    assert_eq!(cli.workdir, Some("bar".to_string()));
}
