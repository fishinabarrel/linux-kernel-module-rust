ifneq ($(KERNELRELEASE),)
obj-m += dummy.o
clean-files := dummy.c

# Some systems for installing kernel headers (e.g. Debian's) happen to
# trigger the out-of-tree build code because the kernel headers directly
# actually just recursively invokes another non-arch-specific one. This
# means that they already generate absolute paths for -I by using the
# flags/addtree make functions.  Some (e.g. Ubuntu's) do not, and
# generate relative paths. We want absolute paths, but we cannot force
# the out-of-tree build code because it won't work on Debian-style
# kernel headers directory (it will look in the mostly-empty kernel
# headers directory instead of the actual one). So we steal the addtree
# and flags functions from scripts/Kbuild.include, and use them _after_
# the build system has generated paths - if any remaining paths are
# relative, we make them absolute with respect to CURDIR. (Unlike the
# upstream addtree function, we prefix -I./foo. We also need to fix
# -include ./include/linux/kconfig.h)
our_addtree = $(if $(patsubst -I%,%,$(1)), \
$(if $(filter-out -I/% -I../%,$(1)),$(patsubst ./%,$(CURDIR)/%,$(patsubst -I%,-I$(CURDIR)/%,$(1))),$(1)),$(1))
our_flags = $(foreach o,$($(1)),$(call our_addtree,$(o)))

$(M)/dummy.c:
	@echo $(NOSTDINC_FLAGS) $(call our_flags,LINUXINCLUDE) $(or $(__c_flags),$(_c_flags)) $(modkern_cflags)
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
