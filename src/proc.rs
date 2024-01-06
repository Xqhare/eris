use sysinfo::Pid;

#[derive(Debug, Clone)]
pub struct Proc {
    pub name: String,
    pub pid: Pid,
    pub parent_name: String,
    pub parent_pid: Pid,
    pub cpu_usage_per: f32,
    pub date: String,
}
