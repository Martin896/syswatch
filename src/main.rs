use std::fs;
use std::process::Command;

fn main(){

    // RAM INFORMATION
    let contents = fs::read_to_string("/proc/meminfo").unwrap();
    
    let mut mem_total: f64 = 0.0;
    let mut mem_available : f64 = 0.0;

    for line in contents.lines() {
        
        if line.starts_with("MemTotal") {
            mem_total = line.split_whitespace().nth(1).unwrap().parse().unwrap();
            
        }

        if line.starts_with("MemAvailable") {
            mem_available = line.split_whitespace().nth(1).unwrap().parse().unwrap();
            
        }
    }
    let total_gb = mem_total / 1048576.0;
    let available_gb = mem_available/ 1048576.0;
    let used_gb = total_gb - available_gb;
    let percent = (used_gb / total_gb) * 100.0;
    
    
    println!("{used_gb:.1}GB Used / {total_gb:.1}GB Total {percent:.0}%");

    //DISK INFORMATION 

    let disk_info = Command::new("df")
    .arg("-h")
    .output()
    .expect("Failed to run command");
    
    let stdout = String::from_utf8_lossy(&disk_info.stdout);
    
    for line in stdout.lines() {
        if line.starts_with("/dev/sda2") {
            let disk_total = line.split_whitespace().nth(1).unwrap();
            let disk_used = line.split_whitespace().nth(2).unwrap();
            let disk_available = line.split_whitespace().nth(3).unwrap();
            let percent = line.split_whitespace().nth(4).unwrap();

            println!("{disk_available} Available {disk_used} Used  / {disk_total} Total {percent}")
        }
    }
}