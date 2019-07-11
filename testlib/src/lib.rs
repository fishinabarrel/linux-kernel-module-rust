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
