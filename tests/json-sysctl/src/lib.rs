#![no_std]

use core::cmp::min;
use core::convert::TryInto;
use core::sync::atomic::{AtomicBool, Ordering};

use serde::Serialize;

use linux_kernel_module::cstr;
use linux_kernel_module::sysctl::Sysctl;
use linux_kernel_module::Mode;

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);
static C: AtomicBool = AtomicBool::new(false);

struct JsonChrdev;

impl linux_kernel_module::file_operations::FileOperations for JsonChrdev {
    const VTABLE: linux_kernel_module::file_operations::FileOperationsVtable =
        linux_kernel_module::file_operations::FileOperationsVtable::builder::<Self>()
            .read()
            .build();

    fn open() -> linux_kernel_module::KernelResult<Self> {
        Ok(JsonChrdev)
    }
}

impl linux_kernel_module::file_operations::Read for JsonChrdev {
    fn read(
        &self,
        _file: &linux_kernel_module::file_operations::File,
        buf: &mut linux_kernel_module::user_ptr::UserSlicePtrWriter,
        offset: u64,
    ) -> linux_kernel_module::KernelResult<()> {
        let o = Output {
            a: A.load(Ordering::Relaxed),
            b: B.load(Ordering::Relaxed),
            c: C.load(Ordering::Relaxed),
        };
        let mut s = serde_json_core::to_vec::<typenum::U32, _>(&o)
            .map_err(|_| linux_kernel_module::Error::ENOMEM)?;
        s.push(b'\n')
            .map_err(|_| linux_kernel_module::Error::ENOMEM)?;
        let start = min(offset.try_into()?, s.len());
        let end = min(start + buf.len(), s.len());
        buf.write(&s[start..end])?;
        Ok(())
    }
}

struct JsonSysctlModule {
    _a: Sysctl<&'static AtomicBool>,
    _b: Sysctl<&'static AtomicBool>,
    _c: Sysctl<&'static AtomicBool>,
    _chrdev_registration: linux_kernel_module::chrdev::Registration,
}

#[derive(Serialize)]
struct Output {
    a: bool,
    b: bool,
    c: bool,
}

impl linux_kernel_module::KernelModule for JsonSysctlModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let chrdev_registration = linux_kernel_module::chrdev::builder(cstr!("json"), 0..1)?
            .register_device::<JsonChrdev>()
            .build()?;
        Ok(JsonSysctlModule {
            _a: Sysctl::register(cstr!("json-sysctl"), cstr!("a"), &A, Mode::from_int(0o666))?,
            _b: Sysctl::register(cstr!("json-sysctl"), cstr!("b"), &B, Mode::from_int(0o666))?,
            _c: Sysctl::register(cstr!("json-sysctl"), cstr!("c"), &C, Mode::from_int(0o666))?,
            _chrdev_registration: chrdev_registration,
        })
    }
}

linux_kernel_module::kernel_module!(
    JsonSysctlModule,
    author: b"Fish in a Barrel Contributors",
    description: b"Use JSON serialization in kernelspace",
    license: b"GPL"
);
