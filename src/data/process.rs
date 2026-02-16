use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessStatus {
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Dead,
    Unknown,
}

impl ProcessStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Running => "Run",
            Self::Sleeping => "Sleep",
            Self::Stopped => "Stop",
            Self::Zombie => "Zombie",
            Self::Dead => "Dead",
            Self::Unknown => "?",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortColumn {
    Pid,
    Name,
    User,
    Cpu,
    Memory,
    Status,
    Threads,
    StartTime,
    DiskRead,
    DiskWrite,
    Ppid,
}

#[derive(Debug, Clone)]
pub struct ProcessEntry {
    pub pid: u32,
    pub name: String,
    pub user: String,
    pub cpu_percent: f32,
    pub memory_bytes: u64,
    pub status: ProcessStatus,
    pub thread_count: u32,
    pub disk_read_bytes: u64,
    pub disk_write_bytes: u64,
    pub start_time: u64,
    pub parent_pid: Option<u32>,
    pub command: String,
    pub is_dead: bool,
}

pub struct ProcessList {
    entries: Vec<ProcessEntry>,
    filtered_indices: Vec<usize>,
    pub sort_column: SortColumn,
    pub sort_ascending: bool,
    pub search_query: String,
    pinned_pids: HashSet<u32>,
    pinned_names: HashSet<String>,
    hidden_names: HashSet<String>,
    /// Last known data for pinned processes (to show as ghost when they die)
    last_pinned_data: HashMap<String, ProcessEntry>,
    /// When each dead pinned process was first detected as dead
    dead_pin_times: HashMap<String, Instant>,
    /// Skip dead pin creation on first load (app restart shouldn't show ghosts)
    initial_load_done: bool,
}

impl ProcessList {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            filtered_indices: Vec::new(),
            sort_column: SortColumn::Cpu,
            sort_ascending: false,
            search_query: String::new(),
            pinned_pids: HashSet::new(),
            pinned_names: HashSet::new(),
            hidden_names: HashSet::new(),
            last_pinned_data: HashMap::new(),
            dead_pin_times: HashMap::new(),
            initial_load_done: false,
        }
    }

    pub fn update(&mut self, new_entries: Vec<ProcessEntry>) {
        // Save last known data for all live pinned processes before replacing
        for entry in &self.entries {
            if self.pinned_names.contains(&entry.name) && !entry.is_dead {
                self.last_pinned_data.insert(entry.name.clone(), entry.clone());
            }
        }

        if !self.initial_load_done {
            // First load: sort and set
            let mut entries = new_entries;
            self.sort_entries(&mut entries);
            self.entries = entries;
            self.initial_load_done = true;
        } else {
            // Subsequent updates: rebuild list preserving old order with fresh data
            let mut new_map: HashMap<u32, ProcessEntry> =
                new_entries.into_iter().map(|e| (e.pid, e)).collect();

            // Keep old order, replace with fresh data (skip dead entries from previous cycle)
            let mut updated: Vec<ProcessEntry> = Vec::with_capacity(new_map.len());
            for old in &self.entries {
                if old.is_dead {
                    continue; // dead entries are re-added below if still dead
                }
                if let Some(fresh) = new_map.remove(&old.pid) {
                    updated.push(fresh);
                }
            }

            // Append brand-new processes at the end
            for (_pid, entry) in new_map {
                updated.push(entry);
            }

            // Detect dead pinned processes and add ghost entries
            let live_names: HashSet<String> = updated.iter().map(|e| e.name.clone()).collect();
            let mut ghosts: Vec<ProcessEntry> = Vec::new();
            for pinned_name in &self.pinned_names {
                if !live_names.contains(pinned_name) {
                    // Pinned process is gone - create ghost from last known data
                    if let Some(mut ghost) = self.last_pinned_data.get(pinned_name).cloned() {
                        ghost.is_dead = true;
                        ghost.cpu_percent = 0.0;
                        ghost.status = ProcessStatus::Dead;
                        ghost.disk_read_bytes = 0;
                        ghost.disk_write_bytes = 0;
                        ghost.thread_count = 0;

                        if !self.dead_pin_times.contains_key(pinned_name) {
                            self.dead_pin_times.insert(pinned_name.clone(), Instant::now());
                        }

                        ghosts.push(ghost);
                    }
                } else {
                    // Process is alive again - remove from dead tracking
                    self.dead_pin_times.remove(pinned_name);
                }
            }
            updated.extend(ghosts);

            self.entries = updated;
        }
        self.rebuild_filter();
    }

    /// Remove dead pinned processes older than the given duration.
    /// Returns names that were pruned.
    pub fn prune_dead_pins(&mut self, max_age: std::time::Duration) -> Vec<String> {
        let now = Instant::now();
        let expired: Vec<String> = self
            .dead_pin_times
            .iter()
            .filter(|(_, died_at)| now.duration_since(**died_at) >= max_age)
            .map(|(name, _)| name.clone())
            .collect();

        for name in &expired {
            self.dead_pin_times.remove(name);
            self.last_pinned_data.remove(name);
            self.pinned_names.remove(name);
            self.pinned_pids.retain(|pid| {
                !self.entries.iter().any(|e| e.pid == *pid && &e.name == name)
            });
            self.entries.retain(|e| !(e.is_dead && &e.name == name));
        }

        if !expired.is_empty() {
            self.rebuild_filter();
        }
        expired
    }

    pub fn dead_pin_count(&self) -> usize {
        self.dead_pin_times.len()
    }

    pub fn visible(&self) -> Vec<&ProcessEntry> {
        self.filtered_indices
            .iter()
            .map(|&i| &self.entries[i])
            .collect()
    }

    pub fn visible_count(&self) -> usize {
        self.filtered_indices.len()
    }

    pub fn find_visible_index_by_pid(&self, pid: u32) -> Option<usize> {
        self.filtered_indices
            .iter()
            .position(|&i| self.entries[i].pid == pid)
    }

    /// Find display row index for a PID, accounting for pinned-first ordering + separator
    pub fn find_display_index_by_pid(&self, pid: u32) -> Option<usize> {
        if !self.has_pins() {
            return self.find_visible_index_by_pid(pid);
        }

        let is_target_pinned = self.pinned_pids.contains(&pid);
        if is_target_pinned {
            // Search in pinned region
            self.filtered_indices
                .iter()
                .filter(|&&i| self.pinned_pids.contains(&self.entries[i].pid))
                .position(|&i| self.entries[i].pid == pid)
        } else {
            // Search in unpinned region, offset by pinned_count + 1 (separator)
            let pinned_count = self
                .filtered_indices
                .iter()
                .filter(|&&i| self.pinned_pids.contains(&self.entries[i].pid))
                .count();
            self.filtered_indices
                .iter()
                .filter(|&&i| !self.pinned_pids.contains(&self.entries[i].pid))
                .position(|&i| self.entries[i].pid == pid)
                .map(|pos| pinned_count + 1 + pos)
        }
    }

    pub fn total_count(&self) -> usize {
        self.entries.len()
    }

    pub fn get_by_visible_index(&self, index: usize) -> Option<&ProcessEntry> {
        self.filtered_indices
            .get(index)
            .map(|&i| &self.entries[i])
    }

    /// Returns the process for a given display row index.
    /// When pins exist, the display order is: pinned processes, separator row, unpinned processes.
    /// The separator row returns None.
    pub fn get_by_display_index(&self, index: usize) -> Option<&ProcessEntry> {
        if !self.has_pins() {
            return self.get_by_visible_index(index);
        }

        let pinned: Vec<usize> = self
            .filtered_indices
            .iter()
            .copied()
            .filter(|&i| self.pinned_pids.contains(&self.entries[i].pid))
            .collect();
        let pinned_count = pinned.len();

        if index < pinned_count {
            // Pinned region
            Some(&self.entries[pinned[index]])
        } else if index == pinned_count {
            // Separator row
            None
        } else {
            // Unpinned region (index - pinned_count - 1 for separator)
            let unpinned: Vec<usize> = self
                .filtered_indices
                .iter()
                .copied()
                .filter(|&i| !self.pinned_pids.contains(&self.entries[i].pid))
                .collect();
            let unpinned_idx = index - pinned_count - 1;
            unpinned.get(unpinned_idx).map(|&i| &self.entries[i])
        }
    }

    /// Total number of display rows including the separator when pins exist.
    pub fn display_row_count(&self) -> usize {
        if self.has_pins() {
            self.filtered_indices.len() + 1 // +1 for separator
        } else {
            self.filtered_indices.len()
        }
    }

    pub fn set_sort(&mut self, column: SortColumn, ascending: bool) {
        self.sort_column = column;
        self.sort_ascending = ascending;
        let mut entries = std::mem::take(&mut self.entries);
        self.sort_entries(&mut entries);
        self.entries = entries;
        self.rebuild_filter();
    }

    pub fn all_entries(&self) -> &[ProcessEntry] {
        &self.entries
    }

    pub fn toggle_pin(&mut self, pid: u32) {
        if !self.pinned_pids.remove(&pid) {
            self.pinned_pids.insert(pid);
            // Also track by name for persistence
            if let Some(entry) = self.entries.iter().find(|e| e.pid == pid) {
                self.pinned_names.insert(entry.name.clone());
            }
        } else {
            // Remove from name set too
            if let Some(entry) = self.entries.iter().find(|e| e.pid == pid) {
                self.pinned_names.remove(&entry.name);
            }
        }
    }

    /// Pin all processes with the given name
    pub fn pin_by_name(&mut self, name: &str) {
        self.pinned_names.insert(name.to_string());
        for entry in &self.entries {
            if entry.name == name {
                self.pinned_pids.insert(entry.pid);
            }
        }
    }

    /// Unpin all processes with the given name
    pub fn unpin_by_name(&mut self, name: &str) {
        self.pinned_names.remove(name);
        let pids_to_remove: Vec<u32> = self
            .entries
            .iter()
            .filter(|e| e.name == name)
            .map(|e| e.pid)
            .collect();
        for pid in pids_to_remove {
            self.pinned_pids.remove(&pid);
        }
        // Also clean up dead pin data for this name
        self.dead_pin_times.remove(name);
        self.last_pinned_data.remove(name);
        self.entries.retain(|e| !(e.is_dead && e.name == name));
        self.rebuild_filter();
    }

    pub fn clear_pins(&mut self) {
        self.pinned_pids.clear();
        self.pinned_names.clear();
        self.dead_pin_times.clear();
        self.last_pinned_data.clear();
        self.entries.retain(|e| !e.is_dead);
        self.rebuild_filter();
    }

    pub fn is_pinned(&self, pid: u32) -> bool {
        self.pinned_pids.contains(&pid)
    }

    pub fn has_pins(&self) -> bool {
        !self.pinned_pids.is_empty()
    }

    pub fn pin_count(&self) -> usize {
        self.pinned_pids.len()
    }

    pub fn toggle_hidden(&mut self, pid: u32) {
        if let Some(entry) = self.entries.iter().find(|e| e.pid == pid) {
            let name = entry.name.clone();
            if self.hidden_names.contains(&name) {
                self.hidden_names.remove(&name);
            } else {
                self.hidden_names.insert(name);
            }
        }
        self.rebuild_filter();
    }

    pub fn is_hidden_name(&self, name: &str) -> bool {
        self.hidden_names.contains(name)
    }

    pub fn hidden_count(&self) -> usize {
        self.hidden_names.len()
    }

    pub fn unhide_by_name(&mut self, name: &str) {
        self.hidden_names.remove(name);
        self.rebuild_filter();
    }

    pub fn clear_hidden(&mut self) {
        self.hidden_names.clear();
        self.rebuild_filter();
    }

    pub fn pinned_names(&self) -> &HashSet<String> {
        &self.pinned_names
    }

    pub fn hidden_names(&self) -> &HashSet<String> {
        &self.hidden_names
    }

    pub fn load_persisted_state(&mut self, pinned: HashSet<String>, hidden: HashSet<String>) {
        self.pinned_names = pinned;
        self.hidden_names = hidden;
    }

    pub fn resolve_pinned_pids(&mut self) {
        // After an update, resolve pinned names to PIDs
        self.pinned_pids.clear();
        for entry in &self.entries {
            if self.pinned_names.contains(&entry.name) {
                self.pinned_pids.insert(entry.pid);
            }
        }
    }

    /// Check if a name is pinned (by name set)
    pub fn is_name_pinned(&self, name: &str) -> bool {
        self.pinned_names.contains(name)
    }

    /// Count how many visible processes share the given name
    pub fn count_by_name(&self, name: &str) -> usize {
        self.filtered_indices
            .iter()
            .filter(|&&i| self.entries[i].name == name)
            .count()
    }

    pub fn pinned_visible(&self) -> Vec<&ProcessEntry> {
        self.filtered_indices
            .iter()
            .map(|&i| &self.entries[i])
            .filter(|e| self.pinned_pids.contains(&e.pid))
            .collect()
    }

    pub fn unpinned_visible(&self) -> Vec<&ProcessEntry> {
        self.filtered_indices
            .iter()
            .map(|&i| &self.entries[i])
            .filter(|e| !self.pinned_pids.contains(&e.pid))
            .collect()
    }

    pub fn set_search(&mut self, query: String) {
        self.search_query = query;
        self.rebuild_filter();
    }

    fn rebuild_filter(&mut self) {
        let query = self.search_query.to_lowercase();
        self.filtered_indices = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                // Always hide hidden processes from main list
                if self.hidden_names.contains(&e.name) {
                    return false;
                }
                if query.is_empty() {
                    return true;
                }
                e.name.to_lowercase().contains(&query)
                    || e.user.to_lowercase().contains(&query)
                    || e.pid.to_string().contains(&query)
            })
            .map(|(i, _)| i)
            .collect();
    }

    fn sort_entries(&self, entries: &mut Vec<ProcessEntry>) {
        let ascending = self.sort_ascending;
        entries.sort_by(|a, b| {
            let ord = match self.sort_column {
                SortColumn::Pid => a.pid.cmp(&b.pid),
                SortColumn::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortColumn::User => a.user.to_lowercase().cmp(&b.user.to_lowercase()),
                SortColumn::Cpu => a
                    .cpu_percent
                    .partial_cmp(&b.cpu_percent)
                    .unwrap_or(Ordering::Equal),
                SortColumn::Memory => a.memory_bytes.cmp(&b.memory_bytes),
                SortColumn::Status => a.status.as_str().cmp(b.status.as_str()),
                SortColumn::Threads => a.thread_count.cmp(&b.thread_count),
                SortColumn::StartTime => a.start_time.cmp(&b.start_time),
                SortColumn::DiskRead => a.disk_read_bytes.cmp(&b.disk_read_bytes),
                SortColumn::DiskWrite => a.disk_write_bytes.cmp(&b.disk_write_bytes),
                SortColumn::Ppid => a.parent_pid.unwrap_or(0).cmp(&b.parent_pid.unwrap_or(0)),
            };
            if ascending {
                ord
            } else {
                ord.reverse()
            }
        });
    }
}
