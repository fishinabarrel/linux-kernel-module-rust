obj-m := testmodule.o
testmodule-objs := $(TEST_NAME).rust.o
KDIR ?= /lib/modules/$(shell uname -r)/build

%.rust.o: target/x86_64-linux-kernel-module/debug/lib%.a
	$(LD) -r -o $@ --whole-archive $<

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
