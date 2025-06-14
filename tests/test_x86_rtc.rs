//! tests/test_x86_rtc.rs
//! Minimal, CI-safe integration tests for the `x86_rtc` crate.

use x86_rtc::*;

/// Smoke-test: ctor + read timestamp must not panic or segfault.
///
/// * 在 x86/x86_64 Linux 用户态无法写 CMOS，但读通常返回某个稳定值；
/// * 在其它架构下 crate 会退化成 stub（返回 2000-01-01 0:0:0）。
#[test]
fn rtc_read_is_safe() {
    let rtc = Rtc::new();
    let ts  = rtc.get_unix_timestamp();

    // 合法区间：1970-01-01 … 2100-01-01
    const MIN: u64 = 0;               // 1970-01-01 00:00:00
    const MAX: u64 = 4_102_444_800;   // 2100-01-01 00:00:00
    assert!(
        (MIN..=MAX).contains(&ts),
        "timestamp {ts} out of plausible range"
    );
}

/// Helper functions must behave as expected.
#[test]
fn helpers_round_trip_and_leap_logic() {
    // ---------- leap year ----------
    assert!(is_leap_year(2000));
    assert!(!is_leap_year(1900));
    assert!(is_leap_year(2024));
    assert!(!is_leap_year(2023));

    // ---------- days in month ----------
    assert_eq!(days_in_month(2, 2024), 29);
    assert_eq!(days_in_month(2, 2023), 28);
    assert_eq!(days_in_month(11, 2023), 30);
    assert_eq!(days_in_month(12, 2023), 31);

    // ---------- BCD round-trip ----------
    for n in 0..100 {
        let bcd = convert_binary_value(n);
        assert_eq!(convert_bcd_value(bcd), n);
    }

    // ---------- epoch conversion ----------
    assert_eq!(seconds_from_date(1970, 1, 1, 0, 0, 0), 0);
    assert_eq!(seconds_from_date(2000, 1, 1, 0, 0, 0), 946_684_800);
}
