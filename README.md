# Linux kernel modules in safe Rust

1. Install [cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild) and the `rust-src` `rustup` component:

```
cargo install cargo-xbuild
rustup component add --toolchain=nightly rust-src
```

2. cd to one of the examples

```
cd hello-world
```

3. Build the static object with cargo xbuild, pointing it at our custom target

```
RUST_TARGET_PATH=$(pwd)/.. cargo xbuild --target x86_64-linux-kernel-module

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
