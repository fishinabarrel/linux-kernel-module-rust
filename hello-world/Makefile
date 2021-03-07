export KDIR ?= /lib/modules/$(shell uname -r)/build

CONFIG_CC_IS_CLANG := y
CLANG ?= clang
ifeq ($(origin CC),default)
CC := ${CLANG}
endif

HEADER_SEMVER ?= $(shell uname -r | grep -oG "[0-9]\+\.[0-9]\+\.[0-9]\+" | head -1)
ifeq ($(shell echo $(HEADER_SEMVER) 4.4.0 | tr " " "\n" | sort -V | head -1), $(HEADER_SEMVER))
CC := gcc
CONFIG_CC_IS_CLANG := n
endif

ifneq (,$(wildcard /etc/centos-release))
KCPPFLAGS += -DOS_CENTOS
endif

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR) CC=$(CC) KCPPFLAGS=$(KCPPFLAGS) CONFIG_CC_IS_CLANG=$(CONFIG_CC_IS_CLANG)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) CC=$(CC) clean
