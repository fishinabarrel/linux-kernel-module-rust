obj-m := helloworld.o
helloworld-objs := target/x86_64-linux-kernel-module/debug/libhello_world.a
EXTRA_LDFLAGS += --entry=init_module

all:
	$(MAKE) -C /lib/modules/$(shell uname -r)/build M=$(CURDIR)

clean:
	$(MAKE) -C /lib/modules/$(shell uname -r)/build M=$(CURDIR) clean
