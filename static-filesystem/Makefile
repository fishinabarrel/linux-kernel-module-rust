obj-m := staticfilesystem.o
staticfilesystem-objs := target/x86_64-linux-kernel-module/debug/libstatic_filesystem.a
EXTRA_LDFLAGS += --entry=init_module

all:
	$(MAKE) -C /lib/modules/$(shell uname -r)/build M=$(CURDIR)

clean:
	$(MAKE) -C /lib/modules/$(shell uname -r)/build M=$(CURDIR) clean
