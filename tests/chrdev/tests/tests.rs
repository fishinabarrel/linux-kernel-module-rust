use std::io::Read;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

use libc;

use kernel_module_testlib::with_kernel_module;

fn get_device_major_number() -> libc::dev_t {
    let devices = fs::read_to_string("/proc/devices").unwrap();
    let dev_no_line = devices
        .lines()
        .find(|l| l.ends_with("chrdev-tests"))
        .unwrap();
    let elements = dev_no_line.rsplitn(2, " ").collect::<Vec<_>>();
    assert_eq!(elements.len(), 2);
    assert_eq!(elements[0], "chrdev-tests");
    return elements[1].trim().parse().unwrap();
}

fn temporary_file_path() -> PathBuf {
    let mut p = env::temp_dir();
    p.push("chrdev-test-device");
    return p;
}

struct UnlinkOnDrop<'a> {
    path: &'a PathBuf,
}

impl Drop for UnlinkOnDrop<'_> {
    fn drop(&mut self) {
        Command::new("sudo")
            .arg("rm")
            .arg(self.path.to_str().unwrap())
            .status()
            .unwrap();
    }
}

fn mknod(path: &PathBuf, device_number: libc::dev_t) -> UnlinkOnDrop {
    Command::new("sudo")
        .arg("mknod")
        .arg(path.to_str().unwrap())
        .arg("c")
        .arg(device_number.to_string())
        .arg("0")
        .status()
        .unwrap();
    return UnlinkOnDrop { path };
}

#[test]
fn test_mknod() {
    with_kernel_module(|| {
        let device_number = get_device_major_number();
        mknod(&temporary_file_path(), device_number);
    });
}

#[test]
fn test_read() {
    with_kernel_module(|| {
        let device_number = get_device_major_number();
        let p = temporary_file_path();
        let _u = mknod(&p, device_number);

        let mut f = fs::File::open(&p).unwrap();
        let mut data = [0; 12];
        f.read_exact(&mut data).unwrap();
        assert_eq!(&data, b"123456789123")
    });
}
