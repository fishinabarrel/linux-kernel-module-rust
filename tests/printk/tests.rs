use std::env;
use std::process::Command;

struct LoadedModule {
    name: String,
}

impl LoadedModule {
    fn load(name: String) -> LoadedModule {
        Command::new("sudo")
            .arg("insmod")
            .arg(&name)
            .status()
            .unwrap();
        return LoadedModule { name };
    }
}

impl Drop for LoadedModule {
    fn drop(&mut self) {
        Command::new("sudo")
            .arg("rmmod")
            .arg(&self.name)
            .status()
            .unwrap();
    }
}

fn with_kernel_module<F: Fn()>(f: F) {
    let _m = LoadedModule::load(env::var("KERNEL_MODULE").unwrap());
    f();
}

fn assert_dmesg_contains(msgs: &[&str]) {
    unimplemented!();
}

#[test]
fn test_printk() {
    with_kernel_module(|| {
        assert_dmesg_contains(&["Single element printk", "", "printk with 2 parameters!"]);
    });
}
