obj-m := testmodule.o
testmodule-objs := $(M)/target/x86_64-linux-kernel-module/debug/lib*.a
EXTRA_LDFLAGS += --entry=init_module

all:
	$(MAKE) -C /lib/modules/$(shell uname -r)/build M=$(CURDIR)

clean:
	$(MAKE) -C /lib/modules/$(shel uname -r)/build M=$(CURDIR) clean
