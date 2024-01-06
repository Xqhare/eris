use std::thread;

use sysinfo::{
    Components, Disks, Networks, System,
};

const CPU_HIGH_THRESHOLD: f32 = 75.0;

fn main() {
    let mut sys = System::new_all();
    let mut loop_counter: usize  = 0;
    loop {
        sys.refresh_cpu();
        // This is enough for 255 cores... Also cpu cores start at 1!
        let mut cpu_core_counter: u8 = 1;
        for cpu in sys.cpus() {
            println!("{} core | {}% usage", cpu_core_counter, cpu.cpu_usage());
            if cpu.cpu_usage() > CPU_HIGH_THRESHOLD {
                println!("HIGH CPU USAGE DETECTED! CPU {}", cpu_core_counter);
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
