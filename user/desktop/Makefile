REPO_ROOT := $(abspath ../..)

.PHONY: all assets host-test golden golden-check performance rv la

all:
	$(MAKE) -C $(REPO_ROOT) -f Makefile.desktop desktop

assets host-test golden golden-check performance rv la:
	$(MAKE) -C $(REPO_ROOT) -f Makefile.desktop desktop-$@
