use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::prelude::FileExt;
use std::path::PathBuf;
use std::process::Command;

use libc;

use tempfile::TempDir;

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
    let mut p = TempDir::new().unwrap().into_path();
    p.push("device");
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

fn mknod(path: &PathBuf, major: libc::dev_t, minor: libc::dev_t) -> UnlinkOnDrop {
    Command::new("sudo")
        .arg("mknod")
        .arg(path.to_str().unwrap())
        .arg("c")
        .arg(major.to_string())
        .arg(minor.to_string())
        .status()
        .unwrap();
    return UnlinkOnDrop { path };
}

const READ_FILE_MINOR: libc::dev_t = 0;
const SEEK_FILE_MINOR: libc::dev_t = 1;

#[test]
fn test_mknod() {
    with_kernel_module(|| {
        let device_number = get_device_major_number();
        mknod(&temporary_file_path(), device_number, READ_FILE_MINOR);
    });
}

#[test]
fn test_read() {
    with_kernel_module(|| {
        let device_number = get_device_major_number();
        let p = temporary_file_path();
        let _u = mknod(&p, device_number, READ_FILE_MINOR);

        let mut f = fs::File::open(&p).unwrap();
        let mut data = [0; 12];
        f.read_exact(&mut data).unwrap();
        assert_eq!(&data, b"123456789123")
    });
}

#[test]
fn test_read_offset() {
    with_kernel_module(|| {
        let device_number = get_device_major_number();
        let p = temporary_file_path();
        let _u = mknod(&p, device_number, READ_FILE_MINOR);

        let mut f = fs::File::open(&p).unwrap();
        let mut data = [0; 5];
        f.read_exact(&mut data).unwrap();
        assert_eq!(&data, b"12345");
        f.read_exact(&mut data).unwrap();
        assert_eq!(&data, b"67891");
    });
}

#[test]
fn test_read_at() {
    with_kernel_module(|| {
        let device_number = get_device_major_number();
        let p = temporary_file_path();
        let _u = mknod(&p, device_number, READ_FILE_MINOR);

        let f = fs::File::open(&p).unwrap();
        let mut data = [0; 5];
        f.read_exact_at(&mut data, 7).unwrap();
        assert_eq!(&data, b"89123");
    });
}

#[test]
fn test_read_unimplemented() {
    with_kernel_module(|| {
        let device_number = get_device_major_number();
        let p = temporary_file_path();
        let _u = mknod(&p, device_number, SEEK_FILE_MINOR);

        let mut f = fs::File::open(&p).unwrap();
        let mut data = [0; 12];
        assert_eq!(
            f.read_exact(&mut data).unwrap_err().raw_os_error().unwrap(),
            libc::EINVAL
        );
    })
}

#[test]
fn test_lseek_unimplemented() {
    with_kernel_module(|| {
        let device_number = get_device_major_number();
        let p = temporary_file_path();
        let _u = mknod(&p, device_number, READ_FILE_MINOR);

        let mut f = fs::File::open(&p).unwrap();
        assert_eq!(
            f.seek(SeekFrom::Start(12))
                .unwrap_err()
                .raw_os_error()
                .unwrap(),
            libc::ESPIPE
        );
        assert_eq!(
            f.seek(SeekFrom::End(-12))
                .unwrap_err()
                .raw_os_error()
                .unwrap(),
            libc::ESPIPE
        );
        assert_eq!(
            f.seek(SeekFrom::Current(12))
                .unwrap_err()
                .raw_os_error()
                .unwrap(),
            libc::ESPIPE
        );
    });
}

#[test]
fn test_lseek() {
    with_kernel_module(|| {
        let device_number = get_device_major_number();
        let p = temporary_file_path();
        let _u = mknod(&p, device_number, SEEK_FILE_MINOR);

        let mut f = fs::File::open(&p).unwrap();
        assert_eq!(f.seek(SeekFrom::Start(12)).unwrap(), 1234);
        assert_eq!(f.seek(SeekFrom::End(-12)).unwrap(), 1234);
        assert_eq!(f.seek(SeekFrom::Current(12)).unwrap(), 1234);

        assert_eq!(
            f.seek(SeekFrom::Start(u64::max_value()))
                .unwrap_err()
                .raw_os_error()
                .unwrap(),
            libc::EINVAL
        );
    });
}
