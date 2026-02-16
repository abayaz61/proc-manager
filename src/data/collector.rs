use std::collections::HashMap;

use sysinfo::{Networks, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System, UpdateKind, Users};

use super::disk::DiskData;
use super::network::NetworkData;
use super::process::{ProcessEntry, ProcessStatus};
use super::system::SystemData;

#[allow(dead_code)]
pub struct DataCollector {
    sys: System,
    networks: Networks,
    users: Users,
    pub system_data: SystemData,
    pub network_data: NetworkData,
    pub disk_data: DiskData,
    thread_counts: HashMap<u32, u32>,
}

impl DataCollector {
    pub fn new() -> Self {
        let sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(sysinfo::CpuRefreshKind::everything())
                .with_memory(sysinfo::MemoryRefreshKind::everything())
                .with_processes(
                    ProcessRefreshKind::nothing()
                        .with_cpu()
                        .with_memory()
                        .with_disk_usage()
                        .with_user(UpdateKind::OnlyIfNotSet)
                        .with_exe(UpdateKind::OnlyIfNotSet),
                ),
        );

        let users = Users::new_with_refreshed_list();

        Self {
            sys,
            networks: Networks::new_with_refreshed_list(),
            users,
            system_data: SystemData::new(),
            network_data: NetworkData::new(),
            disk_data: DiskData::new(),
            thread_counts: HashMap::new(),
        }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory_specifics(sysinfo::MemoryRefreshKind::everything());
        self.sys.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing()
                .with_cpu()
                .with_memory()
                .with_disk_usage()
                .with_exe(UpdateKind::OnlyIfNotSet),
        );
        self.networks.refresh(true);

        self.update_system_data();
        self.update_network_data();
        self.thread_counts = collect_thread_counts();
    }

    fn update_system_data(&mut self) {
        self.system_data.hostname = System::host_name().unwrap_or_default();
        self.system_data.uptime = System::uptime();
        self.system_data.total_memory = self.sys.total_memory();
        self.system_data.used_memory = self.sys.used_memory();
        self.system_data.total_swap = self.sys.total_swap();
        self.system_data.used_swap = self.sys.used_swap();
        self.system_data.global_cpu_usage = self.sys.global_cpu_usage();
        self.system_data.cpu_usage = self.sys.cpus().iter().map(|c| c.cpu_usage()).collect();
    }

    fn update_network_data(&mut self) {
        let (rx, tx) = self
            .networks
            .iter()
            .fold((0u64, 0u64), |(rx, tx), (_, data)| {
                (
                    rx + data.total_received(),
                    tx + data.total_transmitted(),
                )
            });
        self.network_data.update(rx, tx);
    }

    pub fn processes(&self) -> Vec<ProcessEntry> {
        self.sys
            .processes()
            .values()
            .map(|p| {
                let status = match p.status() {
                    sysinfo::ProcessStatus::Run => ProcessStatus::Running,
                    sysinfo::ProcessStatus::Sleep => ProcessStatus::Sleeping,
                    sysinfo::ProcessStatus::Stop => ProcessStatus::Stopped,
                    sysinfo::ProcessStatus::Zombie => ProcessStatus::Zombie,
                    _ => ProcessStatus::Unknown,
                };

                ProcessEntry {
                    pid: p.pid().as_u32(),
                    name: p.name().to_string_lossy().into_owned(),
                    user: p
                        .user_id()
                        .and_then(|uid| {
                            self.users
                                .iter()
                                .find(|u| u.id() == uid)
                                .map(|u| u.name().to_string())
                        })
                        .unwrap_or_default(),
                    cpu_percent: p.cpu_usage(),
                    memory_bytes: p.memory(),
                    status,
                    thread_count: self.thread_counts.get(&p.pid().as_u32()).copied().unwrap_or(0),
                    disk_read_bytes: p.disk_usage().read_bytes,
                    disk_write_bytes: p.disk_usage().written_bytes,
                    start_time: p.start_time(),
                    parent_pid: p.parent().map(|pid| pid.as_u32()),
                    command: p.exe().map(|e| e.to_string_lossy().into_owned()).unwrap_or_default(),
                    is_dead: false,
                }
            })
            .collect()
    }

    pub fn kill_process(&self, pid: u32) -> bool {
        if let Some(process) = self.sys.process(sysinfo::Pid::from_u32(pid)) {
            process.kill()
        } else {
            false
        }
    }
}

#[cfg(target_os = "windows")]
fn collect_thread_counts() -> HashMap<u32, u32> {
    use windows::Win32::Foundation::INVALID_HANDLE_VALUE;
    use windows::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
    };

    let mut counts = HashMap::new();

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
        let snapshot = match snapshot {
            Ok(h) => h,
            Err(_) => return counts,
        };
        if snapshot == INVALID_HANDLE_VALUE {
            return counts;
        }

        let mut entry = THREADENTRY32 {
            dwSize: std::mem::size_of::<THREADENTRY32>() as u32,
            ..Default::default()
        };

        if Thread32First(snapshot, &mut entry).is_ok() {
            loop {
                *counts.entry(entry.th32OwnerProcessID).or_insert(0) += 1;
                if Thread32Next(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }

        let _ = windows::Win32::Foundation::CloseHandle(snapshot);
    }

    counts
}

#[cfg(not(target_os = "windows"))]
fn collect_thread_counts() -> HashMap<u32, u32> {
    // On Linux, sysinfo's tasks() works; we could use /proc/<pid>/status
    // For now, return empty and fall back to 0
    HashMap::new()
}
