obj-m := helloworld.o
helloworld-objs := hello_world.rust.o
KDIR ?= /lib/modules/$(shell uname -r)/build

%.rust.o: target/x86_64-linux-kernel/debug/lib%.a
	$(LD) -r -o $@ --whole-archive $<

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
