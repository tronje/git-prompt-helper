use std::process::Command;


/// Run a git command with all arguments in `args`.
///
/// Returns `stdout` output as a `String`.
fn git_cmd(args: Vec<&str>) -> String {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("Failed to execute 'git'!");

    String::from_utf8(output.stdout).unwrap()
}


/// See if we're inside a git repository/directory.
///
/// This uses `git rev-parse --is-inside-work-tree`, which prints
/// "true\n" or nothing to `stdout`, based on which we return
/// `true` or `false` respectively.
fn inside_repo() -> bool {
    let output = git_cmd(vec!["rev-parse", "--is-inside-work-tree"]);

    if output == "true\n" {
        true
    } else {
        false
    }
}


/// See if the working directory is clean or not.
///
/// This uses `git status --short`. If the output is anything,
/// we return `false`, otherwise `true`.
fn is_clean() -> bool {
    let output = git_cmd(vec!["status", "--short"]);

    if output == "" {
        true
    } else {
        false
    }
}


/// Using `git status`, returns the branch we're currently on.
///
/// If we're not on a branch, but in a detached commit, return
/// the commit hash instead. Return value is a `String`.
fn branch_info() -> String {
    let status = git_cmd(vec!["status"]);

    if status.starts_with("On branch") {
        // "On branch <branchname>" -> index 2
        status.split_whitespace().collect::<Vec<&str>>()[2].to_owned()
    } else if status.starts_with("HEAD detached at") {
        // "HEAD detached at <commithash>" -> index 3
        status.split_whitespace().collect::<Vec<&str>>()[3].to_owned()
    } else if status.starts_with("rebase in progress") {
        // "rebase in progress; onto <commithash>"
        String::from("rebasing")
    } else if status.starts_with("interactive rebase in progress") {
        // "interactive rebase in progress; onto <commithash>"
        String::from("rebasing (interactive)")
    } else {
        // There aren't any other possible outputs of `git status`... I think.
        String::from("error")
    }
}


/// Build our prompt from branch info and whether the working tree is clean.
///
/// If `inside_repo` is `false`, returns an empty `String`.
fn build_prompt() -> String {
    if !inside_repo() {
        return String::from("");
    }

    let clean_indicator = if is_clean() {
        ""
    } else {
        "!"
    };

    format!(
        "[%F{{yellow}}{}%f%F{{red}}{}%f] ",
        branch_info(),
        clean_indicator
    ).to_owned()
}


fn main() {
    print!("{}", build_prompt());
}
