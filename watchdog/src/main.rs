use std::process::Command;
use sysinfo::{ProcessExt, SystemExt};

fn main() {
    let mut system = sysinfo::System::new();

    // First we update all information of our system struct.
    println!("1");
    system.refresh_all();
    println!("2");

    println!("3");
    // Now let's print every process' id and name:
    for (pid, proc_) in system.get_process_list() {
        println!("{}:{} => status: {:?}", pid, proc_.name(), proc_.status());
    }
    println!("4");

    println!("5");
    // Then let's print the temperature of the different components:
    for component in system.get_components_list() {
        println!("{:?}", component);
    }
    println!("6");

    println!("7");
    // And then all disks' information:
    for disk in system.get_disks() {
        println!("{:?}", disk);
    }
    println!("8");

    println!("9");
    // And finally the RAM and SWAP information:
    println!("total memory: {} kB", system.get_total_memory());
    println!("used memory : {} kB", system.get_used_memory());
    println!("total swap  : {} kB", system.get_total_swap());
    println!("used swap   : {} kB", system.get_used_swap());
    println!("10");

    println!("11");
    if let Ok(_) = Command::new("http-load-balance.exe")
    .arg("-c")
    .arg("recogword-v1.0-http.cfg")
    .env("PATH", "D:\\build\\mqtt-load-balance")
    .current_dir("D:\\build\\mqtt-load-balance")
    .spawn() {
        println!("ok");
    }
    println!("12");

    println!("13");
    system.refresh_all();
    println!("--------http-load-balance.exe--------");
    for proc_ in system.get_process_by_name("http-load-balance") {
        println!("{} => status: {:?}", proc_.name(), proc_.status());
        proc_.kill(sysinfo::Signal::Kill);
    }
    println!("--------http-load-balance.exe--------");
    println!("14");
}
