use std::fs::File;
use std::io::Write;

use kernel_module_testlib::{assert_dmesg_contains, with_kernel_module};

#[test]
fn test_for_each_process() {
    File::create("/proc/self/comm")
        .unwrap()
        .write_all(b"areyouthere")
        .unwrap();
    with_kernel_module(|| {
        assert_dmesg_contains(&[b"areyouthere"]);
    });
}
