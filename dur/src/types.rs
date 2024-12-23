// src/types.rs
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

const CACHE_VERSION: u32 = 1;
const CACHE_EXPIRY: Duration = Duration::from_secs(3600 * 24); // 24時間

#[derive(Debug, Clone)]
pub struct DirEntry {
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
    pub children_count: usize,
}

#[derive(Debug, Clone)]
pub enum ScanStatus {
    Processing(String),
    Done,
}

#[derive(Serialize, Deserialize)]
struct CacheEntry {
    size: u64,
    count: usize,
    last_modified: SystemTime,
    timestamp: SystemTime,
    version: u32,
}

#[derive(Clone)]
pub struct EnhancedSizeCache {
    memory_cache: Arc<Mutex<HashMap<PathBuf, (u64, usize)>>>,
    disk_cache_path: PathBuf,
    dirty: Arc<Mutex<bool>>,
}

impl EnhancedSizeCache {
    pub fn new() -> Self {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from(".cache"))
            .join("dur");
        fs::create_dir_all(&cache_dir).unwrap_or_default();

        let cache = Self {
            memory_cache: Arc::new(Mutex::new(HashMap::new())),
            disk_cache_path: cache_dir.join("dircache.json"),
            dirty: Arc::new(Mutex::new(false)),
        };

        cache.load_cache();
        cache
    }

    fn load_cache(&self) {
        if let Ok(file) = File::open(&self.disk_cache_path) {
            let reader = BufReader::new(file);
            if let Ok(cache_data) =
                serde_json::from_reader::<_, HashMap<PathBuf, CacheEntry>>(reader)
            {
                let mut memory_cache = self.memory_cache.lock().unwrap();
                let now = SystemTime::now();

                for (path, entry) in cache_data {
                    if entry.version != CACHE_VERSION {
                        continue;
                    }

                    if let Ok(duration) = now.duration_since(entry.timestamp) {
                        if duration < CACHE_EXPIRY {
                            if let Ok(metadata) = fs::metadata(&path) {
                                if let Ok(last_modified) = metadata.modified() {
                                    if last_modified == entry.last_modified {
                                        memory_cache.insert(path, (entry.size, entry.count));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn save_cache(&self) {
        if !*self.dirty.lock().unwrap() {
            return;
        }

        let memory_cache = self.memory_cache.lock().unwrap();
        let now = SystemTime::now();

        let mut cache_data = HashMap::new();
        for (path, (size, count)) in memory_cache.iter() {
            if let Ok(metadata) = fs::metadata(path) {
                if let Ok(last_modified) = metadata.modified() {
                    cache_data.insert(
                        path.clone(),
                        CacheEntry {
                            size: *size,
                            count: *count,
                            last_modified,
                            timestamp: now,
                            version: CACHE_VERSION,
                        },
                    );
                }
            }
        }

        if let Ok(file) = File::create(&self.disk_cache_path) {
            let writer = BufWriter::new(file);
            if serde_json::to_writer(writer, &cache_data).is_ok() {
                *self.dirty.lock().unwrap() = false;
            }
        }
    }

    pub fn get(&self, path: &PathBuf) -> Option<(u64, usize)> {
        self.memory_cache.lock().unwrap().get(path).copied()
    }

    pub fn insert(&self, path: PathBuf, size: u64, count: usize) {
        self.memory_cache
            .lock()
            .unwrap()
            .insert(path, (size, count));
        *self.dirty.lock().unwrap() = true;
    }
}
