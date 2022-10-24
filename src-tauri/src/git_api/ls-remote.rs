#![deny(warnings)]

use git2::{Direction, Repository};
use structopt::StructOpt;
use structopt::clap::AppSettings;

#[derive(StructOpt)]
#[structopt(setting(AppSettings::NoBinaryName))]
pub struct Args {
    #[structopt(name = "remote")]
    arg_remote: String,
}

pub fn run(args: &Args) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    let remote = &args.arg_remote;
    let mut remote = repo
        .find_remote(remote)
        .or_else(|_| repo.remote_anonymous(remote))?;

    // Connect to the remote and call the printing function for each of the
    // remote references.
    let connection = remote.connect_auth(Direction::Fetch, None, None)?;

    // Get the list of references on the remote and print out their name next to
    // what they point to.
    for head in connection.list()?.iter() {
        println!("{}\t{}", head.oid(), head.name());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    fn main() {
        let args = Args::from_args();
        match run(&args) {
            Ok(()) => {}
            Err(e) => println!("error: {}", e),
        }
    }
}
