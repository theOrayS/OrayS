# Necessary dependencies for the build system
CARGO_AXPLAT_VERSION ?= 0.3.0
AXCONFIG_GEN_VERSION ?= 0.2.1
CARGO_BINUTILS_VERSION ?= 0.4.0

# Tool to parse information about the target package
ifeq ($(shell cargo axplat --version 2>/dev/null),)
  $(info Installing cargo-axplat...)
  $(shell cargo install --locked cargo-axplat --version $(CARGO_AXPLAT_VERSION))
endif

# Tool to generate platform configuration files
ifeq ($(shell axconfig-gen --version 2>/dev/null),)
  $(info Installing axconfig-gen...)
  $(shell cargo install --locked axconfig-gen --version $(AXCONFIG_GEN_VERSION))
endif

# Cargo binutils
ifeq ($(shell rust-objcopy --version 2>/dev/null),)
  $(info Installing cargo-binutils...)
  $(shell cargo install --locked cargo-binutils --version $(CARGO_BINUTILS_VERSION))
endif
