obj-m := helloworld.o
helloworld-objs := target/x86_64-linux-kernel-module/debug/libhello_world.a ../src/printk_helper.o
EXTRA_LDFLAGS += --gc-sections --entry=init_module --undefined=cleanup_module

all:
	$(MAKE) -C /lib/modules/$(shell uname -r)/build M=$(CURDIR)

clean:
	$(MAKE) -C /lib/modules/$(shell uname -r)/build M=$(CURDIR) clean
