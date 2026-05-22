# Necessary dependencies for the build system
CARGO_AXPLAT_VERSION ?= 0.3.0
AXCONFIG_GEN_VERSION ?= 0.2.1
CARGO_BINUTILS_VERSION ?= 0.4.0

# Tool to parse information about the target package
ifeq ($(shell cargo axplat --version 2>/dev/null),)
  $(error cargo-axplat is unavailable. Expected repo-local tools/bin/cargo-axplat or a preinstalled cargo-axplat; remote builds must not install from the network)
endif

# Tool to generate platform configuration files
ifeq ($(shell axconfig-gen --version 2>/dev/null),)
  $(error axconfig-gen is unavailable. Expected repo-local tools/bin/axconfig-gen or a preinstalled axconfig-gen; remote builds must not install from the network)
endif

# Cargo binutils
ifeq ($(shell rust-objcopy --version 2>/dev/null),)
  $(error rust-objcopy is unavailable. Expected repo-local tools/bin/rust-objcopy or rust llvm-tools; remote builds must not install from the network)
endif
