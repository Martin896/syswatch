# Syswatch 
A lightweight Linux System monitor CLI built in Rust. Reads real-time memory (RAM) and disk usage (HDD/SSD) directly from the kernel and system commands.

## What it does

syswatch gives you a quick snapshot of your system's health:
```
8.4GB Used / 15.4GB Total 54%
412 Available 32G Used / 468G  Total 8%
CPU usage 15.3%
```

- Memory: Parses `/proc/meminfo` to calculate used vs total RAM
- Disk: Runs `df -h` and extracts root filesystem usage.
- CPU: Reads `/proc/stat` to calculate overall CPU usage percentage with health warnings

## How it works 
Memory data come from `/proc/meminfo`, a linux virtual file maintained by Linux kernel. The program reads it as a string, iterates through each line, and extracts `MemTotal` and `MemAvailable` values. It converts from kilobyte to gigabyte and calculates usage percentage. 

Disk data comes from running the `df` command as a child process using `std::process::Command`, then parsing the output for the root `(/)` filesystem.

CPU data comes from `/proc/stat`, which tracks time spent in different states (user, system, idle). The program sums all values to get total time, extracts idle time, and calculates the percentage of time the CPU has been active. It also provides health warnings if usage exceeds safe thresholds.

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
- [ ] Continuous monitoring mode with refresh intervalare 
- [ ] colored output for warning threshold.