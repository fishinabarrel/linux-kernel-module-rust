use std::env;
use std::process::Command;

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
