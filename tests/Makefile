ifneq ($(KERNELRELEASE),)
obj-m := testmodule.o
TEST_LIBRARY=target/x86_64-linux-kernel-module/debug/$(TEST_LIBRARY_ARCHIVE)
testmodule-objs := $(TEST_LIBRARY)
EXTRA_LDFLAGS += --entry=init_module

#$(M)/$(TEST_LIBRARY_OBJECT): target/x86_64-linux-kernel-module/debug/$(TEST_LIBRARY_ARCHIVE)
#	$(LD) -r -o $@ --whole-archive $^
else
KDIR ?= /lib/modules/$(shell uname -r)/build

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
endif
