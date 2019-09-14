ifneq ($(KERNELRELEASE),)
obj-m := helloworld.o
helloworld-objs := hello_world.rust.o

$(src)/target/x86_64-linux-kernel/debug/libhello_world.a: $(src)/Cargo.toml $(wildcard $(src)/src/*.c)
	cd $(src); env -u MAKE -u MAKEFLAGS KERNEL_CLFAGS="$(c_flags)" cargo build -Z build-std=core,alloc --target=x86_64-linux-kernel

%.rust.o: target/x86_64-linux-kernel/debug/lib%.a
	$(LD) -r -o $@ --whole-archive $<

else
KDIR ?= /lib/modules/$(shell uname -r)/build
CC ?= clang

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR) CC=$(CC)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) CC=$(CC) clean
endif
