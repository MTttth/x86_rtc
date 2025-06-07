[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/oscomp/arceos)
# x86_rtc

[![Crates.io](https://img.shields.io/crates/v/x86_rtc)](https://crates.io/crates/x86_rtc) 

System Real-Time Clock (RTC) driver for x86 and x86_64 based on CMOS registers. This crate provides a safe abstraction over low-level CMOS I/O ports to read and set system time as seconds since the UNIX epoch.

## Key APIs

- `Rtc::new() -> Rtc`  
  Constructs a new RTC instance by reading the CMOS Status Register B to determine data and hour formats (binary/BCD, 24h/12h).

- `Rtc::get_unix_timestamp(&self) -> u64`  
  Reads the current date and time from the CMOS, handles update-in-progress flags, BCD conversion, and 12/24h adjustments, and returns the timestamp as seconds since January 1, 1970 UTC.

- `Rtc::set_unix_timestamp(&self, unix_time: u64)`  
  Converts a UNIX timestamp back into year/month/day/hour/minute/second, encodes into BCD if necessary, and writes each component back to the CMOS registers.

## Example Usage

```rust
use x86_rtc::Rtc;

fn main() {
    // Disable interrupts or ensure mutual exclusion if needed before reading the RTC.
    let rtc = Rtc::new();

    // Read the current time as UNIX timestamp
    let now = rtc.get_unix_timestamp();
    println!("Current UNIX timestamp: {}", now);

    // Optionally, set the RTC to a known timestamp (e.g., test value)
    // rtc.set_unix_timestamp(1_600_000_000);
}