//! tests/test_x86_rtc.rs
//! Integration-test for the `x86_rtc` crate.

use x86_rtc::Rtc;

/// Simple smoke-test that `Rtc::new()` works and returns something
/// that looks like a sane timestamp.
///
/// * On real x86/x86_64 hardware / QEMU it should be *≈ “now”*
/// * On non-x86 targets the stub implementation always reads zeros,
///   which represents 2000-01-01 00:00:00 UTC.
#[test]
fn rtc_returns_reasonable_timestamp() {
    let rtc = Rtc::new();
    let ts = rtc.get_unix_timestamp();

    // -- branch at run-time: we can’t know which runner we’re on.
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        // The timestamp should be >= 2000-01-01 and < 2100-01-01
        const YEAR_2000: u64 = 946684800; // 2000-01-01T00:00:00Z
        const YEAR_2100: u64 = 4102444800; // 2100-01-01T00:00:00Z
        assert!(
            (YEAR_2000..YEAR_2100).contains(&ts),
            "RTC returned implausible value: {ts}"
        );
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        // Stub always yields 2000-01-01 00:00:00 UTC
        const YEAR_2000: u64 = 946684800;
        assert_eq!(ts, YEAR_2000);
    }
}

/// Round-trip: write a timestamp then read it back.
///
/// *Only compiled on x86/x86_64* because other targets have no-op
/// register writes.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[test]
fn write_then_read_back() {
    if unsafe { libc::ioperm(0x70, 2, 1) } != 0 {
        eprintln!("skip RTC write test: no ioperm");
        return;
    }
    let rtc = Rtc::new();

    // Pick an arbitrary date: 2025-06-14 12:34:56 UTC
    const T_REF: u64 = 1755197696;
    rtc.set_unix_timestamp(T_REF);

    // NOTE: some real machines update seconds *after* the UIP window.
    // It’s ok to read a second later; allow ±1s tolerance.
    let read_back = rtc.get_unix_timestamp();
    assert!(
        (T_REF.saturating_sub(1)..=T_REF + 1).contains(&read_back),
        "wrote {T_REF} but read back {read_back}"
    );
}
