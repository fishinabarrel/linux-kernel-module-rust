use std::fs;

use kernel_module_testlib::{assert_dmesg_contains, with_kernel_module};

#[test]
fn test_get() {
    with_kernel_module(|| {
        fs::write("/proc/sys/rust/sysctl-get-tests/a", "1").unwrap();
    });
    assert_dmesg_contains(&[b"A_VAL: true", b"SYSCTL_A: true"]);
}
