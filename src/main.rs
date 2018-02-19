use std::process::Command;


fn git_cmd(args: Vec<&str>) -> String {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("Failed to execute 'git'!");

    String::from_utf8(output.stdout).unwrap()
}


fn inside_repo() -> bool {
    let output = git_cmd(vec!["rev-parse", "--is-inside-work-tree"]);

    if output == "true\n" {
        true
    } else {
        false
    }
}


fn is_clean() -> bool {
    let output = git_cmd(vec!["status", "--short"]);

    if output == "" {
        true
    } else {
        false
    }
}


fn branch_info() -> String {
    let status = git_cmd(vec!["status"]);

    if status.starts_with("On branch") {
        status.split_whitespace().collect::<Vec<&str>>()[2].to_owned()
    } else if status.starts_with("HEAD detached at") {
        status.split_whitespace().collect::<Vec<&str>>()[3].to_owned()
    } else {
        panic!("Invalid git state!");
    }
}


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
