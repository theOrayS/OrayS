# riscv64 LTP/OS COMP evaluation output
- Command: `./run-eval`
- Exit status: `0`
- Start: `2026-05-19T12:25:55+08:00`
- End: `2026-05-19T12:32:42+08:00`
- Raw log: `eval-reports/ltp-eval-20260519-122555/rv.raw.log`
- ANSI-stripped log: `eval-reports/ltp-eval-20260519-122555/rv.clean.log`

## Groups observed

- basic-musl
- busybox-musl
- cyclictest-musl
- iozone-musl
- iperf-musl
- libcbench-musl
- libctest-musl
- lmbench-musl
- ltp-musl
- lua-musl
- netperf-musl
- unixbench-musl
- basic-glibc
- busybox-glibc
- cyclictest-glibc
- iozone-glibc
- iperf-glibc
- libcbench-glibc
- libctest-glibc
- lmbench-glibc
- ltp-glibc
- lua-glibc
- netperf-glibc
- unixbench-glibc

## Skips reported by evaluator

- `SKIP: iozone throughput mode currently hangs in the evaluator environment`
- `SKIP: libcbench currently triggers an unrecovered allocator exhaustion path`
- `SKIP: libctest still trips unresolved pthread cancellation paths`
- `SKIP: lmbench still triggers an unresolved user-space page-fault path`
- `SKIP: full LTP sweep is too large for the boot-time evaluator smoke run`
- `SKIP: unixbench currently blocks on unresolved executable/runtime compatibility`
- `SKIP: iozone throughput mode currently hangs in the evaluator environment`
- `SKIP: libcbench currently triggers an unrecovered allocator exhaustion path`
- `SKIP: libctest still trips unresolved pthread cancellation paths`
- `SKIP: lmbench still triggers an unresolved user-space page-fault path`
- `SKIP: full LTP sweep is too large for the boot-time evaluator smoke run`
- `SKIP: unixbench currently blocks on unresolved executable/runtime compatibility`

## Full console output (ANSI stripped)

```text
===== LTP riscv64 evaluation start: 2026-05-19T12:25:55+08:00 =====
cwd=/root/oskernel2026-orays
command=./run-eval
make test_build ARCH=riscv64 BUS=mmio \
	KERNEL_FEATURES="alloc,paging,irq,multitask,fs,net" \
	APP_FEATURES="auto-run-tests,uspace" \
	AXCONFIG_WRITES="-w plat.phys-memory-size=0x4000_0000" \
	OUT_DIR=/root/oskernel2026-orays/build/kernels/riscv64 \
	OUT_CONFIG=/root/oskernel2026-orays/build/kernels/riscv64.axconfig.toml \
	TARGET_DIR=/root/oskernel2026-orays/build/kernels/target/riscv64
make[1]: Entering directory '/root/oskernel2026-orays'
make A=examples/shell MODE=release LOG=info SMP=1 FEATURES=alloc,paging,irq,multitask,fs,net \
	ARCH=riscv64 BUS=mmio \
	APP_FEATURES="auto-run-tests,uspace" \
	AXCONFIG_WRITES="-w plat.phys-memory-size=0x4000_0000" \
	OUT_DIR=/root/oskernel2026-orays/build/kernels/riscv64 \
	OUT_CONFIG=/root/oskernel2026-orays/build/kernels/riscv64.axconfig.toml \
	TARGET_DIR=/root/oskernel2026-orays/build/kernels/target/riscv64 \
	build
make[2]: Entering directory '/root/oskernel2026-orays'
axconfig-gen configs/defconfig.toml /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/axplat-riscv64-qemu-virt-0.4.1/axconfig.toml  -w arch=riscv64 -w platform=riscv64-qemu-virt -o "/root/oskernel2026-orays/build/kernels/riscv64.axconfig.toml" -w plat.phys-memory-size=0x4000_0000 -w plat.max-cpu-num=1 -c "/root/oskernel2026-orays/build/kernels/riscv64.axconfig.toml"
    Building App: shell, Arch: riscv64, Platform: riscv64-qemu-virt, App type: rust
cargo -C examples/shell build -Z unstable-options --target riscv64gc-unknown-none-elf --target-dir /root/oskernel2026-orays/build/kernels/target/riscv64 --release  --features "axstd/defplat axstd/log-level-info axstd/bus-mmio axstd/alloc axstd/paging axstd/irq axstd/multitask axstd/fs axstd/net auto-run-tests uspace"
warning: unused import: `super::fragmentation::PacketAssemblerSet`
  --> vendor/smoltcp/src/iface/interface/mod.rs:35:5
   |
35 | use super::fragmentation::PacketAssemblerSet;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `IFACE_MAX_SIXLOWPAN_ADDRESS_CONTEXT_COUNT`
  --> vendor/smoltcp/src/iface/interface/mod.rs:41:43
   |
41 | use crate::config::{IFACE_MAX_ADDR_COUNT, IFACE_MAX_SIXLOWPAN_ADDRESS_CONTEXT_COUNT};
   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused variable: `frag`
  --> vendor/smoltcp/src/iface/interface/ipv4.rs:91:9
   |
91 |         frag: &'a mut FragmentsBuffer,
   |         ^^^^ help: if this is intentional, prefix it with an underscore: `_frag`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unreachable pattern
    --> vendor/smoltcp/src/iface/interface/mod.rs:1177:13
     |
1171 |             Medium::Ethernet => {
     |             ---------------- matches all the relevant values
...
1177 |             _ => (EthernetAddress([0; 6]), tx_token),
     |             ^ no value can reach this
     |
     = note: `#[warn(unreachable_patterns)]` on by default

warning: unreachable pattern
    --> vendor/smoltcp/src/iface/interface/mod.rs:1174:21
     |
1173 |                     (HardwareAddress::Ethernet(addr), tx_token) => (addr, tx_token),
     |                     ------------------------------------------- matches all the relevant values
1174 |                     (_, _) => unreachable!(),
     |                     ^^^^^^ no value can reach this

warning: unused variable: `repr`
    --> vendor/smoltcp/src/iface/interface/mod.rs:1211:26
     |
1211 |             IpRepr::Ipv4(repr) => {
     |                          ^^^^ help: if this is intentional, prefix it with an underscore: `_repr`

warning: variable does not need to be mutable
   --> vendor/smoltcp/src/socket/dns.rs:249:13
    |
249 |         let mut mdns = MulticastDns::Disabled;
    |             ----^^^^
    |             |
    |             help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` on by default

warning: methods `link_local_ipv6_address` and `mldv2_report_packet` are never used
   --> vendor/smoltcp/src/iface/interface/ipv6.rs:171:8
    |
21  | impl InterfaceInner {
    | ------------------- methods in this implementation
...
171 |     fn link_local_ipv6_address(&self) -> Option<Ipv6Address> {
    |        ^^^^^^^^^^^^^^^^^^^^^^^
...
536 |     pub(super) fn mldv2_report_packet<'any>(
    |                   ^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` on by default

warning: variant `HopByHopIcmpv6` is never constructed
   --> vendor/smoltcp/src/iface/packet.rs:215:5
    |
207 | pub(crate) enum IpPayload<'p> {
    |                 --------- variant in this enum
...
215 |     HopByHopIcmpv6(Ipv6HopByHopRepr<'p>, Icmpv6Repr<'p>),
    |     ^^^^^^^^^^^^^^
    |
    = note: `IpPayload` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

warning: method `is_solicited_node_multicast` is never used
   --> vendor/smoltcp/src/wire/ipv6.rs:131:8
    |
79  | pub(crate) trait AddressExt {
    |                  ---------- method in this trait
...
131 |     fn is_solicited_node_multicast(&self) -> bool;
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `emit_header` is never used
   --> vendor/smoltcp/src/wire/udp.rs:275:19
    |
232 | impl Repr {
    | --------- method in this implementation
...
275 |     pub(crate) fn emit_header<T>(&self, packet: &mut Packet<&mut T>, payload_len: usize)
    |                   ^^^^^^^^^^^

warning: `smoltcp` (lib) generated 11 warnings (run `cargo fix --lib -p smoltcp` to apply 3 suggestions)
   Compiling axnet v0.2.0 (/root/oskernel2026-orays/kernel/net/axnet)
warning: methods `recv_loopback` and `recv_loopback_from` are never used
   --> kernel/net/axnet/src/smoltcp_impl/udp.rs:319:8
    |
276 | impl UdpSocket {
    | -------------- methods in this implementation
...
319 |     fn recv_loopback(&self, buf: &mut [u8], remote: Option<IpEndpoint>) -> AxResult<usize> {
    |        ^^^^^^^^^^^^^
...
323 |     fn recv_loopback_from(
    |        ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` on by default

   Compiling axruntime v0.2.0 (/root/oskernel2026-orays/kernel/runtime/axruntime)
   Compiling axfeat v0.2.0 (/root/oskernel2026-orays/api/axfeat)
   Compiling arceos_api v0.2.0 (/root/oskernel2026-orays/api/arceos_api)
   Compiling axstd v0.2.0 (/root/oskernel2026-orays/ulib/axstd)
   Compiling arceos_posix_api v0.2.0 (/root/oskernel2026-orays/api/arceos_posix_api)
warning: `axnet` (lib) generated 1 warning
   Compiling arceos-shell v0.1.0 (/root/oskernel2026-orays/examples/shell)
warning: variable does not need to be mutable
  --> examples/shell/src/uspace/program_loader.rs:91:13
   |
91 |         let mut interp_image = std::fs::read(interp_path.as_str())
   |             ----^^^^^^^^^^^^
   |             |
   |             help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: `arceos-shell` (bin "arceos-shell") generated 1 warning (run `cargo fix --bin "arceos-shell"` to apply 1 suggestion)
    Finished `release` profile [optimized] target(s) in 19.96s
rust-objcopy --binary-architecture=riscv64 /root/oskernel2026-orays/build/kernels/riscv64/shell_riscv64-qemu-virt.elf --strip-all -O binary /root/oskernel2026-orays/build/kernels/riscv64/shell_riscv64-qemu-virt.bin
make[2]: Leaving directory '/root/oskernel2026-orays'
rust-objcopy -I binary -O elf64-littleriscv --rename-section .data=.text,alloc,load,readonly,code /root/oskernel2026-orays/build/kernels/riscv64/shell_riscv64-qemu-virt.bin /root/oskernel2026-orays/build/kernels/kernel-rv.wrap.o
rust-lld -flavor gnu -m elf64lriscv -T scripts/make/riscv64-kernel-wrap.lds /root/oskernel2026-orays/build/kernels/kernel-rv.wrap.o -o /root/oskernel2026-orays/kernel-rv
make[1]: Leaving directory '/root/oskernel2026-orays'
rm -f /tmp/arceos-sdcard-rv.run.qcow2
qemu-img create -f qcow2 -F raw -b /root/oskernel2026-orays/sdcard-rv.img /tmp/arceos-sdcard-rv.run.qcow2
Formatting '/tmp/arceos-sdcard-rv.run.qcow2', fmt=qcow2 cluster_size=65536 extended_l2=off compression_type=zlib size=4294967296 backing_file=/root/oskernel2026-orays/sdcard-rv.img backing_fmt=raw lazy_refcounts=off refcount_bits=16
qemu-system-riscv64 -machine virt -kernel /root/oskernel2026-orays/kernel-rv -m 1G -nographic -smp 1 -bios default -drive file=/tmp/arceos-sdcard-rv.run.qcow2,if=none,format=qcow2,id=x0 \
	-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 -no-reboot -device virtio-net-device,netdev=net -netdev user,id=net \
	-rtc base=utc 

OpenSBI v1.3
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|___/_____|
        | |
        |_|

Platform Name             : riscv-virtio,qemu
Platform Features         : medeleg
Platform HART Count       : 1
Platform IPI Device       : aclint-mswi
Platform Timer Device     : aclint-mtimer @ 10000000Hz
Platform Console Device   : semihosting
Platform HSM Device       : ---
Platform PMU Device       : ---
Platform Reboot Device    : sifive_test
Platform Shutdown Device  : sifive_test
Platform Suspend Device   : ---
Platform CPPC Device      : ---
Firmware Base             : 0x80000000
Firmware Size             : 194 KB
Firmware RW Offset        : 0x20000
Firmware RW Size          : 66 KB
Firmware Heap Offset      : 0x28000
Firmware Heap Size        : 34 KB (total), 2 KB (reserved), 9 KB (used), 22 KB (free)
Firmware Scratch Size     : 4096 B (total), 760 B (used), 3336 B (free)
Runtime SBI Version       : 1.0

Domain0 Name              : root
Domain0 Boot HART         : 0
Domain0 HARTs             : 0*
Domain0 Region00          : 0x0000000002000000-0x000000000200ffff M: (I,R,W) S/U: ()
Domain0 Region01          : 0x0000000080000000-0x000000008001ffff M: (R,X) S/U: ()
Domain0 Region02          : 0x0000000080020000-0x000000008003ffff M: (R,W) S/U: ()
Domain0 Region03          : 0x0000000000000000-0xffffffffffffffff M: (R,W,X) S/U: (R,W,X)
Domain0 Next Address      : 0x0000000080200000
Domain0 Next Arg1         : 0x00000000bf000000
Domain0 Next Mode         : S-mode
Domain0 SysReset          : yes
Domain0 SysSuspend        : yes

Boot HART ID              : 0
Boot HART Domain          : root
Boot HART Priv Version    : v1.10
Boot HART Base ISA        : rv64imafdc
Boot HART ISA Extensions  : time
Boot HART PMP Count       : 16
Boot HART PMP Granularity : 4
Boot HART PMP Address Bits: 54
Boot HART MHPM Count      : 0
Boot HART MIDELEG         : 0x0000000000000222
Boot HART MEDELEG         : 0x000000000000b109

       d8888                            .d88888b.   .d8888b.
      d88888                           d88P" "Y88b d88P  Y88b
     d88P888                           888     888 Y88b.
    d88P 888 888d888  .d8888b  .d88b.  888     888  "Y888b.
   d88P  888 888P"   d88P"    d8P  Y8b 888     888     "Y88b.
  d88P   888 888     888      88888888 888     888       "888
 d8888888888 888     Y88b.    Y8b.     Y88b. .d88P Y88b  d88P
d88P     888 888      "Y8888P  "Y8888   "Y88888P"   "Y8888P"

arch = riscv64
platform = riscv64-qemu-virt
target = riscv64gc-unknown-none-elf
build_mode = release
log_level = info

[  0.026406 0 axruntime:135] Logging is enabled.
[  0.027719 0 axruntime:136] Primary CPU 0 started, arg = 0xbf000000.
[  0.028685 0 axruntime:139] Found physcial memory regions:
[  0.029019 0 axruntime:141]   [PA:0x101000, PA:0x102000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.029509 0 axruntime:141]   [PA:0xc000000, PA:0xc210000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.029737 0 axruntime:141]   [PA:0x10000000, PA:0x10001000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.029955 0 axruntime:141]   [PA:0x10001000, PA:0x10009000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.030181 0 axruntime:141]   [PA:0x30000000, PA:0x40000000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.030373 0 axruntime:141]   [PA:0x40000000, PA:0x80000000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.030567 0 axruntime:141]   [PA:0x80200000, PA:0x802c1000) .text (READ | EXECUTE | RESERVED)
[  0.030754 0 axruntime:141]   [PA:0x802c1000, PA:0x802e9000) .rodata (READ | RESERVED)
[  0.030911 0 axruntime:141]   [PA:0x802e9000, PA:0x802ed000) .data .tdata .tbss .percpu (READ | WRITE | RESERVED)
[  0.031263 0 axruntime:141]   [PA:0x802ed000, PA:0x8032d000) boot stack (READ | WRITE | RESERVED)
[  0.031476 0 axruntime:141]   [PA:0x8032d000, PA:0x80356000) .bss (READ | WRITE | RESERVED)
[  0.031680 0 axruntime:141]   [PA:0x80356000, PA:0xc0000000) free memory (READ | WRITE | FREE)
[  0.031980 0 axruntime:216] Initialize global memory allocator...
[  0.032226 0 axruntime:217]   use TLSF allocator.
[  0.035678 0 axmm:103] Initialize virtual memory management...
[  0.096417 0 axruntime:156] Initialize platform devices...
smp = 1
[  0.097130 0 axtask::api:73] Initialize scheduling...
[  0.099227 0 axtask::api:83]   use FIFO scheduler.
[  0.099621 0 axdriver:152] Initialize device drivers...
[  0.100000 0 axdriver:153]   device model: static
[  0.100864 0 virtio_drivers::device::blk:63] found a block device of size 4194304KB
[  0.102177 0 axdriver::bus::mmio:11] registered a new Block device at [PA:0x10001000, PA:0x10002000): "virtio-blk"
[  0.103054 0 virtio_drivers::device::net::dev_raw:33] negotiated_features Features(MAC | STATUS | RING_INDIRECT_DESC | RING_EVENT_IDX)
[  0.105452 0 axdriver::bus::mmio:11] registered a new Net device at [PA:0x10008000, PA:0x10009000): "virtio-net"
[  0.105907 0 axfs:44] Initialize filesystems...
[  0.106175 0 axfs:47]   use block device 0: "virtio-blk"
[  0.107726 0 axfs::root:336]   detected root filesystem: Ext4
[  0.130910 0 axnet:42] Initialize network subsystem...
[  0.131332 0 axnet:45]   use NIC 0: "virtio-net"
[  0.136646 0 axnet::smoltcp_impl:335] created net interface "eth0":
[  0.136952 0 axnet::smoltcp_impl:336]   ether:    52-54-00-12-34-56
[  0.137447 0 axnet::smoltcp_impl:337]   ip:       10.0.2.15/24
[  0.137958 0 axnet::smoltcp_impl:338]   gateway:  10.0.2.2
[  0.138244 0 axruntime:182] Initialize interrupt handlers...
[  0.138918 0 axruntime:194] Primary CPU 0 init OK.
frame-allocator-diagnostic: process-teardown pid=5 reclaimed_frames=344 before_free=256152 before_allocated=5138 after_free=256496 after_allocated=4794
#### OS COMP TEST GROUP START basic-musl ####
frame-allocator-diagnostic: process-teardown pid=6 reclaimed_frames=344 before_free=256149 before_allocated=5141 after_free=256493 after_allocated=4797
Testing brk :
========== START test_brk ==========
Before alloc,heap pos: 77824
After alloc,heap pos: 77888
Alloc again,heap pos: 77952
========== END test_brk ==========
frame-allocator-diagnostic: process-teardown pid=8 reclaimed_frames=39 before_free=256097 before_allocated=5193 after_free=256136 after_allocated=5154
Testing chdir :
========== START test_chdir ==========
chdir ret: 0
  current working dir : 
========== END test_chdir ==========
frame-allocator-diagnostic: process-teardown pid=9 reclaimed_frames=39 before_free=256090 before_allocated=5200 after_free=256129 after_allocated=5161
Testing clone :
========== START test_clone ==========
  Child says successfully!
frame-allocator-diagnostic: process-teardown pid=11 reclaimed_frames=1 before_free=256073 before_allocated=5217 after_free=256074 after_allocated=5216
clone process successfully.
pid:11
========== END test_clone ==========
frame-allocator-diagnostic: process-teardown pid=10 reclaimed_frames=41 before_free=256074 before_allocated=5216 after_free=256115 after_allocated=5175
Testing close :
========== START test_close ==========
  close 3 success.
========== END test_close ==========
frame-allocator-diagnostic: process-teardown pid=12 reclaimed_frames=39 before_free=256069 before_allocated=5221 after_free=256108 after_allocated=5182
Testing dup2 :
========== START test_dup2 ==========
  from fd 100
========== END test_dup2 ==========
frame-allocator-diagnostic: process-teardown pid=13 reclaimed_frames=39 before_free=256062 before_allocated=5228 after_free=256101 after_allocated=5189
Testing dup :
========== START test_dup ==========
  new fd is 3.
========== END test_dup ==========
frame-allocator-diagnostic: process-teardown pid=14 reclaimed_frames=39 before_free=256055 before_allocated=5235 after_free=256094 after_allocated=5196
Testing execve :
========== START test_execve ==========
  I am test_echo.
execve success.
========== END main ==========
frame-allocator-diagnostic: process-teardown pid=15 reclaimed_frames=39 before_free=256048 before_allocated=5242 after_free=256087 after_allocated=5203
Testing exit :
========== START test_exit ==========
frame-allocator-diagnostic: process-teardown pid=17 reclaimed_frames=0 before_free=256034 before_allocated=5256 after_free=256034 after_allocated=5256
exit OK.
========== END test_exit ==========
frame-allocator-diagnostic: process-teardown pid=16 reclaimed_frames=39 before_free=256034 before_allocated=5256 after_free=256073 after_allocated=5217
Testing fork :
========== START test_fork ==========
  child process.
frame-allocator-diagnostic: process-teardown pid=19 reclaimed_frames=1 before_free=256019 before_allocated=5271 after_free=256020 after_allocated=5270
  parent process. wstatus:0
========== END test_fork ==========
frame-allocator-diagnostic: process-teardown pid=18 reclaimed_frames=39 before_free=256020 before_allocated=5270 after_free=256059 after_allocated=5231
Testing fstat :
========== START test_fstat ==========
fstat ret: 0
fstat: dev: 1, inode: 1012599416, mode: 33206, nlink: 1, size: 52, atime: 0, mtime: 0, ctime: 0
========== END test_fstat ==========
frame-allocator-diagnostic: process-teardown pid=20 reclaimed_frames=39 before_free=256013 before_allocated=5277 after_free=256052 after_allocated=5238
Testing getcwd :
========== START test_getcwd ==========
getcwd: /tmp/testsuite/musl/basic/basic successfully!
========== END test_getcwd ==========
frame-allocator-diagnostic: process-teardown pid=21 reclaimed_frames=39 before_free=256006 before_allocated=5284 after_free=256045 after_allocated=5245
Testing getdents :
========== START test_getdents ==========
open fd:3
getdents fd:-20
getdents success.


========== END test_getdents ==========
frame-allocator-diagnostic: process-teardown pid=22 reclaimed_frames=39 before_free=255999 before_allocated=5291 after_free=256038 after_allocated=5252
Testing getpid :
========== START test_getpid ==========
getpid success.
pid = 23
========== END test_getpid ==========
frame-allocator-diagnostic: process-teardown pid=23 reclaimed_frames=39 before_free=255992 before_allocated=5298 after_free=256031 after_allocated=5259
Testing getppid :
========== START test_getppid ==========
  getppid success. ppid : 7
========== END test_getppid ==========
frame-allocator-diagnostic: process-teardown pid=24 reclaimed_frames=39 before_free=255985 before_allocated=5305 after_free=256024 after_allocated=5266
Testing gettimeofday :
========== START test_gettimeofday ==========
gettimeofday success.
start:2120, end:2207
interval: 87
========== END test_gettimeofday ==========
frame-allocator-diagnostic: process-teardown pid=25 reclaimed_frames=39 before_free=255978 before_allocated=5312 after_free=256017 after_allocated=5273
Testing mkdir_ :
========== START test_mkdir ==========
mkdir ret: 0
  mkdir success.
========== END test_mkdir ==========
frame-allocator-diagnostic: process-teardown pid=26 reclaimed_frames=39 before_free=255971 before_allocated=5319 after_free=256010 after_allocated=5280
Testing mmap :
========== START test_mmap ==========
file len: 27
mmap content:   Hello, mmap successfully!
========== END test_mmap ==========
frame-allocator-diagnostic: process-teardown pid=27 reclaimed_frames=39 before_free=255964 before_allocated=5326 after_free=256003 after_allocated=5287
Testing mount :
========== START test_mount ==========
Mounting dev:/dev/vda2 to ./mnt
mount return: 0
mount successfully
umount return: 0
========== END test_mount ==========
frame-allocator-diagnostic: process-teardown pid=28 reclaimed_frames=39 before_free=255957 before_allocated=5333 after_free=255996 after_allocated=5294
Testing munmap :
========== START test_munmap ==========
file len: 27
munmap return: 0
munmap successfully!
========== END test_munmap ==========
frame-allocator-diagnostic: process-teardown pid=29 reclaimed_frames=39 before_free=255950 before_allocated=5340 after_free=255989 after_allocated=5301
Testing openat :
========== START test_openat ==========
open dir fd: 3
openat fd: 4
openat success.
========== END test_openat ==========
frame-allocator-diagnostic: process-teardown pid=30 reclaimed_frames=39 before_free=255943 before_allocated=5347 after_free=255982 after_allocated=5308
Testing open :
========== START test_open ==========
Hi, this is a text file.
syscalls testing success!

========== END test_open ==========
frame-allocator-diagnostic: process-teardown pid=31 reclaimed_frames=39 before_free=255936 before_allocated=5354 after_free=255975 after_allocated=5315
Testing pipe :
========== START test_pipe ==========
cpid: 33
cpid: 0
frame-allocator-diagnostic: process-teardown pid=33 reclaimed_frames=1 before_free=255921 before_allocated=5369 after_free=255922 after_allocated=5368
  Write to pipe successfully.

========== END test_pipe ==========
frame-allocator-diagnostic: process-teardown pid=32 reclaimed_frames=39 before_free=255922 before_allocated=5368 after_free=255961 after_allocated=5329
Testing read :
========== START test_read ==========
Hi, this is a text file.
syscalls testing success!

========== END test_read ==========
frame-allocator-diagnostic: process-teardown pid=34 reclaimed_frames=39 before_free=255915 before_allocated=5375 after_free=255954 after_allocated=5336
Testing sleep :
========== START test_sleep ==========
sleep success.
========== END test_sleep ==========
frame-allocator-diagnostic: process-teardown pid=35 reclaimed_frames=39 before_free=255908 before_allocated=5382 after_free=255947 after_allocated=5343
Testing times :
========== START test_times ==========
mytimes success
{tms_utime:0, tms_stime:0, tms_cutime:0, tms_cstime:0}
========== END test_times ==========
frame-allocator-diagnostic: process-teardown pid=36 reclaimed_frames=39 before_free=255901 before_allocated=5389 after_free=255940 after_allocated=5350
Testing umount :
========== START test_umount ==========
Mounting dev:/dev/vda2 to ./mnt
mount return: 0
umount success.
return: 0
========== END test_umount ==========
frame-allocator-diagnostic: process-teardown pid=37 reclaimed_frames=39 before_free=255894 before_allocated=5396 after_free=255933 after_allocated=5357
Testing uname :
========== START test_uname ==========
Uname: Linux arceos 6.0.0 ArceOS riscv64 localdomain
========== END test_uname ==========
frame-allocator-diagnostic: process-teardown pid=38 reclaimed_frames=39 before_free=255887 before_allocated=5403 after_free=255926 after_allocated=5364
Testing unlink :
========== START test_unlink ==========
  unlink success!
========== END test_unlink ==========
frame-allocator-diagnostic: process-teardown pid=39 reclaimed_frames=39 before_free=255880 before_allocated=5410 after_free=255919 after_allocated=5371
Testing wait :
========== START test_wait ==========
This is child process
frame-allocator-diagnostic: process-teardown pid=41 reclaimed_frames=1 before_free=255865 before_allocated=5425 after_free=255866 after_allocated=5424
wait child success.
wstatus: 0
========== END test_wait ==========
frame-allocator-diagnostic: process-teardown pid=40 reclaimed_frames=39 before_free=255866 before_allocated=5424 after_free=255905 after_allocated=5385
Testing waitpid :
========== START test_waitpid ==========
This is child process
frame-allocator-diagnostic: process-teardown pid=43 reclaimed_frames=2 before_free=255850 before_allocated=5440 after_free=255852 after_allocated=5438
waitpid successfully.
wstatus: 3
========== END test_waitpid ==========
frame-allocator-diagnostic: process-teardown pid=42 reclaimed_frames=39 before_free=255852 before_allocated=5438 after_free=255891 after_allocated=5399
Testing write :
========== START test_write ==========
Hello operating system contest.
========== END test_write ==========
frame-allocator-diagnostic: process-teardown pid=44 reclaimed_frames=39 before_free=255845 before_allocated=5445 after_free=255884 after_allocated=5406
Testing yield :
========== START test_yield ==========
  I am child process: 46. iteration -2144419840.
  I am child process: 47. iteration -2144419840.
  I am child process: 48. iteration -2144419840.
  I am child process: 46. iteration -2144419840.
  I am child process: 47. iteration -2144419840.
  I am child process: 48. iteration -2144419840.
  I am child process: 46. iteration -2144419840.
  I am child process: 47. iteration -2144419840.
  I am child process: 48. iteration -2144419840.
  I am child process: 46. iteration -2144419840.
  I am child process: 47. iteration -2144419840.
  I am child process: 48. iteration -2144419840.
  I am child process: 46. iteration -2144419840.
frame-allocator-diagnostic: process-teardown pid=46 reclaimed_frames=1 before_free=255814 before_allocated=5476 after_free=255815 after_allocated=5475
  I am child process: 47. iteration -2144419840.
frame-allocator-diagnostic: process-teardown pid=47 reclaimed_frames=1 before_free=255815 before_allocated=5475 after_free=255816 after_allocated=5474
  I am child process: 48. iteration -2144419840.
frame-allocator-diagnostic: process-teardown pid=48 reclaimed_frames=1 before_free=255816 before_allocated=5474 after_free=255817 after_allocated=5473
========== END test_yield ==========
frame-allocator-diagnostic: process-teardown pid=45 reclaimed_frames=39 before_free=255817 before_allocated=5473 after_free=255856 after_allocated=5434
frame-allocator-diagnostic: process-teardown pid=7 reclaimed_frames=345 before_free=255856 before_allocated=5434 after_free=256201 after_allocated=5089
#### OS COMP TEST GROUP END basic-musl ####
frame-allocator-diagnostic: process-teardown pid=49 reclaimed_frames=344 before_free=255852 before_allocated=5438 after_free=256196 after_allocated=5094
frame-allocator-diagnostic: process-teardown pid=4 reclaimed_frames=345 before_free=256196 before_allocated=5094 after_free=256541 after_allocated=4749
#### OS COMP TEST GROUP START busybox-musl ####
frame-allocator-diagnostic: process-teardown pid=51 reclaimed_frames=344 before_free=255839 before_allocated=5451 after_free=256183 after_allocated=5107
#### independent command test
frame-allocator-diagnostic: process-teardown pid=50 reclaimed_frames=344 before_free=256187 before_allocated=5103 after_free=256531 after_allocated=4759
testcase busybox echo "#### independent command test" success
frame-allocator-diagnostic: process-teardown pid=53 reclaimed_frames=344 before_free=255829 before_allocated=5461 after_free=256173 after_allocated=5117
frame-allocator-diagnostic: process-teardown pid=52 reclaimed_frames=345 before_free=256176 before_allocated=5114 after_free=256521 after_allocated=4769
testcase busybox ash -c exit success
frame-allocator-diagnostic: process-teardown pid=55 reclaimed_frames=344 before_free=255819 before_allocated=5471 after_free=256163 after_allocated=5127
frame-allocator-diagnostic: process-teardown pid=54 reclaimed_frames=345 before_free=256166 before_allocated=5124 after_free=256511 after_allocated=4779
testcase busybox sh -c exit success
frame-allocator-diagnostic: process-teardown pid=57 reclaimed_frames=344 before_free=255809 before_allocated=5481 after_free=256153 after_allocated=5137
bbb
frame-allocator-diagnostic: process-teardown pid=56 reclaimed_frames=343 before_free=256158 before_allocated=5132 after_free=256501 after_allocated=4789
testcase busybox basename /aaa/bbb success
frame-allocator-diagnostic: process-teardown pid=59 reclaimed_frames=344 before_free=255800 before_allocated=5490 after_free=256144 after_allocated=5146
    January 1970
Su Mo Tu We Th Fr Sa
             1  2  3
 4  5  6  7  8  9 10
11 12 13 14 15 16 17
18 19 20 21 22 23 24
25 26 27 28 29 30 31
                     
frame-allocator-diagnostic: process-teardown pid=58 reclaimed_frames=345 before_free=256146 before_allocated=5144 after_free=256491 after_allocated=4799
testcase busybox cal success
frame-allocator-diagnostic: process-teardown pid=61 reclaimed_frames=344 before_free=255790 before_allocated=5500 after_free=256134 after_allocated=5156
frame-allocator-diagnostic: process-teardown pid=60 reclaimed_frames=343 before_free=256138 before_allocated=5152 after_free=256481 after_allocated=4809
testcase busybox clear success
frame-allocator-diagnostic: process-teardown pid=63 reclaimed_frames=344 before_free=255780 before_allocated=5510 after_free=256124 after_allocated=5166
Thu Jan  1 00:00:08 UTC 1970
frame-allocator-diagnostic: process-teardown pid=62 reclaimed_frames=343 before_free=256128 before_allocated=5162 after_free=256471 after_allocated=4819
testcase busybox date success
frame-allocator-diagnostic: process-teardown pid=65 reclaimed_frames=344 before_free=255770 before_allocated=5520 after_free=256114 after_allocated=5176
Filesystem           1K-blocks      Used Available Use% Mounted on
devfs                  1045160     20692   1024468   2% /dev
tmpfs                  1045160     20692   1024468   2% /tmp
tmpfs                  1045160     20692   1024468   2% /var
proc                   1045160     20692   1024468   2% /proc
sysfs                  1045160     20692   1024468   2% /sys
frame-allocator-diagnostic: process-teardown pid=64 reclaimed_frames=344 before_free=256117 before_allocated=5173 after_free=256461 after_allocated=4829
testcase busybox df success
frame-allocator-diagnostic: process-teardown pid=67 reclaimed_frames=344 before_free=255759 before_allocated=5531 after_free=256103 after_allocated=5187
/aaa
frame-allocator-diagnostic: process-teardown pid=66 reclaimed_frames=343 before_free=256108 before_allocated=5182 after_free=256451 after_allocated=4839
testcase busybox dirname /aaa/bbb success
frame-allocator-diagnostic: process-teardown pid=69 reclaimed_frames=344 before_free=255750 before_allocated=5540 after_free=256094 after_allocated=5196
frame-allocator-diagnostic: process-teardown pid=68 reclaimed_frames=345 before_free=256096 before_allocated=5194 after_free=256441 after_allocated=4849
testcase busybox dmesg success
frame-allocator-diagnostic: process-teardown pid=71 reclaimed_frames=344 before_free=255740 before_allocated=5550 after_free=256084 after_allocated=5206
0	.
frame-allocator-diagnostic: process-teardown pid=70 reclaimed_frames=344 before_free=256087 before_allocated=5203 after_free=256431 after_allocated=4859
testcase busybox du success
frame-allocator-diagnostic: process-teardown pid=73 reclaimed_frames=344 before_free=255729 before_allocated=5561 after_free=256073 after_allocated=5217
2
frame-allocator-diagnostic: process-teardown pid=72 reclaimed_frames=344 before_free=256077 before_allocated=5213 after_free=256421 after_allocated=4869
testcase busybox expr 1 + 1 success
frame-allocator-diagnostic: process-teardown pid=75 reclaimed_frames=344 before_free=255720 before_allocated=5570 after_free=256064 after_allocated=5226
frame-allocator-diagnostic: process-teardown pid=74 reclaimed_frames=343 before_free=256068 before_allocated=5222 after_free=256411 after_allocated=4879
testcase busybox false success
frame-allocator-diagnostic: process-teardown pid=77 reclaimed_frames=344 before_free=255710 before_allocated=5580 after_free=256054 after_allocated=5236
frame-allocator-diagnostic: process-teardown pid=76 reclaimed_frames=343 before_free=256058 before_allocated=5232 after_free=256401 after_allocated=4889
testcase busybox true success
frame-allocator-diagnostic: process-teardown pid=79 reclaimed_frames=344 before_free=255699 before_allocated=5591 after_free=256043 after_allocated=5247
/musl/ls
frame-allocator-diagnostic: process-teardown pid=78 reclaimed_frames=344 before_free=256047 before_allocated=5243 after_free=256391 after_allocated=4899
testcase busybox which ls success
frame-allocator-diagnostic: process-teardown pid=81 reclaimed_frames=344 before_free=255690 before_allocated=5600 after_free=256034 after_allocated=5256
Linux
frame-allocator-diagnostic: process-teardown pid=80 reclaimed_frames=343 before_free=256038 before_allocated=5252 after_free=256381 after_allocated=4909
testcase busybox uname success
frame-allocator-diagnostic: process-teardown pid=83 reclaimed_frames=344 before_free=255680 before_allocated=5610 after_free=256024 after_allocated=5266
 00:00:14 up 0 min,  0 users,  load average: 0.00, 0.00, 0.00
frame-allocator-diagnostic: process-teardown pid=82 reclaimed_frames=343 before_free=256028 before_allocated=5262 after_free=256371 after_allocated=4919
testcase busybox uptime success
frame-allocator-diagnostic: process-teardown pid=85 reclaimed_frames=344 before_free=255669 before_allocated=5621 after_free=256013 after_allocated=5277
abc
frame-allocator-diagnostic: process-teardown pid=84 reclaimed_frames=343 before_free=256018 before_allocated=5272 after_free=256361 after_allocated=4929
testcase busybox printf "abc\n" success
frame-allocator-diagnostic: process-teardown pid=87 reclaimed_frames=344 before_free=255660 before_allocated=5630 after_free=256004 after_allocated=5286
PID   USER     TIME  COMMAND
frame-allocator-diagnostic: process-teardown pid=86 reclaimed_frames=345 before_free=256006 before_allocated=5284 after_free=256351 after_allocated=4939
testcase busybox ps success
frame-allocator-diagnostic: process-teardown pid=89 reclaimed_frames=344 before_free=255650 before_allocated=5640 after_free=255994 after_allocated=5296
/tmp/testsuite/musl/busybox
frame-allocator-diagnostic: process-teardown pid=88 reclaimed_frames=344 before_free=255997 before_allocated=5293 after_free=256341 after_allocated=4949
testcase busybox pwd success
frame-allocator-diagnostic: process-teardown pid=91 reclaimed_frames=344 before_free=255640 before_allocated=5650 after_free=255984 after_allocated=5306
              total        used        free      shared  buff/cache   available
Mem:              0           0           0           0           0     1039745
-/+ buffers/cache:            0           0
Swap:             0           0           0
frame-allocator-diagnostic: process-teardown pid=90 reclaimed_frames=344 before_free=255987 before_allocated=5303 after_free=256331 after_allocated=4959
testcase busybox free success
frame-allocator-diagnostic: process-teardown pid=93 reclaimed_frames=344 before_free=255630 before_allocated=5660 after_free=255974 after_allocated=5316
Thu Jan  1 00:00:17 1970  0.000000 seconds
frame-allocator-diagnostic: process-teardown pid=92 reclaimed_frames=343 before_free=255978 before_allocated=5312 after_free=256321 after_allocated=4969
testcase busybox hwclock success
frame-allocator-diagnostic: process-teardown pid=95 reclaimed_frames=344 before_free=255619 before_allocated=5671 after_free=255963 after_allocated=5327
frame-allocator-diagnostic: process-teardown pid=94 reclaimed_frames=343 before_free=255615 before_allocated=5675 after_free=255958 after_allocated=5332
testcase busybox sh -c 'sleep 5' & /musl/busybox kill $! success
frame-allocator-diagnostic: process-teardown pid=98 reclaimed_frames=344 before_free=255262 before_allocated=6028 after_free=255606 after_allocated=5684
awk                  head                 sed
basename             kill                 seq
busybox_cmd.txt      line                 sh
busybox_testcode.sh  ln                   sleep
cat                  ls                   sort
chmod                mkdir                tail
cp                   mktemp               tee
cut                  mv                   touch
date                 printf               tr
dirname              ps                   true
echo                 pwd                  uname
expr                 readlink             wc
find                 rm                   xargs
grep                 rmdir
frame-allocator-diagnostic: process-teardown pid=97 reclaimed_frames=347 before_free=255606 before_allocated=5684 after_free=255953 after_allocated=5337
testcase busybox ls success
frame-allocator-diagnostic: process-teardown pid=100 reclaimed_frames=344 before_free=255251 before_allocated=6039 after_free=255595 after_allocated=5695
frame-allocator-diagnostic: process-teardown pid=99 reclaimed_frames=343 before_free=255600 before_allocated=5690 after_free=255943 after_allocated=5347
testcase busybox sleep 1 success
frame-allocator-diagnostic: process-teardown pid=102 reclaimed_frames=344 before_free=255241 before_allocated=6049 after_free=255585 after_allocated=5705
#### file opration test
frame-allocator-diagnostic: process-teardown pid=101 reclaimed_frames=344 before_free=255589 before_allocated=5701 after_free=255933 after_allocated=5357
testcase busybox echo "#### file opration test" success
frame-allocator-diagnostic: process-teardown pid=104 reclaimed_frames=344 before_free=255231 before_allocated=6059 after_free=255575 after_allocated=5715
frame-allocator-diagnostic: process-teardown pid=103 reclaimed_frames=343 before_free=255580 before_allocated=5710 after_free=255923 after_allocated=5367
testcase busybox touch test.txt success
frame-allocator-diagnostic: process-teardown pid=106 reclaimed_frames=344 before_free=255221 before_allocated=6069 after_free=255565 after_allocated=5725
frame-allocator-diagnostic: process-teardown pid=105 reclaimed_frames=344 before_free=255569 before_allocated=5721 after_free=255913 after_allocated=5377
testcase busybox echo "hello world" > test.txt success
frame-allocator-diagnostic: process-teardown pid=108 reclaimed_frames=344 before_free=255211 before_allocated=6079 after_free=255555 after_allocated=5735
hello world
frame-allocator-diagnostic: process-teardown pid=107 reclaimed_frames=344 before_free=255559 before_allocated=5731 after_free=255903 after_allocated=5387
testcase busybox cat test.txt success
frame-allocator-diagnostic: process-teardown pid=110 reclaimed_frames=344 before_free=255201 before_allocated=6089 after_free=255545 after_allocated=5745
frame-allocator-diagnostic: process-teardown pid=96 reclaimed_frames=343 before_free=255545 before_allocated=5745 after_free=255888 after_allocated=5402
l
frame-allocator-diagnostic: process-teardown pid=109 reclaimed_frames=344 before_free=255892 before_allocated=5398 after_free=256236 after_allocated=5054
testcase busybox cut -c 3 test.txt success
frame-allocator-diagnostic: process-teardown pid=112 reclaimed_frames=344 before_free=255534 before_allocated=5756 after_free=255878 after_allocated=5412
0000000 062550 066154 020157 067567 066162 005144
0000014
frame-allocator-diagnostic: process-teardown pid=111 reclaimed_frames=344 before_free=255882 before_allocated=5408 after_free=256226 after_allocated=5064
testcase busybox od test.txt success
frame-allocator-diagnostic: process-teardown pid=114 reclaimed_frames=344 before_free=255524 before_allocated=5766 after_free=255868 after_allocated=5422
hello world
frame-allocator-diagnostic: process-teardown pid=113 reclaimed_frames=344 before_free=255872 before_allocated=5418 after_free=256216 after_allocated=5074
testcase busybox head test.txt success
frame-allocator-diagnostic: process-teardown pid=116 reclaimed_frames=344 before_free=255514 before_allocated=5776 after_free=255858 after_allocated=5432
hello world
frame-allocator-diagnostic: process-teardown pid=115 reclaimed_frames=344 before_free=255862 before_allocated=5428 after_free=256206 after_allocated=5084
testcase busybox tail test.txt success
frame-allocator-diagnostic: process-teardown pid=118 reclaimed_frames=344 before_free=255504 before_allocated=5786 after_free=255848 after_allocated=5442
00000000  68 65 6c 6c 6f 20 77 6f  72 6c 64 0a              |hello world.|
0000000c
frame-allocator-diagnostic: process-teardown pid=117 reclaimed_frames=344 before_free=255852 before_allocated=5438 after_free=256196 after_allocated=5094
testcase busybox hexdump -C test.txt success
frame-allocator-diagnostic: process-teardown pid=120 reclaimed_frames=344 before_free=255494 before_allocated=5796 after_free=255838 after_allocated=5452
6f5902ac237024bdd0c176cb93063dc4  test.txt
frame-allocator-diagnostic: process-teardown pid=119 reclaimed_frames=345 before_free=255841 before_allocated=5449 after_free=256186 after_allocated=5104
testcase busybox md5sum test.txt success
frame-allocator-diagnostic: process-teardown pid=122 reclaimed_frames=344 before_free=255484 before_allocated=5806 after_free=255828 after_allocated=5462
frame-allocator-diagnostic: process-teardown pid=121 reclaimed_frames=344 before_free=255832 before_allocated=5458 after_free=256176 after_allocated=5114
testcase busybox echo "ccccccc" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=124 reclaimed_frames=344 before_free=255474 before_allocated=5816 after_free=255818 after_allocated=5472
frame-allocator-diagnostic: process-teardown pid=123 reclaimed_frames=344 before_free=255822 before_allocated=5468 after_free=256166 after_allocated=5124
testcase busybox echo "bbbbbbb" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=126 reclaimed_frames=344 before_free=255464 before_allocated=5826 after_free=255808 after_allocated=5482
frame-allocator-diagnostic: process-teardown pid=125 reclaimed_frames=344 before_free=255812 before_allocated=5478 after_free=256156 after_allocated=5134
testcase busybox echo "aaaaaaa" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=128 reclaimed_frames=344 before_free=255454 before_allocated=5836 after_free=255798 after_allocated=5492
frame-allocator-diagnostic: process-teardown pid=127 reclaimed_frames=344 before_free=255802 before_allocated=5488 after_free=256146 after_allocated=5144
testcase busybox echo "2222222" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=130 reclaimed_frames=344 before_free=255444 before_allocated=5846 after_free=255788 after_allocated=5502
frame-allocator-diagnostic: process-teardown pid=129 reclaimed_frames=344 before_free=255792 before_allocated=5498 after_free=256136 after_allocated=5154
testcase busybox echo "1111111" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=132 reclaimed_frames=344 before_free=255434 before_allocated=5856 after_free=255778 after_allocated=5512
frame-allocator-diagnostic: process-teardown pid=131 reclaimed_frames=344 before_free=255782 before_allocated=5508 after_free=256126 after_allocated=5164
testcase busybox echo "bbbbbbb" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=134 reclaimed_frames=344 before_free=255424 before_allocated=5866 after_free=255768 after_allocated=5522
frame-allocator-diagnostic: process-teardown pid=135 reclaimed_frames=345 before_free=255409 before_allocated=5881 after_free=255754 after_allocated=5536
1111111
2222222
aaaaaaa
bbbbbbb
ccccccc
hello world
frame-allocator-diagnostic: process-teardown pid=136 reclaimed_frames=344 before_free=255414 before_allocated=5876 after_free=255758 after_allocated=5532
frame-allocator-diagnostic: process-teardown pid=133 reclaimed_frames=348 before_free=255758 before_allocated=5532 after_free=256106 after_allocated=5184
testcase busybox sort test.txt | /musl/busybox uniq success
frame-allocator-diagnostic: process-teardown pid=138 reclaimed_frames=344 before_free=255404 before_allocated=5886 after_free=255748 after_allocated=5542
  File: test.txt
  Size: 60        	Blocks: 0          IO Block: 512    regular file
Device: 1h/1d	Inode: 14331471978328146352  Links: 1
Access: (0666/-rw-rw-rw-)  Uid: (    0/    root)   Gid: (    0/    root)
Access: 1970-01-01 00:00:00.000000000 +0000
Modify: 1970-01-01 00:00:00.000000000 +0000
Change: 1970-01-01 00:00:00.000000000 +0000
frame-allocator-diagnostic: process-teardown pid=137 reclaimed_frames=344 before_free=255752 before_allocated=5538 after_free=256096 after_allocated=5194
testcase busybox stat test.txt success
frame-allocator-diagnostic: process-teardown pid=140 reclaimed_frames=344 before_free=255394 before_allocated=5896 after_free=255738 after_allocated=5552
hello world
ccccccc
bbbbbbb
aaaaaaa
2222222
1111111
bbbbbbb
frame-allocator-diagnostic: process-teardown pid=139 reclaimed_frames=344 before_free=255742 before_allocated=5548 after_free=256086 after_allocated=5204
testcase busybox strings test.txt success
frame-allocator-diagnostic: process-teardown pid=142 reclaimed_frames=344 before_free=255384 before_allocated=5906 after_free=255728 after_allocated=5562
        7         8        60 test.txt
frame-allocator-diagnostic: process-teardown pid=141 reclaimed_frames=344 before_free=255732 before_allocated=5558 after_free=256076 after_allocated=5214
testcase busybox wc test.txt success
frame-allocator-diagnostic: process-teardown pid=144 reclaimed_frames=344 before_free=255374 before_allocated=5916 after_free=255718 after_allocated=5572
frame-allocator-diagnostic: process-teardown pid=143 reclaimed_frames=344 before_free=255722 before_allocated=5568 after_free=256066 after_allocated=5224
testcase busybox [ -f test.txt ] success
frame-allocator-diagnostic: process-teardown pid=146 reclaimed_frames=344 before_free=255364 before_allocated=5926 after_free=255708 after_allocated=5582
hello world
ccccccc
bbbbbbb
aaaaaaa
2222222
1111111
bbbbbbb
frame-allocator-diagnostic: process-teardown pid=145 reclaimed_frames=344 before_free=255712 before_allocated=5578 after_free=256056 after_allocated=5234
testcase busybox more test.txt success
frame-allocator-diagnostic: process-teardown pid=148 reclaimed_frames=344 before_free=255354 before_allocated=5936 after_free=255698 after_allocated=5592
frame-allocator-diagnostic: process-teardown pid=147 reclaimed_frames=343 before_free=255703 before_allocated=5587 after_free=256046 after_allocated=5244
testcase busybox rm test.txt success
frame-allocator-diagnostic: process-teardown pid=150 reclaimed_frames=344 before_free=255344 before_allocated=5946 after_free=255688 after_allocated=5602
frame-allocator-diagnostic: process-teardown pid=149 reclaimed_frames=343 before_free=255693 before_allocated=5597 after_free=256036 after_allocated=5254
testcase busybox mkdir test_dir success
frame-allocator-diagnostic: process-teardown pid=152 reclaimed_frames=344 before_free=255334 before_allocated=5956 after_free=255678 after_allocated=5612
frame-allocator-diagnostic: process-teardown pid=151 reclaimed_frames=343 before_free=255683 before_allocated=5607 after_free=256026 after_allocated=5264
testcase busybox mv test_dir test success
frame-allocator-diagnostic: process-teardown pid=154 reclaimed_frames=344 before_free=255324 before_allocated=5966 after_free=255668 after_allocated=5622
frame-allocator-diagnostic: process-teardown pid=153 reclaimed_frames=343 before_free=255673 before_allocated=5617 after_free=256016 after_allocated=5274
testcase busybox rmdir test success
frame-allocator-diagnostic: process-teardown pid=156 reclaimed_frames=344 before_free=255314 before_allocated=5976 after_free=255658 after_allocated=5632
echo "hello world" > test.txt
grep hello busybox_cmd.txt
frame-allocator-diagnostic: process-teardown pid=155 reclaimed_frames=346 before_free=255660 before_allocated=5630 after_free=256006 after_allocated=5284
testcase busybox grep hello busybox_cmd.txt success
frame-allocator-diagnostic: process-teardown pid=158 reclaimed_frames=344 before_free=255304 before_allocated=5986 after_free=255648 after_allocated=5642
frame-allocator-diagnostic: process-teardown pid=157 reclaimed_frames=344 before_free=255652 before_allocated=5638 after_free=255996 after_allocated=5294
testcase busybox cp busybox_cmd.txt busybox_cmd.bak success
frame-allocator-diagnostic: process-teardown pid=160 reclaimed_frames=344 before_free=255294 before_allocated=5996 after_free=255638 after_allocated=5652
frame-allocator-diagnostic: process-teardown pid=159 reclaimed_frames=343 before_free=255643 before_allocated=5647 after_free=255986 after_allocated=5304
testcase busybox rm busybox_cmd.bak success
frame-allocator-diagnostic: process-teardown pid=162 reclaimed_frames=344 before_free=255284 before_allocated=6006 after_free=255628 after_allocated=5662
./busybox_cmd.txt
frame-allocator-diagnostic: process-teardown pid=161 reclaimed_frames=344 before_free=255632 before_allocated=5658 after_free=255976 after_allocated=5314
testcase busybox find -name "busybox_cmd.txt" success
#### OS COMP TEST GROUP END busybox-musl ####
frame-allocator-diagnostic: process-teardown pid=164 reclaimed_frames=344 before_free=255274 before_allocated=6016 after_free=255618 after_allocated=5672
#### OS COMP TEST GROUP START cyclictest-musl ####
frame-allocator-diagnostic: process-teardown pid=165 reclaimed_frames=344 before_free=255272 before_allocated=6018 after_free=255616 after_allocated=5674
====== cyclictest NO_STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 (  167) P:99 I:1000 C:    986 Min:      2 Act:    3 Avg:   29 Max:   11333
frame-allocator-diagnostic: process-teardown pid=166 reclaimed_frames=63 before_free=255546 before_allocated=5744 after_free=255609 after_allocated=5681
====== cyclictest NO_STRESS_P1 end: success ======
====== cyclictest NO_STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 (  169) P:99 I:1000 C:    988 Min:      2 Act:    3 Avg:   23 Max:    6794
T: 1 (  170) P:99 I:1500 C:    660 Min:      2 Act:   83 Avg:   32 Max:    6717
T: 2 (  171) P:99 I:2000 C:    495 Min:      2 Act:  368 Avg:   44 Max:    5706
T: 3 (  172) P:99 I:2500 C:    398 Min:      2 Act:  393 Avg:   35 Max:    6190
T: 4 (  173) P:99 I:3000 C:    331 Min:      2 Act:   11 Avg:   49 Max:    6676
T: 5 (  174) P:99 I:3500 C:    284 Min:      2 Act:   15 Avg:   49 Max:    6163
T: 6 (  175) P:99 I:4000 C:    250 Min:      2 Act:  338 Avg:   57 Max:    3976
T: 7 (  176) P:99 I:4500 C:    223 Min:      2 Act:   11 Avg:   42 Max:    3637
frame-allocator-diagnostic: process-teardown pid=168 reclaimed_frames=63 before_free=255538 before_allocated=5752 after_free=255601 after_allocated=5689
====== cyclictest NO_STRESS_P8 end: success ======
====== start hackbench ======
Running in process mode with 10 groups using 40 file descriptors each (== 400 tasks)
Each sender will pass 100000000 messages of 100 bytes
frame-allocator-diagnostic: process-teardown pid=178 reclaimed_frames=343 before_free=237818 before_allocated=23472 after_free=238161 after_allocated=23129
====== cyclictest STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 (  580) P:99 I:1000 C:      8 Min: 119554 Act:145110 Avg:139485 Max:  243555
frame-allocator-diagnostic: process-teardown pid=579 reclaimed_frames=63 before_free=238091 before_allocated=23199 after_free=238154 after_allocated=23136
====== cyclictest STRESS_P1 end: success ======
====== cyclictest STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 (  582) P:99 I:1000 C:      7 Min: 121786 Act:171443 Avg:146744 Max:  239035
T: 1 (  583) P:99 I:1500 C:      7 Min: 122090 Act:170912 Avg:146459 Max:  238519
T: 2 (  584) P:99 I:2000 C:      7 Min: 120544 Act:172222 Avg:146536 Max:  237975
T: 3 (  585) P:99 I:2500 C:      8 Min:     37 Act:172241 Avg:127880 Max:  237429
T: 4 (  586) P:99 I:3000 C:      7 Min: 120496 Act:171627 Avg:145808 Max:  236882
T: 5 (  587) P:99 I:3500 C:      7 Min: 120835 Act:171716 Avg:145856 Max:  236339
T: 6 (  588) P:99 I:4000 C:      7 Min: 119524 Act:172723 Avg:145606 Max:  235797
T: 7 (  589) P:99 I:4500 C:      7 Min: 118978 Act:173214 Avg:144851 Max:  235250
frame-allocator-diagnostic: process-teardown pid=581 reclaimed_frames=63 before_free=238084 before_allocated=23206 after_free=238147 after_allocated=23143
====== cyclictest STRESS_P8 end: success ======
frame-allocator-diagnostic: process-teardown pid=590 reclaimed_frames=343 before_free=237799 before_allocated=23491 after_free=238142 after_allocated=23148
frame-allocator-diagnostic: process-teardown pid=591 reclaimed_frames=343 before_free=237794 before_allocated=23496 after_free=238137 after_allocated=23153
====== kill hackbench: success ======
#### OS COMP TEST GROUP END cyclictest-musl ####
frame-allocator-diagnostic: process-teardown pid=592 reclaimed_frames=344 before_free=237788 before_allocated=23502 after_free=238132 after_allocated=23158
frame-allocator-diagnostic: process-teardown pid=163 reclaimed_frames=346 before_free=238132 before_allocated=23158 after_free=238478 after_allocated=22812
#### OS COMP TEST GROUP START iozone-musl ####
SKIP: iozone throughput mode currently hangs in the evaluator environment
#### OS COMP TEST GROUP END iozone-musl ####
frame-allocator-diagnostic: process-teardown pid=594 reclaimed_frames=344 before_free=237777 before_allocated=23513 after_free=238121 after_allocated=23169
#### OS COMP TEST GROUP START iperf-musl ####
frame-allocator-diagnostic: process-teardown pid=595 reclaimed_frames=344 before_free=237774 before_allocated=23516 after_free=238118 after_allocated=23172
frame-allocator-diagnostic: process-teardown pid=596 reclaimed_frames=2 before_free=238051 before_allocated=23239 after_free=238053 after_allocated=23237
frame-allocator-diagnostic: process-teardown pid=597 reclaimed_frames=1 before_free=238047 before_allocated=23243 after_free=238048 after_allocated=23242
====== iperf BASIC_UDP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 0.0.0.0 port 49152 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  5]   0.00-2.03   sec  9.98 KBytes  40.4 Kbits/sec  7  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.03   sec  9.98 KBytes  40.4 Kbits/sec  0.000 ms  0/7 (0%)  sender
[  5]   0.00-2.53   sec  9.98 KBytes  32.3 Kbits/sec  20.948 ms  0/7 (0%)  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=599 reclaimed_frames=57 before_free=237977 before_allocated=23313 after_free=238034 after_allocated=23256
====== iperf BASIC_UDP end: success ======

====== iperf BASIC_TCP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 127.0.0.1 port 49154 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.04   sec  2.50 MBytes  10.3 Mbits/sec    0   0.00 Bytes       
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.04   sec  2.50 MBytes  10.3 Mbits/sec    0             sender
[  5]   0.00-2.42   sec  1.62 MBytes  5.64 Mbits/sec                  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=600 reclaimed_frames=57 before_free=237938 before_allocated=23352 after_free=237995 after_allocated=23295
====== iperf BASIC_TCP end: success ======

====== iperf PARALLEL_UDP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 0.0.0.0 port 49153 connected to 127.0.0.1 port 5001
[  7] local 0.0.0.0 port 49154 connected to 127.0.0.1 port 5001
[  9] local 0.0.0.0 port 49155 connected to 127.0.0.1 port 5001
[ 11] local 0.0.0.0 port 49156 connected to 127.0.0.1 port 5001
[ 13] local 0.0.0.0 port 49157 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  5]   0.00-2.66   sec  4.28 KBytes  13.2 Kbits/sec  3  
[  7]   0.00-2.66   sec  4.28 KBytes  13.2 Kbits/sec  3  
[  9]   0.00-2.66   sec  4.28 KBytes  13.2 Kbits/sec  3  
[ 11]   0.00-2.66   sec  4.28 KBytes  13.2 Kbits/sec  3  
[ 13]   0.00-2.66   sec  4.28 KBytes  13.2 Kbits/sec  3  
[SUM]   0.00-2.66   sec  21.4 KBytes  65.9 Kbits/sec  15  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.66   sec  4.28 KBytes  13.2 Kbits/sec  0.000 ms  0/3 (0%)  sender
[  5]   0.00-3.03   sec  4.28 KBytes  11.6 Kbits/sec  8.980 ms  0/3 (0%)  receiver
[  7]   0.00-2.66   sec  4.28 KBytes  13.2 Kbits/sec  0.000 ms  0/3 (0%)  sender
[  7]   0.00-3.03   sec  4.28 KBytes  11.6 Kbits/sec  8.874 ms  0/3 (0%)  receiver
[  9]   0.00-2.66   sec  4.28 KBytes  13.2 Kbits/sec  0.000 ms  0/3 (0%)  sender
[  9]   0.00-3.03   sec  4.28 KBytes  11.6 Kbits/sec  8.087 ms  0/3 (0%)  receiver
[ 11]   0.00-2.66   sec  4.28 KBytes  13.2 Kbits/sec  0.000 ms  0/3 (0%)  sender
[ 11]   0.00-3.03   sec  4.28 KBytes  11.6 Kbits/sec  1.408 ms  0/3 (0%)  receiver
[ 13]   0.00-2.66   sec  4.28 KBytes  13.2 Kbits/sec  0.000 ms  0/3 (0%)  sender
[ 13]   0.00-3.03   sec  4.28 KBytes  11.6 Kbits/sec  0.784 ms  0/3 (0%)  receiver
[SUM]   0.00-2.66   sec  21.4 KBytes  65.9 Kbits/sec  0.000 ms  0/15 (0%)  sender
[SUM]   0.00-3.03   sec  21.4 KBytes  57.9 Kbits/sec  5.627 ms  0/15 (0%)  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=601 reclaimed_frames=60 before_free=237952 before_allocated=23338 after_free=238012 after_allocated=23278
====== iperf PARALLEL_UDP end: success ======

====== iperf PARALLEL_TCP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 127.0.0.1 port 49157 connected to 127.0.0.1 port 5001
[  7] local 127.0.0.1 port 49158 connected to 127.0.0.1 port 5001
[  9] local 127.0.0.1 port 49159 connected to 127.0.0.1 port 5001
[ 11] local 127.0.0.1 port 49160 connected to 127.0.0.1 port 5001
[ 13] local 127.0.0.1 port 49161 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.01   sec  2.25 MBytes  9.40 Mbits/sec    0   0.00 Bytes       
[  7]   0.00-2.01   sec  2.25 MBytes  9.40 Mbits/sec    0   0.00 Bytes       
[  9]   0.00-2.01   sec  2.25 MBytes  9.40 Mbits/sec    0   0.00 Bytes       
[ 11]   0.00-2.01   sec  2.25 MBytes  9.40 Mbits/sec    0   0.00 Bytes       
[ 13]   0.00-2.01   sec  2.25 MBytes  9.40 Mbits/sec    0   0.00 Bytes       
[SUM]   0.00-2.01   sec  11.2 MBytes  47.0 Mbits/sec    0             
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.01   sec  2.25 MBytes  9.40 Mbits/sec    0             sender
[  5]   0.00-2.56   sec  1.38 MBytes  4.50 Mbits/sec                  receiver
[  7]   0.00-2.01   sec  2.25 MBytes  9.40 Mbits/sec    0             sender
[  7]   0.00-2.56   sec  1.38 MBytes  4.50 Mbits/sec                  receiver
[  9]   0.00-2.01   sec  2.25 MBytes  9.40 Mbits/sec    0             sender
[  9]   0.00-2.56   sec  1.38 MBytes  4.50 Mbits/sec                  receiver
[ 11]   0.00-2.01   sec  2.25 MBytes  9.40 Mbits/sec    0             sender
[ 11]   0.00-2.56   sec  1.38 MBytes  4.50 Mbits/sec                  receiver
[ 13]   0.00-2.01   sec  2.25 MBytes  9.40 Mbits/sec    0             sender
[ 13]   0.00-2.56   sec  1.38 MBytes  4.50 Mbits/sec                  receiver
[SUM]   0.00-2.01   sec  11.2 MBytes  47.0 Mbits/sec    0             sender
[SUM]   0.00-2.56   sec  6.88 MBytes  22.5 Mbits/sec                  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=602 reclaimed_frames=60 before_free=237790 before_allocated=23500 after_free=237850 after_allocated=23440
====== iperf PARALLEL_TCP end: success ======

====== iperf REVERSE_UDP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 0.0.0.0 port 49158 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.39   sec  11.4 KBytes  39.1 Kbits/sec  13.087 ms  0/8 (0%)  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.90   sec  12.8 KBytes  36.3 Kbits/sec  0.000 ms  0/9 (0%)  sender
[  5]   0.00-2.39   sec  11.4 KBytes  39.1 Kbits/sec  13.087 ms  0/8 (0%)  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=603 reclaimed_frames=57 before_free=237945 before_allocated=23345 after_free=238002 after_allocated=23288
====== iperf REVERSE_UDP end: success ======

====== iperf REVERSE_TCP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 127.0.0.1 port 49164 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.04   sec  1.75 MBytes  7.19 Mbits/sec                  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.56   sec  2.75 MBytes  9.02 Mbits/sec    0             sender
[  5]   0.00-2.04   sec  1.75 MBytes  7.19 Mbits/sec                  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=604 reclaimed_frames=57 before_free=237907 before_allocated=23383 after_free=237964 after_allocated=23326
====== iperf REVERSE_TCP end: success ======

#### OS COMP TEST GROUP END iperf-musl ####
frame-allocator-diagnostic: process-teardown pid=605 reclaimed_frames=344 before_free=237647 before_allocated=23643 after_free=237991 after_allocated=23299
frame-allocator-diagnostic: process-teardown pid=593 reclaimed_frames=347 before_free=237991 before_allocated=23299 after_free=238338 after_allocated=22952
#### OS COMP TEST GROUP START libcbench-musl ####
SKIP: libcbench currently triggers an unrecovered allocator exhaustion path
#### OS COMP TEST GROUP END libcbench-musl ####
#### OS COMP TEST GROUP START libctest-musl ####
SKIP: libctest still trips unresolved pthread cancellation paths
#### OS COMP TEST GROUP END libctest-musl ####
#### OS COMP TEST GROUP START lmbench-musl ####
SKIP: lmbench still triggers an unresolved user-space page-fault path
#### OS COMP TEST GROUP END lmbench-musl ####
#### OS COMP TEST GROUP START ltp-musl ####
SKIP: full LTP sweep is too large for the boot-time evaluator smoke run
#### OS COMP TEST GROUP END ltp-musl ####
frame-allocator-diagnostic: process-teardown pid=607 reclaimed_frames=343 before_free=237638 before_allocated=23652 after_free=237981 after_allocated=23309
#### OS COMP TEST GROUP START lua-musl ####
frame-allocator-diagnostic: process-teardown pid=608 reclaimed_frames=344 before_free=237634 before_allocated=23656 after_free=237978 after_allocated=23312
frame-allocator-diagnostic: process-teardown pid=610 reclaimed_frames=88 before_free=237535 before_allocated=23755 after_free=237623 after_allocated=23667
testcase lua date.lua success
frame-allocator-diagnostic: process-teardown pid=609 reclaimed_frames=346 before_free=237622 before_allocated=23668 after_free=237968 after_allocated=23322
frame-allocator-diagnostic: process-teardown pid=612 reclaimed_frames=89 before_free=237524 before_allocated=23766 after_free=237613 after_allocated=23677
testcase lua file_io.lua success
frame-allocator-diagnostic: process-teardown pid=611 reclaimed_frames=346 before_free=237612 before_allocated=23678 after_free=237958 after_allocated=23332
frame-allocator-diagnostic: process-teardown pid=614 reclaimed_frames=88 before_free=237515 before_allocated=23775 after_free=237603 after_allocated=23687
testcase lua max_min.lua success
frame-allocator-diagnostic: process-teardown pid=613 reclaimed_frames=346 before_free=237602 before_allocated=23688 after_free=237948 after_allocated=23342
frame-allocator-diagnostic: process-teardown pid=616 reclaimed_frames=88 before_free=237505 before_allocated=23785 after_free=237593 after_allocated=23697
testcase lua random.lua success
frame-allocator-diagnostic: process-teardown pid=615 reclaimed_frames=346 before_free=237592 before_allocated=23698 after_free=237938 after_allocated=23352
frame-allocator-diagnostic: process-teardown pid=618 reclaimed_frames=89 before_free=237494 before_allocated=23796 after_free=237583 after_allocated=23707
testcase lua remove.lua success
frame-allocator-diagnostic: process-teardown pid=617 reclaimed_frames=346 before_free=237582 before_allocated=23708 after_free=237928 after_allocated=23362
frame-allocator-diagnostic: process-teardown pid=620 reclaimed_frames=89 before_free=237484 before_allocated=23806 after_free=237573 after_allocated=23717
testcase lua round_num.lua success
frame-allocator-diagnostic: process-teardown pid=619 reclaimed_frames=346 before_free=237572 before_allocated=23718 after_free=237918 after_allocated=23372
frame-allocator-diagnostic: process-teardown pid=622 reclaimed_frames=89 before_free=237474 before_allocated=23816 after_free=237563 after_allocated=23727
testcase lua sin30.lua success
frame-allocator-diagnostic: process-teardown pid=621 reclaimed_frames=346 before_free=237562 before_allocated=23728 after_free=237908 after_allocated=23382
frame-allocator-diagnostic: process-teardown pid=624 reclaimed_frames=89 before_free=237464 before_allocated=23826 after_free=237553 after_allocated=23737
testcase lua sort.lua success
frame-allocator-diagnostic: process-teardown pid=623 reclaimed_frames=346 before_free=237552 before_allocated=23738 after_free=237898 after_allocated=23392
frame-allocator-diagnostic: process-teardown pid=626 reclaimed_frames=89 before_free=237454 before_allocated=23836 after_free=237543 after_allocated=23747
testcase lua strings.lua success
frame-allocator-diagnostic: process-teardown pid=625 reclaimed_frames=346 before_free=237542 before_allocated=23748 after_free=237888 after_allocated=23402
#### OS COMP TEST GROUP END lua-musl ####
frame-allocator-diagnostic: process-teardown pid=627 reclaimed_frames=344 before_free=237539 before_allocated=23751 after_free=237883 after_allocated=23407
frame-allocator-diagnostic: process-teardown pid=606 reclaimed_frames=345 before_free=237883 before_allocated=23407 after_free=238228 after_allocated=23062
frame-allocator-diagnostic: process-teardown pid=629 reclaimed_frames=344 before_free=237526 before_allocated=23764 after_free=237870 after_allocated=23420
#### OS COMP TEST GROUP START netperf-musl ####
frame-allocator-diagnostic: process-teardown pid=630 reclaimed_frames=344 before_free=237524 before_allocated=23766 after_free=237868 after_allocated=23422
====== netperf UDP_STREAM begin ======
Starting netserver with host '127.0.0.1' port '12865' and family AF_UNSPEC
MIGRATED UDP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Socket  Message  Elapsed      Messages                
Size    Size     Time         Okay Errors   Throughput
bytes   bytes    secs            #      #   10^6bits/sec

 65536    1000   1.48            7      0       0.04
 65536           1.48            7              0.04

frame-allocator-diagnostic: process-teardown pid=633 reclaimed_frames=29 before_free=237323 before_allocated=23967 after_free=237352 after_allocated=23938
frame-allocator-diagnostic: process-teardown pid=632 reclaimed_frames=254 before_free=237352 before_allocated=23938 after_free=237606 after_allocated=23684
====== netperf UDP_STREAM end: success ======
====== netperf TCP_STREAM begin ======
MIGRATED TCP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Recv   Send    Send                          
Socket Socket  Message  Elapsed              
Size   Size    Size     Time     Throughput  
bytes  bytes   bytes    secs.    10^6bits/sec  

 65536  65536   1000    0.17       49.88   
frame-allocator-diagnostic: process-teardown pid=637 reclaimed_frames=29 before_free=237309 before_allocated=23981 after_free=237338 after_allocated=23952
frame-allocator-diagnostic: process-teardown pid=636 reclaimed_frames=254 before_free=237338 before_allocated=23952 after_free=237592 after_allocated=23698
====== netperf TCP_STREAM end: success ======
====== netperf UDP_RR begin ======
MIGRATED UDP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.48        4.72   
65536  65536 
frame-allocator-diagnostic: process-teardown pid=641 reclaimed_frames=12 before_free=237329 before_allocated=23961 after_free=237341 after_allocated=23949
frame-allocator-diagnostic: process-teardown pid=640 reclaimed_frames=237 before_free=237341 before_allocated=23949 after_free=237578 after_allocated=23712
====== netperf UDP_RR end: success ======
====== netperf TCP_RR begin ======
MIGRATED TCP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.23        5.69   
65536  65536 
frame-allocator-diagnostic: process-teardown pid=645 reclaimed_frames=12 before_free=237315 before_allocated=23975 after_free=237327 after_allocated=23963
frame-allocator-diagnostic: process-teardown pid=644 reclaimed_frames=237 before_free=237327 before_allocated=23963 after_free=237564 after_allocated=23726
====== netperf TCP_RR end: success ======
====== netperf TCP_CRR begin ======
MIGRATED TCP Connect/Request/Response TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.02        6.86   
65536  65536 
frame-allocator-diagnostic: process-teardown pid=649 reclaimed_frames=12 before_free=237301 before_allocated=23989 after_free=237313 after_allocated=23977
frame-allocator-diagnostic: process-teardown pid=648 reclaimed_frames=237 before_free=237313 before_allocated=23977 after_free=237550 after_allocated=23740
====== netperf TCP_CRR end: success ======
frame-allocator-diagnostic: process-teardown pid=652 reclaimed_frames=343 before_free=237202 before_allocated=24088 after_free=237545 after_allocated=23745
frame-allocator-diagnostic: process-teardown pid=631 reclaimed_frames=240 before_free=237545 before_allocated=23745 after_free=237785 after_allocated=23505
#### OS COMP TEST GROUP END netperf-musl ####
frame-allocator-diagnostic: process-teardown pid=653 reclaimed_frames=344 before_free=237436 before_allocated=23854 after_free=237780 after_allocated=23510
frame-allocator-diagnostic: process-teardown pid=628 reclaimed_frames=346 before_free=237780 before_allocated=23510 after_free=238126 after_allocated=23164
#### OS COMP TEST GROUP START unixbench-musl ####
SKIP: unixbench currently blocks on unresolved executable/runtime compatibility
#### OS COMP TEST GROUP END unixbench-musl ####
frame-allocator-diagnostic: process-teardown pid=655 reclaimed_frames=438 before_free=237238 before_allocated=24052 after_free=237676 after_allocated=23614
#### OS COMP TEST GROUP START basic-glibc ####
frame-allocator-diagnostic: process-teardown pid=656 reclaimed_frames=438 before_free=237234 before_allocated=24056 after_free=237672 after_allocated=23618
Testing brk :
========== START test_brk ==========
Before alloc,heap pos: 77824
After alloc,heap pos: 77888
Alloc again,heap pos: 77952
========== END test_brk ==========
frame-allocator-diagnostic: process-teardown pid=658 reclaimed_frames=39 before_free=237182 before_allocated=24108 after_free=237221 after_allocated=24069
Testing chdir :
========== START test_chdir ==========
chdir ret: 0
  current working dir : 
========== END test_chdir ==========
frame-allocator-diagnostic: process-teardown pid=659 reclaimed_frames=39 before_free=237175 before_allocated=24115 after_free=237214 after_allocated=24076
Testing clone :
========== START test_clone ==========
  Child says successfully!
frame-allocator-diagnostic: process-teardown pid=661 reclaimed_frames=1 before_free=237158 before_allocated=24132 after_free=237159 after_allocated=24131
clone process successfully.
pid:661
========== END test_clone ==========
frame-allocator-diagnostic: process-teardown pid=660 reclaimed_frames=41 before_free=237159 before_allocated=24131 after_free=237200 after_allocated=24090
Testing close :
========== START test_close ==========
  close 3 success.
========== END test_close ==========
frame-allocator-diagnostic: process-teardown pid=662 reclaimed_frames=39 before_free=237154 before_allocated=24136 after_free=237193 after_allocated=24097
Testing dup2 :
========== START test_dup2 ==========
  from fd 100
========== END test_dup2 ==========
frame-allocator-diagnostic: process-teardown pid=663 reclaimed_frames=39 before_free=237147 before_allocated=24143 after_free=237186 after_allocated=24104
Testing dup :
========== START test_dup ==========
  new fd is 3.
========== END test_dup ==========
frame-allocator-diagnostic: process-teardown pid=664 reclaimed_frames=39 before_free=237140 before_allocated=24150 after_free=237179 after_allocated=24111
Testing execve :
========== START test_execve ==========
  I am test_echo.
execve success.
========== END main ==========
frame-allocator-diagnostic: process-teardown pid=665 reclaimed_frames=39 before_free=237133 before_allocated=24157 after_free=237172 after_allocated=24118
Testing exit :
========== START test_exit ==========
frame-allocator-diagnostic: process-teardown pid=667 reclaimed_frames=0 before_free=237119 before_allocated=24171 after_free=237119 after_allocated=24171
exit OK.
========== END test_exit ==========
frame-allocator-diagnostic: process-teardown pid=666 reclaimed_frames=39 before_free=237119 before_allocated=24171 after_free=237158 after_allocated=24132
Testing fork :
========== START test_fork ==========
  child process.
frame-allocator-diagnostic: process-teardown pid=669 reclaimed_frames=1 before_free=237104 before_allocated=24186 after_free=237105 after_allocated=24185
  parent process. wstatus:0
========== END test_fork ==========
frame-allocator-diagnostic: process-teardown pid=668 reclaimed_frames=39 before_free=237105 before_allocated=24185 after_free=237144 after_allocated=24146
Testing fstat :
========== START test_fstat ==========
fstat ret: 0
fstat: dev: 1, inode: 1612857110, mode: 33206, nlink: 1, size: 52, atime: 0, mtime: 0, ctime: 0
========== END test_fstat ==========
frame-allocator-diagnostic: process-teardown pid=670 reclaimed_frames=39 before_free=237098 before_allocated=24192 after_free=237137 after_allocated=24153
Testing getcwd :
========== START test_getcwd ==========
getcwd: /tmp/testsuite/glibc/basic/basic successfully!
========== END test_getcwd ==========
frame-allocator-diagnostic: process-teardown pid=671 reclaimed_frames=39 before_free=237091 before_allocated=24199 after_free=237130 after_allocated=24160
Testing getdents :
========== START test_getdents ==========
open fd:3
getdents fd:-20
getdents success.


========== END test_getdents ==========
frame-allocator-diagnostic: process-teardown pid=672 reclaimed_frames=39 before_free=237084 before_allocated=24206 after_free=237123 after_allocated=24167
Testing getpid :
========== START test_getpid ==========
getpid success.
pid = 673
========== END test_getpid ==========
frame-allocator-diagnostic: process-teardown pid=673 reclaimed_frames=39 before_free=237077 before_allocated=24213 after_free=237116 after_allocated=24174
Testing getppid :
========== START test_getppid ==========
  getppid success. ppid : 657
========== END test_getppid ==========
frame-allocator-diagnostic: process-teardown pid=674 reclaimed_frames=39 before_free=237070 before_allocated=24220 after_free=237109 after_allocated=24181
Testing gettimeofday :
========== START test_gettimeofday ==========
gettimeofday success.
start:134792, end:134843
interval: 51
========== END test_gettimeofday ==========
frame-allocator-diagnostic: process-teardown pid=675 reclaimed_frames=39 before_free=237063 before_allocated=24227 after_free=237102 after_allocated=24188
Testing mkdir_ :
========== START test_mkdir ==========
mkdir ret: 0
  mkdir success.
========== END test_mkdir ==========
frame-allocator-diagnostic: process-teardown pid=676 reclaimed_frames=39 before_free=237056 before_allocated=24234 after_free=237095 after_allocated=24195
Testing mmap :
========== START test_mmap ==========
file len: 27
mmap content:   Hello, mmap successfully!
========== END test_mmap ==========
frame-allocator-diagnostic: process-teardown pid=677 reclaimed_frames=39 before_free=237049 before_allocated=24241 after_free=237088 after_allocated=24202
Testing mount :
========== START test_mount ==========
Mounting dev:/dev/vda2 to ./mnt
mount return: 0
mount successfully
umount return: 0
========== END test_mount ==========
frame-allocator-diagnostic: process-teardown pid=678 reclaimed_frames=39 before_free=237042 before_allocated=24248 after_free=237081 after_allocated=24209
Testing munmap :
========== START test_munmap ==========
file len: 27
munmap return: 0
munmap successfully!
========== END test_munmap ==========
frame-allocator-diagnostic: process-teardown pid=679 reclaimed_frames=39 before_free=237035 before_allocated=24255 after_free=237074 after_allocated=24216
Testing openat :
========== START test_openat ==========
open dir fd: 3
openat fd: 4
openat success.
========== END test_openat ==========
frame-allocator-diagnostic: process-teardown pid=680 reclaimed_frames=39 before_free=237028 before_allocated=24262 after_free=237067 after_allocated=24223
Testing open :
========== START test_open ==========
Hi, this is a text file.
syscalls testing success!

========== END test_open ==========
frame-allocator-diagnostic: process-teardown pid=681 reclaimed_frames=39 before_free=237021 before_allocated=24269 after_free=237060 after_allocated=24230
Testing pipe :
========== START test_pipe ==========
cpid: 683
cpid: 0
frame-allocator-diagnostic: process-teardown pid=683 reclaimed_frames=1 before_free=237006 before_allocated=24284 after_free=237007 after_allocated=24283
  Write to pipe successfully.

========== END test_pipe ==========
frame-allocator-diagnostic: process-teardown pid=682 reclaimed_frames=39 before_free=237007 before_allocated=24283 after_free=237046 after_allocated=24244
Testing read :
========== START test_read ==========
Hi, this is a text file.
syscalls testing success!

========== END test_read ==========
frame-allocator-diagnostic: process-teardown pid=684 reclaimed_frames=39 before_free=237000 before_allocated=24290 after_free=237039 after_allocated=24251
Testing sleep :
========== START test_sleep ==========
sleep success.
========== END test_sleep ==========
frame-allocator-diagnostic: process-teardown pid=685 reclaimed_frames=39 before_free=236993 before_allocated=24297 after_free=237032 after_allocated=24258
Testing times :
========== START test_times ==========
mytimes success
{tms_utime:0, tms_stime:0, tms_cutime:0, tms_cstime:0}
========== END test_times ==========
frame-allocator-diagnostic: process-teardown pid=686 reclaimed_frames=39 before_free=236986 before_allocated=24304 after_free=237025 after_allocated=24265
Testing umount :
========== START test_umount ==========
Mounting dev:/dev/vda2 to ./mnt
mount return: 0
umount success.
return: 0
========== END test_umount ==========
frame-allocator-diagnostic: process-teardown pid=687 reclaimed_frames=39 before_free=236979 before_allocated=24311 after_free=237018 after_allocated=24272
Testing uname :
========== START test_uname ==========
Uname: Linux arceos 6.0.0 ArceOS riscv64 localdomain
========== END test_uname ==========
frame-allocator-diagnostic: process-teardown pid=688 reclaimed_frames=39 before_free=236972 before_allocated=24318 after_free=237011 after_allocated=24279
Testing unlink :
========== START test_unlink ==========
  unlink success!
========== END test_unlink ==========
frame-allocator-diagnostic: process-teardown pid=689 reclaimed_frames=39 before_free=236965 before_allocated=24325 after_free=237004 after_allocated=24286
Testing wait :
========== START test_wait ==========
This is child process
frame-allocator-diagnostic: process-teardown pid=691 reclaimed_frames=1 before_free=236950 before_allocated=24340 after_free=236951 after_allocated=24339
wait child success.
wstatus: 0
========== END test_wait ==========
frame-allocator-diagnostic: process-teardown pid=690 reclaimed_frames=39 before_free=236951 before_allocated=24339 after_free=236990 after_allocated=24300
Testing waitpid :
========== START test_waitpid ==========
This is child process
frame-allocator-diagnostic: process-teardown pid=693 reclaimed_frames=2 before_free=236935 before_allocated=24355 after_free=236937 after_allocated=24353
waitpid successfully.
wstatus: 3
========== END test_waitpid ==========
frame-allocator-diagnostic: process-teardown pid=692 reclaimed_frames=39 before_free=236937 before_allocated=24353 after_free=236976 after_allocated=24314
Testing write :
========== START test_write ==========
Hello operating system contest.
========== END test_write ==========
frame-allocator-diagnostic: process-teardown pid=694 reclaimed_frames=39 before_free=236930 before_allocated=24360 after_free=236969 after_allocated=24321
Testing yield :
========== START test_yield ==========
  I am child process: 696. iteration -2144419840.
  I am child process: 697. iteration -2144419840.
  I am child process: 698. iteration -2144419840.
  I am child process: 696. iteration -2144419840.
  I am child process: 697. iteration -2144419840.
  I am child process: 698. iteration -2144419840.
  I am child process: 696. iteration -2144419840.
  I am child process: 697. iteration -2144419840.
  I am child process: 698. iteration -2144419840.
  I am child process: 696. iteration -2144419840.
  I am child process: 697. iteration -2144419840.
  I am child process: 698. iteration -2144419840.
  I am child process: 696. iteration -2144419840.
frame-allocator-diagnostic: process-teardown pid=696 reclaimed_frames=1 before_free=236899 before_allocated=24391 after_free=236900 after_allocated=24390
  I am child process: 697. iteration -2144419840.
frame-allocator-diagnostic: process-teardown pid=697 reclaimed_frames=1 before_free=236900 before_allocated=24390 after_free=236901 after_allocated=24389
  I am child process: 698. iteration -2144419840.
frame-allocator-diagnostic: process-teardown pid=698 reclaimed_frames=1 before_free=236901 before_allocated=24389 after_free=236902 after_allocated=24388
========== END test_yield ==========
frame-allocator-diagnostic: process-teardown pid=695 reclaimed_frames=39 before_free=236902 before_allocated=24388 after_free=236941 after_allocated=24349
frame-allocator-diagnostic: process-teardown pid=657 reclaimed_frames=439 before_free=236941 before_allocated=24349 after_free=237380 after_allocated=23910
#### OS COMP TEST GROUP END basic-glibc ####
frame-allocator-diagnostic: process-teardown pid=699 reclaimed_frames=438 before_free=236937 before_allocated=24353 after_free=237375 after_allocated=23915
frame-allocator-diagnostic: process-teardown pid=654 reclaimed_frames=439 before_free=237375 before_allocated=23915 after_free=237814 after_allocated=23476
#### OS COMP TEST GROUP START busybox-glibc ####
frame-allocator-diagnostic: process-teardown pid=701 reclaimed_frames=438 before_free=236925 before_allocated=24365 after_free=237363 after_allocated=23927
#### independent command test
frame-allocator-diagnostic: process-teardown pid=700 reclaimed_frames=438 before_free=237366 before_allocated=23924 after_free=237804 after_allocated=23486
testcase busybox echo "#### independent command test" success
frame-allocator-diagnostic: process-teardown pid=703 reclaimed_frames=438 before_free=236915 before_allocated=24375 after_free=237353 after_allocated=23937
frame-allocator-diagnostic: process-teardown pid=702 reclaimed_frames=439 before_free=237355 before_allocated=23935 after_free=237794 after_allocated=23496
testcase busybox ash -c exit success
frame-allocator-diagnostic: process-teardown pid=705 reclaimed_frames=438 before_free=236905 before_allocated=24385 after_free=237343 after_allocated=23947
frame-allocator-diagnostic: process-teardown pid=704 reclaimed_frames=439 before_free=237345 before_allocated=23945 after_free=237784 after_allocated=23506
testcase busybox sh -c exit success
frame-allocator-diagnostic: process-teardown pid=707 reclaimed_frames=438 before_free=236895 before_allocated=24395 after_free=237333 after_allocated=23957
bbb
frame-allocator-diagnostic: process-teardown pid=706 reclaimed_frames=438 before_free=237336 before_allocated=23954 after_free=237774 after_allocated=23516
testcase busybox basename /aaa/bbb success
frame-allocator-diagnostic: process-teardown pid=709 reclaimed_frames=438 before_free=236885 before_allocated=24405 after_free=237323 after_allocated=23967
    January 1970
Su Mo Tu We Th Fr Sa
             1  2  3
 4  5  6  7  8  9 10
11 12 13 14 15 16 17
18 19 20 21 22 23 24
25 26 27 28 29 30 31
                     
frame-allocator-diagnostic: process-teardown pid=708 reclaimed_frames=438 before_free=237326 before_allocated=23964 after_free=237764 after_allocated=23526
testcase busybox cal success
frame-allocator-diagnostic: process-teardown pid=711 reclaimed_frames=438 before_free=236875 before_allocated=24415 after_free=237313 after_allocated=23977
frame-allocator-diagnostic: process-teardown pid=710 reclaimed_frames=438 before_free=237316 before_allocated=23974 after_free=237754 after_allocated=23536
testcase busybox clear success
frame-allocator-diagnostic: process-teardown pid=713 reclaimed_frames=438 before_free=236865 before_allocated=24425 after_free=237303 after_allocated=23987
Thu Jan  1 00:02:39 UTC 1970
frame-allocator-diagnostic: process-teardown pid=712 reclaimed_frames=438 before_free=237306 before_allocated=23984 after_free=237744 after_allocated=23546
testcase busybox date success
frame-allocator-diagnostic: process-teardown pid=715 reclaimed_frames=438 before_free=236855 before_allocated=24435 after_free=237293 after_allocated=23997
Filesystem           1K-blocks      Used Available Use% Mounted on
devfs                  1045160     95980    949180   9% /dev
tmpfs                  1045160     95980    949180   9% /tmp
tmpfs                  1045160     95980    949180   9% /var
proc                   1045160     95980    949180   9% /proc
sysfs                  1045160     95980    949180   9% /sys
frame-allocator-diagnostic: process-teardown pid=714 reclaimed_frames=439 before_free=237295 before_allocated=23995 after_free=237734 after_allocated=23556
testcase busybox df success
frame-allocator-diagnostic: process-teardown pid=717 reclaimed_frames=438 before_free=236845 before_allocated=24445 after_free=237283 after_allocated=24007
/aaa
frame-allocator-diagnostic: process-teardown pid=716 reclaimed_frames=438 before_free=237286 before_allocated=24004 after_free=237724 after_allocated=23566
testcase busybox dirname /aaa/bbb success
frame-allocator-diagnostic: process-teardown pid=719 reclaimed_frames=438 before_free=236835 before_allocated=24455 after_free=237273 after_allocated=24017
frame-allocator-diagnostic: process-teardown pid=718 reclaimed_frames=439 before_free=237275 before_allocated=24015 after_free=237714 after_allocated=23576
testcase busybox dmesg success
frame-allocator-diagnostic: process-teardown pid=721 reclaimed_frames=438 before_free=236825 before_allocated=24465 after_free=237263 after_allocated=24027
0	.
frame-allocator-diagnostic: process-teardown pid=720 reclaimed_frames=439 before_free=237265 before_allocated=24025 after_free=237704 after_allocated=23586
testcase busybox du success
frame-allocator-diagnostic: process-teardown pid=723 reclaimed_frames=438 before_free=236815 before_allocated=24475 after_free=237253 after_allocated=24037
2
frame-allocator-diagnostic: process-teardown pid=722 reclaimed_frames=438 before_free=237256 before_allocated=24034 after_free=237694 after_allocated=23596
testcase busybox expr 1 + 1 success
frame-allocator-diagnostic: process-teardown pid=725 reclaimed_frames=438 before_free=236805 before_allocated=24485 after_free=237243 after_allocated=24047
frame-allocator-diagnostic: process-teardown pid=724 reclaimed_frames=438 before_free=237246 before_allocated=24044 after_free=237684 after_allocated=23606
testcase busybox false success
frame-allocator-diagnostic: process-teardown pid=727 reclaimed_frames=438 before_free=236795 before_allocated=24495 after_free=237233 after_allocated=24057
frame-allocator-diagnostic: process-teardown pid=726 reclaimed_frames=438 before_free=237236 before_allocated=24054 after_free=237674 after_allocated=23616
testcase busybox true success
frame-allocator-diagnostic: process-teardown pid=729 reclaimed_frames=438 before_free=236785 before_allocated=24505 after_free=237223 after_allocated=24067
/glibc/ls
frame-allocator-diagnostic: process-teardown pid=728 reclaimed_frames=438 before_free=237226 before_allocated=24064 after_free=237664 after_allocated=23626
testcase busybox which ls success
frame-allocator-diagnostic: process-teardown pid=731 reclaimed_frames=438 before_free=236775 before_allocated=24515 after_free=237213 after_allocated=24077
Linux
frame-allocator-diagnostic: process-teardown pid=730 reclaimed_frames=438 before_free=237216 before_allocated=24074 after_free=237654 after_allocated=23636
testcase busybox uname success
frame-allocator-diagnostic: process-teardown pid=733 reclaimed_frames=438 before_free=236765 before_allocated=24525 after_free=237203 after_allocated=24087
 00:02:56 up 0 min,  0 users,  load average: 0.00, 0.00, 0.00
frame-allocator-diagnostic: process-teardown pid=732 reclaimed_frames=438 before_free=237206 before_allocated=24084 after_free=237644 after_allocated=23646
testcase busybox uptime success
frame-allocator-diagnostic: process-teardown pid=735 reclaimed_frames=438 before_free=236755 before_allocated=24535 after_free=237193 after_allocated=24097
abc
frame-allocator-diagnostic: process-teardown pid=734 reclaimed_frames=438 before_free=237196 before_allocated=24094 after_free=237634 after_allocated=23656
testcase busybox printf "abc\n" success
frame-allocator-diagnostic: process-teardown pid=737 reclaimed_frames=438 before_free=236745 before_allocated=24545 after_free=237183 after_allocated=24107
PID   USER     TIME  COMMAND
frame-allocator-diagnostic: process-teardown pid=736 reclaimed_frames=439 before_free=237185 before_allocated=24105 after_free=237624 after_allocated=23666
testcase busybox ps success
frame-allocator-diagnostic: process-teardown pid=739 reclaimed_frames=438 before_free=236735 before_allocated=24555 after_free=237173 after_allocated=24117
/tmp/testsuite/glibc/busybox
frame-allocator-diagnostic: process-teardown pid=738 reclaimed_frames=438 before_free=237176 before_allocated=24114 after_free=237614 after_allocated=23676
testcase busybox pwd success
frame-allocator-diagnostic: process-teardown pid=741 reclaimed_frames=438 before_free=236725 before_allocated=24565 after_free=237163 after_allocated=24127
              total        used        free      shared  buff/cache   available
Mem:              0           0           0           0           0     1039745
-/+ buffers/cache:            0           0
Swap:             0           0           0
frame-allocator-diagnostic: process-teardown pid=740 reclaimed_frames=438 before_free=237166 before_allocated=24124 after_free=237604 after_allocated=23686
testcase busybox free success
frame-allocator-diagnostic: process-teardown pid=743 reclaimed_frames=438 before_free=236715 before_allocated=24575 after_free=237153 after_allocated=24137
Thu Jan  1 00:03:05 1970  0.000000 seconds
frame-allocator-diagnostic: process-teardown pid=742 reclaimed_frames=438 before_free=237156 before_allocated=24134 after_free=237594 after_allocated=23696
testcase busybox hwclock success
frame-allocator-diagnostic: process-teardown pid=745 reclaimed_frames=438 before_free=236705 before_allocated=24585 after_free=237143 after_allocated=24147
frame-allocator-diagnostic: process-teardown pid=744 reclaimed_frames=438 before_free=236700 before_allocated=24590 after_free=237138 after_allocated=24152
testcase busybox sh -c 'sleep 5' & /glibc/busybox kill $! success
frame-allocator-diagnostic: process-teardown pid=748 reclaimed_frames=438 before_free=236252 before_allocated=25038 after_free=236690 after_allocated=24600
awk
basename
busybox_cmd.txt
busybox_testcode.sh
cat
chmod
cp
cut
date
dirname
echo
expr
find
grep
head
kill
line
ln
ls
mkdir
mktemp
mv
printf
ps
pwd
readlink
rm
rmdir
sed
seq
sh
sleep
sort
tail
tee
touch
tr
true
uname
wc
xargs
frame-allocator-diagnostic: process-teardown pid=747 reclaimed_frames=440 before_free=236691 before_allocated=24599 after_free=237131 after_allocated=24159
testcase busybox ls success
frame-allocator-diagnostic: process-teardown pid=750 reclaimed_frames=438 before_free=236242 before_allocated=25048 after_free=236680 after_allocated=24610
frame-allocator-diagnostic: process-teardown pid=749 reclaimed_frames=438 before_free=236683 before_allocated=24607 after_free=237121 after_allocated=24169
testcase busybox sleep 1 success
frame-allocator-diagnostic: process-teardown pid=752 reclaimed_frames=438 before_free=236232 before_allocated=25058 after_free=236670 after_allocated=24620
frame-allocator-diagnostic: process-teardown pid=746 reclaimed_frames=438 before_free=236670 before_allocated=24620 after_free=237108 after_allocated=24182
#### file opration test
frame-allocator-diagnostic: process-teardown pid=751 reclaimed_frames=438 before_free=237111 before_allocated=24179 after_free=237549 after_allocated=23741
testcase busybox echo "#### file opration test" success
frame-allocator-diagnostic: process-teardown pid=754 reclaimed_frames=438 before_free=236660 before_allocated=24630 after_free=237098 after_allocated=24192
frame-allocator-diagnostic: process-teardown pid=753 reclaimed_frames=438 before_free=237101 before_allocated=24189 after_free=237539 after_allocated=23751
testcase busybox touch test.txt success
frame-allocator-diagnostic: process-teardown pid=756 reclaimed_frames=438 before_free=236650 before_allocated=24640 after_free=237088 after_allocated=24202
frame-allocator-diagnostic: process-teardown pid=755 reclaimed_frames=438 before_free=237091 before_allocated=24199 after_free=237529 after_allocated=23761
testcase busybox echo "hello world" > test.txt success
frame-allocator-diagnostic: process-teardown pid=758 reclaimed_frames=438 before_free=236640 before_allocated=24650 after_free=237078 after_allocated=24212
hello world
frame-allocator-diagnostic: process-teardown pid=757 reclaimed_frames=438 before_free=237081 before_allocated=24209 after_free=237519 after_allocated=23771
testcase busybox cat test.txt success
frame-allocator-diagnostic: process-teardown pid=760 reclaimed_frames=438 before_free=236630 before_allocated=24660 after_free=237068 after_allocated=24222
l
frame-allocator-diagnostic: process-teardown pid=759 reclaimed_frames=438 before_free=237071 before_allocated=24219 after_free=237509 after_allocated=23781
testcase busybox cut -c 3 test.txt success
frame-allocator-diagnostic: process-teardown pid=762 reclaimed_frames=438 before_free=236620 before_allocated=24670 after_free=237058 after_allocated=24232
0000000 062550 066154 020157 067567 066162 005144
0000014
frame-allocator-diagnostic: process-teardown pid=761 reclaimed_frames=438 before_free=237061 before_allocated=24229 after_free=237499 after_allocated=23791
testcase busybox od test.txt success
frame-allocator-diagnostic: process-teardown pid=764 reclaimed_frames=438 before_free=236610 before_allocated=24680 after_free=237048 after_allocated=24242
hello world
frame-allocator-diagnostic: process-teardown pid=763 reclaimed_frames=438 before_free=237051 before_allocated=24239 after_free=237489 after_allocated=23801
testcase busybox head test.txt success
frame-allocator-diagnostic: process-teardown pid=766 reclaimed_frames=438 before_free=236600 before_allocated=24690 after_free=237038 after_allocated=24252
hello world
frame-allocator-diagnostic: process-teardown pid=765 reclaimed_frames=439 before_free=237040 before_allocated=24250 after_free=237479 after_allocated=23811
testcase busybox tail test.txt success
frame-allocator-diagnostic: process-teardown pid=768 reclaimed_frames=438 before_free=236590 before_allocated=24700 after_free=237028 after_allocated=24262
00000000  68 65 6c 6c 6f 20 77 6f  72 6c 64 0a              |hello world.|
0000000c
frame-allocator-diagnostic: process-teardown pid=767 reclaimed_frames=438 before_free=237031 before_allocated=24259 after_free=237469 after_allocated=23821
testcase busybox hexdump -C test.txt success
frame-allocator-diagnostic: process-teardown pid=770 reclaimed_frames=438 before_free=236580 before_allocated=24710 after_free=237018 after_allocated=24272
6f5902ac237024bdd0c176cb93063dc4  test.txt
frame-allocator-diagnostic: process-teardown pid=769 reclaimed_frames=439 before_free=237020 before_allocated=24270 after_free=237459 after_allocated=23831
testcase busybox md5sum test.txt success
frame-allocator-diagnostic: process-teardown pid=772 reclaimed_frames=438 before_free=236570 before_allocated=24720 after_free=237008 after_allocated=24282
frame-allocator-diagnostic: process-teardown pid=771 reclaimed_frames=438 before_free=237011 before_allocated=24279 after_free=237449 after_allocated=23841
testcase busybox echo "ccccccc" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=774 reclaimed_frames=438 before_free=236560 before_allocated=24730 after_free=236998 after_allocated=24292
frame-allocator-diagnostic: process-teardown pid=773 reclaimed_frames=438 before_free=237001 before_allocated=24289 after_free=237439 after_allocated=23851
testcase busybox echo "bbbbbbb" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=776 reclaimed_frames=438 before_free=236550 before_allocated=24740 after_free=236988 after_allocated=24302
frame-allocator-diagnostic: process-teardown pid=775 reclaimed_frames=438 before_free=236991 before_allocated=24299 after_free=237429 after_allocated=23861
testcase busybox echo "aaaaaaa" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=778 reclaimed_frames=438 before_free=236540 before_allocated=24750 after_free=236978 after_allocated=24312
frame-allocator-diagnostic: process-teardown pid=777 reclaimed_frames=438 before_free=236981 before_allocated=24309 after_free=237419 after_allocated=23871
testcase busybox echo "2222222" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=780 reclaimed_frames=438 before_free=236530 before_allocated=24760 after_free=236968 after_allocated=24322
frame-allocator-diagnostic: process-teardown pid=779 reclaimed_frames=438 before_free=236971 before_allocated=24319 after_free=237409 after_allocated=23881
testcase busybox echo "1111111" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=782 reclaimed_frames=438 before_free=236520 before_allocated=24770 after_free=236958 after_allocated=24332
frame-allocator-diagnostic: process-teardown pid=781 reclaimed_frames=438 before_free=236961 before_allocated=24329 after_free=237399 after_allocated=23891
testcase busybox echo "bbbbbbb" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=784 reclaimed_frames=438 before_free=236510 before_allocated=24780 after_free=236948 after_allocated=24342
frame-allocator-diagnostic: process-teardown pid=785 reclaimed_frames=438 before_free=236494 before_allocated=24796 after_free=236932 after_allocated=24358
1111111
2222222
aaaaaaa
bbbbbbb
ccccccc
hello world
frame-allocator-diagnostic: process-teardown pid=786 reclaimed_frames=438 before_free=236500 before_allocated=24790 after_free=236938 after_allocated=24352
frame-allocator-diagnostic: process-teardown pid=783 reclaimed_frames=441 before_free=236938 before_allocated=24352 after_free=237379 after_allocated=23911
testcase busybox sort test.txt | /glibc/busybox uniq success
frame-allocator-diagnostic: process-teardown pid=788 reclaimed_frames=438 before_free=236490 before_allocated=24800 after_free=236928 after_allocated=24362
  File: test.txt
  Size: 60        	Blocks: 0          IO Block: 512    regular file
Device: 1h/1d	Inode: 4368043057645409086  Links: 1
Access: (0666/-rw-rw-rw-)  Uid: (    0/    root)   Gid: (    0/    root)
Access: 1970-01-01 00:00:00.000000000 +0000
Modify: 1970-01-01 00:00:00.000000000 +0000
Change: 1970-01-01 00:00:00.000000000 +0000
frame-allocator-diagnostic: process-teardown pid=787 reclaimed_frames=438 before_free=236931 before_allocated=24359 after_free=237369 after_allocated=23921
testcase busybox stat test.txt success
frame-allocator-diagnostic: process-teardown pid=790 reclaimed_frames=438 before_free=236480 before_allocated=24810 after_free=236918 after_allocated=24372
hello world
ccccccc
bbbbbbb
aaaaaaa
2222222
1111111
bbbbbbb
frame-allocator-diagnostic: process-teardown pid=789 reclaimed_frames=438 before_free=236921 before_allocated=24369 after_free=237359 after_allocated=23931
testcase busybox strings test.txt success
frame-allocator-diagnostic: process-teardown pid=792 reclaimed_frames=438 before_free=236470 before_allocated=24820 after_free=236908 after_allocated=24382
        7         8        60 test.txt
frame-allocator-diagnostic: process-teardown pid=791 reclaimed_frames=438 before_free=236911 before_allocated=24379 after_free=237349 after_allocated=23941
testcase busybox wc test.txt success
frame-allocator-diagnostic: process-teardown pid=794 reclaimed_frames=438 before_free=236460 before_allocated=24830 after_free=236898 after_allocated=24392
frame-allocator-diagnostic: process-teardown pid=793 reclaimed_frames=438 before_free=236901 before_allocated=24389 after_free=237339 after_allocated=23951
testcase busybox [ -f test.txt ] success
frame-allocator-diagnostic: process-teardown pid=796 reclaimed_frames=438 before_free=236450 before_allocated=24840 after_free=236888 after_allocated=24402
hello world
ccccccc
bbbbbbb
aaaaaaa
2222222
1111111
bbbbbbb
frame-allocator-diagnostic: process-teardown pid=795 reclaimed_frames=438 before_free=236891 before_allocated=24399 after_free=237329 after_allocated=23961
testcase busybox more test.txt success
frame-allocator-diagnostic: process-teardown pid=798 reclaimed_frames=438 before_free=236440 before_allocated=24850 after_free=236878 after_allocated=24412
frame-allocator-diagnostic: process-teardown pid=797 reclaimed_frames=438 before_free=236881 before_allocated=24409 after_free=237319 after_allocated=23971
testcase busybox rm test.txt success
frame-allocator-diagnostic: process-teardown pid=800 reclaimed_frames=438 before_free=236430 before_allocated=24860 after_free=236868 after_allocated=24422
frame-allocator-diagnostic: process-teardown pid=799 reclaimed_frames=438 before_free=236871 before_allocated=24419 after_free=237309 after_allocated=23981
testcase busybox mkdir test_dir success
frame-allocator-diagnostic: process-teardown pid=802 reclaimed_frames=438 before_free=236420 before_allocated=24870 after_free=236858 after_allocated=24432
frame-allocator-diagnostic: process-teardown pid=801 reclaimed_frames=438 before_free=236861 before_allocated=24429 after_free=237299 after_allocated=23991
testcase busybox mv test_dir test success
frame-allocator-diagnostic: process-teardown pid=804 reclaimed_frames=438 before_free=236410 before_allocated=24880 after_free=236848 after_allocated=24442
frame-allocator-diagnostic: process-teardown pid=803 reclaimed_frames=438 before_free=236851 before_allocated=24439 after_free=237289 after_allocated=24001
testcase busybox rmdir test success
frame-allocator-diagnostic: process-teardown pid=806 reclaimed_frames=438 before_free=236400 before_allocated=24890 after_free=236838 after_allocated=24452
echo "hello world" > test.txt
grep hello busybox_cmd.txt
frame-allocator-diagnostic: process-teardown pid=805 reclaimed_frames=446 before_free=236833 before_allocated=24457 after_free=237279 after_allocated=24011
testcase busybox grep hello busybox_cmd.txt success
frame-allocator-diagnostic: process-teardown pid=808 reclaimed_frames=438 before_free=236390 before_allocated=24900 after_free=236828 after_allocated=24462
frame-allocator-diagnostic: process-teardown pid=807 reclaimed_frames=438 before_free=236831 before_allocated=24459 after_free=237269 after_allocated=24021
testcase busybox cp busybox_cmd.txt busybox_cmd.bak success
frame-allocator-diagnostic: process-teardown pid=810 reclaimed_frames=438 before_free=236380 before_allocated=24910 after_free=236818 after_allocated=24472
frame-allocator-diagnostic: process-teardown pid=809 reclaimed_frames=438 before_free=236821 before_allocated=24469 after_free=237259 after_allocated=24031
testcase busybox rm busybox_cmd.bak success
frame-allocator-diagnostic: process-teardown pid=812 reclaimed_frames=438 before_free=236370 before_allocated=24920 after_free=236808 after_allocated=24482
./busybox_cmd.txt
frame-allocator-diagnostic: process-teardown pid=811 reclaimed_frames=439 before_free=236810 before_allocated=24480 after_free=237249 after_allocated=24041
testcase busybox find -name "busybox_cmd.txt" success
#### OS COMP TEST GROUP END busybox-glibc ####
frame-allocator-diagnostic: process-teardown pid=814 reclaimed_frames=438 before_free=236360 before_allocated=24930 after_free=236798 after_allocated=24492
#### OS COMP TEST GROUP START cyclictest-glibc ####
frame-allocator-diagnostic: process-teardown pid=815 reclaimed_frames=438 before_free=236357 before_allocated=24933 after_free=236795 after_allocated=24495
====== cyclictest NO_STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 (  817) P:99 I:1000 C:      8 Min:     25 Act:182091 Avg:126315 Max:  215407
frame-allocator-diagnostic: process-teardown pid=816 reclaimed_frames=200 before_free=236587 before_allocated=24703 after_free=236787 after_allocated=24503
====== cyclictest NO_STRESS_P1 end: success ======
====== cyclictest NO_STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 (  819) P:99 I:1000 C:      7 Min: 121406 Act:184355 Avg:148998 Max:  244576
T: 1 (  820) P:99 I:1500 C:      7 Min: 120715 Act:185454 Avg:148950 Max:  244062
T: 2 (  821) P:99 I:2000 C:      7 Min: 121164 Act:183959 Avg:148479 Max:  243513
T: 3 (  822) P:99 I:2500 C:      7 Min: 121115 Act:185450 Avg:148436 Max:  242964
T: 4 (  823) P:99 I:3000 C:      7 Min: 121047 Act:184007 Avg:148405 Max:  242419
T: 5 (  824) P:99 I:3500 C:      7 Min: 119998 Act:183505 Avg:147077 Max:  241870
T: 6 (  825) P:99 I:4000 C:      7 Min: 118971 Act:183995 Avg:147462 Max:  241320
T: 7 (  826) P:99 I:4500 C:      7 Min: 119380 Act:183979 Avg:146985 Max:  240767
frame-allocator-diagnostic: process-teardown pid=818 reclaimed_frames=207 before_free=236565 before_allocated=24725 after_free=236772 after_allocated=24518
====== cyclictest NO_STRESS_P8 end: success ======
====== start hackbench ======
Running in process mode with 10 groups using 40 file descriptors each (== 400 tasks)
Each sender will pass 100000000 messages of 100 bytes
frame-allocator-diagnostic: process-teardown pid=828 reclaimed_frames=438 before_free=213970 before_allocated=47320 after_free=214408 after_allocated=46882
====== cyclictest STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 1230) P:99 I:1000 C:      4 Min: 250074 Act:252606 Avg:286392 Max:  383569
frame-allocator-diagnostic: process-teardown pid=1229 reclaimed_frames=200 before_free=214200 before_allocated=47090 after_free=214400 after_allocated=46890
====== cyclictest STRESS_P1 end: success ======
====== cyclictest STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 1232) P:99 I:1000 C:      4 Min: 253930 Act:277475 Avg:293345 Max:  387562
T: 1 ( 1233) P:99 I:1500 C:      4 Min: 253362 Act:277365 Avg:293053 Max:  387489
T: 2 ( 1234) P:99 I:2000 C:      4 Min: 253476 Act:277864 Avg:293164 Max:  387479
T: 3 ( 1235) P:99 I:2500 C:      4 Min: 252329 Act:278357 Avg:292777 Max:  387469
T: 4 ( 1236) P:99 I:3000 C:      4 Min: 251807 Act:275848 Avg:291885 Max:  387457
T: 5 ( 1237) P:99 I:3500 C:      4 Min: 251909 Act:276339 Avg:292244 Max:  386446
T: 6 ( 1238) P:99 I:4000 C:      4 Min: 251386 Act:275830 Avg:291603 Max:  385434
T: 7 ( 1239) P:99 I:4500 C:      4 Min: 250863 Act:275827 Avg:291088 Max:  384424
frame-allocator-diagnostic: process-teardown pid=1231 reclaimed_frames=207 before_free=214178 before_allocated=47112 after_free=214385 after_allocated=46905
====== cyclictest STRESS_P8 end: success ======
frame-allocator-diagnostic: process-teardown pid=1240 reclaimed_frames=438 before_free=213942 before_allocated=47348 after_free=214380 after_allocated=46910
frame-allocator-diagnostic: process-teardown pid=1241 reclaimed_frames=438 before_free=213937 before_allocated=47353 after_free=214375 after_allocated=46915
====== kill hackbench: success ======
#### OS COMP TEST GROUP END cyclictest-glibc ####
frame-allocator-diagnostic: process-teardown pid=1242 reclaimed_frames=438 before_free=213932 before_allocated=47358 after_free=214370 after_allocated=46920
frame-allocator-diagnostic: process-teardown pid=813 reclaimed_frames=439 before_free=214370 before_allocated=46920 after_free=214809 after_allocated=46481
#### OS COMP TEST GROUP START iozone-glibc ####
SKIP: iozone throughput mode currently hangs in the evaluator environment
#### OS COMP TEST GROUP END iozone-glibc ####
frame-allocator-diagnostic: process-teardown pid=1244 reclaimed_frames=438 before_free=213921 before_allocated=47369 after_free=214359 after_allocated=46931
#### OS COMP TEST GROUP START iperf-glibc ####
frame-allocator-diagnostic: process-teardown pid=1245 reclaimed_frames=438 before_free=213917 before_allocated=47373 after_free=214355 after_allocated=46935
frame-allocator-diagnostic: process-teardown pid=1246 reclaimed_frames=2 before_free=214159 before_allocated=47131 after_free=214161 after_allocated=47129
frame-allocator-diagnostic: process-teardown pid=1247 reclaimed_frames=186 before_free=214159 before_allocated=47131 after_free=214345 after_allocated=46945
====== iperf BASIC_UDP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 0.0.0.0 port 49193 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  5]   0.00-2.42   sec  5.70 KBytes  19.3 Kbits/sec  4  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.42   sec  5.70 KBytes  19.3 Kbits/sec  0.000 ms  0/4 (0%)  sender
[  5]   0.00-3.24   sec  5.70 KBytes  14.4 Kbits/sec  13.856 ms  0/4 (0%)  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=1248 reclaimed_frames=185 before_free=214151 before_allocated=47139 after_free=214336 after_allocated=46954
====== iperf BASIC_UDP end: success ======

====== iperf BASIC_TCP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 127.0.0.1 port 49177 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.37   sec  1.75 MBytes  6.18 Mbits/sec    0   0.00 Bytes       
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.37   sec  1.75 MBytes  6.18 Mbits/sec    0             sender
[  5]   0.00-3.42   sec   896 KBytes  2.15 Mbits/sec                  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=1249 reclaimed_frames=185 before_free=214113 before_allocated=47177 after_free=214298 after_allocated=46992
====== iperf BASIC_TCP end: success ======

====== iperf PARALLEL_UDP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 0.0.0.0 port 49194 connected to 127.0.0.1 port 5001
[  7] local 0.0.0.0 port 49195 connected to 127.0.0.1 port 5001
[  9] local 0.0.0.0 port 49196 connected to 127.0.0.1 port 5001
[ 11] local 0.0.0.0 port 49197 connected to 127.0.0.1 port 5001
[ 13] local 0.0.0.0 port 49198 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  5]   0.00-3.69   sec  2.85 KBytes  6.33 Kbits/sec  2  
[  7]   0.00-3.69   sec  2.85 KBytes  6.33 Kbits/sec  2  
[  9]   0.00-3.69   sec  2.85 KBytes  6.33 Kbits/sec  2  
[ 11]   0.00-3.69   sec  2.85 KBytes  6.33 Kbits/sec  2  
[ 13]   0.00-3.69   sec  2.85 KBytes  6.33 Kbits/sec  2  
[SUM]   0.00-3.69   sec  14.3 KBytes  31.7 Kbits/sec  10  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-3.69   sec  2.85 KBytes  6.33 Kbits/sec  0.000 ms  0/2 (0%)  sender
[  5]   0.00-4.46   sec  2.85 KBytes  5.24 Kbits/sec  3.404 ms  0/2 (0%)  receiver
[  7]   0.00-3.69   sec  2.85 KBytes  6.33 Kbits/sec  0.000 ms  0/2 (0%)  sender
[  7]   0.00-4.46   sec  2.85 KBytes  5.24 Kbits/sec  0.293 ms  0/2 (0%)  receiver
[  9]   0.00-3.69   sec  2.85 KBytes  6.33 Kbits/sec  0.000 ms  0/2 (0%)  sender
[  9]   0.00-4.46   sec  2.85 KBytes  5.24 Kbits/sec  8.478 ms  0/2 (0%)  receiver
[ 11]   0.00-3.69   sec  2.85 KBytes  6.33 Kbits/sec  0.000 ms  0/2 (0%)  sender
[ 11]   0.00-4.46   sec  2.85 KBytes  5.24 Kbits/sec  0.875 ms  0/2 (0%)  receiver
[ 13]   0.00-3.69   sec  2.85 KBytes  6.33 Kbits/sec  0.000 ms  0/2 (0%)  sender
[ 13]   0.00-4.46   sec  2.85 KBytes  5.24 Kbits/sec  1.683 ms  0/2 (0%)  receiver
[SUM]   0.00-3.69   sec  14.3 KBytes  31.7 Kbits/sec  0.000 ms  0/10 (0%)  sender
[SUM]   0.00-4.46   sec  14.3 KBytes  26.2 Kbits/sec  2.947 ms  0/10 (0%)  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=1250 reclaimed_frames=188 before_free=214129 before_allocated=47161 after_free=214317 after_allocated=46973
====== iperf PARALLEL_UDP end: success ======

====== iperf PARALLEL_TCP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 127.0.0.1 port 49180 connected to 127.0.0.1 port 5001
[  7] local 127.0.0.1 port 49181 connected to 127.0.0.1 port 5001
[  9] local 127.0.0.1 port 49182 connected to 127.0.0.1 port 5001
[ 11] local 127.0.0.1 port 49183 connected to 127.0.0.1 port 5001
[ 13] local 127.0.0.1 port 49184 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.58   sec  1.75 MBytes  5.68 Mbits/sec    0   0.00 Bytes       
[  7]   0.00-2.58   sec  1.75 MBytes  5.68 Mbits/sec    0   0.00 Bytes       
[  9]   0.00-2.58   sec  1.75 MBytes  5.68 Mbits/sec    0   0.00 Bytes       
[ 11]   0.00-2.58   sec  1.75 MBytes  5.68 Mbits/sec    0   0.00 Bytes       
[ 13]   0.00-2.58   sec  1.75 MBytes  5.68 Mbits/sec    0   0.00 Bytes       
[SUM]   0.00-2.58   sec  8.75 MBytes  28.4 Mbits/sec    0             
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.58   sec  1.75 MBytes  5.68 Mbits/sec    0             sender
[  5]   0.00-3.43   sec   896 KBytes  2.14 Mbits/sec                  receiver
[  7]   0.00-2.58   sec  1.75 MBytes  5.68 Mbits/sec    0             sender
[  7]   0.00-3.43   sec   896 KBytes  2.14 Mbits/sec                  receiver
[  9]   0.00-2.58   sec  1.75 MBytes  5.68 Mbits/sec    0             sender
[  9]   0.00-3.43   sec   896 KBytes  2.14 Mbits/sec                  receiver
[ 11]   0.00-2.58   sec  1.75 MBytes  5.68 Mbits/sec    0             sender
[ 11]   0.00-3.43   sec   896 KBytes  2.14 Mbits/sec                  receiver
[ 13]   0.00-2.58   sec  1.75 MBytes  5.68 Mbits/sec    0             sender
[ 13]   0.00-3.43   sec   896 KBytes  2.14 Mbits/sec                  receiver
[SUM]   0.00-2.58   sec  8.75 MBytes  28.4 Mbits/sec    0             sender
[SUM]   0.00-3.43   sec  4.38 MBytes  10.7 Mbits/sec                  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=1251 reclaimed_frames=188 before_free=213967 before_allocated=47323 after_free=214155 after_allocated=47135
====== iperf PARALLEL_TCP end: success ======

====== iperf REVERSE_UDP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 0.0.0.0 port 49199 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.18   sec  5.70 KBytes  21.5 Kbits/sec  16.492 ms  0/4 (0%)  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-3.27   sec  7.13 KBytes  17.8 Kbits/sec  0.000 ms  0/5 (0%)  sender
[  5]   0.00-2.18   sec  5.70 KBytes  21.5 Kbits/sec  16.492 ms  0/4 (0%)  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=1252 reclaimed_frames=185 before_free=214122 before_allocated=47168 after_free=214307 after_allocated=46983
====== iperf REVERSE_UDP end: success ======

====== iperf REVERSE_TCP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 127.0.0.1 port 49187 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.12   sec   896 KBytes  3.46 Mbits/sec                  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-3.04   sec  1.75 MBytes  4.83 Mbits/sec    0             sender
[  5]   0.00-2.12   sec   896 KBytes  3.46 Mbits/sec                  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=1253 reclaimed_frames=185 before_free=214084 before_allocated=47206 after_free=214269 after_allocated=47021
====== iperf REVERSE_TCP end: success ======

#### OS COMP TEST GROUP END iperf-glibc ####
frame-allocator-diagnostic: process-teardown pid=1254 reclaimed_frames=438 before_free=213858 before_allocated=47432 after_free=214296 after_allocated=46994
frame-allocator-diagnostic: process-teardown pid=1243 reclaimed_frames=440 before_free=214296 before_allocated=46994 after_free=214736 after_allocated=46554
#### OS COMP TEST GROUP START libcbench-glibc ####
SKIP: libcbench currently triggers an unrecovered allocator exhaustion path
#### OS COMP TEST GROUP END libcbench-glibc ####
#### OS COMP TEST GROUP START libctest-glibc ####
SKIP: libctest still trips unresolved pthread cancellation paths
#### OS COMP TEST GROUP END libctest-glibc ####
#### OS COMP TEST GROUP START lmbench-glibc ####
SKIP: lmbench still triggers an unresolved user-space page-fault path
#### OS COMP TEST GROUP END lmbench-glibc ####
#### OS COMP TEST GROUP START ltp-glibc ####
SKIP: full LTP sweep is too large for the boot-time evaluator smoke run
#### OS COMP TEST GROUP END ltp-glibc ####
frame-allocator-diagnostic: process-teardown pid=1256 reclaimed_frames=438 before_free=213848 before_allocated=47442 after_free=214286 after_allocated=47004
#### OS COMP TEST GROUP START lua-glibc ####
frame-allocator-diagnostic: process-teardown pid=1257 reclaimed_frames=438 before_free=213844 before_allocated=47446 after_free=214282 after_allocated=47008
frame-allocator-diagnostic: process-teardown pid=1259 reclaimed_frames=201 before_free=213632 before_allocated=47658 after_free=213833 after_allocated=47457
testcase lua date.lua success
frame-allocator-diagnostic: process-teardown pid=1258 reclaimed_frames=439 before_free=213833 before_allocated=47457 after_free=214272 after_allocated=47018
frame-allocator-diagnostic: process-teardown pid=1261 reclaimed_frames=201 before_free=213622 before_allocated=47668 after_free=213823 after_allocated=47467
testcase lua file_io.lua success
frame-allocator-diagnostic: process-teardown pid=1260 reclaimed_frames=439 before_free=213823 before_allocated=47467 after_free=214262 after_allocated=47028
frame-allocator-diagnostic: process-teardown pid=1263 reclaimed_frames=201 before_free=213612 before_allocated=47678 after_free=213813 after_allocated=47477
testcase lua max_min.lua success
frame-allocator-diagnostic: process-teardown pid=1262 reclaimed_frames=439 before_free=213813 before_allocated=47477 after_free=214252 after_allocated=47038
frame-allocator-diagnostic: process-teardown pid=1265 reclaimed_frames=201 before_free=213602 before_allocated=47688 after_free=213803 after_allocated=47487
testcase lua random.lua success
frame-allocator-diagnostic: process-teardown pid=1264 reclaimed_frames=439 before_free=213803 before_allocated=47487 after_free=214242 after_allocated=47048
frame-allocator-diagnostic: process-teardown pid=1267 reclaimed_frames=201 before_free=213592 before_allocated=47698 after_free=213793 after_allocated=47497
testcase lua remove.lua success
frame-allocator-diagnostic: process-teardown pid=1266 reclaimed_frames=439 before_free=213793 before_allocated=47497 after_free=214232 after_allocated=47058
frame-allocator-diagnostic: process-teardown pid=1269 reclaimed_frames=201 before_free=213582 before_allocated=47708 after_free=213783 after_allocated=47507
testcase lua round_num.lua success
frame-allocator-diagnostic: process-teardown pid=1268 reclaimed_frames=439 before_free=213783 before_allocated=47507 after_free=214222 after_allocated=47068
frame-allocator-diagnostic: process-teardown pid=1271 reclaimed_frames=201 before_free=213572 before_allocated=47718 after_free=213773 after_allocated=47517
testcase lua sin30.lua success
frame-allocator-diagnostic: process-teardown pid=1270 reclaimed_frames=439 before_free=213773 before_allocated=47517 after_free=214212 after_allocated=47078
frame-allocator-diagnostic: process-teardown pid=1273 reclaimed_frames=201 before_free=213562 before_allocated=47728 after_free=213763 after_allocated=47527
testcase lua sort.lua success
frame-allocator-diagnostic: process-teardown pid=1272 reclaimed_frames=439 before_free=213763 before_allocated=47527 after_free=214202 after_allocated=47088
frame-allocator-diagnostic: process-teardown pid=1275 reclaimed_frames=201 before_free=213552 before_allocated=47738 after_free=213753 after_allocated=47537
testcase lua strings.lua success
frame-allocator-diagnostic: process-teardown pid=1274 reclaimed_frames=439 before_free=213753 before_allocated=47537 after_free=214192 after_allocated=47098
#### OS COMP TEST GROUP END lua-glibc ####
frame-allocator-diagnostic: process-teardown pid=1276 reclaimed_frames=438 before_free=213749 before_allocated=47541 after_free=214187 after_allocated=47103
frame-allocator-diagnostic: process-teardown pid=1255 reclaimed_frames=439 before_free=214187 before_allocated=47103 after_free=214626 after_allocated=46664
frame-allocator-diagnostic: process-teardown pid=1278 reclaimed_frames=438 before_free=213737 before_allocated=47553 after_free=214175 after_allocated=47115
#### OS COMP TEST GROUP START netperf-glibc ####
frame-allocator-diagnostic: process-teardown pid=1279 reclaimed_frames=438 before_free=213734 before_allocated=47556 after_free=214172 after_allocated=47118
====== netperf UDP_STREAM begin ======
Starting netserver with host '127.0.0.1' port '12865' and family AF_UNSPEC
MIGRATED UDP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET
enable_enobufs failed: getprotobyname
Socket  Message  Elapsed      Messages                
Size    Size     Time         Okay Errors   Throughput
bytes   bytes    secs            #      #   10^6bits/sec

 65536    1000   2.06            4      0       0.02
 65536           2.06            4              0.02

frame-allocator-diagnostic: process-teardown pid=1282 reclaimed_frames=42 before_free=213031 before_allocated=48259 after_free=213073 after_allocated=48217
frame-allocator-diagnostic: process-teardown pid=1281 reclaimed_frames=549 before_free=213073 before_allocated=48217 after_free=213622 after_allocated=47668
====== netperf UDP_STREAM end: success ======
====== netperf TCP_STREAM begin ======
frame-allocator-diagnostic: process-teardown pid=1286 reclaimed_frames=42 before_free=213018 before_allocated=48272 after_free=213060 after_allocated=48230
MIGRATED TCP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET
Recv   Send    Send                          
Socket Socket  Message  Elapsed              
Size   Size    Size     Time     Throughput  
bytes  bytes   bytes    secs.    10^6bits/sec  

 65536  65536   1000    0.30       27.73   
catcher: timer popped with times_up != 0
frame-allocator-diagnostic: process-teardown pid=1285 reclaimed_frames=548 before_free=213060 before_allocated=48230 after_free=213608 after_allocated=47682
====== netperf TCP_STREAM end: success ======
====== netperf UDP_RR begin ======
MIGRATED UDP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      2.05        1.95   
65536  65536 
frame-allocator-diagnostic: process-teardown pid=1290 reclaimed_frames=24 before_free=213040 before_allocated=48250 after_free=213064 after_allocated=48226
frame-allocator-diagnostic: process-teardown pid=1289 reclaimed_frames=530 before_free=213064 before_allocated=48226 after_free=213594 after_allocated=47696
====== netperf UDP_RR end: success ======
====== netperf TCP_RR begin ======
frame-allocator-diagnostic: process-teardown pid=1294 reclaimed_frames=24 before_free=213026 before_allocated=48264 after_free=213050 after_allocated=48240
MIGRATED TCP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.59        2.52   
65536  65536 
frame-allocator-diagnostic: process-teardown pid=1293 reclaimed_frames=530 before_free=213050 before_allocated=48240 after_free=213580 after_allocated=47710
====== netperf TCP_RR end: success ======
====== netperf TCP_CRR begin ======
frame-allocator-diagnostic: process-teardown pid=1298 reclaimed_frames=24 before_free=213012 before_allocated=48278 after_free=213036 after_allocated=48254
MIGRATED TCP Connect/Request/Response TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.18        3.38   
65536  65536 
frame-allocator-diagnostic: process-teardown pid=1297 reclaimed_frames=530 before_free=213036 before_allocated=48254 after_free=213566 after_allocated=47724
====== netperf TCP_CRR end: success ======
frame-allocator-diagnostic: process-teardown pid=1301 reclaimed_frames=438 before_free=213123 before_allocated=48167 after_free=213561 after_allocated=47729
frame-allocator-diagnostic: process-teardown pid=1280 reclaimed_frames=529 before_free=213561 before_allocated=47729 after_free=214090 after_allocated=47200
#### OS COMP TEST GROUP END netperf-glibc ####
frame-allocator-diagnostic: process-teardown pid=1302 reclaimed_frames=438 before_free=213647 before_allocated=47643 after_free=214085 after_allocated=47205
frame-allocator-diagnostic: process-teardown pid=1277 reclaimed_frames=439 before_free=214085 before_allocated=47205 after_free=214524 after_allocated=46766
#### OS COMP TEST GROUP START unixbench-glibc ####
SKIP: unixbench currently blocks on unresolved executable/runtime compatibility
#### OS COMP TEST GROUP END unixbench-glibc ####
[386.548539 0:2 axplat_riscv64_qemu_virt::power:28] Shutting down...
===== LTP riscv64 evaluation end: 2026-05-19T12:32:42+08:00 status=0 =====
```
