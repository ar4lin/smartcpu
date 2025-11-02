# smartcpu

A GTK4-based CPU thread management tool for Linux with real-time frequency and governor monitoring.

<img width="430" height="611" alt="изображение" src="https://github.com/user-attachments/assets/1cb355b8-2699-4118-8006-efda82df6a27" />

## Features

- **Thread Control**: Enable/disable individual CPU threads
- **Bulk Operations**: Enable or disable all threads at once
- **CPU Frequency Monitoring**: View current frequency for each thread
- **Governor Display**: See the current CPU frequency governor for each thread
- **Dark Theme**: Modern dark theme interface
- **Real-time Status**: Visual indication of active/inactive threads

## Requirements

- Linux with `/sys/devices/system/cpu` interface
- GTK4 development libraries
- Root access (for modifying CPU settings)

## Building from Source

```bash
# Install dependencies (Ubuntu/Debian)
sudo apt install libgtk-4-dev build-essential

# Build the project
cd smartcpu
cargo build --release
```

## Running

1. Download or build the executable
2. Open terminal in folder
3. Run command: `chmod +x ./smartcpu` (only first run)
4. Run with: `sudo -E ./smartcpu`
5. Enjoy!

## Usage

- **Individual Control**: Click checkboxes to enable/disable specific threads
- **Enable All**: Activate all CPU threads
- **Disable All**: Deactivate all CPU threads (except thread 0)
- **View Info**: Thread status, frequency, and governor are displayed for each thread

> **Note**: Thread 0 cannot be disabled as it's the boot processor.

## Version History

- **0.1.1**: Added disable all functionality, CPU frequency display, governor display, improved UI
- **0.1.0**: Initial release
