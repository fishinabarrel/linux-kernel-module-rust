ifneq ($(KERNELRELEASE),)
obj-m := helloworld.o
helloworld-objs := libhello_world.o
EXTRA_LDFLAGS += --entry=init_module
KDIR ?= /lib/modules/$(shell uname -r)/build

$(M)/libhello_world.o: target/x86_64-linux-kernel-module/debug/libhello_world.a
	$(LD) -r -o $@ --whole-archive $^
else
all:
	$(MAKE) -C $(KDIR) M=$(CURDIR)

clean:
	$(MAKE) -C /lib/modules/$(shell uname -r)/build M=$(CURDIR) clean
endif
