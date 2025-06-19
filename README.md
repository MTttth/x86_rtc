
# x86_rtc

[![Crates.io](https://img.shields.io/crates/v/x86_rtc)](https://crates.io/crates/x86_rtc)
[![Docs.rs](https://docs.rs/x86_rtc/badge.svg)](https://docs.rs/x86_rtc)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/arceos-org/x86_rtc)

A **no\_std** CMOS–based *Real‑Time Clock* (RTC) driver for **x86 / x86_64** PCs and QEMU guests.  
It offers a safe, minimal wrapper around the legacy CMOS I/O ports (`0x70`/`0x71`) so you can **read _or_ set the wall‑clock time** as a UNIX timestamp.

---

## Installation

```toml
[dependencies]
x86_rtc = "0.1"
```

*MSRV ≥ 1.71* (matches latest stable channel).

> **Platform note** — on non‑x86 targets the crate compiles to “stubs” (all zeros / no‑ops), so you can still share code across multi‑arch projects without `cfg` explosions.

---

## Public API

| Function                  | Description                                                                                                       |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `Rtc::new()`              | Create an instance; snapshots Status Register B to learn **BCD vs. Binary** and **12 h vs. 24 h** formats.        |
| `get_unix_timestamp()`    | Spin‑locks around the *update‑in‑progress* flag, converts CMOS date + time into seconds since **1970‑01‑01 UTC**. |
| `set_unix_timestamp(u64)` | Converts a UNIX timestamp back to calendar fields (handles leap‑years & BCD) and writes them to the RTC.          |

---

## Quick Start

```rust
use x86_rtc::Rtc;

fn main() {
    // SAFETY: on real HW you should disable interrupts to avoid half‑updated reads.
    // In a hobby OS this is typically done with `x86_64::instructions::interrupts::disable();`
    let rtc = Rtc::new();

    let now = rtc.get_unix_timestamp();
    println!("UNIX time: {now}");

    // … or push a timestamp back (e.g. after getting NTP time)
    // rtc.set_unix_timestamp(1_725_888_000); // 2025‑07‑01 00:00:00 UTC
}
```

### Feature Flags

| Feature | Default? | Effect                                                                  |
| ------- | -------- | ----------------------------------------------------------------------- |
| *none*  | ✅        | Build for x86/x86\_64 with CMOS I/O; other targets produce dummy stubs. |
| `std`   | ❌        | Enable `std` for use in userspace binaries/tests.                       |

---

## How it works

1. Select CMOS register via **0x70** (with NMI masked), then read/write via **0x71**.
2. Interpret Status Reg B bits to decide **BCD ↔ Binary** and **12 h ↔ 24 h** handling.
3. Guard against partial updates using the **UIP** flag in Status Reg A.
4. Convert calendar <→ epoch seconds using an internal, `const fn`/`no_std` mktime clone.

---

## Safety & Soundness

* All port I/O is wrapped in `unsafe` blocks but exposed through safe methods.
* Spin‑waits use `core::hint::spin_loop()` — no busy‑waits once data is stable.
* `set_unix_timestamp` performs bounds checking and BCD conversion *before* writing.
