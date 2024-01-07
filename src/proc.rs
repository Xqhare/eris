use sysinfo::Pid;

#[derive(Debug, Clone)]
pub struct Proc {
    pub name: String,
    pub pid: Pid,
    pub parent_name: String,
    pub parent_pid: Pid,
    pub cpu_usage_per: f32,
    pub date: String,
    pub vir_mem: u64,
    pub total_disc_read: u64,
    pub total_disc_write: u64,
    pub run_time: u64,
    pub usr_id: String,
}
