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

fn temporary_file_path() -> PathBuf {
    let mut p = TempDir::new().unwrap().into_path();
    p.push("node");
    return p;
}

struct ImageFile {
    pub path: PathBuf,
}

impl ImageFile {
    fn new(path: PathBuf) -> ImageFile {
        Command::new("touch")
            .arg(path.to_str().unwrap())
            .status()
            .unwrap();
        ImageFile { path }
    }

    fn zero_init(&mut self) {
        Command::new("dd")
            .arg("bs=4096")
            .arg("count=1024")
            .arg("if=/dev/zero")
            .arg(format!("of={}", self.path.to_str().unwrap()))
            .status()
            .unwrap();
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
    pub path: PathBuf,
    _img: ImageFile,
}

impl LoopDev {
    fn new(path: PathBuf, img: ImageFile) -> LoopDev {
        Command::new("sudo")
            .arg("losetup")
            .arg(path.to_str().unwrap())
            .arg(img.path.to_str().unwrap())
            .status()
            .unwrap();
        LoopDev {
            path: path,
            _img: img,
        }
    }
}

impl Drop for LoopDev {
    fn drop(&mut self) {
        Command::new("sudo")
            .arg("losetup")
            .arg("-d")
            .arg(self.path.to_str().unwrap())
            .status()
            .unwrap();
    }
}

struct Mountpoint {
    pub path: PathBuf,
}

impl Mountpoint {
    fn new(path: PathBuf) -> Mountpoint {
        Command::new("mkdir")
            .arg(path.to_str().unwrap())
            .status()
            .unwrap();
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
        Command::new("sudo")
            .arg("mount")
            .arg("-o")
            .arg("loop")
            .arg("-t")
            .arg("testfs")
            .arg(dev.path.to_str().unwrap())
            .arg(mp.path.to_str().unwrap())
            .status()
            .unwrap();
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
        let mut img = ImageFile::new(PathBuf::from("testfs-loop-image"));
        img.zero_init();
        let dev = LoopDev::new(PathBuf::from("/dev/loop0"), img);
        let mp = Mountpoint::new(PathBuf::from("testfs-loop-mountpoint"));
        let mount = Mount::new(dev, mp);

        assert_dmesg_contains(&[b"TestFS fill_super successfull."]);

        drop(mount);
    });
}
