ifneq ($(KERNELRELEASE),)
obj-m := testmodule.o
testmodule-objs := $(TEST_NAME).rust.o

$(src)/target/x86_64-linux-kernel/debug/lib%.a: $(src)/$(TEST_PATH)/Cargo.toml $(wildcard $(src)/$(TEST_PATH)/src/*.rs)
	cd $(src)/$(TEST_PATH); env -u MAKE -u MAKEFLAGS KERNEL_CLFAGS="$(c_flags)" RUSTFLAGS="-Dwarnings" CARGO_TARGET_DIR=../target cargo build -Z build-std=core,alloc --target=x86_64-linux-kernel

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
