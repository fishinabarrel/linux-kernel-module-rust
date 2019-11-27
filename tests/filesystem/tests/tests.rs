use std::fs;
use std::process::Command;

use tempfile;

use kernel_module_testlib::{assert_dmesg_contains, with_kernel_module};

#[test]
fn test_proc_filesystems() {
    let filesystems = fs::read_to_string("/proc/filesystems").unwrap();
    assert!(!filesystems.contains("testfs"));

    with_kernel_module(|| {
        let filesystems = fs::read_to_string("/proc/filesystems").unwrap();
        assert!(filesystems.contains("testfs"));
    });

    let filesystems = fs::read_to_string("/proc/filesystems").unwrap();
    assert!(!filesystems.contains("testfs"));
}

struct Mount {
    mountpoint: tempfile::TempDir,
    _image: tempfile::NamedTempFile,
}

impl Mount {
    fn new(image: tempfile::NamedTempFile, mountpoint: tempfile::TempDir) -> Mount {
        let status = Command::new("sudo")
            .arg("mount")
            .arg(image.path().to_str().unwrap())
            .arg(mountpoint.path().to_str().unwrap())
            .arg("--types")
            .arg("testfs")
            .arg("--options")
            .arg("loop")
            .status()
            .unwrap();
        assert!(status.success());
        Mount {
            mountpoint: mountpoint,
            _image: image,
        }
    }
}

impl Drop for Mount {
    fn drop(&mut self) {
        // $ man 8 mount: Since Linux 2.6.25 auto-destruction of loop devices is
        // supported, meaning that any loop device allocated by mount will be
        // freed by umount independently of /etc/mtab.
        Command::new("sudo")
            .arg("umount")
            .arg(self.mountpoint.path().to_str().unwrap())
            .status()
            .unwrap();
    }
}

#[test]
fn test_fill_super() {
    with_kernel_module(|| {
        let image = tempfile::Builder::new()
            .prefix("testfs-image-")
            .tempfile()
            .unwrap();
        let dd_status = Command::new("dd")
            .arg("bs=4096")
            .arg("count=1024")
            .arg("if=/dev/zero")
            .arg(format!("of={}", image.path().to_str().unwrap()))
            .arg("status=none") // no spam
            .status()
            .unwrap();
        assert!(dd_status.success());
        let mountpoint = tempfile::Builder::new()
            .prefix("testfs-mountpoint-")
            .tempdir()
            .unwrap();
        let mount = Mount::new(image, mountpoint);

        assert_dmesg_contains(&[b"testfs-fill_super-marker"]);

        drop(mount);

        assert_dmesg_contains(&[b"testfs-put_super-marker"]);
    });
}
