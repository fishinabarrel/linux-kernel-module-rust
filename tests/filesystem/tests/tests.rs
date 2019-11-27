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

struct LoopDev {
    pub path: String,
    _image: tempfile::NamedTempFile,
}

impl LoopDev {
    fn new(image: tempfile::NamedTempFile) -> LoopDev {
        let image_path = image.path().to_str().unwrap();

        let status = Command::new("sudo")
            .arg("losetup")
            .arg("--find") // ... first available loop device.
            .arg(image_path)
            .status()
            .unwrap();
        assert!(status.success());

        // Get the name of the loop device that was availble:
        let result = Command::new("sudo")
            .arg("losetup")
            .arg("--associated")
            .arg(image_path)
            .arg("--noheadings")
            .arg("--output")
            .arg("NAME")
            .output()
            .unwrap();
        let output = String::from_utf8(result.stdout).unwrap();

        LoopDev {
            path: String::from(output.as_str().trim()),
            _image: image,
        }
    }
}

impl Drop for LoopDev {
    fn drop(&mut self) {
        Command::new("sudo")
            .arg("losetup")
            .arg("--detach")
            .arg(&self.path)
            .status()
            .unwrap();
    }
}

struct Mount {
    mountpoint: tempfile::TempDir,
    _dev: LoopDev,
}

impl Mount {
    fn new(dev: LoopDev, mountpoint: tempfile::TempDir) -> Mount {
        let status = Command::new("sudo")
            .arg("mount")
            .arg("--options")
            .arg("loop")
            .arg("--types")
            .arg("testfs")
            .arg(&dev.path)
            .arg(mountpoint.path().to_str().unwrap())
            .status()
            .unwrap();
        assert!(status.success());
        Mount {
            mountpoint: mountpoint,
            _dev: dev,
        }
    }
}

impl Drop for Mount {
    fn drop(&mut self) {
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
        let status = Command::new("dd")
            .arg("bs=4096")
            .arg("count=1024")
            .arg("if=/dev/zero")
            .arg(format!("of={}", image.path().to_str().unwrap()))
            .arg("status=none") // no spam
            .status()
            .unwrap();
        assert!(status.success());

        let loop_dev = LoopDev::new(image);
        let mountpoint = tempfile::Builder::new()
            .prefix("testfs-mountpoint-")
            .tempdir()
            .unwrap();

        let mount = Mount::new(loop_dev, mountpoint);

        assert_dmesg_contains(&[b"testfs-fill_super-marker"]);

        drop(mount);

        assert_dmesg_contains(&[b"testfs-put_super-marker"]);
    });
}
