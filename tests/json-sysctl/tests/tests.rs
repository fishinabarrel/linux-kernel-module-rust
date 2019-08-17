use std::fs;

use kernel_module_testlib::*;

#[test]
fn test_json() {
    with_kernel_module(|| {
        let device_number = get_device_major_number("json");
        let p = temporary_file_path();
        let _u = mknod(&p, device_number, 0);

        assert_eq!(
            std::str::from_utf8(&fs::read(&p).unwrap()).unwrap(),
            "{\"a\":false,\"b\":false,\"c\":false}\n"
        );
        fs::write("/proc/sys/json-sysctl/a", "1").unwrap();
        assert_eq!(
            std::str::from_utf8(&fs::read(&p).unwrap()).unwrap(),
            "{\"a\":true,\"b\":false,\"c\":false}\n"
        );
    });
}
