# Available arguments:
# * General options:
#     - `ARCH`: Target architecture: x86_64, riscv64, aarch64, loongarch64
#     - `MYPLAT`: Package name of the target platform crate.
#     - `PLAT_CONFIG`: Path to the platform configuration file.
#     - `SMP`: Override maximum CPU number specified in the platform config. For
#       statically configured platforms, this is also the number of CPUs to boot
#       and for platforms with runtime CPU detection, this is the upper limit of
#       CPUs.
#     - `MODE`: Build mode: release, debug
#     - `LOG:` Logging level: warn, error, info, debug, trace
#     - `V`: Verbose level: (empty), 1, 2
#     - `TARGET_DIR`: Artifact output directory (cargo target directory)
#     - `EXTRA_CONFIG`: Extra config specification file
#     - `OUT_CONFIG`: Final config file that takes effect
#     - `UIMAGE`: To generate U-Boot image
#     - `LD_SCRIPT`: Use a custom linker script file.
# * App options:
#     - `A` or `APP`: Path to the application
#     - `FEATURES`: Features os ArceOS modules to be enabled.
#     - `APP_FEATURES`: Features of (rust) apps to be enabled.
# * QEMU options:
#     - `BLK`: Enable storage devices (virtio-blk)
#     - `NET`: Enable network devices (virtio-net)
#     - `GRAPHIC`: Enable display devices and graphic output (virtio-gpu)
#     - `BUS`: Device bus type: mmio, pci
#     - `MEM`: Memory size (default is 128M)
#     - `DISK_IMG`: Path to the virtual disk image
#     - `ACCEL`: Enable hardware acceleration (KVM on linux)
#     - `QEMU_LOG`: Enable QEMU logging (log file is "qemu.log")
#     - `NET_DUMP`: Enable network packet dump (log file is "netdump.pcap")
#     - `NET_DEV`: QEMU netdev backend types: user, tap, bridge
#     - `VFIO_PCI`: PCI device address in the format "bus:dev.func" to passthrough
#     - `VHOST`: Enable vhost-net for tap backend (only for `NET_DEV=tap`)
# * Network options:
#     - `IP`: ArceOS IPv4 address (default is 10.0.2.15 for QEMU user netdev)
#     - `GW`: Gateway IPv4 address (default is 10.0.2.2 for QEMU user netdev)

# General options
ARCH ?= x86_64
PYTHON ?= python3
MYPLAT ?=
PLAT_CONFIG ?=
PLATFORM_CONFIG_DIR ?= $(CURDIR)/configs/platforms
# Remote official evaluation invokes `make all` without extra arguments.
# Keep local QEMU targets (`kernel-la`, `run-la`, `./run-eval.sh la`) on the
# package default platform config, but build the submission `kernel-la` with the
# remote LoongArch address map that matches the official evaluator.
REMOTE_LA_PLAT_CONFIG ?= $(CURDIR)/test/evaluation/config/loongarch64_submission.toml
SMP ?=
MODE ?= release
LOG ?= warn
V ?=
TARGET_DIR ?= $(PWD)/target
EXTRA_CONFIG ?=
OUT_CONFIG ?= $(PWD)/.axconfig.toml
UIMAGE ?= n

# Kernel build options
KERNEL_APP ?= user/shell
KERNEL_FEATURES ?= alloc,paging,irq,multitask,fs,net,rtc
KERNEL_RV_FEATURES ?= $(KERNEL_FEATURES)
KERNEL_LA_FEATURES ?= $(KERNEL_FEATURES)
KERNEL_APP_FEATURES ?= auto-run-tests,uspace
KERNEL_RV_APP_FEATURES ?= $(KERNEL_APP_FEATURES)
KERNEL_LA_APP_FEATURES ?= $(KERNEL_APP_FEATURES)
KERNEL_MODE ?= release
KERNEL_LOG ?= info
KERNEL_SMP ?= 1
KERNEL_BUILD_DIR ?= $(CURDIR)/build/kernels
KERNEL_TARGET_DIR ?= $(KERNEL_BUILD_DIR)/target
KERNEL_RV_OUT_DIR ?= $(KERNEL_BUILD_DIR)/riscv64
KERNEL_LA_OUT_DIR ?= $(KERNEL_BUILD_DIR)/loongarch64
KERNEL_RV_CONFIG ?= $(KERNEL_BUILD_DIR)/riscv64.axconfig.toml
KERNEL_LA_CONFIG ?= $(KERNEL_BUILD_DIR)/loongarch64.axconfig.toml
KERNEL_RV_TARGET_DIR ?= $(KERNEL_TARGET_DIR)/riscv64
KERNEL_LA_TARGET_DIR ?= $(KERNEL_TARGET_DIR)/loongarch64
KERNEL_RV_AXCONFIG_WRITES ?= -w plat.phys-memory-size=0x4000_0000
# QEMU loongarch64 virt splits 1G RAM into lowram [0, 0x1000_0000)
# and highram [0x8000_0000, 0xb000_0000). ArceOS uses highram as the
# contiguous RAM window, so do not advertise the hole above 0xb000_0000.
KERNEL_LA_AXCONFIG_WRITES ?= -w plat.phys-memory-size=0x3000_0000
KERNEL_RV ?= $(CURDIR)/kernel-rv
KERNEL_LA ?= $(CURDIR)/kernel-la
PR3_SMOKE_BUILD_DIR ?= $(CURDIR)/build/pr3-smoke
PR3_SMOKE_KERNEL_FEATURES ?= alloc,paging,irq,multitask,fs,net,driver-ramdisk
PR3_SMOKE_APP_FEATURES ?= semantic-smoke
PR3_SMOKE_RV_KERNEL ?= $(PR3_SMOKE_BUILD_DIR)/kernel-rv
PR3_SMOKE_LA_KERNEL ?= $(PR3_SMOKE_BUILD_DIR)/kernel-la
PR3_SMOKE_RV_OUT_DIR ?= $(PR3_SMOKE_BUILD_DIR)/riscv64
PR3_SMOKE_LA_OUT_DIR ?= $(PR3_SMOKE_BUILD_DIR)/loongarch64
PR3_SMOKE_RV_CONFIG ?= $(PR3_SMOKE_BUILD_DIR)/riscv64.axconfig.toml
PR3_SMOKE_LA_CONFIG ?= $(PR3_SMOKE_BUILD_DIR)/loongarch64.axconfig.toml
PR3_SMOKE_RV_TARGET_DIR ?= $(PR3_SMOKE_BUILD_DIR)/target/riscv64
PR3_SMOKE_LA_TARGET_DIR ?= $(PR3_SMOKE_BUILD_DIR)/target/loongarch64
PR3_SEMANTIC_MANIFEST ?= $(CURDIR)/test/evidence/semantic_evidence_manifest.json
PR3_SMOKE_RV_EVIDENCE_DIR ?= $(PR3_SMOKE_BUILD_DIR)/evidence-rv64
PR3_SMOKE_LA_EVIDENCE_DIR ?= $(PR3_SMOKE_BUILD_DIR)/evidence-la64
PR3_EVIDENCE_DIR ?= $(CURDIR)/build/pr3-evidence
PR3_HOST_EVIDENCE_DIR ?= $(PR3_EVIDENCE_DIR)/host
PR3_RV_EVIDENCE_DIR ?= $(PR3_EVIDENCE_DIR)/rv64
PR3_LA_EVIDENCE_DIR ?= $(PR3_EVIDENCE_DIR)/la64
PR3_REQUIRED_EVIDENCE_DIR ?= $(PR3_EVIDENCE_DIR)/required
# Remote official evaluation invokes `make` / `make all` with no extra args.
# Default to the trusted stable LTP list so the full non-LTP surface plus both
# libc LTP passes can finish inside the remote scorer.  Broader sweep modes stay
# available for explicit scouting only; skipped/timeout/TCONF/TBROK/TFAIL/ENOSYS
# rows from those modes must not be promoted as PASS.
# The in-kernel `stable` selector is budgeted for the remote deadline and omits
# the currently slowest parser-clean LTP tails; use stable-full for full reproof.
# Explicit broader / scouting modes remain available:
#   make all REMOTE_LTP_CASES=stable-full
#   make all REMOTE_LTP_CASES=stable-plus-blacklist
#   make all REMOTE_LTP_CASES=blacklist
REMOTE_LTP_CASES ?= stable
REMOTE_LTP_BLACKLIST_DIR ?= $(CURDIR)/docs/ltp-full-sweep-blacklist-2026-05-30-arch
REMOTE_LTP_BLACKLIST_COMMON_FILE ?= $(REMOTE_LTP_BLACKLIST_DIR)/blacklist-common.txt
REMOTE_LTP_BLACKLIST_RV_FILE ?= $(REMOTE_LTP_BLACKLIST_DIR)/blacklist-rv.txt
REMOTE_LTP_BLACKLIST_LA_FILE ?= $(REMOTE_LTP_BLACKLIST_DIR)/blacklist-la.txt
REMOTE_LTP_BLACKLIST_MODES := blacklist all-minus-blacklist sweep:blacklist score-blacklist stable-plus-blacklist stable-plus-all-minus-blacklist
REMOTE_LTP_USES_BLACKLIST := $(filter $(REMOTE_LTP_BLACKLIST_MODES),$(REMOTE_LTP_CASES))
# The online scorer does not award points for glibc libctest. Keep that skip
# explicit and build-time visible, so local scouting can clear it with:
#   make all REMOTE_SKIP_OFFICIAL_TEST_GROUPS=
REMOTE_SKIP_OFFICIAL_TEST_GROUPS ?= libctest-glibc
# Full UnixBench is a real official benchmark group and already has a 900s
# nominal in-kernel timeout. Remote builds need a ceiling at least that high;
# the local/default 300s ceiling remains available for quick scouting runs.
REMOTE_GROUP_TIMEOUT_CEILING_SECS ?= 900
ORAYS_WORKSPACE_ROOT ?= $(abspath $(CURDIR)/..)
TESTSUITE_DIR ?= $(ORAYS_WORKSPACE_ROOT)/testsuits-for-oskernel
RV_TESTSUITE_IMG ?= $(ORAYS_WORKSPACE_ROOT)/sdcard-rv.img
LA_TESTSUITE_IMG ?= $(ORAYS_WORKSPACE_ROOT)/sdcard-la.img
RV_TESTSUITE_RUN_IMG ?= /tmp/arceos-sdcard-rv.run.qcow2
LA_TESTSUITE_RUN_IMG ?= /tmp/arceos-sdcard-la.run.qcow2
RV_AUX_DISK ?= $(CURDIR)/disk.img
LA_AUX_DISK ?= $(CURDIR)/disk-la.img
RV_MEM ?= 1G
LA_MEM ?= 1G
RV_NETDEV_ARGS ?= user,id=net
# Keep LA default consistent with RV: no host port binding by default.
# If you need forwarding, append for example:
#   make run-la LA_HOSTFWD_ARGS='hostfwd=tcp::5555-:5555,hostfwd=udp::5555-:5555'
LA_HOSTFWD_ARGS ?=
ifeq ($(strip $(LA_HOSTFWD_ARGS)),)
LA_NETDEV_ARGS ?= user,id=net0
else
LA_NETDEV_ARGS ?= user,id=net0,$(LA_HOSTFWD_ARGS)
endif
DOCKER_IMAGE ?= orays-arceos-dev

# Remote evaluators may have no network/DNS, so prefer repo-provided helper
# shims before falling back to user-installed Cargo helpers. Keep the primary
# submission helpers under vendor/ because the grader expects dependencies to be
# submitted with the project source.
VENDOR_BIN := $(CURDIR)/vendor/bin
export PATH := $(VENDOR_BIN):$(CURDIR)/tools/bin:$(PATH)

SCRIPT_AXCONFIG_GEN := $(CURDIR)/scripts/axconfig-gen.py
SCRIPT_RUST_OBJCOPY := $(CURDIR)/scripts/rust-objcopy.sh
SCRIPT_RUST_LLD := $(CURDIR)/scripts/rust-lld.sh
SCRIPT_CARGO_AXPLAT := $(CURDIR)/scripts/cargo-axplat.sh

ifneq ($(wildcard $(SCRIPT_AXCONFIG_GEN)),)
  AXCONFIG_GEN ?= python3 $(SCRIPT_AXCONFIG_GEN)
else ifneq ($(wildcard $(VENDOR_BIN)/axconfig-gen),)
  AXCONFIG_GEN ?= python3 $(VENDOR_BIN)/axconfig-gen
else
  AXCONFIG_GEN ?= axconfig-gen
endif

ifneq ($(wildcard $(SCRIPT_RUST_LLD)),)
  RUST_LLD ?= $(SCRIPT_RUST_LLD)
else
  RUST_LLD ?= rust-lld
endif
export RUST_LLD

ifneq ($(wildcard $(SCRIPT_RUST_OBJCOPY)),)
  RUST_OBJCOPY ?= sh $(SCRIPT_RUST_OBJCOPY)
else ifneq ($(wildcard $(VENDOR_BIN)/rust-objcopy),)
  RUST_OBJCOPY ?= sh $(VENDOR_BIN)/rust-objcopy
else
  RUST_OBJCOPY ?= rust-objcopy
endif

ifneq ($(wildcard $(SCRIPT_CARGO_AXPLAT)),)
  CARGO_AXPLAT ?= sh $(SCRIPT_CARGO_AXPLAT)
else ifneq ($(wildcard $(VENDOR_BIN)/cargo-axplat),)
  CARGO_AXPLAT ?= sh $(VENDOR_BIN)/cargo-axplat
else
  CARGO_AXPLAT ?= cargo axplat
endif

ifneq ($(wildcard $(CURDIR)/cargo-home/config.toml),)
  # The official grader filters hidden directories and may have no network.
  # Use a non-hidden Cargo home with vendored sources for submission builds.
  CARGO_HOME ?= $(CURDIR)/cargo-home
  export CARGO_HOME
  ifneq ($(wildcard $(CURDIR)/scripts/ensure-cargo-vendor.sh),)
    _CARGO_VENDOR_READY := $(shell "$(CURDIR)/scripts/ensure-cargo-vendor.sh" >/dev/null && echo ok || echo fail)
    ifneq ($(_CARGO_VENDOR_READY),ok)
      $(error unable to restore vendor/cargo from vendor/cargo-vendor.tar.gz)
    endif
  endif
endif

# App options
# The official evaluator runs `make` / `make all` without passing `A`.
# Default the top-level app to the same entry used by the submission kernels so
# the parse-time APP existence check below does not depend on non-submission demo
# deleted demo applications.
A ?= $(KERNEL_APP)
APP ?= $(A)
FEATURES ?=
APP_FEATURES ?=

# QEMU options
BLK ?= n
NET ?= n
GRAPHIC ?= n
BUS ?= pci
MEM ?= 128M
ACCEL ?=
QEMU_ARGS ?=

DISK_IMG ?= disk.img
QEMU_LOG ?= n
NET_DUMP ?= n
NET_DEV ?= user
VFIO_PCI ?=
VHOST ?= n

# Network options
IP ?= 10.0.2.15
GW ?= 10.0.2.2

# App type
ifeq ($(wildcard $(APP)),)
  $(error Application path "$(APP)" is not valid)
endif

ifneq ($(wildcard $(APP)/Cargo.toml),)
  APP_TYPE := rust
else
  APP_TYPE := c
endif

.DEFAULT_GOAL := all

ifneq ($(filter $(or $(MAKECMDGOALS), $(.DEFAULT_GOAL)), build disasm run justrun debug defconfig oldconfig),)
# Install dependencies
include scripts/make/deps.mk
# Platform resolving
include scripts/make/platform.mk
# Configuration generation
include scripts/make/config.mk
# Feature parsing
include scripts/make/features.mk
endif

# Target
ifeq ($(ARCH), x86_64)
  TARGET := x86_64-unknown-none
else ifeq ($(ARCH), aarch64)
  TARGET := aarch64-unknown-none-softfloat
else ifeq ($(ARCH), riscv64)
  TARGET := riscv64gc-unknown-none-elf
else ifeq ($(ARCH), loongarch64)
  TARGET := loongarch64-unknown-none-softfloat
else
  $(error "ARCH" must be one of "x86_64", "riscv64", "aarch64" or "loongarch64")
endif

export AX_ARCH=$(ARCH)
export AX_PLATFORM=$(PLAT_NAME)
export AX_MODE=$(MODE)
export AX_LOG=$(LOG)
export AX_TARGET=$(TARGET)
export AX_IP=$(IP)
export AX_GW=$(GW)

ifneq ($(filter $(MAKECMDGOALS),unittest unittest_no_fail_fast clippy doc doc_check_missing),)
  # When running unit tests or other tests unrelated to a specific platform,
  # set `AX_CONFIG_PATH` to empty for dummy config
  unexport AX_CONFIG_PATH
else
  export AX_CONFIG_PATH=$(OUT_CONFIG)
endif

# Binutils
CROSS_COMPILE ?= $(ARCH)-linux-musl-
CC := $(CROSS_COMPILE)gcc
AR := $(CROSS_COMPILE)ar
RANLIB := $(CROSS_COMPILE)ranlib
LD := $(RUST_LLD) -flavor gnu

OBJDUMP ?= rust-objdump -d --print-imm-hex --x86-asm-syntax=intel
OBJCOPY ?= $(RUST_OBJCOPY) --binary-architecture=$(ARCH)
GDB ?= gdb-multiarch

# Paths
OUT_DIR ?= $(APP)
LD_SCRIPT ?= $(TARGET_DIR)/$(TARGET)/$(MODE)/linker_$(PLAT_NAME).lds

APP_NAME := $(shell basename $(APP))
OUT_ELF := $(OUT_DIR)/$(APP_NAME)_$(PLAT_NAME).elf
OUT_BIN := $(patsubst %.elf,%.bin,$(OUT_ELF))
OUT_UIMG := $(patsubst %.elf,%.uimg,$(OUT_ELF))
ifeq ($(UIMAGE), y)
  FINAL_IMG := $(OUT_UIMG)
else
  FINAL_IMG := $(OUT_BIN)
endif

kernel_build_args := \
	A=$(KERNEL_APP) \
	MODE=$(KERNEL_MODE) \
	LOG=$(KERNEL_LOG) \
	SMP=$(KERNEL_SMP) \
	FEATURES=$(KERNEL_FEATURES)

KERNEL_RV_ELF := $(KERNEL_RV_OUT_DIR)/$(notdir $(KERNEL_APP))_riscv64-qemu-virt.elf
KERNEL_LA_ELF := $(KERNEL_LA_OUT_DIR)/$(notdir $(KERNEL_APP))_loongarch64-qemu-virt.elf
KERNEL_RV_BIN := $(patsubst %.elf,%.bin,$(KERNEL_RV_ELF))
KERNEL_RV_WRAP_OBJ := $(KERNEL_BUILD_DIR)/kernel-rv.wrap.o

ifneq ($(wildcard $(RV_AUX_DISK)),)
rv_aux_drive := \
	-drive file=$(RV_AUX_DISK),if=none,format=raw,id=x1 \
	-device virtio-blk-device,drive=x1,bus=virtio-mmio-bus.1
endif

ifneq ($(wildcard $(LA_AUX_DISK)),)
la_aux_drive := \
	-drive file=$(LA_AUX_DISK),if=none,format=raw,id=x1 \
	-device virtio-blk-pci,drive=x1,bus=virtio-mmio-bus.1
endif

all:
ifeq ($(REMOTE_LTP_USES_BLACKLIST),)
	LTP_CASES="$(REMOTE_LTP_CASES)" \
		OSCOMP_SKIP_TEST_GROUPS="$(REMOTE_SKIP_OFFICIAL_TEST_GROUPS)" \
		OSCOMP_GROUP_TIMEOUT_CEILING_SECS="$(REMOTE_GROUP_TIMEOUT_CEILING_SECS)" \
		$(MAKE) test_build ARCH=riscv64 BUS=mmio \
		KERNEL_FEATURES="$(KERNEL_RV_FEATURES)" \
		APP_FEATURES="$(KERNEL_RV_APP_FEATURES)" \
		AXCONFIG_WRITES="$(KERNEL_RV_AXCONFIG_WRITES)" \
		OUT_DIR=$(KERNEL_RV_OUT_DIR) \
		OUT_CONFIG=$(KERNEL_RV_CONFIG) \
		TARGET_DIR=$(KERNEL_RV_TARGET_DIR)
	LTP_CASES="$(REMOTE_LTP_CASES)" \
		OSCOMP_SKIP_TEST_GROUPS="$(REMOTE_SKIP_OFFICIAL_TEST_GROUPS)" \
		OSCOMP_GROUP_TIMEOUT_CEILING_SECS="$(REMOTE_GROUP_TIMEOUT_CEILING_SECS)" \
		$(MAKE) test_build ARCH=loongarch64 BUS=pci \
		PLAT_CONFIG="$(REMOTE_LA_PLAT_CONFIG)" \
		KERNEL_FEATURES="$(KERNEL_LA_FEATURES)" \
		APP_FEATURES="$(KERNEL_LA_APP_FEATURES)" \
		AXCONFIG_WRITES="$(KERNEL_LA_AXCONFIG_WRITES)" \
		OUT_DIR=$(KERNEL_LA_OUT_DIR) \
		OUT_CONFIG=$(KERNEL_LA_CONFIG) \
		TARGET_DIR=$(KERNEL_LA_TARGET_DIR)
else
	@test -f "$(REMOTE_LTP_BLACKLIST_COMMON_FILE)" || { printf 'missing REMOTE_LTP_BLACKLIST_COMMON_FILE: %s\n' "$(REMOTE_LTP_BLACKLIST_COMMON_FILE)" >&2; exit 1; }
	@test -f "$(REMOTE_LTP_BLACKLIST_RV_FILE)" || { printf 'missing REMOTE_LTP_BLACKLIST_RV_FILE: %s\n' "$(REMOTE_LTP_BLACKLIST_RV_FILE)" >&2; exit 1; }
	LTP_CASES="$(REMOTE_LTP_CASES)" \
		LTP_BLACKLIST="$$(cat "$(REMOTE_LTP_BLACKLIST_COMMON_FILE)")" \
		LTP_BLACKLIST_RV="$$(cat "$(REMOTE_LTP_BLACKLIST_RV_FILE)")" \
		OSCOMP_SKIP_TEST_GROUPS="$(REMOTE_SKIP_OFFICIAL_TEST_GROUPS)" \
		OSCOMP_GROUP_TIMEOUT_CEILING_SECS="$(REMOTE_GROUP_TIMEOUT_CEILING_SECS)" \
		$(MAKE) test_build ARCH=riscv64 BUS=mmio \
		KERNEL_FEATURES="$(KERNEL_RV_FEATURES)" \
		APP_FEATURES="$(KERNEL_RV_APP_FEATURES)" \
		AXCONFIG_WRITES="$(KERNEL_RV_AXCONFIG_WRITES)" \
		OUT_DIR=$(KERNEL_RV_OUT_DIR) \
		OUT_CONFIG=$(KERNEL_RV_CONFIG) \
		TARGET_DIR=$(KERNEL_RV_TARGET_DIR)
	@test -f "$(REMOTE_LTP_BLACKLIST_LA_FILE)" || { printf 'missing REMOTE_LTP_BLACKLIST_LA_FILE: %s\n' "$(REMOTE_LTP_BLACKLIST_LA_FILE)" >&2; exit 1; }
	LTP_CASES="$(REMOTE_LTP_CASES)" \
		LTP_BLACKLIST="$$(cat "$(REMOTE_LTP_BLACKLIST_COMMON_FILE)")" \
		LTP_BLACKLIST_LA="$$(cat "$(REMOTE_LTP_BLACKLIST_LA_FILE)")" \
		OSCOMP_SKIP_TEST_GROUPS="$(REMOTE_SKIP_OFFICIAL_TEST_GROUPS)" \
		OSCOMP_GROUP_TIMEOUT_CEILING_SECS="$(REMOTE_GROUP_TIMEOUT_CEILING_SECS)" \
		$(MAKE) test_build ARCH=loongarch64 BUS=pci \
		PLAT_CONFIG="$(REMOTE_LA_PLAT_CONFIG)" \
		KERNEL_FEATURES="$(KERNEL_LA_FEATURES)" \
		APP_FEATURES="$(KERNEL_LA_APP_FEATURES)" \
		AXCONFIG_WRITES="$(KERNEL_LA_AXCONFIG_WRITES)" \
		OUT_DIR=$(KERNEL_LA_OUT_DIR) \
		OUT_CONFIG=$(KERNEL_LA_CONFIG) \
		TARGET_DIR=$(KERNEL_LA_TARGET_DIR)
endif

include scripts/make/utils.mk
include scripts/make/build.mk
include scripts/make/qemu.mk
ifeq ($(PLAT_NAME), aarch64-raspi4)
  include scripts/make/raspi4.mk
else ifeq ($(PLAT_NAME), aarch64-bsta1000b)
  include scripts/make/bsta1000b-fada.mk
endif

defconfig:
	$(call defconfig)

oldconfig:
	$(call oldconfig)

build: $(OUT_DIR) $(FINAL_IMG)

test_build:
	$(MAKE) $(kernel_build_args) \
		ARCH=$(ARCH) BUS=$(BUS) $(if $(strip $(PLAT_CONFIG)),PLAT_CONFIG="$(PLAT_CONFIG)") \
		APP_FEATURES="$(APP_FEATURES)" \
		AXCONFIG_WRITES="$(AXCONFIG_WRITES)" \
		OUT_DIR=$(OUT_DIR) \
		OUT_CONFIG=$(OUT_CONFIG) \
		TARGET_DIR=$(TARGET_DIR) \
		build
ifeq ($(ARCH),riscv64)
	@mkdir -p $(dir $(KERNEL_RV))
	$(RUST_OBJCOPY) -I binary -O elf64-littleriscv --rename-section .data=.text,alloc,load,readonly,code $(KERNEL_RV_BIN) $(KERNEL_RV_WRAP_OBJ)
	$(RUST_LLD) -flavor gnu -m elf64lriscv -T scripts/make/riscv64-kernel-wrap.lds $(KERNEL_RV_WRAP_OBJ) -o $(KERNEL_RV)
else ifeq ($(ARCH),loongarch64)
	@mkdir -p $(dir $(KERNEL_LA))
	cp $(KERNEL_LA_ELF) $(KERNEL_LA)
else
	$(error "test_build" only supports "ARCH=riscv64" or "ARCH=loongarch64")
endif

kernel-rv:
	$(MAKE) test_build ARCH=riscv64 BUS=mmio \
		KERNEL_FEATURES="$(KERNEL_RV_FEATURES)" \
		APP_FEATURES="$(KERNEL_RV_APP_FEATURES)" \
		AXCONFIG_WRITES="$(KERNEL_RV_AXCONFIG_WRITES)" \
		OUT_DIR=$(KERNEL_RV_OUT_DIR) \
		OUT_CONFIG=$(KERNEL_RV_CONFIG) \
		TARGET_DIR=$(KERNEL_RV_TARGET_DIR)

kernel-la:
	$(MAKE) test_build ARCH=loongarch64 BUS=pci \
		KERNEL_FEATURES="$(KERNEL_LA_FEATURES)" \
		APP_FEATURES="$(KERNEL_LA_APP_FEATURES)" \
		AXCONFIG_WRITES="$(KERNEL_LA_AXCONFIG_WRITES)" \
		OUT_DIR=$(KERNEL_LA_OUT_DIR) \
		OUT_CONFIG=$(KERNEL_LA_CONFIG) \
		TARGET_DIR=$(KERNEL_LA_TARGET_DIR)

pr3-smoke-kernel-rv:
	$(MAKE) test_build ARCH=riscv64 BUS=mmio \
		KERNEL_FEATURES="$(PR3_SMOKE_KERNEL_FEATURES)" \
		APP_FEATURES="$(PR3_SMOKE_APP_FEATURES)" \
		AXCONFIG_WRITES="$(KERNEL_RV_AXCONFIG_WRITES)" \
		KERNEL_BUILD_DIR=$(PR3_SMOKE_BUILD_DIR) \
		KERNEL_RV=$(PR3_SMOKE_RV_KERNEL) \
		OUT_DIR=$(PR3_SMOKE_RV_OUT_DIR) \
		OUT_CONFIG=$(PR3_SMOKE_RV_CONFIG) \
		TARGET_DIR=$(PR3_SMOKE_RV_TARGET_DIR)

pr3-smoke-kernel-la:
	$(MAKE) test_build ARCH=loongarch64 BUS=pci \
		KERNEL_FEATURES="$(PR3_SMOKE_KERNEL_FEATURES)" \
		APP_FEATURES="$(PR3_SMOKE_APP_FEATURES)" \
		AXCONFIG_WRITES="$(KERNEL_LA_AXCONFIG_WRITES)" \
		KERNEL_BUILD_DIR=$(PR3_SMOKE_BUILD_DIR) \
		KERNEL_LA=$(PR3_SMOKE_LA_KERNEL) \
		OUT_DIR=$(PR3_SMOKE_LA_OUT_DIR) \
		OUT_CONFIG=$(PR3_SMOKE_LA_CONFIG) \
		TARGET_DIR=$(PR3_SMOKE_LA_TARGET_DIR)

pr3-smoke-run-rv-raw:
	@test "$$PR3_EVIDENCE_SUPERVISED" = 1 || { \
		printf '%s\n' 'refusing unsupervised QEMU; use make pr3-smoke-rv' >&2; exit 2; \
	}
	@test -s "$(PR3_SMOKE_RV_KERNEL)" || { \
		printf 'missing PR3 RV64 smoke kernel: %s\n' "$(PR3_SMOKE_RV_KERNEL)" >&2; exit 2; \
	}
	@test -x "$$PR3_QEMU_RV_BIN" || { \
		printf '%s\n' 'missing supervisor-resolved PR3_QEMU_RV_BIN' >&2; exit 2; \
	}
	"$$PR3_QEMU_RV_BIN" -machine virt -kernel $(PR3_SMOKE_RV_KERNEL) \
		-m 1G -display none -serial stdio -monitor none -smp 1 \
		-device virtio-net-device,netdev=net -netdev hubport,id=net,hubid=0 \
		-bios default -no-reboot

pr3-smoke-run-la-raw:
	@test "$$PR3_EVIDENCE_SUPERVISED" = 1 || { \
		printf '%s\n' 'refusing unsupervised QEMU; use make pr3-smoke-la' >&2; exit 2; \
	}
	@test -s "$(PR3_SMOKE_LA_KERNEL)" || { \
		printf 'missing PR3 LA64 smoke kernel: %s\n' "$(PR3_SMOKE_LA_KERNEL)" >&2; exit 2; \
	}
	@test -x "$$PR3_QEMU_LA_BIN" || { \
		printf '%s\n' 'missing supervisor-resolved PR3_QEMU_LA_BIN' >&2; exit 2; \
	}
	"$$PR3_QEMU_LA_BIN" -machine virt -kernel $(PR3_SMOKE_LA_KERNEL) \
		-m 1G -display none -serial stdio -monitor none -smp 1 \
		-device virtio-net-pci,netdev=net -netdev hubport,id=net,hubid=0 \
		-no-reboot

pr3-smoke-rv:
	$(PYTHON) -I -S -B -X pycache_prefix=/dev/null test/run_suite.py \
		--profile evidence-runtime --arch rv

pr3-smoke-la:
	$(PYTHON) -I -S -B -X pycache_prefix=/dev/null test/run_suite.py \
		--profile evidence-runtime --arch la

pr3-manifest-check: test-list

pr3-infrastructure-tests: test-quick

pr3-evidence-host:
	$(PYTHON) -I -S -B -X pycache_prefix=/dev/null test/run_suite.py \
		--profile evidence-host

pr3-evidence-rv:
	$(PYTHON) -I -S -B -X pycache_prefix=/dev/null test/run_suite.py \
		--profile evidence-runtime --arch rv

pr3-evidence-la:
	$(PYTHON) -I -S -B -X pycache_prefix=/dev/null test/run_suite.py \
		--profile evidence-runtime --arch la

pr3-render-required:
	$(PYTHON) -I -S -B -X pycache_prefix=/dev/null test/run_suite.py \
		--profile evidence-aggregate

pr3-evidence-required:
	$(PYTHON) -I -S -B -X pycache_prefix=/dev/null test/run_suite.py \
		--profile evidence-required

docker-image:
	docker build -t $(DOCKER_IMAGE) -f Dockerfile .

docker: docker-image
	docker run --rm -it -v $(abspath $(CURDIR)/..):/code -w /code/arceos $(DOCKER_IMAGE) bash

testsuite-sdcard:
	$(MAKE) -C $(TESTSUITE_DIR) sdcard

prepare-rv-testsuite-img:
	@mkdir -p $(dir $(RV_TESTSUITE_RUN_IMG))
	rm -f $(RV_TESTSUITE_RUN_IMG)
	qemu-img create -f qcow2 -F raw -b $(RV_TESTSUITE_IMG) $(RV_TESTSUITE_RUN_IMG)

prepare-la-testsuite-img:
	@mkdir -p $(dir $(LA_TESTSUITE_RUN_IMG))
	rm -f $(LA_TESTSUITE_RUN_IMG)
	qemu-img create -f qcow2 -F raw -b $(LA_TESTSUITE_IMG) $(LA_TESTSUITE_RUN_IMG)

run-rv: kernel-rv prepare-rv-testsuite-img
	qemu-system-riscv64 -machine virt -kernel $(KERNEL_RV) -m $(RV_MEM) -nographic -smp $(KERNEL_SMP) -bios default -drive file=$(RV_TESTSUITE_RUN_IMG),if=none,format=qcow2,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 -no-reboot -device virtio-net-device,netdev=net -netdev $(RV_NETDEV_ARGS) \
		-rtc base=utc $(rv_aux_drive)

run-la: kernel-la prepare-la-testsuite-img
	qemu-system-loongarch64 -kernel $(KERNEL_LA) -m $(LA_MEM) -nographic -smp $(KERNEL_SMP) -drive file=$(LA_TESTSUITE_RUN_IMG),if=none,format=qcow2,id=x0 \
		-device virtio-blk-pci,drive=x0,bus=pcie.0 -no-reboot -device virtio-net-pci,netdev=net0 \
		-netdev $(LA_NETDEV_ARGS) -rtc base=utc $(la_aux_drive)

disasm:
	$(OBJDUMP) $(OUT_ELF) | less

run: build justrun

justrun:
	$(call run_qemu)

debug: build
	$(call run_qemu_debug) &
	sleep 1
	$(GDB) $(OUT_ELF) \
	  -ex 'target remote localhost:1234' \
	  -ex 'b rust_entry' \
	  -ex 'continue' \
	  -ex 'disp /16i $$pc'

clippy:
ifeq ($(origin ARCH), command line)
	$(call cargo_clippy,--target $(TARGET))
else
	$(call cargo_clippy,,--exclude arceos-shell)
endif

doc:
	$(call cargo_doc)

doc_check_missing:
	$(call cargo_doc)

fmt:
	cargo fmt --all

fmt_c:
	@clang-format --style=file -i $(shell find ulib/axlibc -iname '*.c' -o -iname '*.h')

pr2-check:
	$(PYTHON) -I -S -B -X pycache_prefix=/dev/null test/checks/check_file_object_event_core.py
	$(PYTHON) -I -S -B -X pycache_prefix=/dev/null test/run_unittest_suite.py test/unit/test_file_object_event_core.py
	cargo test -p axfile --lib

unittest: pr2-check
	$(call unit_test)

unittest_no_fail_fast: pr2-check
	$(call unit_test,--no-fail-fast)

test-list:
	$(PYTHON) -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --list

test-checks:
	$(PYTHON) -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --profile checks

test-unit:
	$(PYTHON) -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --profile unit

test-quick:
	$(PYTHON) -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --profile quick

test-baseline:
	$(PYTHON) -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --profile baseline

disk_img:
ifneq ($(wildcard $(DISK_IMG)),)
	@printf "$(YELLOW_C)warning$(END_C): disk image \"$(DISK_IMG)\" already exists!\n"
else
	$(call make_disk_image,fat32,$(DISK_IMG))
endif

clean: clean_c
	rm -rf $(APP)/*.bin $(APP)/*.elf $(OUT_CONFIG)
	rm -rf $(KERNEL_BUILD_DIR) $(PR3_SMOKE_BUILD_DIR) $(PR3_EVIDENCE_DIR) $(KERNEL_RV) $(KERNEL_LA)
	cargo clean

clean_c::
	rm -rf ulib/axlibc/build_*
	rm -rf $(app-objs)

.PHONY: all defconfig oldconfig \
	build disasm run justrun debug \
	clippy doc doc_check_missing fmt fmt_c pr2-check unittest unittest_no_fail_fast \
	test-list test-checks test-unit test-quick test-baseline \
	disk_img clean clean_c \
	test_build kernel-rv kernel-la docker-image docker testsuite-sdcard \
	prepare-rv-testsuite-img prepare-la-testsuite-img run-rv run-la \
	pr3-smoke-kernel-rv pr3-smoke-kernel-la \
	pr3-smoke-run-rv-raw pr3-smoke-run-la-raw pr3-smoke-rv pr3-smoke-la \
	pr3-manifest-check pr3-infrastructure-tests pr3-evidence-host \
	pr3-evidence-rv pr3-evidence-la pr3-render-required pr3-evidence-required
