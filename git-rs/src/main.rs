use git2::Repository;
use std::env;
use std::process;

fn main() {
    // コマンドライン引数を取得します
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <url> <directory>", args[0]);
        process::exit(1);
    }

    let url = &args[1];
    let directory = &args[2];

    match clone_repo(url, directory) {
        Ok(()) => println!("Repository successfully cloned."),
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}

fn clone_repo(url: &str, directory: &str) -> Result<(), git2::Error> {
    let repo = Repository::clone(url, directory)?;
    // 追加のGit操作はここで実装できます
    Ok(())
}
