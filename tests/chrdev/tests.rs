extern crate kernel_module_tests;

use kernel_module_tests::with_kernel_module;
use std::fs;

#[test]
fn test_proc_devices() {
    with_kernel_module(|| {
        let devices = fs::read_to_string("/proc/devices").unwrap();
        let dev_no_line = devices
            .lines()
            .find(|l| l.ends_with("chrdev-tests"))
            .unwrap();
        let elements = dev_no_line.rsplitn(2, " ").collect::<Vec<_>>();
        assert_eq!(elements.len(), 2);
        assert_eq!(elements[0], "chrdev-tests");
        assert!(elements[1].trim().parse::<u32>().is_ok());
    });

    let devices = fs::read_to_string("/proc/devices").unwrap();
    assert!(
        devices
            .lines()
            .find(|l| l.ends_with("chrdev-tests"))
            .is_none()
    );
}
