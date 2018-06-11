use std::fs;

#[test]
fn test_read_bool_default() {
    with_kernel_module(|| {
        assert_eq!(fs::read_to_string("/proc/sys/rust/sysctl-tests/a").unwrap(), "0\n");
    });
}
