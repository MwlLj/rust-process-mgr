use std::process::Command;
use sysinfo::{ProcessExt, SystemExt};

fn main() {
    let mut system = sysinfo::System::new();

    // First we update all information of our system struct.
    system.refresh_all();

    // Now let's print every process' id and name:
    for (pid, proc_) in system.get_process_list() {
        println!("{}:{} => status: {:?}", pid, proc_.name(), proc_.status());
    }

    // Then let's print the temperature of the different components:
    for component in system.get_components_list() {
        println!("{:?}", component);
    }

    // And then all disks' information:
    for disk in system.get_disks() {
        println!("{:?}", disk);
    }

    // And finally the RAM and SWAP information:
    println!("total memory: {} kB", system.get_total_memory());
    println!("used memory : {} kB", system.get_used_memory());
    println!("total swap  : {} kB", system.get_total_swap());
    println!("used swap   : {} kB", system.get_used_swap());

    if let Ok(_) = Command::new("http-load-balance.exe")
    .arg("-c")
    .arg("recogword-v1.0-http.cfg")
    .env("PATH", "D:\\build\\mqtt-load-balance")
    .current_dir("D:\\build\\mqtt-load-balance")
    .spawn() {
        println!("ok");
    }

    system.refresh_all();
    println!("--------http-load-balance.exe--------");
    for proc_ in system.get_process_by_name("http-load-balance") {
        println!("{} => status: {:?}", proc_.name(), proc_.status());
        proc_.kill(sysinfo::Signal::Kill);
    }
    println!("--------http-load-balance.exe--------");

    if let Ok(_) = Command::new("cfgs")
    .env("PATH", "/data/local/deviceservice/miscroservice")
    .current_dir("/data/local/deviceservice/miscroservice")
    .spawn() {
        println!("ok");
    }

    system.refresh_all();
    println!("--------cfgs--------");
    for proc_ in system.get_process_by_name("cfgs") {
        println!("{} => status: {:?}", proc_.name(), proc_.status());
    }
    println!("--------cfgs--------");
}
