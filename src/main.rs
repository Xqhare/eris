use std::thread;

use sysinfo::{
    Components, Disks, Networks, System, Pid,
};

const CPU_HIGH_THRESHOLD: f32 = 75.0;

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
    loop {
        sys.refresh_cpu();
        sys.refresh_processes();
        // This is enough for 255 cores... Also cpu cores start at 1!
        let mut cpu_core_counter: u8 = 1;
        for cpu in sys.cpus() {
            println!("{} core | {}% usage", cpu_core_counter, cpu.cpu_usage());
            if cpu.cpu_usage() > CPU_HIGH_THRESHOLD {
                println!("HIGH CPU USAGE DETECTED! CPU {}", cpu_core_counter);
                let mut highest_cpu_usage: Pid = Pid::from_u32(u32::default());
                let mut highest_cpu_usage_perc: f32 = f32::default();
                let mut name = "";
                for (pid, process) in sys.processes() {
                    if process.cpu_usage() > highest_cpu_usage_perc {
                        highest_cpu_usage_perc = process.cpu_usage() / cpu_cores;
                        highest_cpu_usage = pid.to_owned();
                        name = process.name();
                        
                    }
                }
                println!("[{highest_cpu_usage}] ({name}) {highest_cpu_usage_perc}");
            }
            cpu_core_counter += 1;
        }
        cpu_core_counter = 1;

        println!("loop {}", loop_counter);
        
        loop_counter += 1;
        // This waits for 200ms!
        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}

fn sysinfo_test () {
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();
    // For accurate CPU info update twice for reference point.
    sys.refresh_all();
    println!("TEST");
    println!("One min: {:?}", System::load_average().one);
    println!("five min: {:?}", System::load_average().five);
    println!("fifteen min: {:?}", System::load_average().fifteen);
    
    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // Display system information:
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());

    // Display processes ID, name na disk usage:
    for (pid, process) in sys.processes() {
        println!("[{pid}] {} {:?} | cpu%: {} | parent id {:?}", process.name(), process.disk_usage(), process.cpu_usage(), process.parent(),);
    }

    // We display all disks' information:
    println!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{disk:?}");
    }

    // Network interfaces name, data received and data transmitted:
    let networks = Networks::new_with_refreshed_list();
    println!("=> networks:");
    for (interface_name, data) in &networks {
        println!("{interface_name}: {}/{} B", data.received(), data.transmitted());
    }

    // Components temperature:
    let components = Components::new_with_refreshed_list();
    println!("=> components:");
    for component in &components {
        println!("{component:?}");
    }
}
