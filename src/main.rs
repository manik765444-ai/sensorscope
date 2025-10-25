// main.rs
extern crate dirs;
extern crate hostname;
extern crate nix;
extern crate pretty_bytes;
extern crate sysinfo;

use dirs::home_dir;
use hostname::get;
use nix::sys::sysinfo::{SysInfo, Uptime};
use pretty_bytes::{converter::convert, PrettyBytes};
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;
use sysinfo::{System, SystemExt};

// Define custom error types
#[derive(Debug)]
enum MonitorError {
    SysInfo(sysinfo::Error),
    Io(io::Error),
    Nix(nix::Error),
}

impl fmt::Display for MonitorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MonitorError::SysInfo(e) => write!(f, "SysInfo Error: {}", e),
            MonitorError::Io(e) => write!(f, "IO Error: {}", e),
            MonitorError::Nix(e) => write!(f, "Nix Error: {}", e),
        }
    }
}

impl Error for MonitorError {}

impl From<sysinfo::Error> for MonitorError {
    fn from(e: sysinfo::Error) -> Self {
        MonitorError::SysInfo(e)
    }
}

impl From<io::Error> for MonitorError {
    fn from(e: io::Error) -> Self {
        MonitorError::Io(e)
    }
}

impl From<nix::Error> for MonitorError {
    fn from(e: nix::Error) -> Self {
        MonitorError::Nix(e)
    }
}

// Defines a struct to hold system information
#[derive(Debug)]
struct SystemInfo {
    hostname: String,
    uptime: u64,
    current_user: String,
    cpu: f64,
    memory: f64,
    total_memory: u64,
    disk_usage: f64,
    total_disk_space: u64,
    home_dir: PathBuf,
    os: String,
    kernel: String,
    architecture: String,
    cores: u16,
    processes: u32,
}

fn get_system_info() -> Result<SystemInfo, MonitorError> {
    let sys = System::new_all();
    let info = SysInfo::new().map_err(Into::into)?;

    let hostname = get().map_err(|_| MonitorError::Io(io::Error::last_os_error()))?;
    let uptime = info.uptime().as_secs();
    let current_user = env::var("USER")?;
    let cpu = sys.global_processor_info().cpu_usage();
    let memory = sys.used_memory() as f64 / sys.total_memory() as f64;
    let total_memory = sys.total_memory();
    let disk_usage = sys.total_used_space() as f64 / sys.total_space() as f64;
    let total_disk_space = sys.total_space();
    let home_dir = home_dir().ok_or(MonitorError::Io(io::Error::last_os_error()))?;
    let os = sys.name();
    let kernel = sys.kernel_version();
    let architecture = sys.long_os_version();
    let cores = sys.physical_core_count();
    let processes = sys.processes().len() as u32;

    Ok(SystemInfo {
        hostname,
        uptime,
        current_user,
        cpu,
        memory,
        total_memory,
        disk_usage,
        total_disk_space,
        home_dir,
        os,
        kernel,
        architecture,
        cores,
        processes,
    })
}

fn print_info(info: SystemInfo) {
    println!("Hostname: {}", info.hostname);
    println!("Uptime: {}\n", convert(info.uptime).unwrap());

    println!("Current User: {}", info.current_user);
    println!("CPU Usage: {:.2}%", info.cpu);
    println!("Memory Usage: {:.2}%", info.memory * 100.0);
    println!("Total Memory: {}", PrettyBytes(info.total_memory));
    println!("Disk Usage: {:.2}%", info.disk_usage * 100.0);
    println!("Total Disk Space: {}", PrettyBytes(info.total_disk_space));

    println!("Home Directory: {}", info.home_dir.display());

    println!("Operating System: {}", info.os);
    println!("Kernel: {}", info.kernel);
    println!("Architecture: {}", info.architecture);
    println!("CPU Cores: {}", info.cores);
    println!("Processes: {}\n", info.processes);
}

fn main() {
    match get_system_info() {
        Ok(info) => print_info(info),
        Err(err) => eprintln!("An error occurred: {}", err),
    }
}
```

```rust
// Cargo.toml
[package]
name = "system_monitor"
version = "0.1.0"
edition = "2021"

[dependencies]
dirs = "5.0.0"
hostname = "0.3.1"
nix = "0.24.3"
pretty-bytes = "1.1.0"
sysinfo = "0.23.5"