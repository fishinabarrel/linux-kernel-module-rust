use std::fs;
use std::path::PathBuf;
use std::process::Command;

use tempfile::TempDir;

use kernel_module_testlib::{with_kernel_module, assert_dmesg_contains};

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

fn temporary_file_path(name: &str) -> PathBuf {
    let mut p = TempDir::new().unwrap().into_path();
    p.push(name);
    return p;
}

struct ImageFile {
    pub path: PathBuf,
}

impl ImageFile {
    fn new(path: PathBuf) -> ImageFile {
        let status = Command::new("touch")
            .arg(path.to_str().unwrap())
            .status()
            .unwrap();
        assert!(status.success());
        ImageFile { path }
    }

    fn zero_init(&mut self) {
        let status = Command::new("dd")
            .arg("bs=4096")
            .arg("count=1024")
            .arg("if=/dev/zero")
            .arg(format!("of={}", self.path.to_str().unwrap()))
            .status()
            .unwrap();
        assert!(status.success());
    }
}

impl Drop for ImageFile {
    fn drop(&mut self) {
        Command::new("rm")
            .arg(self.path.to_str().unwrap())
            .status()
            .unwrap();
    }
}

struct LoopDev {
    pub path: String,
    _img: ImageFile,
}

impl LoopDev {
    fn new(img: ImageFile) -> LoopDev {
        // -f finds first available loop device.
        let status = Command::new("sudo").arg("losetup")
            .arg("-f")
            .arg(img.path.to_str().unwrap())
            .status()
            .unwrap();
        assert!(status.success());

        // Get the name of the loop device that was availble.
        let output = String::from_utf8(
            Command::new("sudo").arg("losetup")
                .arg("--associated").arg(img.path.to_str().unwrap())
                .arg("--noheadings")
                .arg("--output").arg("NAME")
                .output()
                .unwrap()
                .stdout
        ).unwrap();

        LoopDev {
            path: String::from(output.as_str().trim()),
            _img: img,
        }
    }
}

impl Drop for LoopDev {
    fn drop(&mut self) {
        Command::new("sudo").arg("losetup")
            .arg("-d").arg(&self.path)
            .status()
            .unwrap();
    }
}

struct Mountpoint {
    pub path: PathBuf,
}

impl Mountpoint {
    fn new(path: PathBuf) -> Mountpoint {
        let status = Command::new("mkdir")
            .arg(path.to_str().unwrap())
            .status()
            .unwrap();
        assert!(status.success());
        Mountpoint { path }
    }
}

impl Drop for Mountpoint {
    fn drop(&mut self) {
        Command::new("rm")
            .arg("-rfd")
            .arg(self.path.to_str().unwrap())
            .status()
            .unwrap();
    }
}

struct Mount {
    mp: Mountpoint,
    _dev: LoopDev,
}

impl Mount {
    fn new(dev: LoopDev, mp: Mountpoint) -> Mount {
        let status = Command::new("sudo").arg("mount")
            .arg("-o").arg("loop")
            .arg("-t").arg("testfs")
            .arg(&dev.path)
            .arg(mp.path.to_str().unwrap())
            .status()
            .unwrap();
        assert!(status.success());
        Mount {
            mp: mp,
            _dev: dev,
        }
    }
}

impl Drop for Mount {
    fn drop(&mut self) {
        Command::new("sudo")
            .arg("umount")
            .arg(self.mp.path.to_str().unwrap())
            .status()
            .unwrap();
    }
}

#[test]
fn test_fill_super() {
    with_kernel_module(|| {
        let mut img = ImageFile::new(temporary_file_path("testfs_image"));
        img.zero_init();
        let dev = LoopDev::new(img);
        let mp = Mountpoint::new(temporary_file_path("testfs_mountpoint"));
        let mount = Mount::new(dev, mp);

        assert_dmesg_contains(&[b"TestFS fill_super executed."]);

        drop(mount);

        assert_dmesg_contains(&[b"TestFS put_super executed."]);
    });
}
