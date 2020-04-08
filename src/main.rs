use std::env;

use git2::{Branch, Repository, RepositoryOpenFlags, RepositoryState};

fn get_ref_name(repo: &Repository) -> String {
    if repo.is_empty().unwrap() {
        return String::from("empty");
    }

    let reference = repo.head().expect("Could not get HEAD reference");

    if reference.is_branch() {
        Branch::wrap(reference)
            .name()
            .unwrap()
            .expect("Branch name not valid UTF-8!")
            .to_owned()
    } else if reference.is_tag() {
        reference
            .peel_to_tag()
            .unwrap()
            .name()
            .expect("Tag name is not valid UTF-8!")
            .to_owned()
    } else {
        panic!("Unexpected reference type! {:?}");
    }
}

fn main() {
    let repo = match Repository::open_ext(
        env::var("PWD").unwrap(),
        RepositoryOpenFlags::empty(),
        &[env::var("HOME").unwrap()],
    ) {
        Ok(r) => r,
        Err(_) => return,
    };

    let clean_indicator = match repo.statuses(None).unwrap().is_empty() {
        true => "",
        false => "!",
    };

    let branch_name;

    let branch_info = match repo.state() {
        RepositoryState::Clean => {
            branch_name = get_ref_name(&repo).to_owned();
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

    print!(
        "[%F{{yellow}}{}%f%F{{red}}{}%f] ",
        branch_info, clean_indicator
    );
}
