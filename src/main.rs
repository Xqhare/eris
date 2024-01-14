use std::{thread, time::Duration, collections::HashMap, hash::RandomState, sync::{Arc, atomic::{AtomicBool, Ordering}}, io::Error};

use chrono::{Utc, SecondsFormat};
use signal_hook::{consts::TERM_SIGNALS, flag};
use sysinfo::{
    System, Pid, Process,
};

use crate::proc::Proc;

mod proc;
mod jisard;

const CPU_HIGH_THRESHOLD: f32 = 75.0;
const MAIN_PROCESS_NAME: &str = "systemd";
const ALT_PROC_NAME: &str = "gnome-shell";
const PROCESS_LOGGING_AMOUNT: usize = 15;
// This needs to be longer than the min cpu update interval of 200ms.
const UPDATE_INTERVAL: Duration = Duration::from_millis(500);
const FILENAME: &str = "eris.json";


fn main() -> Result<(), Error> {
    // Eris init
    let mut sys = System::new_all();
    let real_cpu_cores: Option<usize> = sys.physical_core_count();
    let cpu_cores: f32 = {
        if real_cpu_cores.is_some() {
            // I just assume standard hyperthreading. Nothing fancy, but also no single threads!
            // I really like this syntax
            let parsing = real_cpu_cores.unwrap() as f32;
            parsing * 2.0
        } else {
            // Hyperthreading for a single core default too, however this is the fallback, so use 1
            // for data consistency for the user.
            1.0
        }
    };
    sys.refresh_all();
    // I believe with the rework of loop into while, the start of main will be executed again,
    // as it has to run the for loop to determine
    let mut fist_start = true;
    if fist_start {
            thread::sleep(UPDATE_INTERVAL);
            fist_start = false;
        }
    // System interrupts
    // A thread share save boolean. It's passed to `flag::register` setting it to true for the first kill command recieved.
    let term_now = Arc::new(AtomicBool::new(false));
    for sig in TERM_SIGNALS {
        // If second termination signal is recieved, I'll just kill myself
        flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
        // This will arm the above for a second time. Order is improtant!
        flag::register(*sig, Arc::clone(&term_now))?;
    }
        
    // This is the main loop. As this is supposed to be put in autorun an be on forever, it loops as long as `term_now` is set to false.
    while !term_now.load(Ordering::Relaxed) {
        sys.refresh_cpu();
        sys.refresh_processes();
        sys.refresh_cpu_usage();
        // This is enough for 255 cores... Also cpu cores start at 1!
        for cpu in sys.cpus() {
            if cpu.cpu_usage() > CPU_HIGH_THRESHOLD {
                // Determining of parent process:
                let cpu_hogs_parents = cpu_hogs_parents(cpu_hogs(sys.processes()));
                let mut new_proc_data: Vec<Proc> = Default::default();
                // Gets the current date with milliseconds and timezone.
                let date = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, false);
                for hog in cpu_hogs_parents {
                    let hog_pid = hog.0.0;
                    let hog_proc = hog.0.1;
                    let hog_name = hog_proc.name().to_string();
                    // Divide by cpu_cores to get percentage per single core. numbers will be
                    // larger than 100% otherwise.
                    let cpu_usage_perc = hog_proc.cpu_usage() / cpu_cores;
                    let parent_pid = hog.1;
                    let parent_proc = sys.process(parent_pid);
                    let mut parent_name: String = Default::default();
                    if parent_proc.is_some() {
                        parent_name = parent_proc.unwrap().name().to_string();
                    } else {
                        parent_name = "ErisFoundNoParent".to_string();
                    }
                    let vir_mem: u64 = hog_proc.virtual_memory();
                    let disc_use = hog_proc.disk_usage();
                    let total_disc_read: u64 = disc_use.total_read_bytes;
                    let total_disc_write: u64 = disc_use.total_written_bytes;
                    let run_time: u64 = hog_proc.run_time();
                    let usr_id = {
                        if hog_proc.user_id().is_some() {
                            hog_proc.user_id().unwrap().to_string()
                        } else {
                            "ErisFoundNoUser".to_string()
                        }
                    };
                    let new_proc = Proc {name: hog_name, pid: hog_pid, parent_name, parent_pid, cpu_usage_per: cpu_usage_perc, date: date.clone(), vir_mem, total_disc_read, total_disc_write, run_time, usr_id};
                    new_proc_data.push(new_proc);
                }
                jisard::write_state(new_proc_data, FILENAME);
            }
        }
        // Remove the wait for system SIGTERM, no need to delay.
        if !term_now.load(Ordering::Relaxed) {
            thread::sleep(UPDATE_INTERVAL);
        }
    }
    // This code is only executed AFTER a kill signal was recieved.
    // But as long as eris is shut down gracefully, and the final json is saved succesfully, there
    // really is nothing left to do.
    
    // Testing of giving a return signal to kernel
    for sig in TERM_SIGNALS {
        flag::register_conditional_shutdown(*sig, 1, Arc::new(AtomicBool::new(true)))?;
    }
    Ok(())
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

