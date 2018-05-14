Prints the compiler arguments needed to run Clang against the kernel
headers. We use this for running libclang from bindgen.

Normal usage: run `make -s` and look at stdout (errors go to stderr). If
you need to build for a specific set of kernel headers, add
something like `KDIR=/lib/modules/3.16.0-4-amd64/build`. If your clang
command is not `clang`, add something like `CLANG=clang-3.8`.

This generates a non-functional kernel module in the process. To clean
up, `make clean`.
