# Syswatch 
A lightweight Linux System monitor CLI built in Rust. Reads real-time memory (RAM) and disk usage (HDD/SSD) directly from the kernel and system commands.

## What it does

syswatch gives you a quick snapshot of your system's health in two lines:
```
8.4GB Used / 15.4GB Total 54%
412 Available 32G Used / 468G  Total 8%
```

- Memory: Parses `/proc/meminfo` to calculate used vs total RAM
- Disk: Runs `df -h` and extracts root filesystem usage.

## How it works 
Memory data come from `/proc/meminfo`, a linux virtual file maintained by Linux kernel. The program reads it as a string, iterates through each line, and extracts `MemTotal` and `MemAvailable` values. It converts from kilobyte to gigabyte and calculates usage percentage. 

Disk data comes from running the `df` command as a child process using `std::process::Command`, then parsing the output for the root `(/)` filesystem.

## Run it

```
git clone https://github.com/Martin896/syswatch.git 
cd syswatch 
cargo run
```
Requires a Rust and Linux system.

## Built with 
- Rust (standard Library only - no external dependencies)
- Linux /proc filesystem
- `df` system command 

## Roadmap 
- [x] Add CPU usage from `/proc/stat`
- [ ] Add network interface stats
- [ ] Continuous monitoring mode with refresh interval
- [ ] colored output for warning threshold.