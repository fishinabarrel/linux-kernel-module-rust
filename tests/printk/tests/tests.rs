use kernel_module_testlib::{assert_dmesg_contains, with_kernel_module};

#[test]
fn test_printk() {
    with_kernel_module(|| {
        assert_dmesg_contains(&[b"Single element printk", b"", b"printk with 2 parameters!"]);
    });
}
