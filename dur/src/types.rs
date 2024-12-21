use dashmap::DashMap;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DirEntry {
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
    pub is_scanned: bool,
    pub children_count: usize,
}

pub struct SizeCache {
    cache: Arc<DashMap<PathBuf, (u64, usize)>>,
}

impl SizeCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
        }
    }

    pub fn get(&self, path: &PathBuf) -> Option<(u64, usize)> {
        self.cache.get(path).map(|v| *v)
    }

    pub fn insert(&self, path: PathBuf, size: u64, count: usize) {
        self.cache.insert(path, (size, count));
    }
}
