obj-m := helloworld.o
helloworld-objs := target/x86_64-linux-kernel-module/debug/libhello_world.a
EXTRA_LDFLAGS += --entry=init_module
KDIR ?= /lib/modules/$(shell uname -r)/build

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
