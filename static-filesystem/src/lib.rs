#[macro_use]
extern crate linux_kernel_module;

struct StaticFileSystem;

impl KernelModule for StaticFileSystem {
    fn init() -> Result<(), ()> {

    }

    fn exit() {

    }
}

kernel_module!(StaticFileSystem);
