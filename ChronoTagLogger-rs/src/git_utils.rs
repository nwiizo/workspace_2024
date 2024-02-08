use git2::{Repository, Sort};

/// Gitリポジトリ内のタグが指定された順序であるか確認する。
///
/// # Arguments
///
/// * `repo` - 確認するGitリポジトリの参照。
/// * `tag1` - 最初のタグ。
/// * `tag2` - 二番目のタグ。
///
/// # Returns
///
/// * `bool` - タグが指定された順序であれば `true`、そうでなければ `false`。
pub fn check_tags_order(repo: &Repository, tag1: &str, tag2: &str) -> bool {
    let tags = match repo.tag_names(None) {
        Ok(t) => t,
        Err(_) => return false,
    };

    let mut tags_ordered = Vec::new();

    for tag in tags.iter().flatten() {
        tags_ordered.push(tag);
    }

    tags_ordered.sort_by_cached_key(|&tag| {
        repo.revparse_single(tag).unwrap().id()
    });

    let mut iter = tags_ordered.iter();
    let mut found_tag1 = false;

    while let Some(&tag) = iter.next() {
        if tag == tag1 {
            found_tag1 = true;
        } else if found_tag1 && tag == tag2 {
            return true;
        }
    }

    false
}

/// リポジトリのベースURLを取得する。
///
/// # Arguments
///
/// * `repo` - URLを取得するGitリポジトリの参照。
///
/// # Returns
///
/// * `Option<String>` - ベースURLが取得できた場合は `Some` でラップされたURL、そうでなければ `None`。
pub fn get_base_url(repo: &Repository) -> Option<String> {
    repo.find_remote("origin").ok()?.url().map(|url| {
        let url = url.replace("git@", "https://")
                     .replace(".git", "");
        // '://' の後に余分なスラッシュがないように調整
        let url = url.replace(":///", "://");

        if url.contains("github.com") {
            format!("{}/commit/", url)  // GitHub用に '/commit/' を追加
        } else {
            format!("{}/-/commit/", url) // その他のGitサーバー用に '/-/commit/' を追加
        }
    })
}

/// 指定されたタグ間のコミットログをMarkdown形式で表示する。
///
/// # Arguments
///
/// * `repo` - コミットログを取得するGitリポジトリの参照。
/// * `tag1` - 開始タグ。
/// * `tag2` - 終了タグ。
/// * `base_url` - コミットへのリンクを作成するためのベースURL。
pub fn display_commit_logs(repo: &Repository, tag1: &str, tag2: &str, base_url: &str) {
    let revrange = format!("{}..{}", tag1, tag2);
    let revspec = match repo.revparse(&revrange) {
        Ok(spec) => spec,
        Err(_) => {
            eprintln!("Invalid revision range: {}", revrange);
            return;
        }
    };

    let mut revwalk = match repo.revwalk() {
        Ok(walk) => walk,
        Err(_) => {
            eprintln!("Failed to create revwalk");
            return;
        }
    };

    if revwalk.push(revspec.from().unwrap().id()).is_err() {
        eprintln!("Failed to push to revwalk");
        return;
    }

    revwalk.set_sorting(Sort::REVERSE);

    for id in revwalk {
        if let Ok(id) = id {
            let commit = match repo.find_commit(id) {
                Ok(commit) => commit,
                Err(_) => continue,
            };

            let message = commit.summary().unwrap_or("No commit message");
            println!("- [{}]({}{}) {}", &id.to_string()[..7], base_url, &id, message);
        }
    }
}
