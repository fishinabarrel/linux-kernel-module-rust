obj-m := examplesysctl.o
examplesysctl-objs := target/x86_64-linux-kernel-module/debug/libexample_sysctl.a
EXTRA_LDFLAGS += --entry=init_module
KDIR ?= /lib/modules/$(shell uname -r)/build

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
