#![no_std]

use linux_kernel_module::{self, println, rcu, sched};

struct ForEachProcessTestModule;

impl linux_kernel_module::KernelModule for ForEachProcessTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let g = rcu::RcuReadGuard::new();
        for mut p in sched::each_process(&g) {
            let comm = p.comm();
            let comm_until_nul = comm.split(|c| *c == 0).next().unwrap();
            println!(
                "for-each-process: {:8} {}",
                p.tgid(),
                core::str::from_utf8(comm_until_nul).unwrap_or("[invalid UTF-8]")
            );
        }
        Ok(ForEachProcessTestModule)
    }
}

linux_kernel_module::kernel_module!(
    ForEachProcessTestModule,
    author: b"Fish in a Barrel Contributors",
    description: b"A module for testing EachProcess",
    license: b"GPL"
);
