//! tests/test_x86_rtc.rs
//! CI-safe integration tests for the public API of `x86_rtc`.

use core::time::Duration;
use x86_rtc::Rtc;

#[test]
fn rtc_timestamp_is_plausible_and_monotonic() {
    let rtc = Rtc::new();

    let t1 = rtc.get_unix_timestamp();

    const MIN: u64 = 0; // 1970-01-01T00:00:00Z
    const MAX: u64 = 4_102_444_800; // 2100-01-01T00:00:00Z
    assert!(
        (MIN..=MAX).contains(&t1),
        "first read {t1} out of plausible range"
    );

    std::thread::sleep(Duration::from_millis(20));

    let t2 = rtc.get_unix_timestamp();

    assert!(
        t2 >= t1,
        "timestamp went backwards: first {t1}, second {t2}"
    );
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[test]
fn stub_returns_epoch_2000() {
    let rtc = Rtc::new();
    let ts = rtc.get_unix_timestamp();
    const YEAR_2000: u64 = 946_684_800; // 2000-01-01T00:00:00Z
    assert_eq!(ts, YEAR_2000, "stub should return constant 2000-01-01");
}
