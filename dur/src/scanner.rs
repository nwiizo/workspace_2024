// src/scanner.rs
use crate::types::{DirEntry, EnhancedSizeCache, ScanStatus};
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;

const CHUNK_SIZE: usize = 1000;

pub fn should_skip(entry: &Path) -> bool {
    if let Some(path_str) = entry.to_str() {
        path_str.starts_with("/proc")
            || path_str.starts_with("/sys")
            || path_str.starts_with("/dev")
            || path_str.starts_with(".git")
            || path_str.starts_with("node_modules")
            || path_str.starts_with("target")
    } else {
        false
    }
}

pub fn get_dir_info(
    path: &Path,
    pb: Option<&ProgressBar>,
    status_tx: Option<&Sender<ScanStatus>>,
) -> (u64, usize) {
    let total_size = Arc::new(Mutex::new(0u64));
    let count = Arc::new(Mutex::new(0usize));
    let processed = Arc::new(Mutex::new(HashSet::new()));

    let entries: Vec<_> = WalkDir::new(path)
        .min_depth(1)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !should_skip(e.path()))
        .collect();

    entries.par_iter().for_each(|entry| {
        if let Ok(metadata) = entry.metadata() {
            let path = entry.path().to_path_buf();
            let mut processed = processed.lock().unwrap();

            if !processed.contains(&path) {
                if metadata.is_file() {
                    let mut size = total_size.lock().unwrap();
                    *size += metadata.len();
                    if let Some(pb) = pb {
                        pb.inc(1);
                    }
                    if let Some(tx) = status_tx {
                        let _ = tx.send(ScanStatus::Processing(format!(
                            "Scanning: {}",
                            entry.path().display()
                        )));
                    }
                }
                let mut count_lock = count.lock().unwrap();
                *count_lock += 1;
                processed.insert(path);
            }
        }
    });

    // MutexGuardのライフタイムを明示的に制限
    let size = *total_size.lock().unwrap();
    let count_val = *count.lock().unwrap();
    (size, count_val)
}

pub fn scan_dir_lazy(
    path: &Path,
    cache: EnhancedSizeCache,
    status_tx: &Sender<ScanStatus>,
) -> (Vec<DirEntry>, u64, u64) {
    let pb = ProgressBar::new_spinner();
    pb.set_message(format!("Scanning {}", path.display()));

    let processed_paths = Arc::new(Mutex::new(HashSet::with_capacity(CHUNK_SIZE)));

    // チャンク単位でのエントリ収集
    let mut all_entries = Vec::new();
    let mut walker = WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !should_skip(e.path()))
        .peekable();

    while walker.peek().is_some() {
        let chunk: Vec<_> = walker.by_ref().take(CHUNK_SIZE).collect();

        let chunk_entries: Vec<_> = chunk
            .into_par_iter()
            .filter_map(|entry| {
                let metadata = entry.metadata().ok()?;
                let path = entry.path().to_path_buf();

                let mut paths = processed_paths.lock().unwrap();
                if paths.insert(path.clone()) {
                    Some((path, metadata))
                } else {
                    None
                }
            })
            .collect();

        all_entries.extend(chunk_entries);

        // メモリ使用量を抑えるために定期的にキャッシュを保存
        if all_entries.len() % (CHUNK_SIZE * 10) == 0 {
            cache.save_cache();
        }
    }

    // ファイルとディレクトリに分割
    let (files, dirs): (Vec<_>, Vec<_>) = all_entries
        .into_iter()
        .partition(|(_, metadata)| metadata.is_file());

    // ファイル処理
    let total_files_size: u64 = files.iter().map(|(_, metadata)| metadata.len()).sum();

    let mut file_entries: Vec<_> = files
        .into_iter()
        .map(|(path, metadata)| DirEntry {
            path,
            size: metadata.len(),
            is_dir: false,
            children_count: 0,
        })
        .collect();

    // ディレクトリ処理
    let dir_entries: Vec<_> = dirs
        .into_par_iter()
        .filter_map(|(path, _)| {
            if let Some((cached_size, count)) = cache.get(&path) {
                let _ = status_tx.send(ScanStatus::Processing(format!(
                    "Using cache for: {}",
                    path.display()
                )));
                Some(DirEntry {
                    path,
                    size: cached_size,
                    is_dir: true,
                    children_count: count,
                })
            } else {
                let _ = status_tx.send(ScanStatus::Processing(format!(
                    "Scanning directory: {}",
                    path.display()
                )));
                let (size, count) = get_dir_info(&path, None, Some(status_tx));
                cache.insert(path.clone(), size, count);
                Some(DirEntry {
                    path,
                    size,
                    is_dir: true,
                    children_count: count,
                })
            }
        })
        .collect();

    let dir_size: u64 = dir_entries.iter().map(|e| e.size).sum();
    file_entries.extend(dir_entries);

    // サイズでソートして上位20件を取得
    file_entries.par_sort_unstable_by(|a, b| b.size.cmp(&a.size));
    let shown_entries: Vec<_> = file_entries.into_iter().take(20).collect();

    let total_size = total_files_size + dir_size;
    let shown_size: u64 = shown_entries.iter().map(|e| e.size).sum();
    let others_size = total_size.saturating_sub(shown_size);

    let _ = status_tx.send(ScanStatus::Done);
    pb.finish_and_clear();

    cache.save_cache();

    (shown_entries, total_size, others_size)
}
