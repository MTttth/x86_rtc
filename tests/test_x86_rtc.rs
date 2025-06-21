//! tests/test_x86_rtc.rs
//! Integration tests for x86_rtc that are safe on CI.
//
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn have_cmos_io_priv() -> bool {
    unsafe { libc::ioperm(0x70, 2, 1) == 0 }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[test]
fn rtc_read_monotonic_or_skip() {
    if !have_cmos_io_priv() {
        eprintln!("skip: no port-IO privilege");
        return;
    }

    let rtc = x86_rtc::Rtc::new();
    let first = rtc.get_unix_timestamp();
    let second = rtc.get_unix_timestamp();
    assert!(second >= first, "time went backwards: {first}->{second}");
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[test]
fn rtc_write_then_read_back_or_skip() {
    if !have_cmos_io_priv() {
        eprintln!("skip: no port-IO privilege");
        return;
    }

    const T_REF: u64 = 1_755_197_696; // 2025-06-14 12:34:56 UTC
    let rtc = x86_rtc::Rtc::new();
    rtc.set_unix_timestamp(T_REF);

    let got = rtc.get_unix_timestamp();
    assert!(
        (T_REF.saturating_sub(1)..=T_REF + 1).contains(&got),
        "wrote {T_REF} read {got}"
    );
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[test]
fn stub_const_epoch_2000() {
    let ts = x86_rtc::Rtc::new().get_unix_timestamp();
    assert_eq!(ts, 946_684_800); // 2000-01-01 00:00:00 UTC
}
