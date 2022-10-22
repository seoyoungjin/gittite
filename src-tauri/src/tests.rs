//
// mkdir /tmp/test.gir
// git init /tmp/test.git
// git init --bare /tmp/test-bare.git
//
// - not existence repository
// - exist but not a repository case
// - repository inited but no commit
// - normal repository

// cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use std::panic;
    use git2::{ErrorCode, ErrorClass};
    use git2::Repository;

    #[test]
    fn repository() {
        let result = Repository::open("not_existent");
        // Error { code: -3, klass: 2, message: "failed to ..." }
        // println!("{:?}", result.err());
        assert_eq!(result.is_err(), true);
        match result {
            Ok(repo) => (),
            Err(e) => assert_eq!(e.code(), ErrorCode::NotFound),
        };

        let result = Repository::open("/tmp/test.dir");
        // Error { code: -3, klass: 6, message: "could not find ..."}
        assert_eq!(result.is_err(), true);
        match result {
            Ok(repo) => (),
            Err(e) => assert_eq!(e.class(), ErrorClass::Repository),
        };

        let result = Repository::open("/tmp/test.git");
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn bare_repository() {
        let result = Repository::open("/tmp/test-bare.git");
        let repo = match result {
            Ok(repo) => repo,
            Err(e) => panic!("panic"),
        };
        assert_eq!(repo.is_bare(), true);
    }
}
