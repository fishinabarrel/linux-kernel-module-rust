KDIR ?= /lib/modules/$(shell uname -r)/build
CLANG ?= clang

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR) CC=$(CLANG)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
