use kernel_module_testlib::with_kernel_module;

#[test]
fn test_module_loads() {
    with_kernel_module(|| {});
}
