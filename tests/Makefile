ifneq ($(KERNELRELEASE),)
obj-m := testmodule.o
testmodule-objs := $(TEST_LIBRARY_OBJECT)
EXTRA_LDFLAGS += --entry=init_module
KDIR ?= /lib/modules/$(shell uname -r)/build

$(M)/$(TEST_LIBRARY_OBJECT): target/x86_64-linux-kernel-module/debug/$(TEST_LIBRARY_ARCHIVE)
	$(LD) -r -o $@ --whole-archive $^
else
all:
	$(MAKE) -C $(KDIR) M=$(CURDIR)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
endif
