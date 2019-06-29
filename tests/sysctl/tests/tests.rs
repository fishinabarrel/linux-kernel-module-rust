use std::fs;
use std::path::Path;

use kernel_module_testlib::with_kernel_module;

#[test]
fn test_read_bool_default() {
    with_kernel_module(|| {
        assert_eq!(
            fs::read_to_string("/proc/sys/rust/sysctl-tests/a").unwrap(),
            "0\n"
        );
    });
}

#[test]
fn test_write_bool() {
    with_kernel_module(|| {
        fs::write("/proc/sys/rust/sysctl-tests/a", "1").unwrap();
        assert_eq!(
            fs::read_to_string("/proc/sys/rust/sysctl-tests/a").unwrap(),
            "1\n"
        );
    });
}

#[test]
fn test_write_bool_whitespace() {
    with_kernel_module(|| {
        fs::write("/proc/sys/rust/sysctl-tests/a", "  1\t").unwrap();
        assert_eq!(
            fs::read_to_string("/proc/sys/rust/sysctl-tests/a").unwrap(),
            "1\n"
        );
    });
}

#[test]
fn test_file_doesnt_exit_after_module_unloaded() {
    with_kernel_module(|| {
        assert!(Path::new("/proc/sys/rust/sysctl-tests/a").exists());
    });
    assert!(!Path::new("/proc/sys/rust/sysctl-tests/a").exists());
}
