use std::env;

use git2::{Repository, RepositoryOpenFlags, RepositoryState};

fn get_ref_name(repo: &Repository) -> String {
    if repo.is_empty().unwrap() {
        return String::from("empty");
    }

    let reference = match repo.head() {
        Ok(head) => head,
        Err(_) => return String::from("unknown"),
    };

    if reference.is_branch() || reference.is_tag() {
        String::from(
            reference
                .shorthand()
                .expect("Reference shorthand is not valid UTF-8!"),
        )
    } else {
        hex::encode(&reference.peel_to_commit().unwrap().id().as_bytes()[0..6])
    }
}

fn main() {
    let repo = match Repository::open_ext(
        env::var("PWD").unwrap(),
        RepositoryOpenFlags::empty(),
        [env::var("HOME").unwrap()],
    ) {
        Ok(r) => r,
        Err(_) => return,
    };

    // Doesn't work as expected
    // let clean_indicator = match repo.statuses(None).unwrap().is_empty() {
    //     true => "",
    //     false => "!",
    // };

    let branch_name;

    let branch_info = match repo.state() {
        RepositoryState::Clean => {
            branch_name = get_ref_name(&repo);
            &branch_name
        }
        RepositoryState::Merge => "merging",
        RepositoryState::Revert => "reverting",
        RepositoryState::RevertSequence => "reverting",
        RepositoryState::CherryPick => "cherry-picking",
        RepositoryState::CherryPickSequence => "cherry-picking",
        RepositoryState::Bisect => "bisecting",
        RepositoryState::Rebase => "rebasing",
        RepositoryState::RebaseInteractive => "rebasing",
        RepositoryState::RebaseMerge => "rebase-merge",
        RepositoryState::ApplyMailbox => "apply-mailbox",
        RepositoryState::ApplyMailboxOrRebase => "apply-mailbox-or-rebase",
    };

    print!("[%F{{yellow}}{}%f] ", branch_info);
}
