use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::prelude::FileExt;

use kernel_module_testlib::*;

const DEVICE_NAME: &'static str = "chrdev-tests";
const READ_FILE_MINOR: libc::dev_t = 0;
const SEEK_FILE_MINOR: libc::dev_t = 1;
const WRITE_FILE_MINOR: libc::dev_t = 2;

#[test]
fn test_mknod() {
    with_kernel_module(|| {
        let device_number = get_device_major_number(DEVICE_NAME);
        mknod(&temporary_file_path(), device_number, READ_FILE_MINOR);
    });
}

#[test]
fn test_read() {
    with_kernel_module(|| {
        let device_number = get_device_major_number(DEVICE_NAME);
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
        let device_number = get_device_major_number(DEVICE_NAME);
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
        let device_number = get_device_major_number(DEVICE_NAME);
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
        let device_number = get_device_major_number(DEVICE_NAME);
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
        let device_number = get_device_major_number(DEVICE_NAME);
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
        let device_number = get_device_major_number(DEVICE_NAME);
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

#[test]
fn test_write_unimplemented() {
    with_kernel_module(|| {
        let device_number = get_device_major_number(DEVICE_NAME);
        let p = temporary_file_path();
        let _u = mknod(&p, device_number, READ_FILE_MINOR);

        let mut f = fs::OpenOptions::new().write(true).open(&p).unwrap();
        assert_eq!(
            f.write(&[1, 2, 3]).unwrap_err().raw_os_error().unwrap(),
            libc::EINVAL
        );
    })
}

#[test]
fn test_write() {
    with_kernel_module(|| {
        let device_number = get_device_major_number(DEVICE_NAME);
        let p = temporary_file_path();
        let _u = mknod(&p, device_number, WRITE_FILE_MINOR);

        let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&p)
            .unwrap();
        assert_eq!(f.write(&[1, 2, 3]).unwrap(), 3);

        let mut buf = [0; 1];
        f.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"3");

        assert_eq!(f.write(&[1, 2, 3, 4, 5]).unwrap(), 5);

        let mut buf = [0; 1];
        f.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"8");
    })
}
