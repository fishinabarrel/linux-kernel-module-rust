# Linux kernel modules in safe Rust

**For most purposes, if you're interested in writing Linux kernel modules in Rust, you should look at https://github.com/Rust-for-Linux/linux, which is an effort to contribute that process to the upstream kernel.**

This is a framework for writing loadable Linux kernel modules in Rust,
using safe abstractions around kernel interfaces and primitives.

For more information on the motivation and goals for this project, check
out [our presentation at Linux Security Summit North America
2019](https://ldpreload.com/p/kernel-modules-in-rust-lssna2019.pdf)
and the [video on YouTube](https://www.youtube.com/watch?v=RyY01fRyGhM).
We're immediately focusing on making this project viable for out-of-tree
modules, but we also see this project as a testing ground for whether
in-tree components could be written in Rust.

There is a simple demo module in the hello-world directory, as well as
various other examples in the tests/ directory.

## Design

We run [bindgen](https://github.com/rust-lang/rust-bindgen) on the
kernel headers to generate automatic Rust FFI bindings. bindgen is
powered by [Clang](https://clang.llvm.org), so we use the kernel's
own build system to determine the appropriate CFLAGS. Then we write safe
bindings to these types (see the various files inside `src/`).

Each kernel module in Rust lives in a `staticlib` crate, which generates
a `.a` file. We pass this object to the Linux kernel's own module build
system for linking into a `.ko`.

The kernel is inherently multi-threaded: kernel resources can be
accessed from multiple userspace processes at once, which causes
multiple threads of execution inside the kernel to handle system calls
(or interrupts). Therefore, the `KernelModule` type is
[`Sync`](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html),
so all data shared by a kernel module must be safe to access
concurrently (such as by implementing locking).

## System requirements

We're currently only running CI on Linux 4.15 (Ubuntu Xenial) on amd64,
although we intend to support kernels from 4.4 through the latest
release. Please report a bug (or better yet, send in a patch!) if your
kernel doesn't work.

Other architectures aren't implemented yet, but should work as long as
there's Rust and LLVM support - see [#112][]
for expected status. They'll need `src/c_types.rs` ported, too.

You'll need to have [Rust](https://www.rust-lang.org) - in particular
Rust nightly, as we use [some unstable
features](https://github.com/fishinabarrel/linux-kernel-module-rust/issues/41) -
and [Clang](https://clang.llvm.org) installed. You need LLVM/Clang 9
(released September 2019) or higher for multiple reasons, primarily
[support for `asm goto`][]. If you're on Debian, Ubuntu, or a derivative,
https://apt.llvm.org is great.

If the binary named `clang` is too old, make sure to set the `CC` or
`CLANG` environment variable appropriately, e.g., `CC=clang-9`.

Very recent kernels may require newer versions of Clang - try Clang 11
if older versions don't work for you.

[#112]: https://github.com/fishinabarrel/linux-kernel-module-rust/issues/112
[support for `asm goto`]: https://github.com/fishinabarrel/linux-kernel-module-rust/issues/123

## Building hello-world

1. Install clang, kernel headers, and the `rust-src` and `rustfmt` components
from `rustup`:

```
apt-get install llvm clang linux-headers-"$(uname -r)" # or the equivalent for your OS
rustup component add --toolchain=nightly rust-src rustfmt
```

2. cd to one of the examples

```
cd hello-world
```

3. Build the kernel module using the Linux kernel build system (kbuild), this
will invoke `cargo` to build the Rust code

```
make
```

4. Load and unload the module!

```
sudo insmod helloworld.ko
sudo rmmod helloworld
dmesg | tail
```
