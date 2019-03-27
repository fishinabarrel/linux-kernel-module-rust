ifneq ($(KERNELRELEASE),)
obj-m := testmodule.o
testmodule-objs := $(TEST_LIBRARY)
EXTRA_LDFLAGS += --entry=init_module
KDIR ?= /lib/modules/$(shell uname -r)/build

$(M)/$(TEST_LIBRARY): $(TEST_LIBRARY_ARCHIVE)
	$(LD) -r -o $@ --whole-archive $^
else
all:
	$(MAKE) -C $(KDIR) M=$(CURDIR)

clean:
	$(MAKE) -C /lib/modules/$(shell uname -r)/build M=$(CURDIR) clean
endif
