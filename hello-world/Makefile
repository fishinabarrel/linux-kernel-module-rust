obj-m := helloworld.o
helloworld-objs := libhello_world.o
EXTRA_LDFLAGS += --entry=init_module
KDIR ?= /lib/modules/$(shell uname -r)/build

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
