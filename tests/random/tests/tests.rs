use std::collections::HashSet;
use std::fs;
use std::io::Read;

use kernel_module_testlib::with_kernel_module;

#[test]
fn test_random_entropy_read() {
    with_kernel_module(|| {
        let mut keys = HashSet::new();
        for _ in 0..1024 {
            let mut key = [0; 16];
            let mut f = fs::File::open("/proc/sys/rust/random-tests/entropy").unwrap();
            f.read_exact(&mut key).unwrap();
            keys.insert(key);
        }
        assert_eq!(keys.len(), 1024);
    });
}

#[test]
fn test_random_entropy_write() {
    with_kernel_module(|| {
        fs::write("/proc/sys/rust/random-tests/entropy", b"1234567890").unwrap();
    });
}
