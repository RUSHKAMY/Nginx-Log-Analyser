# Nginx Log Analyser with Rust

![Rust](https://img.shields.io/badge/Rust-1.70+-blue.svg)

A high-performance Nginx log analyzer written in Rust, providing insightful statistics about web server traffic patterns.

## Technology Stack

### 1. Programming Language:
- **Rust** - Primary implementation language (2021 edition)

### 2. Key Libraries (Crates):
- **regex** - For efficient log pattern matching
- **chrono** - DateTime parsing and handling
- **clap** - Command-line argument parsing
- **serde_json** - JSON output formatting (optional)
- **rayon** - Parallel processing for large log files

### 3. Rust Standard Library (std):
- **std::fs** - File system operations
- **std::io** - Buffered I/O operations
- **std::collections** - Efficient data structures for aggregation

### 4. Key Features:
- **High-performance parsing** - Processes GBs of logs in seconds
- **Multi-threaded analysis** - Utilizes all CPU cores
- **Memory-safe** - No unsafe code blocks
- **Cross-platform** - Works on Windows, Linux, and macOS

### 5. Analysis Capabilities:
- Request frequency by endpoint
- Traffic patterns by hour/day
- HTTP status code distribution
- Top client IP addresses
- Request method statistics

## Installation & Usage

## Compilation and Usage

### Prerequisites
- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Nginx access log file (default looks for `nginx-access.log.txt` in project root)

### Installation
```bash
git clone https://github.com/RUSHKAMY/Nginx-Log-Analyser.git
cd Nginx-Log-Analyser
```
## Building the Project

```bash

# Debug build (for development)
cargo build

# Release build (optimized for production)
cargo build --release
```

## Running the Analyzer

```bash
# Basic usage (analyzes default log file)
./target/release/nginx-log-analyzer

# Specify custom log file path
./target/release/nginx-log-analyzer /path/to/your/access.log
```
## Output
### The analyzer will display four reports:

* Top 5 IP addresses by request count

* Top 5 requested paths

* Top 5 HTTP status codes

* Top 5 user agents

### Example output:

```text
TOP 5 IP: 
192.168.1.1 - 542 requests
10.0.0.1 - 421 requests
...

TOP 5 PATH: 
/ - 1200 requests
/api/users - 850 requests
...

TOP 5 CODES: 
200 - 3200 requests
404 - 150 requests
...

TOP 5 USERS REQUEST: 
Mozilla/5.0 - 2800 requests
curl/7.68.0 - 450 requests
...
```


## Platform-Specific Notes

### Windows
```powershell
# Build
cargo build --release

# Run
.\target\release\nginx-log-analyzer.exe
```

### Linux/macOS
```bash
# Install build dependencies (Ubuntu/Debian)
sudo apt install build-essential

# Build and run
cargo build --release
./target/release/nginx-log-analyzer
```

### Performance Tips 
```bash 
cargo build --release && ./target/release/nginx-log-analyzer large-access.log
```


## Conclusion

This Nginx log analyzer demonstrates Rust's power for efficient log processing and analysis. Key advantages:

- **Blazing fast performance** - leverages Rust's zero-cost abstractions
- **Memory safety** - 100% safe Rust code
- **Actionable insights** - clear web traffic statistics
- **Modular architecture** - each analyzer is independent
- **Cross-platform** - works on Windows, Linux, macOS


![gif](https://www.nonograms.ru/files/user/upload/55392_6ad56b2108d114e87d980e2fdfe7394e.gif)