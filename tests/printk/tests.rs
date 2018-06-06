fn with_kernel_module<F: Fn()>(f: F) {
    unimplemented!();
}

fn assert_dmesg_contains(msgs: &[&str]) {
    unimplemented!();
}

#[test]
fn test_printk() {
    with_kernel_module(|| {
        assert_dmesg_contains(&[
            "Single element printk", "", "printk with 2 parameters!"
        ]);
    });
}
