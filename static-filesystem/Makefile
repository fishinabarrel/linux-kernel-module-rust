obj-m := staticfilesystem.o
staticfilesystem-objs := target/x86_64-linux-kernel-module/debug/libstatic_filesystem.a ../src/printk_helper.o
EXTRA_LDFLAGS += --gc-sections --entry=init_module --undefined=cleanup_module

all:
	$(MAKE) -C /lib/modules/$(shell uname -r)/build M=$(CURDIR)

clean:
	$(MAKE) -C /lib/modules/$(shell uname -r)/build M=$(CURDIR) clean
