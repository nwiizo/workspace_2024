mod git_utils;

use git2::Repository;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <tag1> <tag2>", args[0]);
        process::exit(1);
    }

    let tag1 = &args[1];
    let tag2 = &args[2];

    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Failed to open Git repository: {}", e);
            process::exit(1);
        }
    };

    if !git_utils::check_tags_order(&repo, tag1, tag2) {
        eprintln!("Error: Tags are not in chronological order.");
        process::exit(1);
    }

    let base_url = git_utils::get_base_url(&repo).unwrap_or_else(|| String::from(""));
    git_utils::display_commit_logs(&repo, tag1, tag2, &base_url);
}
