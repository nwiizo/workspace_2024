// src/scanner.rs
use crate::types::{DirEntry, SizeCache};
use indicatif::ProgressBar;
use std::path::Path;
use walkdir::WalkDir;

pub fn should_skip(entry: &Path) -> bool {
    if let Some(path_str) = entry.to_str() {
        path_str.starts_with("/proc")
            || path_str.starts_with("/sys")
            || path_str.starts_with("/dev")
    } else {
        false
    }
}

pub fn get_dir_info(path: &Path, pb: Option<&ProgressBar>) -> (u64, usize) {
    let mut total_size = 0u64;
    let mut count = 0usize;

    for entry in WalkDir::new(path)
        .min_depth(1)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !should_skip(e.path()))
    {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() {
                total_size += metadata.len();
                if let Some(pb) = pb {
                    pb.inc(1);
                }
            }
            count += 1;
        }
    }

    (total_size, count)
}

pub fn scan_dir_lazy(path: &Path, cache: &SizeCache) -> (Vec<DirEntry>, u64, u64) {
    let pb = ProgressBar::new_spinner();
    pb.set_message(format!("Scanning {}", path.display()));

    let mut entries = Vec::new();
    let mut total_size = 0u64;
    let mut shown_size = 0u64;
    let mut processed_count = 0usize;

    let walker = WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .follow_links(false);

    for entry in walker
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !should_skip(e.path()))
    {
        if let Ok(metadata) = entry.metadata() {
            let path = entry.path().to_path_buf();
            let is_dir = metadata.is_dir();

            let size = if is_dir {
                if let Some((cached_size, _)) = cache.get(&path) {
                    cached_size
                } else {
                    let (dir_size, count) = get_dir_info(&path, Some(&pb));
                    cache.insert(path.clone(), dir_size, count);
                    dir_size
                }
            } else {
                metadata.len()
            };

            total_size += size;
            processed_count += 1;

            if processed_count <= 20 {
                shown_size += size;
                let children_count = if is_dir {
                    cache.get(&path).map(|(_, count)| count).unwrap_or(0)
                } else {
                    0
                };

                entries.push(DirEntry {
                    path: path.clone(),
                    size,
                    is_dir,
                    is_scanned: true,
                    children_count,
                });
            }
        }
    }

    pb.finish_and_clear();
    entries.sort_by(|a, b| b.size.cmp(&a.size));
    let others_size = total_size.saturating_sub(shown_size);

    (entries, total_size, others_size)
}
