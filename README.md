# Linux kernel modules in safe Rust

1. Install [cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild):

```
cargo install cargo-xbuild
```

2. cd to one of the examples

```
cd hello-world
```

3. Build the static object with cargo xbuild, pointing it at our custom target

```
RUST_TARGET_PATH=~/linux-kernel-module-rust cargo xbuild --target x86_64-linux-kernel-module

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
