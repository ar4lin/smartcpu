use std::process::exit;
use nix::unistd::Uid;

use crate::ui::run_app;

mod cpu_ctrl;
mod terminal;
mod ui;

fn main() {
    ensure_root();
    
    let x = cpu_ctrl::get_cpu_threads_count();
    println!("[DBG] Threads count: {}", x.to_string());
    println!("[DBG] Cores count: {}", (x / 2).to_string());

    run_app();
}


fn ensure_root() {
    if !Uid::effective().is_root() { 
        eprintln!("\x1b[1;31mRoot required!\x1b[0m Run the app with \"sudo -E\"");
        exit(0); 
    }
}

