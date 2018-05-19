[DomminusCarnufex rs-kernel](https://dominuscarnufex.github.io/cours/rs-kernel/en.html)
===

This one adds a syscall to the Linux kernel and rebuilds it. The important steps:

* add a Makefile rule to build an rlib (an ar archive) using `rustc`,
  specifying `#![crate_type="rlib"]` in the source
* extract the `.o` file from the rlib, and rename it to match the expected name
* add the `.o` file to `obj-y`

They also edit the syscall interrupt handler (in asm) to dispatch to
their syscalls.

No reference to lang items. It uses stable `#[no_std]` without `#[no_core]` -
I'd expect the resulting object file just happens not to reference libcore, but
more complicated objects could.

No custom compiler flags etc. other than `#[no_std]`.

[saschagrunert/kmod](https://github.com/saschagrunert/kmod)
===

A custom Makefile drives the build, by copying a Makefile.in that looks
like a normal Linux module Makefile to a build directory, and then
invoking the usual `make -C /lib/modules/$(uname -r)/build M=$PWD` (see
[Documentation/kbuild/modules.txt](https://www.kernel.org/doc/Documentation/kbuild/modules.txt)
).

The custom Makefile sets `RUSTCFLAGS` to
`-O -C code-model=kernel -C relocation-model=static`.

There's also a `module.c` file containing the usual `MODULE_LICENSE`
etc. macros, plus prototypes for `init_module` and `cleanup_module`.

Makefile.in does the following:

* build the module with `cargo rustc`, specifiyng
  `crate_type = ['staticlib']` in Cargo.toml
* set `foo-objs := module.o libfoo.a` (note that order is significant:
  as with shared libraries, static libraries need to come after things
  that depend on them)
* set `obj-m := foo.o`

(The `foo-objs` syntax is for "composite objects," which is documented in
[Documentation/kbuild/makefiles.txt](https://kernel.org/doc/Documentation/kbuild/makefiles.txt)
section 3.3, except for the bit where `-objs` is synonymous to `-y`; see
[scripts/Makefile.lib](https://github.com/torvalds/linux/blob/master/scripts/Makefile.lib)
for that. It works by combining the listed objects with `ld -r` aka
`--relocatable` to generate `foo.o`.)

I couldn't get the out-of-tree build to work right with my 3.16 (Debian
oldstable) kernel; it builds fine if you get rid of the extra Makefile
(rename Makefile.in to Makefile) and get rid of the `src` variable there,
making it build in-tree.

kmod includes two other source files besides `lib.rs` and `module.c`,
which aren't very interesting.  `lang_items.rs` has ~empty
implementations of all three lang-items, and `print.rs` has a
straightforward FFI binding to `printk`, plus a macro called `println!`
that doesn't actually implement the format-string logic. These should be
wired up to Linux's unwinding / OOPSing mechanisms and to `core::fmt`
respectively.

I can replicate the Makefile parts of this by themselves. The following
Makefile successfully links a kernel module out of one.rs and two.c:

```make
obj-m := three.o
three-objs := two.o libone.a
lib%.a: %.rs
	rustc -o $@ -O $<
```

(Since the point of this Makefile is just to make linking work, I'm
being lazy and _not_ specifying `#![no_std]` in the Rust source; libstd
gets copied into `libone.a` but the linker eliminates it as dead code
when building `three.c`. That lets me avoid thinking about lang-items.)

[tsgates/rust.ko](https://github.com/tsgates/rust.ko)
===

This one is much older (2013, last updated 2016) but much more complete.
It brings up a couple of important points:

* You need a custom target file to set things like `disable_redzone` and
  no use of floating point. (Their custom target file also sets
  `code-model` to `kernel` and `relocation-model` to `static`, as with
  the previous one.)
  - Apparently you can just use "+soft-float" in target features these
    days, See [this comment on rust-lang/rfcs#1364](https://github.com/rust-lang/rfcs/issues/1364#issuecomment-234404686).
* The kernel internal ABI has no guarantees at all, and you want to write
  against the kernel C API, using bindgen.
  - One limitation is that bindgen doesn't support C macros.

[pmj/rustykext](https://github.com/pmj/rustykext)
===

MacOS. Also uses `-C soft-float` and `-C no-redzone=y` (and `-C
no-stack-check`, which is no longer necessary).
