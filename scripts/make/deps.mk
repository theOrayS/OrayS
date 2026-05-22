# Necessary dependencies for the build system
CARGO_AXPLAT_VERSION ?= 0.3.0
AXCONFIG_GEN_VERSION ?= 0.2.1
CARGO_BINUTILS_VERSION ?= 0.4.0

# Tool to generate platform configuration files
ifeq ($(shell $(AXCONFIG_GEN) --version 2>/dev/null),)
  $(error axconfig-gen is unavailable. Expected vendor/bin/axconfig-gen, repo-local tools/bin/axconfig-gen, or a preinstalled axconfig-gen; remote builds must not install from the network)
endif

# Cargo binutils
ifeq ($(shell $(RUST_OBJCOPY) --version 2>/dev/null),)
  $(error rust-objcopy is unavailable. Expected vendor/bin/rust-objcopy, repo-local tools/bin/rust-objcopy, or rust llvm-tools; remote builds must not install from the network)
endif
