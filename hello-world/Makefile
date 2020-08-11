KDIR ?= /lib/modules/$(shell uname -r)/build

CLANG ?= clang
ifeq ($(origin CC),default)
CC := ${CLANG}
endif

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR) CC=$(CC) CONFIG_CC_IS_CLANG=y cmd_cc_o_c='$$(CC) $$(filter-out -D%_PLUGIN,$$(filter-out -fplugin%,$$(c_flags))) -c -o $$@ $$<'

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) CC=$(CC) clean
