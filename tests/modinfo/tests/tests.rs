use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use kernel_module_testlib::with_kernel_module;

#[test]
fn test_modinfo() {
    let module = env::var("KERNEL_MODULE").unwrap();

    for (key, value) in &[
        ("author", "Fish in a Barrel Contributors"),
        ("description", "Empty module for testing modinfo"),
        ("license", "GPL"),
    ] {
        let modinfo = Command::new("modinfo")
            .arg("-F")
            .arg(key)
            .arg(&module)
            .output()
            .unwrap();
        assert!(modinfo.status.success());
        assert_eq!(&std::str::from_utf8(&modinfo.stdout).unwrap().trim(), value);
    }
}

#[test]
fn test_no_proprietary_taint() {
    let module = env::var("KERNEL_MODULE").unwrap();
    let module_name = Path::new(&module).file_stem().unwrap();
    let sysfs_path = Path::new("/sys/module").join(module_name).join("taint");

    with_kernel_module(|| {
        let taints = fs::read_to_string(&sysfs_path).unwrap();
        assert!(!taints.contains("P"));
    });
}
