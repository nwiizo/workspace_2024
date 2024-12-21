use crate::{
    scanner::scan_dir_lazy,
    types::{DirEntry, SizeCache},
};
use anyhow::Result;
use std::path::PathBuf;

pub struct App {
    pub entries: Vec<DirEntry>,
    pub current_path: PathBuf,
    pub selected: usize,
    pub total_size: u64,
    pub size_cache: SizeCache,
    pub others_size: u64,
}

impl App {
    pub fn new(path: PathBuf, threads: usize) -> Result<App> {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()?;

        let size_cache = SizeCache::new();
        let (entries, total_size, others_size) = scan_dir_lazy(&path, &size_cache);

        Ok(App {
            entries,
            current_path: path,
            selected: 0,
            total_size,
            size_cache,
            others_size,
        })
    }

    pub fn up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub fn down(&mut self) {
        if self.selected + 1 < self.entries.len() {
            self.selected += 1;
        }
    }

    pub fn enter(&mut self) -> Result<bool> {
        if let Some(entry) = self.entries.get(self.selected) {
            if entry.is_dir {
                let new_path = entry.path.clone();
                let (entries, total_size, others_size) = scan_dir_lazy(&new_path, &self.size_cache);
                self.entries = entries;
                self.total_size = total_size;
                self.others_size = others_size;
                self.current_path = new_path;
                self.selected = 0;
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn back(&mut self) -> Result<bool> {
        if let Some(parent) = self.current_path.parent().map(|p| p.to_path_buf()) {
            let (entries, total_size, others_size) = scan_dir_lazy(&parent, &self.size_cache);
            self.entries = entries;
            self.total_size = total_size;
            self.others_size = others_size;
            self.current_path = parent;
            self.selected = 0;
            return Ok(true);
        }
        Ok(false)
    }
}
