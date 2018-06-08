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
    Command::new("sudo").arg("dmesg").arg("-C").status().unwrap();
    let _m = LoadedModule::load(env::var("KERNEL_MODULE").unwrap());
    f();
}

fn assert_dmesg_contains(msgs: &[&[u8]]) {
    let output = Command::new("dmesg").output().unwrap();
    let lines = output.stdout.split(|x| *x == b'\n').collect::<Vec<_>>();
    for msg in msgs {
       assert!(lines.iter().find(|l| l.ends_with(msg)).is_some());
    }
}

#[test]
fn test_printk() {
    with_kernel_module(|| {
        assert_dmesg_contains(&[b"Single element printk", b"", b"printk with 2 parameters!"]);
    });
}
