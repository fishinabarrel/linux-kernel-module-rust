obj-m := staticfilesystem.o
staticfilesystem-objs := target/debug/libstatic_filesystem.a ../src/printk_helper.o

all:
	$(MAKE) -C /lib/modules/$(shell uname -r)/build M=$(CURDIR)

clean:
	$(MAKE) -C /lib/modules/$(shell uname -r)/build M=$(CURDIR) clean
