use std::process::Command;

use kernel_module_tests::with_kernel_module;

fn assert_dmesg_contains(msgs: &[&[u8]]) {
    let output = Command::new("dmesg").output().unwrap();
    let lines = output.stdout.split(|x| *x == b'\n').collect::<Vec<_>>();
    let mut lines: &[&[u8]] = &lines;
    for msg in msgs {
        let pos = lines.iter().position(|l| l.ends_with(msg));
        assert!(pos.is_some());
        lines = &lines[pos.unwrap()..];
    }
}

#[test]
fn test_printk() {
    with_kernel_module(|| {
        assert_dmesg_contains(&[b"Single element printk", b"", b"printk with 2 parameters!"]);
    });
}
