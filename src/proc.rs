use sysinfo::Pid;


#[derive(Debug, Clone, Copy)]
struct Proc {
    name: &str,
    pid: Pid,
    parent_name: &str,
    parent_pid: Pid,
    cpu_usage_per: f32,
    date: String,
}

impl Proc {
    fn new(name: &str, pid: Pid, parent_name: &str, parent_pid: Pid, cpu_usage_per: f32, date: String) -> Self {
        Proc { name, pid, parent_name, parent_pid, cpu_usage_per, date }
    }
}
