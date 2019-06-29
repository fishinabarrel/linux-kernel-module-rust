use std::fs;

use kernel_module_testlib::with_kernel_module;

#[test]
fn test_proc_devices() {
    with_kernel_module(|| {
        let devices = fs::read_to_string("/proc/devices").unwrap();
        let dev_no_line = devices
            .lines()
            .find(|l| l.ends_with("chrdev-region-allocation-tests"))
            .unwrap();
        let elements = dev_no_line.rsplitn(2, " ").collect::<Vec<_>>();
        assert_eq!(elements.len(), 2);
        assert_eq!(elements[0], "chrdev-region-allocation-tests");
        assert!(elements[1].trim().parse::<u32>().is_ok());
    });

    let devices = fs::read_to_string("/proc/devices").unwrap();
    assert!(devices
        .lines()
        .find(|l| l.ends_with("chrdev-region-allocation-tests"))
        .is_none());
}
