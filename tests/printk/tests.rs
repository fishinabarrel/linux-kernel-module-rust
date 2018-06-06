
#[test]
fn test_printk() {
    with_kernel_module(|| {
        assert_dmesg_contains(&[
            "Single element printk", "", "printk with 2 parameters!"
        ]);
    });
}
