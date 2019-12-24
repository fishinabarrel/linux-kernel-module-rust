#![no_std]

use alloc::vec;

use linux_kernel_module::sysctl::{Sysctl, SysctlStorage};
use linux_kernel_module::{self, cstr, random, Mode};

struct EntropySource;

impl SysctlStorage for EntropySource {
    fn store_value(&self, _data: &[u8]) -> (usize, linux_kernel_module::KernelResult<()>) {
        (0, Err(linux_kernel_module::Error::EINVAL))
    }

    fn read_value(
        &self,
        data: &mut linux_kernel_module::user_ptr::UserSlicePtrWriter,
    ) -> (usize, linux_kernel_module::KernelResult<()>) {
        let mut storage = vec![0; data.len()];
        if let Err(e) = random::getrandom(&mut storage) {
            return (0, Err(e));
        }
        (storage.len(), data.write(&storage))
    }
}

struct RandomTestModule {
    _sysctl_entropy: Sysctl<EntropySource>,
}

impl linux_kernel_module::KernelModule for RandomTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        Ok(RandomTestModule {
            _sysctl_entropy: Sysctl::register(
                cstr!("rust/random-tests"),
                cstr!("entropy"),
                EntropySource,
                Mode::from_int(0o444),
            )?,
        })
    }
}

linux_kernel_module::kernel_module!(
    RandomTestModule,
    author: "Fish in a Barrel Contributors",
    description: "A module for testing the CSPRNG",
    license: "GPL"
);
