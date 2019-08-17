# Linux kernel modules in safe Rust

1. Make sure you have Rust installed, as well as [LLVM/Clang 3.9 or higher](https://github.com/rust-lang/rust-bindgen/issues/1316) and kernel headers. Install [cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild) and the `rust-src` and `rustfmt` components for `rustup` component:

```
apt-get install llvm clang linux-headers-"$(uname -r)" # or the equivalent for your OS
cargo install cargo-xbuild
rustup component add --toolchain=nightly rust-src rustfmt
```

2. cd to one of the examples

```
cd hello-world
```

3. Build the static object with cargo xbuild, pointing it at our custom target

```
cargo xbuild --target $(pwd)/../x86_64-linux-kernel-module.json
```

4. Build the kernel module using the Linux kernel build system (kbuild)

```
make
```

5. Load and unload the module!

```
sudo insmod helloworld.ko
sudo rmmod helloworld
dmesg | tail
```
