ifneq ($(KERNELRELEASE),)
obj-m += dummy.o
clean-files := dummy.c

$(M)/dummy.c:
	@echo $(NOSTDINC_FLAGS) $(LINUXINCLUDE) $(__c_flags) $(modkern_cflags)
	@touch $@

.PHONY: $(M)/dummy.c
else
KDIR ?= /lib/modules/$(shell uname -r)/build
CLANG ?= clang
all:
	$(MAKE) -C $(KDIR) M=$(CURDIR) CC=$(CLANG)
clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
endif
