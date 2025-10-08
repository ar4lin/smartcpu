use std::process::Command;
use crate::terminal;

pub fn get_cpu_threads_count() -> i32{
    let sh_command = "cat /sys/devices/system/cpu/present";

    let output = terminal::run_command(sh_command);
    
    let count_str = output
    .trim()
    .split('-')
    .last()
    .unwrap();

    let count: i32 = count_str
    .parse()
    .expect("Not a valid number");

    count + 1
}

pub fn is_thread_active(thread: &String) -> bool{
    let sh_command = format!("cat /sys/devices/system/cpu/cpu{}/online", thread)
    .to_string();

    let output = terminal::run_command(&sh_command);

    if output == "1" {
        return true;
    }
    return false;
}

pub fn enable_thread(thread: i32){
    let path = format!("/sys/devices/system/cpu/cpu{}/online", thread);

    let output = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(format!("echo 1 > {}", path))
        .output()
        .expect("Error launch sudo");

    if output.status.success() {
        println!("Thread {} activated", thread);
    } else {
        eprintln!("Error with thread activation {}", thread);
    }
}

pub fn disable_thread(thread: i32){
    let path = format!("/sys/devices/system/cpu/cpu{}/online", thread);

    let output = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(format!("echo 0 > {}", path))
        .output()
        .expect("Error launch sudo");

    if output.status.success() {
        println!("Thread {} deacivated", thread);
    } else {
        eprintln!("Error with thread deactivation {}", thread);
    }
}

pub fn enable_all(){
    let threads_count = get_cpu_threads_count();
    for thread in 1..threads_count {
        if is_thread_active(&thread.to_string()) {
            println!("\x1b[1;31mThread {} already activated \x1b[0m", thread);
            continue;
        }
        
        enable_thread(thread);
    }
    println!("All activated (Restart app to refresh UI)")
}