ifneq ($(KERNELRELEASE),)
obj-m := helloworld.o
helloworld-objs := libhello_world.o
EXTRA_LDFLAGS += --entry=init_module

$(M)/libhello_world.o: target/x86_64-linux-kernel-module/debug/libhello_world.a
	$(LD) -r -o $@ --whole-archive $^
else
KDIR ?= /lib/modules/$(shell uname -r)/build

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
endif
