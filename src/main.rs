use std::{thread, time::Duration, collections::HashMap, hash::RandomState};

use sysinfo::{
    System, Pid, Process,
};

const CPU_HIGH_THRESHOLD: f32 = 75.0;
const MAIN_PROCESS_NAME: &str = "systemd";
const ALT_PROC_NAME: &str = "gnome-shell";
const PROCESS_LOGGING_AMOUNT: usize = 15;
// This needs to be longer than the min cpu update interval of 200ms.
const UPDATE_INTERVAL: Duration = Duration::from_millis(500);

fn main() {
    let mut sys = System::new_all();
    let mut loop_counter: usize  = 0;
    let real_cpu_cores: Option<usize> = sys.physical_core_count();
    // I really like this syntax
    let cpu_cores: f32 = {
        if real_cpu_cores.is_some() {
            // I just assume standard hyperthreading. Nothing fancy, but also no single threads!
            let parsing = real_cpu_cores.unwrap() as f32;
            parsing * 2.0
        } else {
            // Hyperthreading for a single core default too, however this is the fallback, so use 1
            // for data consistency for the user.
            1.0
        }
    };
    println!("CPU CORES {cpu_cores}");
    sys.refresh_all();
    // This waits for 200ms! This is on top of any processing time eris itself needs.
    thread::sleep(UPDATE_INTERVAL);
    // This is the main loop. As this is supposed to be put in autorun an be on forever, it loops
    // forever.
    loop {
        sys.refresh_cpu();
        sys.refresh_processes();
        sys.refresh_cpu_usage();
        // This is enough for 255 cores... Also cpu cores start at 1!
        let mut cpu_core_counter: u8 = 1;
        for cpu in sys.cpus() {
            // println!("{} core | {}% usage", cpu_core_counter, cpu.cpu_usage());
            if cpu.cpu_usage() > CPU_HIGH_THRESHOLD {
                println!("HIGH CPU USAGE DETECTED! CPU {}", cpu_core_counter);
                // Determining of parent process:
                let cpu_hogs_parents = cpu_hogs_parents(cpu_hogs(sys.processes()));
                // WIP: CAN PANIC!!
                for hog in cpu_hogs_parents {
                    let mut sys2 = System::new();
                    sys2.refresh_processes();
                    let hog_pid = hog.0.0;
                    let hog_proc = hog.0.1;
                    let hog_name = hog_proc.name();
                    let cpu_usage_perc = hog_proc.cpu_usage();
                    let parent_pid = hog.1;
                    let parent_name = sys2.process(parent_pid).unwrap().name();
                    println!("[{hog_pid}] ({hog_name}) PARENT {parent_name} | {cpu_usage_perc}");
                }
            }
            cpu_core_counter += 1;
        }
        cpu_core_counter = 1;

        println!("loop {}", loop_counter);
        loop_counter += 1;
        thread::sleep(UPDATE_INTERVAL);
    }
}
// This determines the actual process, not what parent it belongs to.
fn cpu_hogs(processes: &HashMap<Pid, Process, RandomState>) -> Vec<(Pid, &Process)> {
    let mut out: Vec<(Pid, &Process)> = Vec::new();
    for (pid, process) in processes {
        if out.len() < PROCESS_LOGGING_AMOUNT {
            if out.len() == 0 {
                out.push((pid.to_owned(), process));
            } else {
                let mut index: usize = 0;
                let mut insert = false;
                for entry in &out {
                    if entry.1.cpu_usage() < process.cpu_usage() {
                        insert = true;
                        break;
                    }
                    index += 1;
                }
                if insert {
                    out.insert(index, (pid.to_owned(), process));
                } else {
                    out.push((pid.to_owned(), process));
                }
            }
        } else {
            let mut index: usize = 0;
            let mut insert = false;
            for entry in &out {
                if entry.1.cpu_usage() < process.cpu_usage() {
                    insert = true;
                    break;
                }
                index += 1;
            }
            if insert {
                out.remove(index);
                out.insert(index, (pid.to_owned(), process))
            }
        }
    }
    return out;
}
/// Returns the cpu hog first, the parent second.
fn cpu_hogs_parents(cpu_hogs: Vec<(Pid, &Process)>) -> Vec<((Pid, &Process), Pid)> {
    let mut out: Vec<((Pid, &Process), Pid)> = Vec::new();
    let mut sys = System::new();
    sys.refresh_processes();
    for hog in cpu_hogs {
        let mut parent: (Pid, &Process) = hog;
        loop {
            let poss_parent = parent.1.parent();
            if poss_parent.is_some() {
                let poss_parent_proc = sys.process(poss_parent.unwrap());
                if poss_parent_proc.is_some() {
                    if poss_parent_proc.unwrap().name() == MAIN_PROCESS_NAME || poss_parent_proc.unwrap().name() == ALT_PROC_NAME {
                        break;
                    } else {
                        parent = (poss_parent.unwrap(), poss_parent_proc.unwrap());
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        out.push((hog, parent.0));
    }
    return out;
}
