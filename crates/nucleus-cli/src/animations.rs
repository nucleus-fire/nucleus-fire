//! Nucleus CLI Animations
//! 
//! Advanced terminal animations for startup, builds, and status updates.

use std::io::{Write, stdout};
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// ANSI color codes
pub mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";      
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";
    
    // Backgrounds
    pub const BG_CYAN: &str = "\x1b[46m";
}

/// Clear the current line
pub fn clear_line() {
    print!("\r\x1b[K");
    stdout().flush().unwrap();
}

/// Display the animated Nucleus startup banner
pub fn show_startup_banner() {
    use colors::*;
    
    // Clear screen
    print!("\x1b[2J\x1b[H");
    stdout().flush().unwrap();
    
    let frames = [
        format!("{CYAN}       ⚛      {RESET}"),
        format!("{CYAN}      ╭⚛╮     {RESET}"),
        format!("{CYAN}     ╭─⚛─╮    {RESET}"),
        format!("{CYAN}    ╭──⚛──╮   {RESET}"),
        format!("{CYAN}   ╭───⚛───╮  {RESET}"),
    ];
    
    for frame in frames {
        print!("\x1b[H\n\n");
        println!("     {}", frame);
        stdout().flush().unwrap();
        std::thread::sleep(Duration::from_millis(60));
    }

    let logo = format!(r#"
{CYAN}   ╭───⚛───╮   {MAGENTA}{BOLD}N U C L E U S{RESET}
{CYAN}  ╭─────────╮  {RESET}{DIM}High-Performance Framework{RESET}
{CYAN}  │    {WHITE}●{CYAN}    │  {RESET}{DIM}v3.5.0{RESET}
{CYAN}  ╰────┬────╯  {RESET}
{CYAN}       │       {RESET}
"#);

    print!("\x1b[H\n\n");
    println!("{}", logo);
    stdout().flush().unwrap();
    std::thread::sleep(Duration::from_millis(200));
}

/// Run a spinner for a closure task
pub fn with_spinner<F, T>(message: &str, f: F) -> T 
where F: FnOnce() -> T
{
    use colors::*;
    print!("{RESET}"); 
    let spinner_frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let done = Arc::new(AtomicBool::new(false));
    let done_clone = done.clone();
    let msg = message.to_string();
    
    let handle = std::thread::spawn(move || {
        let mut i = 0;
        while !done_clone.load(Ordering::Relaxed) {
            let frame = spinner_frames[i % spinner_frames.len()];
            print!("\r{CYAN}{}{RESET} {}   ", frame, msg);
            stdout().flush().unwrap();
            std::thread::sleep(Duration::from_millis(80));
            i += 1;
        }
    });

    let result = f();
    done.store(true, Ordering::Relaxed);
    handle.join().unwrap();
    
    clear_line();
    println!("\r{GREEN}✔{RESET} {}", message);
    result
}

/// Animated build status display
pub fn show_build_start() {
    println!();
}

/// Animated build step with icon
pub fn build_step(icon: &str, message: &str) {
    use colors::*;
    println!("  {CYAN}{icon}{RESET} {}", message);
}

/// Dev server startup animation
pub fn show_dev_server_start(port: u16) {
    use colors::*;
    
    println!();
    println!("{BOLD}🚀 Reactor Online{RESET}");
    println!();
    
    // Status Box
    println!("  {DIM}╭──────────────────────────────────────────────╮{RESET}");
    println!("  {DIM}│{RESET}  {BOLD}{GREEN}●{RESET} {BOLD}Server Active{RESET}                           {DIM}│{RESET}");
    println!("  {DIM}│{RESET}                                            {DIM}│{RESET}");
    println!("  {DIM}│{RESET}  {CYAN}➜{RESET}  {BOLD}Local:{RESET}    {CYAN}http://localhost:{port}/{RESET}       {DIM}│{RESET}");
    println!("  {DIM}│{RESET}  {CYAN}➜{RESET}  {BOLD}Network:{RESET}  {DIM}use --host to expose{RESET}        {DIM}│{RESET}");
    println!("  {DIM}│{RESET}                                            {DIM}│{RESET}");
    println!("  {DIM}│{RESET}  {YELLOW}⚡{RESET} {BOLD}HMR:{RESET}      {GREEN}Active{RESET}                       {DIM}│{RESET}");
    println!("  {DIM}╰──────────────────────────────────────────────╯{RESET}");
    println!();
    println!("  {DIM}Watching for changes...{RESET}");
}

/// File change notification 
pub fn show_file_change(path: &str) {
    use colors::*;
    clear_line();
    println!("\n{YELLOW}⚡ Changed:{RESET} {BOLD}{}{RESET}", path);
}

/// Rebuild notification
pub fn show_rebuild_start() {
    use colors::*;
    print!("{DIM}⟳ Rebuilding...{RESET}");
    stdout().flush().unwrap();
}

pub fn show_rebuild_complete() {
    use colors::*;
    clear_line();
    println!("{GREEN}✔ Rebuild complete{RESET}");
}
