use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use tempfile::TempDir;

struct LoadedModule {
    name: String,
}

impl LoadedModule {
    fn load(name: String) -> LoadedModule {
        let status = Command::new("sudo")
            .arg("insmod")
            .arg(&name)
            .status()
            .unwrap();
        assert!(status.success());
        return LoadedModule { name };
    }
}

impl Drop for LoadedModule {
    fn drop(&mut self) {
        let status = Command::new("sudo")
            .arg("rmmod")
            .arg(&self.name)
            .status()
            .unwrap();
        assert!(status.success());
    }
}

pub fn with_kernel_module<F: Fn()>(f: F) {
    let status = Command::new("sudo")
        .arg("dmesg")
        .arg("-C")
        .status()
        .unwrap();
    assert!(status.success());
    let _m = LoadedModule::load(env::var("KERNEL_MODULE").unwrap());
    f();
}

pub fn assert_dmesg_contains(msgs: &[&[u8]]) {
    let output = Command::new("dmesg").output().unwrap();
    assert!(output.status.success());
    let lines = output.stdout.split(|x| *x == b'\n').collect::<Vec<_>>();
    let mut lines: &[&[u8]] = &lines;
    for msg in msgs {
        let pos = lines.iter().position(|l| l.ends_with(msg));
        assert!(pos.is_some());
        lines = &lines[pos.unwrap()..];
    }
}

pub fn get_device_major_number(name: &str) -> libc::dev_t {
    let devices = fs::read_to_string("/proc/devices").unwrap();
    let dev_no_line = devices.lines().find(|l| l.ends_with(name)).unwrap();
    let elements = dev_no_line.rsplitn(2, " ").collect::<Vec<_>>();
    assert_eq!(elements.len(), 2);
    assert_eq!(elements[0], name);
    return elements[1].trim().parse().unwrap();
}

pub fn temporary_file_path() -> PathBuf {
    let mut p = TempDir::new().unwrap().into_path();
    p.push("device");
    return p;
}

pub struct UnlinkOnDrop<'a> {
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

pub fn mknod(path: &PathBuf, major: libc::dev_t, minor: libc::dev_t) -> UnlinkOnDrop {
    Command::new("sudo")
        .arg("mknod")
        .arg("--mode=a=rw")
        .arg(path.to_str().unwrap())
        .arg("c")
        .arg(major.to_string())
        .arg(minor.to_string())
        .status()
        .unwrap();
    return UnlinkOnDrop { path };
}
