// src/app.rs
use crate::{
    scanner::scan_dir_lazy,
    types::{DirEntry, EnhancedSizeCache, ScanStatus},
};
use anyhow::Result;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::time::Instant;

pub struct App {
    pub entries: Vec<DirEntry>,
    pub current_path: PathBuf,
    pub initial_path: PathBuf,
    pub selected: usize,
    pub scroll_offset: usize,
    pub total_size: u64,
    pub size_cache: EnhancedSizeCache,
    pub others_size: u64,
    scan_status_rx: Receiver<ScanStatus>,
    current_status: Option<String>,
    pub is_scanning: bool,
    pub show_popup: bool,
    last_action: Instant,
    needs_refresh: bool,
    window_size: Option<(u16, u16)>,
}

impl App {
    pub fn new(path: PathBuf) -> Result<App> {
        let size_cache = EnhancedSizeCache::new();
        let (status_tx, status_rx) = mpsc::channel();
        let initial_path = path.clone();
        let (entries, total_size, others_size) =
            scan_dir_lazy(&path, size_cache.clone(), &status_tx);

        Ok(App {
            entries,
            current_path: path,
            initial_path,
            selected: 0,
            scroll_offset: 0,
            total_size,
            size_cache,
            others_size,
            scan_status_rx: status_rx,
            current_status: None,
            is_scanning: true,
            show_popup: true,
            last_action: Instant::now(),
            needs_refresh: true,
            window_size: None,
        })
    }

    pub fn reset_scroll(&mut self) {
        self.selected = 0;
        self.scroll_offset = 0;
        self.mark_action();
    }

    pub fn up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            if self.selected < self.scroll_offset {
                self.scroll_offset = self.selected;
            }
        }
        self.mark_action();
    }

    pub fn down(&mut self) {
        let max = self.entries.len() + if self.others_size > 0 { 1 } else { 0 };
        if self.selected + 1 < max {
            self.selected += 1;
            if let Some((_, height)) = self.window_size {
                let visible_items = (height as usize).saturating_sub(3);
                if self.selected >= self.scroll_offset + visible_items {
                    self.scroll_offset = self.selected.saturating_sub(visible_items) + 1;
                }
            }
        }
        self.mark_action();
    }

    pub fn enter(&mut self) -> Result<bool> {
        if self.selected >= self.entries.len() {
            return Ok(false);
        }

        if let Some(entry) = self.entries.get(self.selected) {
            if entry.is_dir {
                self.is_scanning = true;
                self.show_popup = true;
                let new_path = entry.path.clone();
                let (status_tx, new_rx) = mpsc::channel();
                let (entries, total_size, others_size) =
                    scan_dir_lazy(&new_path, self.size_cache.clone(), &status_tx);
                self.entries = entries;
                self.total_size = total_size;
                self.others_size = others_size;
                self.current_path = new_path;
                self.scan_status_rx = new_rx;
                self.reset_scroll();
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn back(&mut self) -> Result<bool> {
        if let Some(parent) = self.current_path.parent().map(|p| p.to_path_buf()) {
            if parent.starts_with(&self.initial_path) {
                self.is_scanning = true;
                self.show_popup = true;
                let (status_tx, new_rx) = mpsc::channel();
                let (entries, total_size, others_size) =
                    scan_dir_lazy(&parent, self.size_cache.clone(), &status_tx);
                self.entries = entries;
                self.total_size = total_size;
                self.others_size = others_size;
                self.current_path = parent;
                self.scan_status_rx = new_rx;
                self.reset_scroll();
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn handle_resize(&mut self, width: u16, height: u16) {
        self.window_size = Some((width, height));
        if let Some((_, height)) = self.window_size {
            let visible_items = (height as usize).saturating_sub(3);
            if self.selected >= self.scroll_offset + visible_items {
                self.scroll_offset = self.selected.saturating_sub(visible_items) + 1;
            }
        }
        self.needs_refresh = true;
        self.mark_action();
    }

    pub fn get_visible_entries(&self) -> Vec<&DirEntry> {
        let visible_count = if let Some((_, height)) = self.window_size {
            (height as usize).saturating_sub(3)
        } else {
            self.entries.len()
        };

        self.entries
            .iter()
            .skip(self.scroll_offset)
            .take(visible_count)
            .collect()
    }

    pub fn update_status(&mut self) -> bool {
        let mut status_changed = false;
        while let Ok(status) = self.scan_status_rx.try_recv() {
            match status {
                ScanStatus::Processing(msg) => {
                    self.current_status = Some(msg);
                    self.is_scanning = true;
                    self.show_popup = true;
                    status_changed = true;
                }
                ScanStatus::Done => {
                    self.current_status = None;
                    self.is_scanning = false;
                    self.show_popup = false;
                    status_changed = true;
                    self.size_cache.save_cache();
                }
            }
        }
        if status_changed {
            self.needs_refresh = true;
        }
        status_changed
    }

    pub fn cleanup(&mut self) {
        self.size_cache.save_cache();
    }

    pub fn get_scan_status(&self) -> Option<String> {
        self.current_status.clone()
    }

    pub fn is_scanning(&self) -> bool {
        self.is_scanning
    }

    pub fn mark_action(&mut self) {
        self.last_action = Instant::now();
        self.needs_refresh = true;
    }

    pub fn needs_refresh(&mut self) -> bool {
        if self.needs_refresh {
            self.needs_refresh = false;
            return true;
        }
        if self.last_action.elapsed().as_secs() >= 10 {
            self.mark_action();
            return true;
        }
        false
    }

    pub fn get_window_size(&self) -> Option<(u16, u16)> {
        self.window_size
    }
}
