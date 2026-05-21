# loongarch64 LTP/OS COMP evaluation output
- Command: `./run-eval la`
- Exit status: `0`
- Start: `2026-05-19T12:32:55+08:00`
- End: `2026-05-19T12:42:47+08:00`
- Raw log: `eval-reports/ltp-eval-20260519-122555/la.raw.log`
- ANSI-stripped log: `eval-reports/ltp-eval-20260519-122555/la.clean.log`

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
===== LTP loongarch64 evaluation start: 2026-05-19T12:32:55+08:00 =====
cwd=/root/oskernel2026-orays
command=./run-eval la
make test_build ARCH=loongarch64 BUS=pci \
	KERNEL_FEATURES="alloc,paging,irq,multitask,fs,net" \
	APP_FEATURES="auto-run-tests,uspace" \
	AXCONFIG_WRITES="-w plat.phys-memory-size=0x3000_0000" \
	OUT_DIR=/root/oskernel2026-orays/build/kernels/loongarch64 \
	OUT_CONFIG=/root/oskernel2026-orays/build/kernels/loongarch64.axconfig.toml \
	TARGET_DIR=/root/oskernel2026-orays/build/kernels/target/loongarch64
make[1]: Entering directory '/root/oskernel2026-orays'
make A=examples/shell MODE=release LOG=info SMP=1 FEATURES=alloc,paging,irq,multitask,fs,net \
	ARCH=loongarch64 BUS=pci \
	APP_FEATURES="auto-run-tests,uspace" \
	AXCONFIG_WRITES="-w plat.phys-memory-size=0x3000_0000" \
	OUT_DIR=/root/oskernel2026-orays/build/kernels/loongarch64 \
	OUT_CONFIG=/root/oskernel2026-orays/build/kernels/loongarch64.axconfig.toml \
	TARGET_DIR=/root/oskernel2026-orays/build/kernels/target/loongarch64 \
	build
make[2]: Entering directory '/root/oskernel2026-orays'
axconfig-gen configs/defconfig.toml /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/axplat-loongarch64-qemu-virt-0.4.2/axconfig.toml  -w arch=loongarch64 -w platform=loongarch64-qemu-virt -o "/root/oskernel2026-orays/build/kernels/loongarch64.axconfig.toml" -w plat.phys-memory-size=0x3000_0000 -w plat.max-cpu-num=1 -c "/root/oskernel2026-orays/build/kernels/loongarch64.axconfig.toml"
    Building App: shell, Arch: loongarch64, Platform: loongarch64-qemu-virt, App type: rust
cargo -C examples/shell build -Z unstable-options --target loongarch64-unknown-none-softfloat --target-dir /root/oskernel2026-orays/build/kernels/target/loongarch64 --release  --features "axstd/defplat axstd/log-level-info axstd/alloc axstd/paging axstd/irq axstd/multitask axstd/fs axstd/net auto-run-tests uspace"
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
warning: arceos_posix_api@0.2.0: using checked-in src/ctypes_gen.rs; libclang may not support target loongarch64-unknown-none-softfloat
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
    Finished `release` profile [optimized] target(s) in 18.76s
rust-objcopy --binary-architecture=loongarch64 /root/oskernel2026-orays/build/kernels/loongarch64/shell_loongarch64-qemu-virt.elf --strip-all -O binary /root/oskernel2026-orays/build/kernels/loongarch64/shell_loongarch64-qemu-virt.bin
make[2]: Leaving directory '/root/oskernel2026-orays'
cp /root/oskernel2026-orays/build/kernels/loongarch64/shell_loongarch64-qemu-virt.elf /root/oskernel2026-orays/kernel-la
make[1]: Leaving directory '/root/oskernel2026-orays'
rm -f /tmp/arceos-sdcard-la.run.qcow2
qemu-img create -f qcow2 -F raw -b /root/oskernel2026-orays/sdcard-la.img /tmp/arceos-sdcard-la.run.qcow2
Formatting '/tmp/arceos-sdcard-la.run.qcow2', fmt=qcow2 cluster_size=65536 extended_l2=off compression_type=zlib size=4294967296 backing_file=/root/oskernel2026-orays/sdcard-la.img backing_fmt=raw lazy_refcounts=off refcount_bits=16
qemu-system-loongarch64 -kernel /root/oskernel2026-orays/kernel-la -m 1G -nographic -smp 1 -drive file=/tmp/arceos-sdcard-la.run.qcow2,if=none,format=qcow2,id=x0 \
	-device virtio-blk-pci,drive=x0,bus=pcie.0 -no-reboot -device virtio-net-pci,netdev=net0 \
	-netdev user,id=net0 -rtc base=utc 

       d8888                            .d88888b.   .d8888b.
      d88888                           d88P" "Y88b d88P  Y88b
     d88P888                           888     888 Y88b.
    d88P 888 888d888  .d8888b  .d88b.  888     888  "Y888b.
   d88P  888 888P"   d88P"    d8P  Y8b 888     888     "Y88b.
  d88P   888 888     888      88888888 888     888       "888
 d8888888888 888     Y88b.    Y8b.     Y88b. .d88P Y88b  d88P
d88P     888 888      "Y8888P  "Y8888   "Y88888P"   "Y8888P"

arch = loongarch64
platform = loongarch64-qemu-virt
target = loongarch64-unknown-none-softfloat
build_mode = release
log_level = info

[  0.004518 0 axruntime:135] Logging is enabled.
[  0.006067 0 axruntime:136] Primary CPU 0 started, arg = 0x0.
[  0.007323 0 axruntime:139] Found physcial memory regions:
[  0.007835 0 axruntime:141]   [PA:0x100d0000, PA:0x100d1000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.008509 0 axruntime:141]   [PA:0x100e0000, PA:0x100e1000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.008946 0 axruntime:141]   [PA:0x1fe00000, PA:0x1fe01000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.009371 0 axruntime:141]   [PA:0x20000000, PA:0x30000000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.009829 0 axruntime:141]   [PA:0x40000000, PA:0x40020000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.010248 0 axruntime:141]   [PA:0x80000000, PA:0x800fd000) .text (READ | EXECUTE | RESERVED)
[  0.010632 0 axruntime:141]   [PA:0x800fd000, PA:0x80126000) .rodata (READ | RESERVED)
[  0.010981 0 axruntime:141]   [PA:0x80126000, PA:0x8012b000) .data .tdata .tbss .percpu (READ | WRITE | RESERVED)
[  0.011421 0 axruntime:141]   [PA:0x8012b000, PA:0x8016b000) boot stack (READ | WRITE | RESERVED)
[  0.011813 0 axruntime:141]   [PA:0x8016b000, PA:0x80192000) .bss (READ | WRITE | RESERVED)
[  0.012233 0 axruntime:141]   [PA:0x80192000, PA:0xb0000000) free memory (READ | WRITE | FREE)
[  0.012766 0 axruntime:216] Initialize global memory allocator...
[  0.013180 0 axruntime:217]   use TLSF allocator.
[  0.015709 0 axmm:103] Initialize virtual memory management...
[  0.049332 0 axruntime:156] Initialize platform devices...
smp = 1
[  0.050042 0 axtask::api:73] Initialize scheduling...
[  0.052060 0 axtask::api:83]   use FIFO scheduler.
[  0.052482 0 axdriver:152] Initialize device drivers...
[  0.052873 0 axdriver:153]   device model: static
[  0.059755 0 virtio_drivers::device::blk:63] found a block device of size 4194304KB
[  0.061344 0 axdriver::bus::pci:107] registered a new Block device at 00:01.0: "virtio-blk"
[  0.069996 0 virtio_drivers::device::net::dev_raw:33] negotiated_features Features(MAC | STATUS | RING_INDIRECT_DESC | RING_EVENT_IDX | VERSION_1)
[  0.084959 0 axdriver::bus::pci:107] registered a new Net device at 00:02.0: "virtio-net"
[  0.132406 0 axfs:44] Initialize filesystems...
[  0.133457 0 axfs:47]   use block device 0: "virtio-blk"
[  0.137224 0 axfs::root:336]   detected root filesystem: Ext4
[  0.160803 0 axnet:42] Initialize network subsystem...
[  0.161254 0 axnet:45]   use NIC 0: "virtio-net"
[  0.165016 0 axnet::smoltcp_impl:335] created net interface "eth0":
[  0.165475 0 axnet::smoltcp_impl:336]   ether:    52-54-00-12-34-56
[  0.166094 0 axnet::smoltcp_impl:337]   ip:       10.0.2.15/24
[  0.166718 0 axnet::smoltcp_impl:338]   gateway:  10.0.2.2
[  0.167133 0 axruntime:182] Initialize interrupt handlers...
[  0.168344 0 axruntime:194] Primary CPU 0 init OK.
frame-allocator-diagnostic: process-teardown pid=5 reclaimed_frames=509 before_free=191371 before_allocated=4835 after_free=191880 after_allocated=4326
#### OS COMP TEST GROUP START basic-musl ####
frame-allocator-diagnostic: process-teardown pid=7 reclaimed_frames=509 before_free=190843 before_allocated=5363 after_free=191352 after_allocated=4854
Testing brk :
========== START test_brk ==========
Before alloc,heap pos: 77824
After alloc,heap pos: 77888
Alloc again,heap pos: 77952
========== END test_brk ==========
frame-allocator-diagnostic: process-teardown pid=9 reclaimed_frames=49 before_free=190772 before_allocated=5434 after_free=190821 after_allocated=5385
Testing chdir :
========== START test_chdir ==========
chdir ret: 0
  current working dir : 
========== END test_chdir ==========
frame-allocator-diagnostic: process-teardown pid=10 reclaimed_frames=49 before_free=190762 before_allocated=5444 after_free=190811 after_allocated=5395
Testing clone :
========== START test_clone ==========
  Child says successfully!
frame-allocator-diagnostic: process-teardown pid=12 reclaimed_frames=1 before_free=190741 before_allocated=5465 after_free=190742 after_allocated=5464
clone process successfully.
pid:12
========== END test_clone ==========
frame-allocator-diagnostic: process-teardown pid=11 reclaimed_frames=51 before_free=190742 before_allocated=5464 after_free=190793 after_allocated=5413
Testing close :
========== START test_close ==========
  close 3 success.
========== END test_close ==========
frame-allocator-diagnostic: process-teardown pid=13 reclaimed_frames=49 before_free=190734 before_allocated=5472 after_free=190783 after_allocated=5423
Testing dup2 :
========== START test_dup2 ==========
  from fd 100
========== END test_dup2 ==========
frame-allocator-diagnostic: process-teardown pid=14 reclaimed_frames=49 before_free=190724 before_allocated=5482 after_free=190773 after_allocated=5433
Testing dup :
========== START test_dup ==========
  new fd is 3.
========== END test_dup ==========
frame-allocator-diagnostic: process-teardown pid=15 reclaimed_frames=49 before_free=190714 before_allocated=5492 after_free=190763 after_allocated=5443
Testing execve :
========== START test_execve ==========
  I am test_echo.
execve success.
========== END main ==========
frame-allocator-diagnostic: process-teardown pid=16 reclaimed_frames=49 before_free=190704 before_allocated=5502 after_free=190753 after_allocated=5453
Testing exit :
========== START test_exit ==========
frame-allocator-diagnostic: process-teardown pid=18 reclaimed_frames=0 before_free=190686 before_allocated=5520 after_free=190686 after_allocated=5520
exit OK.
========== END test_exit ==========
frame-allocator-diagnostic: process-teardown pid=17 reclaimed_frames=49 before_free=190686 before_allocated=5520 after_free=190735 after_allocated=5471
Testing fork :
========== START test_fork ==========
  child process.
frame-allocator-diagnostic: process-teardown pid=20 reclaimed_frames=1 before_free=190667 before_allocated=5539 after_free=190668 after_allocated=5538
  parent process. wstatus:0
========== END test_fork ==========
frame-allocator-diagnostic: process-teardown pid=19 reclaimed_frames=49 before_free=190668 before_allocated=5538 after_free=190717 after_allocated=5489
Testing fstat :
========== START test_fstat ==========
fstat ret: 0
fstat: dev: 1, inode: 1012599416, mode: 33206, nlink: 1, size: 52, atime: 0, mtime: 0, ctime: 0
========== END test_fstat ==========
frame-allocator-diagnostic: process-teardown pid=21 reclaimed_frames=49 before_free=190658 before_allocated=5548 after_free=190707 after_allocated=5499
Testing getcwd :
========== START test_getcwd ==========
getcwd: /tmp/testsuite/musl/basic/basic successfully!
========== END test_getcwd ==========
frame-allocator-diagnostic: process-teardown pid=22 reclaimed_frames=49 before_free=190648 before_allocated=5558 after_free=190697 after_allocated=5509
Testing getdents :
========== START test_getdents ==========
open fd:3
getdents fd:-20
getdents success.


========== END test_getdents ==========
frame-allocator-diagnostic: process-teardown pid=23 reclaimed_frames=49 before_free=190638 before_allocated=5568 after_free=190687 after_allocated=5519
Testing getpid :
========== START test_getpid ==========
getpid success.
pid = 24
========== END test_getpid ==========
frame-allocator-diagnostic: process-teardown pid=24 reclaimed_frames=49 before_free=190628 before_allocated=5578 after_free=190677 after_allocated=5529
Testing getppid :
========== START test_getppid ==========
  getppid success. ppid : 8
========== END test_getppid ==========
frame-allocator-diagnostic: process-teardown pid=25 reclaimed_frames=49 before_free=190618 before_allocated=5588 after_free=190667 after_allocated=5539
Testing gettimeofday :
========== START test_gettimeofday ==========
gettimeofday success.
start:7065, end:7111
interval: 46
========== END test_gettimeofday ==========
frame-allocator-diagnostic: process-teardown pid=26 reclaimed_frames=49 before_free=190608 before_allocated=5598 after_free=190657 after_allocated=5549
Testing mkdir_ :
========== START test_mkdir ==========
mkdir ret: 0
  mkdir success.
========== END test_mkdir ==========
frame-allocator-diagnostic: process-teardown pid=27 reclaimed_frames=49 before_free=190598 before_allocated=5608 after_free=190647 after_allocated=5559
Testing mmap :
========== START test_mmap ==========
file len: 27
mmap content:   Hello, mmap successfully!
========== END test_mmap ==========
frame-allocator-diagnostic: process-teardown pid=28 reclaimed_frames=49 before_free=190588 before_allocated=5618 after_free=190637 after_allocated=5569
Testing mount :
========== START test_mount ==========
Mounting dev:/dev/vda2 to ./mnt
mount return: 0
mount successfully
umount return: 0
========== END test_mount ==========
frame-allocator-diagnostic: process-teardown pid=29 reclaimed_frames=49 before_free=190578 before_allocated=5628 after_free=190627 after_allocated=5579
Testing munmap :
========== START test_munmap ==========
file len: 27
munmap return: 0
munmap successfully!
========== END test_munmap ==========
frame-allocator-diagnostic: process-teardown pid=30 reclaimed_frames=49 before_free=190568 before_allocated=5638 after_free=190617 after_allocated=5589
Testing openat :
========== START test_openat ==========
open dir fd: 3
openat fd: 4
openat success.
========== END test_openat ==========
frame-allocator-diagnostic: process-teardown pid=31 reclaimed_frames=49 before_free=190558 before_allocated=5648 after_free=190607 after_allocated=5599
Testing open :
========== START test_open ==========
Hi, this is a text file.
syscalls testing success!

========== END test_open ==========
frame-allocator-diagnostic: process-teardown pid=32 reclaimed_frames=49 before_free=190548 before_allocated=5658 after_free=190597 after_allocated=5609
Testing pipe :
========== START test_pipe ==========
cpid: 34
cpid: 0
frame-allocator-diagnostic: process-teardown pid=34 reclaimed_frames=1 before_free=190529 before_allocated=5677 after_free=190530 after_allocated=5676
  Write to pipe successfully.

========== END test_pipe ==========
frame-allocator-diagnostic: process-teardown pid=33 reclaimed_frames=49 before_free=190530 before_allocated=5676 after_free=190579 after_allocated=5627
Testing read :
========== START test_read ==========
Hi, this is a text file.
syscalls testing success!

========== END test_read ==========
frame-allocator-diagnostic: process-teardown pid=35 reclaimed_frames=49 before_free=190520 before_allocated=5686 after_free=190569 after_allocated=5637
Testing sleep :
========== START test_sleep ==========
sleep success.
========== END test_sleep ==========
frame-allocator-diagnostic: process-teardown pid=36 reclaimed_frames=49 before_free=190510 before_allocated=5696 after_free=190559 after_allocated=5647
Testing times :
========== START test_times ==========
mytimes success
{tms_utime:0, tms_stime:0, tms_cutime:0, tms_cstime:0}
========== END test_times ==========
frame-allocator-diagnostic: process-teardown pid=37 reclaimed_frames=49 before_free=190500 before_allocated=5706 after_free=190549 after_allocated=5657
Testing umount :
========== START test_umount ==========
Mounting dev:/dev/vda2 to ./mnt
mount return: 0
umount success.
return: 0
========== END test_umount ==========
frame-allocator-diagnostic: process-teardown pid=38 reclaimed_frames=49 before_free=190490 before_allocated=5716 after_free=190539 after_allocated=5667
Testing uname :
========== START test_uname ==========
Uname: Linux arceos 6.0.0 ArceOS loongarch64 localdomain
========== END test_uname ==========
frame-allocator-diagnostic: process-teardown pid=39 reclaimed_frames=49 before_free=190480 before_allocated=5726 after_free=190529 after_allocated=5677
Testing unlink :
========== START test_unlink ==========
  unlink success!
========== END test_unlink ==========
frame-allocator-diagnostic: process-teardown pid=40 reclaimed_frames=49 before_free=190470 before_allocated=5736 after_free=190519 after_allocated=5687
Testing wait :
========== START test_wait ==========
This is child process
frame-allocator-diagnostic: process-teardown pid=42 reclaimed_frames=1 before_free=190451 before_allocated=5755 after_free=190452 after_allocated=5754
wait child success.
wstatus: 0
========== END test_wait ==========
frame-allocator-diagnostic: process-teardown pid=41 reclaimed_frames=49 before_free=190452 before_allocated=5754 after_free=190501 after_allocated=5705
Testing waitpid :
========== START test_waitpid ==========
This is child process
frame-allocator-diagnostic: process-teardown pid=44 reclaimed_frames=2 before_free=190432 before_allocated=5774 after_free=190434 after_allocated=5772
waitpid successfully.
wstatus: 3
========== END test_waitpid ==========
frame-allocator-diagnostic: process-teardown pid=43 reclaimed_frames=49 before_free=190434 before_allocated=5772 after_free=190483 after_allocated=5723
Testing write :
========== START test_write ==========
Hello operating system contest.
========== END test_write ==========
frame-allocator-diagnostic: process-teardown pid=45 reclaimed_frames=49 before_free=190424 before_allocated=5782 after_free=190473 after_allocated=5733
Testing yield :
========== START test_yield ==========
  I am child process: 47. iteration 0.
  I am child process: 48. iteration 1.
  I am child process: 49. iteration 2.
  I am child process: 47. iteration 0.
  I am child process: 48. iteration 1.
  I am child process: 49. iteration 2.
  I am child process: 47. iteration 0.
  I am child process: 48. iteration 1.
  I am child process: 49. iteration 2.
  I am child process: 47. iteration 0.
  I am child process: 48. iteration 1.
  I am child process: 49. iteration 2.
  I am child process: 47. iteration 0.
frame-allocator-diagnostic: process-teardown pid=47 reclaimed_frames=1 before_free=190387 before_allocated=5819 after_free=190388 after_allocated=5818
  I am child process: 48. iteration 1.
frame-allocator-diagnostic: process-teardown pid=48 reclaimed_frames=1 before_free=190388 before_allocated=5818 after_free=190389 after_allocated=5817
  I am child process: 49. iteration 2.
frame-allocator-diagnostic: process-teardown pid=49 reclaimed_frames=1 before_free=190389 before_allocated=5817 after_free=190390 after_allocated=5816
========== END test_yield ==========
frame-allocator-diagnostic: process-teardown pid=46 reclaimed_frames=49 before_free=190390 before_allocated=5816 after_free=190439 after_allocated=5767
frame-allocator-diagnostic: process-teardown pid=8 reclaimed_frames=512 before_free=190440 before_allocated=5766 after_free=190952 after_allocated=5254
#### OS COMP TEST GROUP END basic-musl ####
frame-allocator-diagnostic: process-teardown pid=50 reclaimed_frames=509 before_free=190435 before_allocated=5771 after_free=190944 after_allocated=5262
frame-allocator-diagnostic: process-teardown pid=6 reclaimed_frames=511 before_free=190945 before_allocated=5261 after_free=191456 after_allocated=4750
frame-allocator-diagnostic: process-teardown pid=4 reclaimed_frames=510 before_free=191458 before_allocated=4748 after_free=191968 after_allocated=4238
#### OS COMP TEST GROUP START busybox-musl ####
frame-allocator-diagnostic: process-teardown pid=52 reclaimed_frames=509 before_free=190931 before_allocated=5275 after_free=191440 after_allocated=4766
#### independent command test
frame-allocator-diagnostic: process-teardown pid=53 reclaimed_frames=509 before_free=190923 before_allocated=5283 after_free=191432 after_allocated=4774
frame-allocator-diagnostic: process-teardown pid=51 reclaimed_frames=510 before_free=191434 before_allocated=4772 after_free=191944 after_allocated=4262
testcase busybox echo "#### independent command test" success
frame-allocator-diagnostic: process-teardown pid=55 reclaimed_frames=509 before_free=190907 before_allocated=5299 after_free=191416 after_allocated=4790
frame-allocator-diagnostic: process-teardown pid=56 reclaimed_frames=512 before_free=190896 before_allocated=5310 after_free=191408 after_allocated=4798
frame-allocator-diagnostic: process-teardown pid=54 reclaimed_frames=510 before_free=191410 before_allocated=4796 after_free=191920 after_allocated=4286
testcase busybox ash -c exit success
frame-allocator-diagnostic: process-teardown pid=58 reclaimed_frames=509 before_free=190883 before_allocated=5323 after_free=191392 after_allocated=4814
frame-allocator-diagnostic: process-teardown pid=59 reclaimed_frames=511 before_free=190873 before_allocated=5333 after_free=191384 after_allocated=4822
frame-allocator-diagnostic: process-teardown pid=57 reclaimed_frames=510 before_free=191386 before_allocated=4820 after_free=191896 after_allocated=4310
testcase busybox sh -c exit success
frame-allocator-diagnostic: process-teardown pid=61 reclaimed_frames=509 before_free=190859 before_allocated=5347 after_free=191368 after_allocated=4838
bbb
frame-allocator-diagnostic: process-teardown pid=62 reclaimed_frames=508 before_free=190852 before_allocated=5354 after_free=191360 after_allocated=4846
frame-allocator-diagnostic: process-teardown pid=60 reclaimed_frames=510 before_free=191362 before_allocated=4844 after_free=191872 after_allocated=4334
testcase busybox basename /aaa/bbb success
frame-allocator-diagnostic: process-teardown pid=64 reclaimed_frames=509 before_free=190835 before_allocated=5371 after_free=191344 after_allocated=4862
    January 1970
Su Mo Tu We Th Fr Sa
             1  2  3
 4  5  6  7  8  9 10
11 12 13 14 15 16 17
18 19 20 21 22 23 24
25 26 27 28 29 30 31
                     
frame-allocator-diagnostic: process-teardown pid=65 reclaimed_frames=511 before_free=190825 before_allocated=5381 after_free=191336 after_allocated=4870
frame-allocator-diagnostic: process-teardown pid=63 reclaimed_frames=510 before_free=191338 before_allocated=4868 after_free=191848 after_allocated=4358
testcase busybox cal success
frame-allocator-diagnostic: process-teardown pid=67 reclaimed_frames=509 before_free=190811 before_allocated=5395 after_free=191320 after_allocated=4886
frame-allocator-diagnostic: process-teardown pid=68 reclaimed_frames=508 before_free=190804 before_allocated=5402 after_free=191312 after_allocated=4894
frame-allocator-diagnostic: process-teardown pid=66 reclaimed_frames=510 before_free=191314 before_allocated=4892 after_free=191824 after_allocated=4382
testcase busybox clear success
frame-allocator-diagnostic: process-teardown pid=70 reclaimed_frames=509 before_free=190787 before_allocated=5419 after_free=191296 after_allocated=4910
Thu Jan  1 00:00:21 UTC 1970
frame-allocator-diagnostic: process-teardown pid=71 reclaimed_frames=508 before_free=190780 before_allocated=5426 after_free=191288 after_allocated=4918
frame-allocator-diagnostic: process-teardown pid=69 reclaimed_frames=510 before_free=191290 before_allocated=4916 after_free=191800 after_allocated=4406
testcase busybox date success
frame-allocator-diagnostic: process-teardown pid=73 reclaimed_frames=509 before_free=190763 before_allocated=5443 after_free=191272 after_allocated=4934
Filesystem           1K-blocks      Used Available Use% Mounted on
devfs                   784824     21812    763012   3% /dev
tmpfs                   784824     21812    763012   3% /tmp
tmpfs                   784824     21812    763012   3% /var
proc                    784824     21812    763012   3% /proc
sysfs                   784824     21812    763012   3% /sys
frame-allocator-diagnostic: process-teardown pid=74 reclaimed_frames=510 before_free=190754 before_allocated=5452 after_free=191264 after_allocated=4942
frame-allocator-diagnostic: process-teardown pid=72 reclaimed_frames=510 before_free=191266 before_allocated=4940 after_free=191776 after_allocated=4430
testcase busybox df success
frame-allocator-diagnostic: process-teardown pid=76 reclaimed_frames=509 before_free=190739 before_allocated=5467 after_free=191248 after_allocated=4958
/aaa
frame-allocator-diagnostic: process-teardown pid=77 reclaimed_frames=508 before_free=190732 before_allocated=5474 after_free=191240 after_allocated=4966
frame-allocator-diagnostic: process-teardown pid=75 reclaimed_frames=510 before_free=191242 before_allocated=4964 after_free=191752 after_allocated=4454
testcase busybox dirname /aaa/bbb success
frame-allocator-diagnostic: process-teardown pid=79 reclaimed_frames=509 before_free=190715 before_allocated=5491 after_free=191224 after_allocated=4982
frame-allocator-diagnostic: process-teardown pid=80 reclaimed_frames=511 before_free=190705 before_allocated=5501 after_free=191216 after_allocated=4990
frame-allocator-diagnostic: process-teardown pid=78 reclaimed_frames=510 before_free=191218 before_allocated=4988 after_free=191728 after_allocated=4478
testcase busybox dmesg success
frame-allocator-diagnostic: process-teardown pid=82 reclaimed_frames=509 before_free=190691 before_allocated=5515 after_free=191200 after_allocated=5006
0	.
frame-allocator-diagnostic: process-teardown pid=83 reclaimed_frames=509 before_free=190683 before_allocated=5523 after_free=191192 after_allocated=5014
frame-allocator-diagnostic: process-teardown pid=81 reclaimed_frames=510 before_free=191194 before_allocated=5012 after_free=191704 after_allocated=4502
testcase busybox du success
frame-allocator-diagnostic: process-teardown pid=85 reclaimed_frames=509 before_free=190667 before_allocated=5539 after_free=191176 after_allocated=5030
2
frame-allocator-diagnostic: process-teardown pid=86 reclaimed_frames=510 before_free=190658 before_allocated=5548 after_free=191168 after_allocated=5038
frame-allocator-diagnostic: process-teardown pid=84 reclaimed_frames=510 before_free=191170 before_allocated=5036 after_free=191680 after_allocated=4526
testcase busybox expr 1 + 1 success
frame-allocator-diagnostic: process-teardown pid=88 reclaimed_frames=509 before_free=190643 before_allocated=5563 after_free=191152 after_allocated=5054
frame-allocator-diagnostic: process-teardown pid=89 reclaimed_frames=508 before_free=190636 before_allocated=5570 after_free=191144 after_allocated=5062
frame-allocator-diagnostic: process-teardown pid=87 reclaimed_frames=510 before_free=191146 before_allocated=5060 after_free=191656 after_allocated=4550
testcase busybox false success
frame-allocator-diagnostic: process-teardown pid=91 reclaimed_frames=509 before_free=190619 before_allocated=5587 after_free=191128 after_allocated=5078
frame-allocator-diagnostic: process-teardown pid=92 reclaimed_frames=508 before_free=190612 before_allocated=5594 after_free=191120 after_allocated=5086
frame-allocator-diagnostic: process-teardown pid=90 reclaimed_frames=510 before_free=191122 before_allocated=5084 after_free=191632 after_allocated=4574
testcase busybox true success
frame-allocator-diagnostic: process-teardown pid=94 reclaimed_frames=509 before_free=190595 before_allocated=5611 after_free=191104 after_allocated=5102
/musl/ls
frame-allocator-diagnostic: process-teardown pid=95 reclaimed_frames=509 before_free=190587 before_allocated=5619 after_free=191096 after_allocated=5110
frame-allocator-diagnostic: process-teardown pid=93 reclaimed_frames=510 before_free=191098 before_allocated=5108 after_free=191608 after_allocated=4598
testcase busybox which ls success
frame-allocator-diagnostic: process-teardown pid=97 reclaimed_frames=509 before_free=190571 before_allocated=5635 after_free=191080 after_allocated=5126
Linux
frame-allocator-diagnostic: process-teardown pid=98 reclaimed_frames=508 before_free=190564 before_allocated=5642 after_free=191072 after_allocated=5134
frame-allocator-diagnostic: process-teardown pid=96 reclaimed_frames=510 before_free=191074 before_allocated=5132 after_free=191584 after_allocated=4622
testcase busybox uname success
frame-allocator-diagnostic: process-teardown pid=100 reclaimed_frames=509 before_free=190547 before_allocated=5659 after_free=191056 after_allocated=5150
 00:00:31 up 0 min,  0 users,  load average: 0.00, 0.00, 0.00
frame-allocator-diagnostic: process-teardown pid=101 reclaimed_frames=508 before_free=190540 before_allocated=5666 after_free=191048 after_allocated=5158
frame-allocator-diagnostic: process-teardown pid=99 reclaimed_frames=510 before_free=191050 before_allocated=5156 after_free=191560 after_allocated=4646
testcase busybox uptime success
frame-allocator-diagnostic: process-teardown pid=103 reclaimed_frames=509 before_free=190523 before_allocated=5683 after_free=191032 after_allocated=5174
abc
frame-allocator-diagnostic: process-teardown pid=104 reclaimed_frames=508 before_free=190516 before_allocated=5690 after_free=191024 after_allocated=5182
frame-allocator-diagnostic: process-teardown pid=102 reclaimed_frames=510 before_free=191026 before_allocated=5180 after_free=191536 after_allocated=4670
testcase busybox printf "abc\n" success
frame-allocator-diagnostic: process-teardown pid=106 reclaimed_frames=509 before_free=190499 before_allocated=5707 after_free=191008 after_allocated=5198
PID   USER     TIME  COMMAND
frame-allocator-diagnostic: process-teardown pid=107 reclaimed_frames=512 before_free=190488 before_allocated=5718 after_free=191000 after_allocated=5206
frame-allocator-diagnostic: process-teardown pid=105 reclaimed_frames=510 before_free=191002 before_allocated=5204 after_free=191512 after_allocated=4694
testcase busybox ps success
frame-allocator-diagnostic: process-teardown pid=109 reclaimed_frames=509 before_free=190475 before_allocated=5731 after_free=190984 after_allocated=5222
/tmp/testsuite/musl/busybox
frame-allocator-diagnostic: process-teardown pid=110 reclaimed_frames=509 before_free=190467 before_allocated=5739 after_free=190976 after_allocated=5230
frame-allocator-diagnostic: process-teardown pid=108 reclaimed_frames=510 before_free=190978 before_allocated=5228 after_free=191488 after_allocated=4718
testcase busybox pwd success
frame-allocator-diagnostic: process-teardown pid=112 reclaimed_frames=509 before_free=190451 before_allocated=5755 after_free=190960 after_allocated=5246
              total        used        free      shared  buff/cache   available
Mem:              0           0           0           0           0      781966
-/+ buffers/cache:            0           0
Swap:             0           0           0
frame-allocator-diagnostic: process-teardown pid=113 reclaimed_frames=509 before_free=190443 before_allocated=5763 after_free=190952 after_allocated=5254
frame-allocator-diagnostic: process-teardown pid=111 reclaimed_frames=510 before_free=190954 before_allocated=5252 after_free=191464 after_allocated=4742
testcase busybox free success
frame-allocator-diagnostic: process-teardown pid=115 reclaimed_frames=509 before_free=190427 before_allocated=5779 after_free=190936 after_allocated=5270
Thu Jan  1 00:00:37 1970  0.000000 seconds
frame-allocator-diagnostic: process-teardown pid=116 reclaimed_frames=508 before_free=190420 before_allocated=5786 after_free=190928 after_allocated=5278
frame-allocator-diagnostic: process-teardown pid=114 reclaimed_frames=510 before_free=190930 before_allocated=5276 after_free=191440 after_allocated=4766
testcase busybox hwclock success
frame-allocator-diagnostic: process-teardown pid=118 reclaimed_frames=509 before_free=190403 before_allocated=5803 after_free=190912 after_allocated=5294
frame-allocator-diagnostic: process-teardown pid=120 reclaimed_frames=508 before_free=189863 before_allocated=6343 after_free=190371 after_allocated=5835
frame-allocator-diagnostic: process-teardown pid=117 reclaimed_frames=511 before_free=189870 before_allocated=6336 after_free=190381 after_allocated=5825
testcase busybox sh -c 'sleep 5' & /musl/busybox kill $! success
frame-allocator-diagnostic: process-teardown pid=123 reclaimed_frames=509 before_free=189344 before_allocated=6862 after_free=189853 after_allocated=6353
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
frame-allocator-diagnostic: process-teardown pid=124 reclaimed_frames=510 before_free=189335 before_allocated=6871 after_free=189845 after_allocated=6361
frame-allocator-diagnostic: process-teardown pid=122 reclaimed_frames=510 before_free=189847 before_allocated=6359 after_free=190357 after_allocated=5849
testcase busybox ls success
frame-allocator-diagnostic: process-teardown pid=126 reclaimed_frames=509 before_free=189320 before_allocated=6886 after_free=189829 after_allocated=6377
frame-allocator-diagnostic: process-teardown pid=127 reclaimed_frames=508 before_free=189313 before_allocated=6893 after_free=189821 after_allocated=6385
frame-allocator-diagnostic: process-teardown pid=125 reclaimed_frames=510 before_free=189823 before_allocated=6383 after_free=190333 after_allocated=5873
testcase busybox sleep 1 success
frame-allocator-diagnostic: process-teardown pid=129 reclaimed_frames=509 before_free=189296 before_allocated=6910 after_free=189805 after_allocated=6401
#### file opration test
frame-allocator-diagnostic: process-teardown pid=130 reclaimed_frames=509 before_free=189288 before_allocated=6918 after_free=189797 after_allocated=6409
frame-allocator-diagnostic: process-teardown pid=128 reclaimed_frames=510 before_free=189799 before_allocated=6407 after_free=190309 after_allocated=5897
testcase busybox echo "#### file opration test" success
frame-allocator-diagnostic: process-teardown pid=132 reclaimed_frames=509 before_free=189272 before_allocated=6934 after_free=189781 after_allocated=6425
frame-allocator-diagnostic: process-teardown pid=133 reclaimed_frames=508 before_free=189265 before_allocated=6941 after_free=189773 after_allocated=6433
frame-allocator-diagnostic: process-teardown pid=121 reclaimed_frames=508 before_free=189773 before_allocated=6433 after_free=190281 after_allocated=5925
frame-allocator-diagnostic: process-teardown pid=131 reclaimed_frames=510 before_free=190283 before_allocated=5923 after_free=190793 after_allocated=5413
frame-allocator-diagnostic: process-teardown pid=119 reclaimed_frames=510 before_free=190794 before_allocated=5412 after_free=191304 after_allocated=4902
testcase busybox touch test.txt success
frame-allocator-diagnostic: process-teardown pid=135 reclaimed_frames=509 before_free=190267 before_allocated=5939 after_free=190776 after_allocated=5430
frame-allocator-diagnostic: process-teardown pid=136 reclaimed_frames=509 before_free=190259 before_allocated=5947 after_free=190768 after_allocated=5438
frame-allocator-diagnostic: process-teardown pid=134 reclaimed_frames=510 before_free=190770 before_allocated=5436 after_free=191280 after_allocated=4926
testcase busybox echo "hello world" > test.txt success
frame-allocator-diagnostic: process-teardown pid=138 reclaimed_frames=509 before_free=190243 before_allocated=5963 after_free=190752 after_allocated=5454
hello world
frame-allocator-diagnostic: process-teardown pid=139 reclaimed_frames=509 before_free=190235 before_allocated=5971 after_free=190744 after_allocated=5462
frame-allocator-diagnostic: process-teardown pid=137 reclaimed_frames=510 before_free=190746 before_allocated=5460 after_free=191256 after_allocated=4950
testcase busybox cat test.txt success
frame-allocator-diagnostic: process-teardown pid=141 reclaimed_frames=509 before_free=190219 before_allocated=5987 after_free=190728 after_allocated=5478
l
frame-allocator-diagnostic: process-teardown pid=142 reclaimed_frames=510 before_free=190210 before_allocated=5996 after_free=190720 after_allocated=5486
frame-allocator-diagnostic: process-teardown pid=140 reclaimed_frames=510 before_free=190722 before_allocated=5484 after_free=191232 after_allocated=4974
testcase busybox cut -c 3 test.txt success
frame-allocator-diagnostic: process-teardown pid=144 reclaimed_frames=509 before_free=190195 before_allocated=6011 after_free=190704 after_allocated=5502
0000000 062550 066154 020157 067567 066162 005144
0000014
frame-allocator-diagnostic: process-teardown pid=145 reclaimed_frames=510 before_free=190186 before_allocated=6020 after_free=190696 after_allocated=5510
frame-allocator-diagnostic: process-teardown pid=143 reclaimed_frames=510 before_free=190698 before_allocated=5508 after_free=191208 after_allocated=4998
testcase busybox od test.txt success
frame-allocator-diagnostic: process-teardown pid=147 reclaimed_frames=509 before_free=190171 before_allocated=6035 after_free=190680 after_allocated=5526
hello world
frame-allocator-diagnostic: process-teardown pid=148 reclaimed_frames=509 before_free=190163 before_allocated=6043 after_free=190672 after_allocated=5534
frame-allocator-diagnostic: process-teardown pid=146 reclaimed_frames=510 before_free=190674 before_allocated=5532 after_free=191184 after_allocated=5022
testcase busybox head test.txt success
frame-allocator-diagnostic: process-teardown pid=150 reclaimed_frames=509 before_free=190147 before_allocated=6059 after_free=190656 after_allocated=5550
hello world
frame-allocator-diagnostic: process-teardown pid=151 reclaimed_frames=511 before_free=190137 before_allocated=6069 after_free=190648 after_allocated=5558
frame-allocator-diagnostic: process-teardown pid=149 reclaimed_frames=510 before_free=190650 before_allocated=5556 after_free=191160 after_allocated=5046
testcase busybox tail test.txt success
frame-allocator-diagnostic: process-teardown pid=153 reclaimed_frames=509 before_free=190123 before_allocated=6083 after_free=190632 after_allocated=5574
00000000  68 65 6c 6c 6f 20 77 6f  72 6c 64 0a              |hello world.|
0000000c
frame-allocator-diagnostic: process-teardown pid=154 reclaimed_frames=510 before_free=190114 before_allocated=6092 after_free=190624 after_allocated=5582
frame-allocator-diagnostic: process-teardown pid=152 reclaimed_frames=510 before_free=190626 before_allocated=5580 after_free=191136 after_allocated=5070
testcase busybox hexdump -C test.txt success
frame-allocator-diagnostic: process-teardown pid=156 reclaimed_frames=509 before_free=190099 before_allocated=6107 after_free=190608 after_allocated=5598
6f5902ac237024bdd0c176cb93063dc4  test.txt
frame-allocator-diagnostic: process-teardown pid=157 reclaimed_frames=509 before_free=190091 before_allocated=6115 after_free=190600 after_allocated=5606
frame-allocator-diagnostic: process-teardown pid=155 reclaimed_frames=510 before_free=190602 before_allocated=5604 after_free=191112 after_allocated=5094
testcase busybox md5sum test.txt success
frame-allocator-diagnostic: process-teardown pid=159 reclaimed_frames=509 before_free=190075 before_allocated=6131 after_free=190584 after_allocated=5622
frame-allocator-diagnostic: process-teardown pid=160 reclaimed_frames=509 before_free=190067 before_allocated=6139 after_free=190576 after_allocated=5630
frame-allocator-diagnostic: process-teardown pid=158 reclaimed_frames=510 before_free=190578 before_allocated=5628 after_free=191088 after_allocated=5118
testcase busybox echo "ccccccc" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=162 reclaimed_frames=509 before_free=190051 before_allocated=6155 after_free=190560 after_allocated=5646
frame-allocator-diagnostic: process-teardown pid=163 reclaimed_frames=509 before_free=190043 before_allocated=6163 after_free=190552 after_allocated=5654
frame-allocator-diagnostic: process-teardown pid=161 reclaimed_frames=510 before_free=190554 before_allocated=5652 after_free=191064 after_allocated=5142
testcase busybox echo "bbbbbbb" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=165 reclaimed_frames=509 before_free=190027 before_allocated=6179 after_free=190536 after_allocated=5670
frame-allocator-diagnostic: process-teardown pid=166 reclaimed_frames=509 before_free=190019 before_allocated=6187 after_free=190528 after_allocated=5678
frame-allocator-diagnostic: process-teardown pid=164 reclaimed_frames=510 before_free=190530 before_allocated=5676 after_free=191040 after_allocated=5166
testcase busybox echo "aaaaaaa" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=168 reclaimed_frames=509 before_free=190003 before_allocated=6203 after_free=190512 after_allocated=5694
frame-allocator-diagnostic: process-teardown pid=169 reclaimed_frames=509 before_free=189995 before_allocated=6211 after_free=190504 after_allocated=5702
frame-allocator-diagnostic: process-teardown pid=167 reclaimed_frames=510 before_free=190506 before_allocated=5700 after_free=191016 after_allocated=5190
testcase busybox echo "2222222" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=171 reclaimed_frames=509 before_free=189979 before_allocated=6227 after_free=190488 after_allocated=5718
frame-allocator-diagnostic: process-teardown pid=172 reclaimed_frames=509 before_free=189971 before_allocated=6235 after_free=190480 after_allocated=5726
frame-allocator-diagnostic: process-teardown pid=170 reclaimed_frames=510 before_free=190482 before_allocated=5724 after_free=190992 after_allocated=5214
testcase busybox echo "1111111" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=174 reclaimed_frames=509 before_free=189955 before_allocated=6251 after_free=190464 after_allocated=5742
frame-allocator-diagnostic: process-teardown pid=175 reclaimed_frames=509 before_free=189947 before_allocated=6259 after_free=190456 after_allocated=5750
frame-allocator-diagnostic: process-teardown pid=173 reclaimed_frames=510 before_free=190458 before_allocated=5748 after_free=190968 after_allocated=5238
testcase busybox echo "bbbbbbb" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=177 reclaimed_frames=509 before_free=189931 before_allocated=6275 after_free=190440 after_allocated=5766
frame-allocator-diagnostic: process-teardown pid=178 reclaimed_frames=510 before_free=189909 before_allocated=6297 after_free=190419 after_allocated=5787
1111111
2222222
aaaaaaa
bbbbbbb
ccccccc
hello world
frame-allocator-diagnostic: process-teardown pid=179 reclaimed_frames=509 before_free=189915 before_allocated=6291 after_free=190424 after_allocated=5782
frame-allocator-diagnostic: process-teardown pid=176 reclaimed_frames=510 before_free=190426 before_allocated=5780 after_free=190936 after_allocated=5270
testcase busybox sort test.txt | /musl/busybox uniq success
frame-allocator-diagnostic: process-teardown pid=181 reclaimed_frames=509 before_free=189899 before_allocated=6307 after_free=190408 after_allocated=5798
  File: test.txt
  Size: 60        	Blocks: 0          IO Block: 512    regular file
Device: 1h/1d	Inode: 14331471978328146352  Links: 1
Access: (0666/-rw-rw-rw-)  Uid: (    0/    root)   Gid: (    0/    root)
Access: 1970-01-01 00:00:00.000000000 +0000
Modify: 1970-01-01 00:00:00.000000000 +0000
Change: 1970-01-01 00:00:00.000000000 +0000
frame-allocator-diagnostic: process-teardown pid=182 reclaimed_frames=510 before_free=189890 before_allocated=6316 after_free=190400 after_allocated=5806
frame-allocator-diagnostic: process-teardown pid=180 reclaimed_frames=510 before_free=190402 before_allocated=5804 after_free=190912 after_allocated=5294
testcase busybox stat test.txt success
frame-allocator-diagnostic: process-teardown pid=184 reclaimed_frames=509 before_free=189875 before_allocated=6331 after_free=190384 after_allocated=5822
hello world
ccccccc
bbbbbbb
aaaaaaa
2222222
1111111
bbbbbbb
frame-allocator-diagnostic: process-teardown pid=185 reclaimed_frames=510 before_free=189866 before_allocated=6340 after_free=190376 after_allocated=5830
frame-allocator-diagnostic: process-teardown pid=183 reclaimed_frames=510 before_free=190378 before_allocated=5828 after_free=190888 after_allocated=5318
testcase busybox strings test.txt success
frame-allocator-diagnostic: process-teardown pid=187 reclaimed_frames=509 before_free=189851 before_allocated=6355 after_free=190360 after_allocated=5846
        7         8        60 test.txt
frame-allocator-diagnostic: process-teardown pid=188 reclaimed_frames=509 before_free=189843 before_allocated=6363 after_free=190352 after_allocated=5854
frame-allocator-diagnostic: process-teardown pid=186 reclaimed_frames=510 before_free=190354 before_allocated=5852 after_free=190864 after_allocated=5342
testcase busybox wc test.txt success
frame-allocator-diagnostic: process-teardown pid=190 reclaimed_frames=509 before_free=189827 before_allocated=6379 after_free=190336 after_allocated=5870
frame-allocator-diagnostic: process-teardown pid=191 reclaimed_frames=509 before_free=189817 before_allocated=6389 after_free=190326 after_allocated=5880
frame-allocator-diagnostic: process-teardown pid=189 reclaimed_frames=512 before_free=190328 before_allocated=5878 after_free=190840 after_allocated=5366
testcase busybox [ -f test.txt ] success
frame-allocator-diagnostic: process-teardown pid=193 reclaimed_frames=509 before_free=189803 before_allocated=6403 after_free=190312 after_allocated=5894
hello world
ccccccc
bbbbbbb
aaaaaaa
2222222
1111111
bbbbbbb
frame-allocator-diagnostic: process-teardown pid=194 reclaimed_frames=509 before_free=189795 before_allocated=6411 after_free=190304 after_allocated=5902
frame-allocator-diagnostic: process-teardown pid=192 reclaimed_frames=510 before_free=190306 before_allocated=5900 after_free=190816 after_allocated=5390
testcase busybox more test.txt success
frame-allocator-diagnostic: process-teardown pid=196 reclaimed_frames=509 before_free=189779 before_allocated=6427 after_free=190288 after_allocated=5918
frame-allocator-diagnostic: process-teardown pid=197 reclaimed_frames=508 before_free=189772 before_allocated=6434 after_free=190280 after_allocated=5926
frame-allocator-diagnostic: process-teardown pid=195 reclaimed_frames=510 before_free=190282 before_allocated=5924 after_free=190792 after_allocated=5414
testcase busybox rm test.txt success
frame-allocator-diagnostic: process-teardown pid=199 reclaimed_frames=509 before_free=189755 before_allocated=6451 after_free=190264 after_allocated=5942
frame-allocator-diagnostic: process-teardown pid=200 reclaimed_frames=508 before_free=189748 before_allocated=6458 after_free=190256 after_allocated=5950
frame-allocator-diagnostic: process-teardown pid=198 reclaimed_frames=510 before_free=190258 before_allocated=5948 after_free=190768 after_allocated=5438
testcase busybox mkdir test_dir success
frame-allocator-diagnostic: process-teardown pid=202 reclaimed_frames=509 before_free=189731 before_allocated=6475 after_free=190240 after_allocated=5966
frame-allocator-diagnostic: process-teardown pid=203 reclaimed_frames=508 before_free=189724 before_allocated=6482 after_free=190232 after_allocated=5974
frame-allocator-diagnostic: process-teardown pid=201 reclaimed_frames=510 before_free=190234 before_allocated=5972 after_free=190744 after_allocated=5462
testcase busybox mv test_dir test success
frame-allocator-diagnostic: process-teardown pid=205 reclaimed_frames=509 before_free=189707 before_allocated=6499 after_free=190216 after_allocated=5990
frame-allocator-diagnostic: process-teardown pid=206 reclaimed_frames=508 before_free=189700 before_allocated=6506 after_free=190208 after_allocated=5998
frame-allocator-diagnostic: process-teardown pid=204 reclaimed_frames=510 before_free=190210 before_allocated=5996 after_free=190720 after_allocated=5486
testcase busybox rmdir test success
frame-allocator-diagnostic: process-teardown pid=208 reclaimed_frames=509 before_free=189683 before_allocated=6523 after_free=190192 after_allocated=6014
echo "hello world" > test.txt
grep hello busybox_cmd.txt
frame-allocator-diagnostic: process-teardown pid=209 reclaimed_frames=510 before_free=189674 before_allocated=6532 after_free=190184 after_allocated=6022
frame-allocator-diagnostic: process-teardown pid=207 reclaimed_frames=510 before_free=190186 before_allocated=6020 after_free=190696 after_allocated=5510
testcase busybox grep hello busybox_cmd.txt success
frame-allocator-diagnostic: process-teardown pid=211 reclaimed_frames=509 before_free=189659 before_allocated=6547 after_free=190168 after_allocated=6038
frame-allocator-diagnostic: process-teardown pid=212 reclaimed_frames=509 before_free=189651 before_allocated=6555 after_free=190160 after_allocated=6046
frame-allocator-diagnostic: process-teardown pid=210 reclaimed_frames=510 before_free=190162 before_allocated=6044 after_free=190672 after_allocated=5534
testcase busybox cp busybox_cmd.txt busybox_cmd.bak success
frame-allocator-diagnostic: process-teardown pid=214 reclaimed_frames=509 before_free=189635 before_allocated=6571 after_free=190144 after_allocated=6062
frame-allocator-diagnostic: process-teardown pid=215 reclaimed_frames=508 before_free=189628 before_allocated=6578 after_free=190136 after_allocated=6070
frame-allocator-diagnostic: process-teardown pid=213 reclaimed_frames=510 before_free=190138 before_allocated=6068 after_free=190648 after_allocated=5558
testcase busybox rm busybox_cmd.bak success
frame-allocator-diagnostic: process-teardown pid=217 reclaimed_frames=509 before_free=189611 before_allocated=6595 after_free=190120 after_allocated=6086
./busybox_cmd.txt
frame-allocator-diagnostic: process-teardown pid=218 reclaimed_frames=510 before_free=189602 before_allocated=6604 after_free=190112 after_allocated=6094
frame-allocator-diagnostic: process-teardown pid=216 reclaimed_frames=510 before_free=190114 before_allocated=6092 after_free=190624 after_allocated=5582
testcase busybox find -name "busybox_cmd.txt" success
#### OS COMP TEST GROUP END busybox-musl ####
frame-allocator-diagnostic: process-teardown pid=220 reclaimed_frames=509 before_free=189586 before_allocated=6620 after_free=190095 after_allocated=6111
#### OS COMP TEST GROUP START cyclictest-musl ####
frame-allocator-diagnostic: process-teardown pid=222 reclaimed_frames=509 before_free=189058 before_allocated=7148 after_free=189567 after_allocated=6639
====== cyclictest NO_STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 (  224) P:99 I:1000 C:    993 Min:      1 Act:    2 Avg:   14 Max:    7123
frame-allocator-diagnostic: process-teardown pid=223 reclaimed_frames=212 before_free=189346 before_allocated=6860 after_free=189558 after_allocated=6648
====== cyclictest NO_STRESS_P1 end: success ======
====== cyclictest NO_STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 (  226) P:99 I:1000 C:    971 Min:      1 Act:    6 Avg:   38 Max:   10420
T: 1 (  227) P:99 I:1500 C:    651 Min:      1 Act:  257 Avg:   54 Max:   10303
T: 2 (  228) P:99 I:2000 C:    489 Min:      1 Act:  427 Avg:   59 Max:   10287
T: 3 (  229) P:99 I:2500 C:    393 Min:      1 Act:  420 Avg:   79 Max:    9762
T: 4 (  230) P:99 I:3000 C:    328 Min:      1 Act:   50 Avg:   86 Max:   10221
T: 5 (  231) P:99 I:3500 C:    281 Min:      1 Act:   58 Avg:   87 Max:    8198
T: 6 (  232) P:99 I:4000 C:    246 Min:      1 Act:  319 Avg:   89 Max:    8182
T: 7 (  233) P:99 I:4500 C:    221 Min:      1 Act:   48 Avg:   85 Max:    6937
frame-allocator-diagnostic: process-teardown pid=225 reclaimed_frames=212 before_free=189337 before_allocated=6869 after_free=189549 after_allocated=6657
====== cyclictest NO_STRESS_P8 end: success ======
====== start hackbench ======
Running in process mode with 10 groups using 40 file descriptors each (== 400 tasks)
Each sender will pass 100000000 messages of 100 bytes
frame-allocator-diagnostic: process-teardown pid=235 reclaimed_frames=508 before_free=171218 before_allocated=24988 after_free=171726 after_allocated=24480
====== cyclictest STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 (  637) P:99 I:1000 C:      8 Min:     52 Act:138802 Avg:133886 Max:  205696
frame-allocator-diagnostic: process-teardown pid=636 reclaimed_frames=212 before_free=171506 before_allocated=24700 after_free=171718 after_allocated=24488
====== cyclictest STRESS_P1 end: success ======
====== cyclictest STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 (  639) P:99 I:1000 C:     11 Min:     31 Act:190473 Avg:106056 Max:  209679
T: 1 (  640) P:99 I:1500 C:      7 Min: 139402 Act:190924 Avg:166973 Max:  209477
T: 2 (  641) P:99 I:2000 C:      7 Min: 138883 Act:191490 Avg:166754 Max:  208973
T: 3 (  642) P:99 I:2500 C:      7 Min: 138866 Act:191034 Avg:166248 Max:  208470
T: 4 (  643) P:99 I:3000 C:      7 Min: 137850 Act:189911 Avg:165933 Max:  207966
T: 5 (  644) P:99 I:3500 C:      7 Min: 138833 Act:190557 Avg:165870 Max:  207461
T: 6 (  645) P:99 I:4000 C:      7 Min: 138815 Act:190100 Avg:165933 Max:  206953
T: 7 (  646) P:99 I:4500 C:      7 Min: 136290 Act:188630 Avg:165061 Max:  206439
frame-allocator-diagnostic: process-teardown pid=638 reclaimed_frames=212 before_free=171498 before_allocated=24708 after_free=171710 after_allocated=24496
====== cyclictest STRESS_P8 end: success ======
frame-allocator-diagnostic: process-teardown pid=647 reclaimed_frames=508 before_free=171194 before_allocated=25012 after_free=171702 after_allocated=24504
frame-allocator-diagnostic: process-teardown pid=648 reclaimed_frames=508 before_free=171186 before_allocated=25020 after_free=171694 after_allocated=24512
====== kill hackbench: success ======
#### OS COMP TEST GROUP END cyclictest-musl ####
frame-allocator-diagnostic: process-teardown pid=649 reclaimed_frames=509 before_free=171177 before_allocated=25029 after_free=171686 after_allocated=24520
frame-allocator-diagnostic: process-teardown pid=221 reclaimed_frames=514 before_free=171686 before_allocated=24520 after_free=172200 after_allocated=24006
frame-allocator-diagnostic: process-teardown pid=219 reclaimed_frames=511 before_free=172202 before_allocated=24004 after_free=172713 after_allocated=23493
#### OS COMP TEST GROUP START iozone-musl ####
SKIP: iozone throughput mode currently hangs in the evaluator environment
#### OS COMP TEST GROUP END iozone-musl ####
frame-allocator-diagnostic: process-teardown pid=651 reclaimed_frames=509 before_free=171676 before_allocated=24530 after_free=172185 after_allocated=24021
#### OS COMP TEST GROUP START iperf-musl ####
frame-allocator-diagnostic: process-teardown pid=653 reclaimed_frames=509 before_free=171148 before_allocated=25058 after_free=171657 after_allocated=24549
frame-allocator-diagnostic: process-teardown pid=654 reclaimed_frames=2 before_free=171572 before_allocated=24634 after_free=171574 after_allocated=24632
frame-allocator-diagnostic: process-teardown pid=655 reclaimed_frames=2 before_free=171564 before_allocated=24642 after_free=171566 after_allocated=24640
====== iperf BASIC_UDP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 0.0.0.0 port 49152 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  5]   0.00-2.26   sec  9.98 KBytes  36.2 Kbits/sec  7  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.26   sec  9.98 KBytes  36.2 Kbits/sec  0.000 ms  0/7 (0%)  sender
[  5]   0.00-2.68   sec  9.98 KBytes  30.5 Kbits/sec  28.422 ms  0/7 (0%)  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=657 reclaimed_frames=69 before_free=171484 before_allocated=24722 after_free=171553 after_allocated=24653
====== iperf BASIC_UDP end: success ======

====== iperf BASIC_TCP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 127.0.0.1 port 49154 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.03   sec  2.25 MBytes  9.32 Mbits/sec    0   0.00 Bytes       
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.03   sec  2.25 MBytes  9.32 Mbits/sec    0             sender
[  5]   0.00-2.46   sec  1.38 MBytes  4.69 Mbits/sec                  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=658 reclaimed_frames=69 before_free=171444 before_allocated=24762 after_free=171513 after_allocated=24693
====== iperf BASIC_TCP end: success ======

====== iperf PARALLEL_UDP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 0.0.0.0 port 49153 connected to 127.0.0.1 port 5001
[  7] local 0.0.0.0 port 49154 connected to 127.0.0.1 port 5001
[  9] local 0.0.0.0 port 49155 connected to 127.0.0.1 port 5001
[ 11] local 0.0.0.0 port 49156 connected to 127.0.0.1 port 5001
[ 13] local 0.0.0.0 port 49157 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  5]   0.00-2.96   sec  4.28 KBytes  11.8 Kbits/sec  3  
[  7]   0.00-2.96   sec  4.28 KBytes  11.8 Kbits/sec  3  
[  9]   0.00-2.96   sec  4.28 KBytes  11.8 Kbits/sec  3  
[ 11]   0.00-2.96   sec  4.28 KBytes  11.8 Kbits/sec  3  
[ 13]   0.00-2.96   sec  4.28 KBytes  11.8 Kbits/sec  3  
[SUM]   0.00-2.96   sec  21.4 KBytes  59.2 Kbits/sec  15  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.96   sec  4.28 KBytes  11.8 Kbits/sec  0.000 ms  0/3 (0%)  sender
[  5]   0.00-3.38   sec  4.28 KBytes  10.4 Kbits/sec  2.830 ms  0/3 (0%)  receiver
[  7]   0.00-2.96   sec  4.28 KBytes  11.8 Kbits/sec  0.000 ms  0/3 (0%)  sender
[  7]   0.00-3.38   sec  4.28 KBytes  10.4 Kbits/sec  0.717 ms  0/3 (0%)  receiver
[  9]   0.00-2.96   sec  4.28 KBytes  11.8 Kbits/sec  0.000 ms  0/3 (0%)  sender
[  9]   0.00-3.38   sec  4.28 KBytes  10.4 Kbits/sec  2.038 ms  0/3 (0%)  receiver
[ 11]   0.00-2.96   sec  4.28 KBytes  11.8 Kbits/sec  0.000 ms  0/3 (0%)  sender
[ 11]   0.00-3.38   sec  4.28 KBytes  10.4 Kbits/sec  0.511 ms  0/3 (0%)  receiver
[ 13]   0.00-2.96   sec  4.28 KBytes  11.8 Kbits/sec  0.000 ms  0/3 (0%)  sender
[ 13]   0.00-3.38   sec  4.28 KBytes  10.4 Kbits/sec  0.194 ms  0/3 (0%)  receiver
[SUM]   0.00-2.96   sec  21.4 KBytes  59.2 Kbits/sec  0.000 ms  0/15 (0%)  sender
[SUM]   0.00-3.38   sec  21.4 KBytes  51.8 Kbits/sec  1.258 ms  0/15 (0%)  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=659 reclaimed_frames=70 before_free=171460 before_allocated=24746 after_free=171530 after_allocated=24676
====== iperf PARALLEL_UDP end: success ======

====== iperf PARALLEL_TCP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 127.0.0.1 port 49157 connected to 127.0.0.1 port 5001
[  7] local 127.0.0.1 port 49158 connected to 127.0.0.1 port 5001
[  9] local 127.0.0.1 port 49159 connected to 127.0.0.1 port 5001
[ 11] local 127.0.0.1 port 49160 connected to 127.0.0.1 port 5001
[ 13] local 127.0.0.1 port 49161 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.27   sec  2.25 MBytes  8.30 Mbits/sec    0   0.00 Bytes       
[  7]   0.00-2.27   sec  2.25 MBytes  8.30 Mbits/sec    0   0.00 Bytes       
[  9]   0.00-2.27   sec  2.25 MBytes  8.30 Mbits/sec    0   0.00 Bytes       
[ 11]   0.00-2.27   sec  2.25 MBytes  8.30 Mbits/sec    0   0.00 Bytes       
[ 13]   0.00-2.27   sec  2.25 MBytes  8.30 Mbits/sec    0   0.00 Bytes       
[SUM]   0.00-2.27   sec  11.2 MBytes  41.5 Mbits/sec    0             
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.27   sec  2.25 MBytes  8.30 Mbits/sec    0             sender
[  5]   0.00-2.79   sec  1.38 MBytes  4.13 Mbits/sec                  receiver
[  7]   0.00-2.27   sec  2.25 MBytes  8.30 Mbits/sec    0             sender
[  7]   0.00-2.79   sec  1.38 MBytes  4.13 Mbits/sec                  receiver
[  9]   0.00-2.27   sec  2.25 MBytes  8.30 Mbits/sec    0             sender
[  9]   0.00-2.79   sec  1.38 MBytes  4.13 Mbits/sec                  receiver
[ 11]   0.00-2.27   sec  2.25 MBytes  8.30 Mbits/sec    0             sender
[ 11]   0.00-2.79   sec  1.38 MBytes  4.13 Mbits/sec                  receiver
[ 13]   0.00-2.27   sec  2.25 MBytes  8.30 Mbits/sec    0             sender
[ 13]   0.00-2.79   sec  1.38 MBytes  4.13 Mbits/sec                  receiver
[SUM]   0.00-2.27   sec  11.2 MBytes  41.5 Mbits/sec    0             sender
[SUM]   0.00-2.79   sec  6.88 MBytes  20.7 Mbits/sec                  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=660 reclaimed_frames=69 before_free=171297 before_allocated=24909 after_free=171366 after_allocated=24840
====== iperf PARALLEL_TCP end: success ======

====== iperf REVERSE_UDP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 0.0.0.0 port 49158 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.13   sec  9.98 KBytes  38.4 Kbits/sec  6.452 ms  0/7 (0%)  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.83   sec  11.4 KBytes  33.1 Kbits/sec  0.000 ms  0/8 (0%)  sender
[  5]   0.00-2.13   sec  9.98 KBytes  38.4 Kbits/sec  6.452 ms  0/7 (0%)  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=661 reclaimed_frames=69 before_free=171451 before_allocated=24755 after_free=171520 after_allocated=24686
====== iperf REVERSE_UDP end: success ======

====== iperf REVERSE_TCP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 127.0.0.1 port 49164 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.02   sec  1.50 MBytes  6.23 Mbits/sec                  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.51   sec  2.50 MBytes  8.34 Mbits/sec    0             sender
[  5]   0.00-2.02   sec  1.50 MBytes  6.23 Mbits/sec                  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=662 reclaimed_frames=69 before_free=171412 before_allocated=24794 after_free=171481 after_allocated=24725
====== iperf REVERSE_TCP end: success ======

#### OS COMP TEST GROUP END iperf-musl ####
frame-allocator-diagnostic: process-teardown pid=663 reclaimed_frames=509 before_free=170997 before_allocated=25209 after_free=171506 after_allocated=24700
frame-allocator-diagnostic: process-teardown pid=652 reclaimed_frames=514 before_free=171506 before_allocated=24700 after_free=172020 after_allocated=24186
frame-allocator-diagnostic: process-teardown pid=650 reclaimed_frames=510 before_free=172022 before_allocated=24184 after_free=172532 after_allocated=23674
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
frame-allocator-diagnostic: process-teardown pid=665 reclaimed_frames=508 before_free=171496 before_allocated=24710 after_free=172004 after_allocated=24202
#### OS COMP TEST GROUP START lua-musl ####
frame-allocator-diagnostic: process-teardown pid=667 reclaimed_frames=509 before_free=170967 before_allocated=25239 after_free=171476 after_allocated=24730
frame-allocator-diagnostic: process-teardown pid=669 reclaimed_frames=95 before_free=170853 before_allocated=25353 after_free=170948 after_allocated=25258
testcase lua date.lua success
frame-allocator-diagnostic: process-teardown pid=668 reclaimed_frames=513 before_free=170947 before_allocated=25259 after_free=171460 after_allocated=24746
frame-allocator-diagnostic: process-teardown pid=671 reclaimed_frames=95 before_free=170837 before_allocated=25369 after_free=170932 after_allocated=25274
testcase lua file_io.lua success
frame-allocator-diagnostic: process-teardown pid=670 reclaimed_frames=513 before_free=170931 before_allocated=25275 after_free=171444 after_allocated=24762
frame-allocator-diagnostic: process-teardown pid=673 reclaimed_frames=95 before_free=170821 before_allocated=25385 after_free=170916 after_allocated=25290
testcase lua max_min.lua success
frame-allocator-diagnostic: process-teardown pid=672 reclaimed_frames=513 before_free=170915 before_allocated=25291 after_free=171428 after_allocated=24778
frame-allocator-diagnostic: process-teardown pid=675 reclaimed_frames=95 before_free=170805 before_allocated=25401 after_free=170900 after_allocated=25306
testcase lua random.lua success
frame-allocator-diagnostic: process-teardown pid=674 reclaimed_frames=513 before_free=170899 before_allocated=25307 after_free=171412 after_allocated=24794
frame-allocator-diagnostic: process-teardown pid=677 reclaimed_frames=95 before_free=170789 before_allocated=25417 after_free=170884 after_allocated=25322
testcase lua remove.lua success
frame-allocator-diagnostic: process-teardown pid=676 reclaimed_frames=513 before_free=170883 before_allocated=25323 after_free=171396 after_allocated=24810
frame-allocator-diagnostic: process-teardown pid=679 reclaimed_frames=96 before_free=170772 before_allocated=25434 after_free=170868 after_allocated=25338
testcase lua round_num.lua success
frame-allocator-diagnostic: process-teardown pid=678 reclaimed_frames=513 before_free=170867 before_allocated=25339 after_free=171380 after_allocated=24826
frame-allocator-diagnostic: process-teardown pid=681 reclaimed_frames=96 before_free=170756 before_allocated=25450 after_free=170852 after_allocated=25354
testcase lua sin30.lua success
frame-allocator-diagnostic: process-teardown pid=680 reclaimed_frames=513 before_free=170851 before_allocated=25355 after_free=171364 after_allocated=24842
frame-allocator-diagnostic: process-teardown pid=683 reclaimed_frames=95 before_free=170741 before_allocated=25465 after_free=170836 after_allocated=25370
testcase lua sort.lua success
frame-allocator-diagnostic: process-teardown pid=682 reclaimed_frames=513 before_free=170835 before_allocated=25371 after_free=171348 after_allocated=24858
frame-allocator-diagnostic: process-teardown pid=685 reclaimed_frames=95 before_free=170725 before_allocated=25481 after_free=170820 after_allocated=25386
testcase lua strings.lua success
frame-allocator-diagnostic: process-teardown pid=684 reclaimed_frames=513 before_free=170819 before_allocated=25387 after_free=171332 after_allocated=24874
#### OS COMP TEST GROUP END lua-musl ####
frame-allocator-diagnostic: process-teardown pid=686 reclaimed_frames=509 before_free=170815 before_allocated=25391 after_free=171324 after_allocated=24882
frame-allocator-diagnostic: process-teardown pid=666 reclaimed_frames=511 before_free=171325 before_allocated=24881 after_free=171836 after_allocated=24370
frame-allocator-diagnostic: process-teardown pid=664 reclaimed_frames=510 before_free=171838 before_allocated=24368 after_free=172348 after_allocated=23858
frame-allocator-diagnostic: process-teardown pid=688 reclaimed_frames=509 before_free=171311 before_allocated=24895 after_free=171820 after_allocated=24386
#### OS COMP TEST GROUP START netperf-musl ####
frame-allocator-diagnostic: process-teardown pid=690 reclaimed_frames=509 before_free=170783 before_allocated=25423 after_free=171292 after_allocated=24914
====== netperf UDP_STREAM begin ======
Starting netserver with host '127.0.0.1' port '12865' and family AF_UNSPEC
MIGRATED UDP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Socket  Message  Elapsed      Messages                
Size    Size     Time         Okay Errors   Throughput
bytes   bytes    secs            #      #   10^6bits/sec

 65536    1000   1.66            7      0       0.03
 65536           1.66            7              0.03

frame-allocator-diagnostic: process-teardown pid=693 reclaimed_frames=36 before_free=170693 before_allocated=25513 after_free=170729 after_allocated=25477
frame-allocator-diagnostic: process-teardown pid=692 reclaimed_frames=278 before_free=170729 before_allocated=25477 after_free=171007 after_allocated=25199
====== netperf UDP_STREAM end: success ======
====== netperf TCP_STREAM begin ======
MIGRATED TCP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Recv   Send    Send                          
Socket Socket  Message  Elapsed              
Size   Size    Size     Time     Throughput  
bytes  bytes   bytes    secs.    10^6bits/sec  

 65536  65536   1000    0.18       45.53   
frame-allocator-diagnostic: process-teardown pid=697 reclaimed_frames=36 before_free=170676 before_allocated=25530 after_free=170712 after_allocated=25494
frame-allocator-diagnostic: process-teardown pid=696 reclaimed_frames=278 before_free=170712 before_allocated=25494 after_free=170990 after_allocated=25216
====== netperf TCP_STREAM end: success ======
====== netperf UDP_RR begin ======
MIGRATED UDP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.71        4.10   
65536  65536 
frame-allocator-diagnostic: process-teardown pid=701 reclaimed_frames=15 before_free=170702 before_allocated=25504 after_free=170717 after_allocated=25489
frame-allocator-diagnostic: process-teardown pid=700 reclaimed_frames=257 before_free=170717 before_allocated=25489 after_free=170974 after_allocated=25232
====== netperf UDP_RR end: success ======
====== netperf TCP_RR begin ======
MIGRATED TCP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.17        5.11   
65536  65536 
frame-allocator-diagnostic: process-teardown pid=705 reclaimed_frames=15 before_free=170686 before_allocated=25520 after_free=170701 after_allocated=25505
frame-allocator-diagnostic: process-teardown pid=704 reclaimed_frames=257 before_free=170701 before_allocated=25505 after_free=170958 after_allocated=25248
====== netperf TCP_RR end: success ======
====== netperf TCP_CRR begin ======
MIGRATED TCP Connect/Request/Response TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.13        6.17   
65536  65536 
frame-allocator-diagnostic: process-teardown pid=709 reclaimed_frames=15 before_free=170670 before_allocated=25536 after_free=170685 after_allocated=25521
frame-allocator-diagnostic: process-teardown pid=708 reclaimed_frames=257 before_free=170685 before_allocated=25521 after_free=170942 after_allocated=25264
====== netperf TCP_CRR end: success ======
frame-allocator-diagnostic: process-teardown pid=712 reclaimed_frames=508 before_free=170426 before_allocated=25780 after_free=170934 after_allocated=25272
frame-allocator-diagnostic: process-teardown pid=691 reclaimed_frames=260 before_free=170934 before_allocated=25272 after_free=171194 after_allocated=25012
#### OS COMP TEST GROUP END netperf-musl ####
frame-allocator-diagnostic: process-teardown pid=713 reclaimed_frames=509 before_free=170677 before_allocated=25529 after_free=171186 after_allocated=25020
frame-allocator-diagnostic: process-teardown pid=689 reclaimed_frames=514 before_free=171186 before_allocated=25020 after_free=171700 after_allocated=24506
frame-allocator-diagnostic: process-teardown pid=687 reclaimed_frames=510 before_free=171702 before_allocated=24504 after_free=172212 after_allocated=23994
#### OS COMP TEST GROUP START unixbench-musl ####
SKIP: unixbench currently blocks on unresolved executable/runtime compatibility
#### OS COMP TEST GROUP END unixbench-musl ####
frame-allocator-diagnostic: process-teardown pid=715 reclaimed_frames=817 before_free=170561 before_allocated=25645 after_free=171378 after_allocated=24828
#### OS COMP TEST GROUP START basic-glibc ####
frame-allocator-diagnostic: process-teardown pid=717 reclaimed_frames=818 before_free=169727 before_allocated=26479 after_free=170545 after_allocated=25661
Testing brk :
========== START test_brk ==========
Before alloc,heap pos: 77824
After alloc,heap pos: 77888
Alloc again,heap pos: 77952
========== END test_brk ==========
frame-allocator-diagnostic: process-teardown pid=719 reclaimed_frames=49 before_free=169659 before_allocated=26547 after_free=169708 after_allocated=26498
Testing chdir :
========== START test_chdir ==========
chdir ret: 0
  current working dir : 
========== END test_chdir ==========
frame-allocator-diagnostic: process-teardown pid=720 reclaimed_frames=49 before_free=169647 before_allocated=26559 after_free=169696 after_allocated=26510
Testing clone :
========== START test_clone ==========
  Child says successfully!
frame-allocator-diagnostic: process-teardown pid=722 reclaimed_frames=1 before_free=169625 before_allocated=26581 after_free=169626 after_allocated=26580
clone process successfully.
pid:722
========== END test_clone ==========
frame-allocator-diagnostic: process-teardown pid=721 reclaimed_frames=51 before_free=169626 before_allocated=26580 after_free=169677 after_allocated=26529
Testing close :
========== START test_close ==========
  close 3 success.
========== END test_close ==========
frame-allocator-diagnostic: process-teardown pid=723 reclaimed_frames=49 before_free=169617 before_allocated=26589 after_free=169666 after_allocated=26540
Testing dup2 :
========== START test_dup2 ==========
  from fd 100
========== END test_dup2 ==========
frame-allocator-diagnostic: process-teardown pid=724 reclaimed_frames=49 before_free=169606 before_allocated=26600 after_free=169655 after_allocated=26551
Testing dup :
========== START test_dup ==========
  new fd is 3.
========== END test_dup ==========
frame-allocator-diagnostic: process-teardown pid=725 reclaimed_frames=49 before_free=169595 before_allocated=26611 after_free=169644 after_allocated=26562
Testing execve :
========== START test_execve ==========
  I am test_echo.
execve success.
========== END main ==========
frame-allocator-diagnostic: process-teardown pid=726 reclaimed_frames=49 before_free=169584 before_allocated=26622 after_free=169633 after_allocated=26573
Testing exit :
========== START test_exit ==========
frame-allocator-diagnostic: process-teardown pid=728 reclaimed_frames=0 before_free=169565 before_allocated=26641 after_free=169565 after_allocated=26641
exit OK.
========== END test_exit ==========
frame-allocator-diagnostic: process-teardown pid=727 reclaimed_frames=49 before_free=169565 before_allocated=26641 after_free=169614 after_allocated=26592
Testing fork :
========== START test_fork ==========
  child process.
frame-allocator-diagnostic: process-teardown pid=730 reclaimed_frames=1 before_free=169545 before_allocated=26661 after_free=169546 after_allocated=26660
  parent process. wstatus:0
========== END test_fork ==========
frame-allocator-diagnostic: process-teardown pid=729 reclaimed_frames=49 before_free=169546 before_allocated=26660 after_free=169595 after_allocated=26611
Testing fstat :
========== START test_fstat ==========
fstat ret: 0
fstat: dev: 1, inode: 1612857110, mode: 33206, nlink: 1, size: 52, atime: 0, mtime: 0, ctime: 0
========== END test_fstat ==========
frame-allocator-diagnostic: process-teardown pid=731 reclaimed_frames=49 before_free=169535 before_allocated=26671 after_free=169584 after_allocated=26622
Testing getcwd :
========== START test_getcwd ==========
getcwd: /tmp/testsuite/glibc/basic/basic successfully!
========== END test_getcwd ==========
frame-allocator-diagnostic: process-teardown pid=732 reclaimed_frames=49 before_free=169524 before_allocated=26682 after_free=169573 after_allocated=26633
Testing getdents :
========== START test_getdents ==========
open fd:3
getdents fd:-20
getdents success.


========== END test_getdents ==========
frame-allocator-diagnostic: process-teardown pid=733 reclaimed_frames=49 before_free=169513 before_allocated=26693 after_free=169562 after_allocated=26644
Testing getpid :
========== START test_getpid ==========
getpid success.
pid = 734
========== END test_getpid ==========
frame-allocator-diagnostic: process-teardown pid=734 reclaimed_frames=49 before_free=169502 before_allocated=26704 after_free=169551 after_allocated=26655
Testing getppid :
========== START test_getppid ==========
  getppid success. ppid : 718
========== END test_getppid ==========
frame-allocator-diagnostic: process-teardown pid=735 reclaimed_frames=49 before_free=169491 before_allocated=26715 after_free=169540 after_allocated=26666
Testing gettimeofday :
========== START test_gettimeofday ==========
gettimeofday success.
start:194511, end:194557
interval: 46
========== END test_gettimeofday ==========
frame-allocator-diagnostic: process-teardown pid=736 reclaimed_frames=49 before_free=169480 before_allocated=26726 after_free=169529 after_allocated=26677
Testing mkdir_ :
========== START test_mkdir ==========
mkdir ret: 0
  mkdir success.
========== END test_mkdir ==========
frame-allocator-diagnostic: process-teardown pid=737 reclaimed_frames=49 before_free=169469 before_allocated=26737 after_free=169518 after_allocated=26688
Testing mmap :
========== START test_mmap ==========
file len: 27
mmap content:   Hello, mmap successfully!
========== END test_mmap ==========
frame-allocator-diagnostic: process-teardown pid=738 reclaimed_frames=49 before_free=169458 before_allocated=26748 after_free=169507 after_allocated=26699
Testing mount :
========== START test_mount ==========
Mounting dev:/dev/vda2 to ./mnt
mount return: 0
mount successfully
umount return: 0
========== END test_mount ==========
frame-allocator-diagnostic: process-teardown pid=739 reclaimed_frames=49 before_free=169447 before_allocated=26759 after_free=169496 after_allocated=26710
Testing munmap :
========== START test_munmap ==========
file len: 27
munmap return: 0
munmap successfully!
========== END test_munmap ==========
frame-allocator-diagnostic: process-teardown pid=740 reclaimed_frames=49 before_free=169436 before_allocated=26770 after_free=169485 after_allocated=26721
Testing openat :
========== START test_openat ==========
open dir fd: 3
openat fd: 4
openat success.
========== END test_openat ==========
frame-allocator-diagnostic: process-teardown pid=741 reclaimed_frames=49 before_free=169425 before_allocated=26781 after_free=169474 after_allocated=26732
Testing open :
========== START test_open ==========
Hi, this is a text file.
syscalls testing success!

========== END test_open ==========
frame-allocator-diagnostic: process-teardown pid=742 reclaimed_frames=49 before_free=169414 before_allocated=26792 after_free=169463 after_allocated=26743
Testing pipe :
========== START test_pipe ==========
cpid: 744
cpid: 0
frame-allocator-diagnostic: process-teardown pid=744 reclaimed_frames=1 before_free=169394 before_allocated=26812 after_free=169395 after_allocated=26811
  Write to pipe successfully.

========== END test_pipe ==========
frame-allocator-diagnostic: process-teardown pid=743 reclaimed_frames=49 before_free=169395 before_allocated=26811 after_free=169444 after_allocated=26762
Testing read :
========== START test_read ==========
Hi, this is a text file.
syscalls testing success!

========== END test_read ==========
frame-allocator-diagnostic: process-teardown pid=745 reclaimed_frames=49 before_free=169384 before_allocated=26822 after_free=169433 after_allocated=26773
Testing sleep :
========== START test_sleep ==========
sleep success.
========== END test_sleep ==========
frame-allocator-diagnostic: process-teardown pid=746 reclaimed_frames=49 before_free=169373 before_allocated=26833 after_free=169422 after_allocated=26784
Testing times :
========== START test_times ==========
mytimes success
{tms_utime:0, tms_stime:0, tms_cutime:0, tms_cstime:0}
========== END test_times ==========
frame-allocator-diagnostic: process-teardown pid=747 reclaimed_frames=49 before_free=169362 before_allocated=26844 after_free=169411 after_allocated=26795
Testing umount :
========== START test_umount ==========
Mounting dev:/dev/vda2 to ./mnt
mount return: 0
umount success.
return: 0
========== END test_umount ==========
frame-allocator-diagnostic: process-teardown pid=748 reclaimed_frames=49 before_free=169351 before_allocated=26855 after_free=169400 after_allocated=26806
Testing uname :
========== START test_uname ==========
Uname: Linux arceos 6.0.0 ArceOS loongarch64 localdomain
========== END test_uname ==========
frame-allocator-diagnostic: process-teardown pid=749 reclaimed_frames=49 before_free=169340 before_allocated=26866 after_free=169389 after_allocated=26817
Testing unlink :
========== START test_unlink ==========
  unlink success!
========== END test_unlink ==========
frame-allocator-diagnostic: process-teardown pid=750 reclaimed_frames=49 before_free=169329 before_allocated=26877 after_free=169378 after_allocated=26828
Testing wait :
========== START test_wait ==========
This is child process
frame-allocator-diagnostic: process-teardown pid=752 reclaimed_frames=1 before_free=169309 before_allocated=26897 after_free=169310 after_allocated=26896
wait child success.
wstatus: 0
========== END test_wait ==========
frame-allocator-diagnostic: process-teardown pid=751 reclaimed_frames=49 before_free=169310 before_allocated=26896 after_free=169359 after_allocated=26847
Testing waitpid :
========== START test_waitpid ==========
This is child process
frame-allocator-diagnostic: process-teardown pid=754 reclaimed_frames=2 before_free=169289 before_allocated=26917 after_free=169291 after_allocated=26915
waitpid successfully.
wstatus: 3
========== END test_waitpid ==========
frame-allocator-diagnostic: process-teardown pid=753 reclaimed_frames=49 before_free=169291 before_allocated=26915 after_free=169340 after_allocated=26866
Testing write :
========== START test_write ==========
Hello operating system contest.
========== END test_write ==========
frame-allocator-diagnostic: process-teardown pid=755 reclaimed_frames=49 before_free=169280 before_allocated=26926 after_free=169329 after_allocated=26877
Testing yield :
========== START test_yield ==========
  I am child process: 757. iteration 0.
  I am child process: 758. iteration 1.
  I am child process: 759. iteration 2.
  I am child process: 757. iteration 0.
  I am child process: 758. iteration 1.
  I am child process: 759. iteration 2.
  I am child process: 757. iteration 0.
  I am child process: 758. iteration 1.
  I am child process: 759. iteration 2.
  I am child process: 757. iteration 0.
  I am child process: 758. iteration 1.
  I am child process: 759. iteration 2.
  I am child process: 757. iteration 0.
frame-allocator-diagnostic: process-teardown pid=757 reclaimed_frames=1 before_free=169242 before_allocated=26964 after_free=169243 after_allocated=26963
  I am child process: 758. iteration 1.
frame-allocator-diagnostic: process-teardown pid=758 reclaimed_frames=1 before_free=169243 before_allocated=26963 after_free=169244 after_allocated=26962
  I am child process: 759. iteration 2.
frame-allocator-diagnostic: process-teardown pid=759 reclaimed_frames=1 before_free=169244 before_allocated=26962 after_free=169245 after_allocated=26961
========== END test_yield ==========
frame-allocator-diagnostic: process-teardown pid=756 reclaimed_frames=49 before_free=169245 before_allocated=26961 after_free=169294 after_allocated=26912
frame-allocator-diagnostic: process-teardown pid=718 reclaimed_frames=820 before_free=169294 before_allocated=26912 after_free=170114 after_allocated=26092
#### OS COMP TEST GROUP END basic-glibc ####
frame-allocator-diagnostic: process-teardown pid=760 reclaimed_frames=818 before_free=169288 before_allocated=26918 after_free=170106 after_allocated=26100
frame-allocator-diagnostic: process-teardown pid=716 reclaimed_frames=820 before_free=170106 before_allocated=26100 after_free=170926 after_allocated=25280
frame-allocator-diagnostic: process-teardown pid=714 reclaimed_frames=820 before_free=170926 before_allocated=25280 after_free=171746 after_allocated=24460
#### OS COMP TEST GROUP START busybox-glibc ####
frame-allocator-diagnostic: process-teardown pid=762 reclaimed_frames=817 before_free=170095 before_allocated=26111 after_free=170912 after_allocated=25294
#### independent command test
frame-allocator-diagnostic: process-teardown pid=763 reclaimed_frames=817 before_free=170088 before_allocated=26118 after_free=170905 after_allocated=25301
frame-allocator-diagnostic: process-teardown pid=761 reclaimed_frames=820 before_free=170905 before_allocated=25301 after_free=171725 after_allocated=24481
testcase busybox echo "#### independent command test" success
frame-allocator-diagnostic: process-teardown pid=765 reclaimed_frames=817 before_free=170074 before_allocated=26132 after_free=170891 after_allocated=25315
frame-allocator-diagnostic: process-teardown pid=766 reclaimed_frames=819 before_free=170065 before_allocated=26141 after_free=170884 after_allocated=25322
frame-allocator-diagnostic: process-teardown pid=764 reclaimed_frames=820 before_free=170884 before_allocated=25322 after_free=171704 after_allocated=24502
testcase busybox ash -c exit success
frame-allocator-diagnostic: process-teardown pid=768 reclaimed_frames=817 before_free=170053 before_allocated=26153 after_free=170870 after_allocated=25336
frame-allocator-diagnostic: process-teardown pid=769 reclaimed_frames=818 before_free=170045 before_allocated=26161 after_free=170863 after_allocated=25343
frame-allocator-diagnostic: process-teardown pid=767 reclaimed_frames=820 before_free=170863 before_allocated=25343 after_free=171683 after_allocated=24523
testcase busybox sh -c exit success
frame-allocator-diagnostic: process-teardown pid=771 reclaimed_frames=817 before_free=170032 before_allocated=26174 after_free=170849 after_allocated=25357
bbb
frame-allocator-diagnostic: process-teardown pid=772 reclaimed_frames=817 before_free=170025 before_allocated=26181 after_free=170842 after_allocated=25364
frame-allocator-diagnostic: process-teardown pid=770 reclaimed_frames=820 before_free=170842 before_allocated=25364 after_free=171662 after_allocated=24544
testcase busybox basename /aaa/bbb success
frame-allocator-diagnostic: process-teardown pid=774 reclaimed_frames=817 before_free=170011 before_allocated=26195 after_free=170828 after_allocated=25378
    January 1970
Su Mo Tu We Th Fr Sa
             1  2  3
 4  5  6  7  8  9 10
11 12 13 14 15 16 17
18 19 20 21 22 23 24
25 26 27 28 29 30 31
                     
frame-allocator-diagnostic: process-teardown pid=775 reclaimed_frames=818 before_free=170003 before_allocated=26203 after_free=170821 after_allocated=25385
frame-allocator-diagnostic: process-teardown pid=773 reclaimed_frames=820 before_free=170821 before_allocated=25385 after_free=171641 after_allocated=24565
testcase busybox cal success
frame-allocator-diagnostic: process-teardown pid=777 reclaimed_frames=817 before_free=169990 before_allocated=26216 after_free=170807 after_allocated=25399
frame-allocator-diagnostic: process-teardown pid=778 reclaimed_frames=817 before_free=169983 before_allocated=26223 after_free=170800 after_allocated=25406
frame-allocator-diagnostic: process-teardown pid=776 reclaimed_frames=820 before_free=170800 before_allocated=25406 after_free=171620 after_allocated=24586
testcase busybox clear success
frame-allocator-diagnostic: process-teardown pid=780 reclaimed_frames=817 before_free=169969 before_allocated=26237 after_free=170786 after_allocated=25420
Thu Jan  1 00:03:56 UTC 1970
frame-allocator-diagnostic: process-teardown pid=781 reclaimed_frames=818 before_free=169961 before_allocated=26245 after_free=170779 after_allocated=25427
frame-allocator-diagnostic: process-teardown pid=779 reclaimed_frames=820 before_free=170779 before_allocated=25427 after_free=171599 after_allocated=24607
testcase busybox date success
frame-allocator-diagnostic: process-teardown pid=783 reclaimed_frames=817 before_free=169948 before_allocated=26258 after_free=170765 after_allocated=25441
Filesystem           1K-blocks      Used Available Use% Mounted on
devfs                   784824    105068    679756  13% /dev
tmpfs                   784824    105068    679756  13% /tmp
tmpfs                   784824    105068    679756  13% /var
proc                    784824    105068    679756  13% /proc
sysfs                   784824    105068    679756  13% /sys
frame-allocator-diagnostic: process-teardown pid=784 reclaimed_frames=819 before_free=169939 before_allocated=26267 after_free=170758 after_allocated=25448
frame-allocator-diagnostic: process-teardown pid=782 reclaimed_frames=820 before_free=170758 before_allocated=25448 after_free=171578 after_allocated=24628
testcase busybox df success
frame-allocator-diagnostic: process-teardown pid=786 reclaimed_frames=817 before_free=169927 before_allocated=26279 after_free=170744 after_allocated=25462
/aaa
frame-allocator-diagnostic: process-teardown pid=787 reclaimed_frames=818 before_free=169919 before_allocated=26287 after_free=170737 after_allocated=25469
frame-allocator-diagnostic: process-teardown pid=785 reclaimed_frames=820 before_free=170737 before_allocated=25469 after_free=171557 after_allocated=24649
testcase busybox dirname /aaa/bbb success
frame-allocator-diagnostic: process-teardown pid=789 reclaimed_frames=817 before_free=169906 before_allocated=26300 after_free=170723 after_allocated=25483
frame-allocator-diagnostic: process-teardown pid=790 reclaimed_frames=818 before_free=169898 before_allocated=26308 after_free=170716 after_allocated=25490
frame-allocator-diagnostic: process-teardown pid=788 reclaimed_frames=820 before_free=170716 before_allocated=25490 after_free=171536 after_allocated=24670
testcase busybox dmesg success
frame-allocator-diagnostic: process-teardown pid=792 reclaimed_frames=817 before_free=169885 before_allocated=26321 after_free=170702 after_allocated=25504
0	.
frame-allocator-diagnostic: process-teardown pid=793 reclaimed_frames=819 before_free=169876 before_allocated=26330 after_free=170695 after_allocated=25511
frame-allocator-diagnostic: process-teardown pid=791 reclaimed_frames=820 before_free=170695 before_allocated=25511 after_free=171515 after_allocated=24691
testcase busybox du success
frame-allocator-diagnostic: process-teardown pid=795 reclaimed_frames=817 before_free=169864 before_allocated=26342 after_free=170681 after_allocated=25525
2
frame-allocator-diagnostic: process-teardown pid=796 reclaimed_frames=818 before_free=169856 before_allocated=26350 after_free=170674 after_allocated=25532
frame-allocator-diagnostic: process-teardown pid=794 reclaimed_frames=820 before_free=170674 before_allocated=25532 after_free=171494 after_allocated=24712
testcase busybox expr 1 + 1 success
frame-allocator-diagnostic: process-teardown pid=798 reclaimed_frames=817 before_free=169843 before_allocated=26363 after_free=170660 after_allocated=25546
frame-allocator-diagnostic: process-teardown pid=799 reclaimed_frames=817 before_free=169836 before_allocated=26370 after_free=170653 after_allocated=25553
frame-allocator-diagnostic: process-teardown pid=797 reclaimed_frames=820 before_free=170653 before_allocated=25553 after_free=171473 after_allocated=24733
testcase busybox false success
frame-allocator-diagnostic: process-teardown pid=801 reclaimed_frames=817 before_free=169822 before_allocated=26384 after_free=170639 after_allocated=25567
frame-allocator-diagnostic: process-teardown pid=802 reclaimed_frames=817 before_free=169815 before_allocated=26391 after_free=170632 after_allocated=25574
frame-allocator-diagnostic: process-teardown pid=800 reclaimed_frames=820 before_free=170632 before_allocated=25574 after_free=171452 after_allocated=24754
testcase busybox true success
frame-allocator-diagnostic: process-teardown pid=804 reclaimed_frames=817 before_free=169801 before_allocated=26405 after_free=170618 after_allocated=25588
/glibc/ls
frame-allocator-diagnostic: process-teardown pid=805 reclaimed_frames=818 before_free=169793 before_allocated=26413 after_free=170611 after_allocated=25595
frame-allocator-diagnostic: process-teardown pid=803 reclaimed_frames=820 before_free=170611 before_allocated=25595 after_free=171431 after_allocated=24775
testcase busybox which ls success
frame-allocator-diagnostic: process-teardown pid=807 reclaimed_frames=817 before_free=169780 before_allocated=26426 after_free=170597 after_allocated=25609
Linux
frame-allocator-diagnostic: process-teardown pid=808 reclaimed_frames=818 before_free=169772 before_allocated=26434 after_free=170590 after_allocated=25616
frame-allocator-diagnostic: process-teardown pid=806 reclaimed_frames=820 before_free=170590 before_allocated=25616 after_free=171410 after_allocated=24796
testcase busybox uname success
frame-allocator-diagnostic: process-teardown pid=810 reclaimed_frames=817 before_free=169759 before_allocated=26447 after_free=170576 after_allocated=25630
 00:04:29 up 0 min,  0 users,  load average: 0.00, 0.00, 0.00
frame-allocator-diagnostic: process-teardown pid=811 reclaimed_frames=818 before_free=169751 before_allocated=26455 after_free=170569 after_allocated=25637
frame-allocator-diagnostic: process-teardown pid=809 reclaimed_frames=820 before_free=170569 before_allocated=25637 after_free=171389 after_allocated=24817
testcase busybox uptime success
frame-allocator-diagnostic: process-teardown pid=813 reclaimed_frames=817 before_free=169738 before_allocated=26468 after_free=170555 after_allocated=25651
abc
frame-allocator-diagnostic: process-teardown pid=814 reclaimed_frames=818 before_free=169730 before_allocated=26476 after_free=170548 after_allocated=25658
frame-allocator-diagnostic: process-teardown pid=812 reclaimed_frames=820 before_free=170548 before_allocated=25658 after_free=171368 after_allocated=24838
testcase busybox printf "abc\n" success
frame-allocator-diagnostic: process-teardown pid=816 reclaimed_frames=817 before_free=169717 before_allocated=26489 after_free=170534 after_allocated=25672
PID   USER     TIME  COMMAND
frame-allocator-diagnostic: process-teardown pid=817 reclaimed_frames=819 before_free=169708 before_allocated=26498 after_free=170527 after_allocated=25679
frame-allocator-diagnostic: process-teardown pid=815 reclaimed_frames=820 before_free=170527 before_allocated=25679 after_free=171347 after_allocated=24859
testcase busybox ps success
frame-allocator-diagnostic: process-teardown pid=819 reclaimed_frames=817 before_free=169696 before_allocated=26510 after_free=170513 after_allocated=25693
/tmp/testsuite/glibc/busybox
frame-allocator-diagnostic: process-teardown pid=820 reclaimed_frames=818 before_free=169688 before_allocated=26518 after_free=170506 after_allocated=25700
frame-allocator-diagnostic: process-teardown pid=818 reclaimed_frames=820 before_free=170506 before_allocated=25700 after_free=171326 after_allocated=24880
testcase busybox pwd success
frame-allocator-diagnostic: process-teardown pid=822 reclaimed_frames=817 before_free=169675 before_allocated=26531 after_free=170492 after_allocated=25714
              total        used        free      shared  buff/cache   available
Mem:              0           0           0           0           0      781966
-/+ buffers/cache:            0           0
Swap:             0           0           0
frame-allocator-diagnostic: process-teardown pid=823 reclaimed_frames=818 before_free=169667 before_allocated=26539 after_free=170485 after_allocated=25721
frame-allocator-diagnostic: process-teardown pid=821 reclaimed_frames=820 before_free=170485 before_allocated=25721 after_free=171305 after_allocated=24901
testcase busybox free success
frame-allocator-diagnostic: process-teardown pid=825 reclaimed_frames=817 before_free=169654 before_allocated=26552 after_free=170471 after_allocated=25735
Thu Jan  1 00:04:45 1970  0.000000 seconds
frame-allocator-diagnostic: process-teardown pid=826 reclaimed_frames=818 before_free=169646 before_allocated=26560 after_free=170464 after_allocated=25742
frame-allocator-diagnostic: process-teardown pid=824 reclaimed_frames=820 before_free=170464 before_allocated=25742 after_free=171284 after_allocated=24922
testcase busybox hwclock success
frame-allocator-diagnostic: process-teardown pid=828 reclaimed_frames=817 before_free=169633 before_allocated=26573 after_free=170450 after_allocated=25756
frame-allocator-diagnostic: process-teardown pid=830 reclaimed_frames=817 before_free=168790 before_allocated=27416 after_free=169607 after_allocated=26599
frame-allocator-diagnostic: process-teardown pid=827 reclaimed_frames=820 before_free=168794 before_allocated=27412 after_free=169614 after_allocated=26592
testcase busybox sh -c 'sleep 5' & /glibc/busybox kill $! success
frame-allocator-diagnostic: process-teardown pid=833 reclaimed_frames=817 before_free=167963 before_allocated=28243 after_free=168780 after_allocated=27426
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
frame-allocator-diagnostic: process-teardown pid=834 reclaimed_frames=820 before_free=167953 before_allocated=28253 after_free=168773 after_allocated=27433
frame-allocator-diagnostic: process-teardown pid=832 reclaimed_frames=820 before_free=168773 before_allocated=27433 after_free=169593 after_allocated=26613
testcase busybox ls success
frame-allocator-diagnostic: process-teardown pid=836 reclaimed_frames=817 before_free=167942 before_allocated=28264 after_free=168759 after_allocated=27447
frame-allocator-diagnostic: process-teardown pid=831 reclaimed_frames=817 before_free=168759 before_allocated=27447 after_free=169576 after_allocated=26630
frame-allocator-diagnostic: process-teardown pid=829 reclaimed_frames=818 before_free=169564 before_allocated=26642 after_free=170382 after_allocated=25824
frame-allocator-diagnostic: process-teardown pid=837 reclaimed_frames=817 before_free=169570 before_allocated=26636 after_free=170387 after_allocated=25819
frame-allocator-diagnostic: process-teardown pid=835 reclaimed_frames=820 before_free=170387 before_allocated=25819 after_free=171207 after_allocated=24999
testcase busybox sleep 1 success
frame-allocator-diagnostic: process-teardown pid=839 reclaimed_frames=817 before_free=169556 before_allocated=26650 after_free=170373 after_allocated=25833
#### file opration test
frame-allocator-diagnostic: process-teardown pid=840 reclaimed_frames=817 before_free=169549 before_allocated=26657 after_free=170366 after_allocated=25840
frame-allocator-diagnostic: process-teardown pid=838 reclaimed_frames=820 before_free=170366 before_allocated=25840 after_free=171186 after_allocated=25020
testcase busybox echo "#### file opration test" success
frame-allocator-diagnostic: process-teardown pid=842 reclaimed_frames=817 before_free=169535 before_allocated=26671 after_free=170352 after_allocated=25854
frame-allocator-diagnostic: process-teardown pid=843 reclaimed_frames=817 before_free=169528 before_allocated=26678 after_free=170345 after_allocated=25861
frame-allocator-diagnostic: process-teardown pid=841 reclaimed_frames=820 before_free=170345 before_allocated=25861 after_free=171165 after_allocated=25041
testcase busybox touch test.txt success
frame-allocator-diagnostic: process-teardown pid=845 reclaimed_frames=817 before_free=169514 before_allocated=26692 after_free=170331 after_allocated=25875
frame-allocator-diagnostic: process-teardown pid=846 reclaimed_frames=817 before_free=169507 before_allocated=26699 after_free=170324 after_allocated=25882
frame-allocator-diagnostic: process-teardown pid=844 reclaimed_frames=820 before_free=170324 before_allocated=25882 after_free=171144 after_allocated=25062
testcase busybox echo "hello world" > test.txt success
frame-allocator-diagnostic: process-teardown pid=848 reclaimed_frames=817 before_free=169493 before_allocated=26713 after_free=170310 after_allocated=25896
hello world
frame-allocator-diagnostic: process-teardown pid=849 reclaimed_frames=817 before_free=169486 before_allocated=26720 after_free=170303 after_allocated=25903
frame-allocator-diagnostic: process-teardown pid=847 reclaimed_frames=820 before_free=170303 before_allocated=25903 after_free=171123 after_allocated=25083
testcase busybox cat test.txt success
frame-allocator-diagnostic: process-teardown pid=851 reclaimed_frames=817 before_free=169472 before_allocated=26734 after_free=170289 after_allocated=25917
l
frame-allocator-diagnostic: process-teardown pid=852 reclaimed_frames=818 before_free=169464 before_allocated=26742 after_free=170282 after_allocated=25924
frame-allocator-diagnostic: process-teardown pid=850 reclaimed_frames=820 before_free=170282 before_allocated=25924 after_free=171102 after_allocated=25104
testcase busybox cut -c 3 test.txt success
frame-allocator-diagnostic: process-teardown pid=854 reclaimed_frames=817 before_free=169451 before_allocated=26755 after_free=170268 after_allocated=25938
0000000 062550 066154 020157 067567 066162 005144
0000014
frame-allocator-diagnostic: process-teardown pid=855 reclaimed_frames=818 before_free=169443 before_allocated=26763 after_free=170261 after_allocated=25945
frame-allocator-diagnostic: process-teardown pid=853 reclaimed_frames=820 before_free=170261 before_allocated=25945 after_free=171081 after_allocated=25125
testcase busybox od test.txt success
frame-allocator-diagnostic: process-teardown pid=857 reclaimed_frames=817 before_free=169430 before_allocated=26776 after_free=170247 after_allocated=25959
hello world
frame-allocator-diagnostic: process-teardown pid=858 reclaimed_frames=818 before_free=169422 before_allocated=26784 after_free=170240 after_allocated=25966
frame-allocator-diagnostic: process-teardown pid=856 reclaimed_frames=820 before_free=170240 before_allocated=25966 after_free=171060 after_allocated=25146
testcase busybox head test.txt success
frame-allocator-diagnostic: process-teardown pid=860 reclaimed_frames=817 before_free=169409 before_allocated=26797 after_free=170226 after_allocated=25980
hello world
frame-allocator-diagnostic: process-teardown pid=861 reclaimed_frames=819 before_free=169400 before_allocated=26806 after_free=170219 after_allocated=25987
frame-allocator-diagnostic: process-teardown pid=859 reclaimed_frames=820 before_free=170219 before_allocated=25987 after_free=171039 after_allocated=25167
testcase busybox tail test.txt success
frame-allocator-diagnostic: process-teardown pid=863 reclaimed_frames=817 before_free=169388 before_allocated=26818 after_free=170205 after_allocated=26001
00000000  68 65 6c 6c 6f 20 77 6f  72 6c 64 0a              |hello world.|
0000000c
frame-allocator-diagnostic: process-teardown pid=864 reclaimed_frames=818 before_free=169380 before_allocated=26826 after_free=170198 after_allocated=26008
frame-allocator-diagnostic: process-teardown pid=862 reclaimed_frames=820 before_free=170198 before_allocated=26008 after_free=171018 after_allocated=25188
testcase busybox hexdump -C test.txt success
frame-allocator-diagnostic: process-teardown pid=866 reclaimed_frames=817 before_free=169367 before_allocated=26839 after_free=170184 after_allocated=26022
6f5902ac237024bdd0c176cb93063dc4  test.txt
frame-allocator-diagnostic: process-teardown pid=867 reclaimed_frames=819 before_free=169358 before_allocated=26848 after_free=170177 after_allocated=26029
frame-allocator-diagnostic: process-teardown pid=865 reclaimed_frames=820 before_free=170177 before_allocated=26029 after_free=170997 after_allocated=25209
testcase busybox md5sum test.txt success
frame-allocator-diagnostic: process-teardown pid=869 reclaimed_frames=817 before_free=169346 before_allocated=26860 after_free=170163 after_allocated=26043
frame-allocator-diagnostic: process-teardown pid=870 reclaimed_frames=817 before_free=169339 before_allocated=26867 after_free=170156 after_allocated=26050
frame-allocator-diagnostic: process-teardown pid=868 reclaimed_frames=820 before_free=170156 before_allocated=26050 after_free=170976 after_allocated=25230
testcase busybox echo "ccccccc" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=872 reclaimed_frames=817 before_free=169325 before_allocated=26881 after_free=170142 after_allocated=26064
frame-allocator-diagnostic: process-teardown pid=873 reclaimed_frames=817 before_free=169318 before_allocated=26888 after_free=170135 after_allocated=26071
frame-allocator-diagnostic: process-teardown pid=871 reclaimed_frames=820 before_free=170135 before_allocated=26071 after_free=170955 after_allocated=25251
testcase busybox echo "bbbbbbb" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=875 reclaimed_frames=817 before_free=169304 before_allocated=26902 after_free=170121 after_allocated=26085
frame-allocator-diagnostic: process-teardown pid=876 reclaimed_frames=817 before_free=169297 before_allocated=26909 after_free=170114 after_allocated=26092
frame-allocator-diagnostic: process-teardown pid=874 reclaimed_frames=820 before_free=170114 before_allocated=26092 after_free=170934 after_allocated=25272
testcase busybox echo "aaaaaaa" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=878 reclaimed_frames=817 before_free=169283 before_allocated=26923 after_free=170100 after_allocated=26106
frame-allocator-diagnostic: process-teardown pid=879 reclaimed_frames=817 before_free=169276 before_allocated=26930 after_free=170093 after_allocated=26113
frame-allocator-diagnostic: process-teardown pid=877 reclaimed_frames=820 before_free=170093 before_allocated=26113 after_free=170913 after_allocated=25293
testcase busybox echo "2222222" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=881 reclaimed_frames=817 before_free=169262 before_allocated=26944 after_free=170079 after_allocated=26127
frame-allocator-diagnostic: process-teardown pid=882 reclaimed_frames=817 before_free=169255 before_allocated=26951 after_free=170072 after_allocated=26134
frame-allocator-diagnostic: process-teardown pid=880 reclaimed_frames=820 before_free=170072 before_allocated=26134 after_free=170892 after_allocated=25314
testcase busybox echo "1111111" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=884 reclaimed_frames=817 before_free=169241 before_allocated=26965 after_free=170058 after_allocated=26148
frame-allocator-diagnostic: process-teardown pid=885 reclaimed_frames=817 before_free=169234 before_allocated=26972 after_free=170051 after_allocated=26155
frame-allocator-diagnostic: process-teardown pid=883 reclaimed_frames=820 before_free=170051 before_allocated=26155 after_free=170871 after_allocated=25335
testcase busybox echo "bbbbbbb" >> test.txt success
frame-allocator-diagnostic: process-teardown pid=887 reclaimed_frames=817 before_free=169220 before_allocated=26986 after_free=170037 after_allocated=26169
frame-allocator-diagnostic: process-teardown pid=888 reclaimed_frames=818 before_free=169200 before_allocated=27006 after_free=170018 after_allocated=26188
1111111
2222222
aaaaaaa
bbbbbbb
ccccccc
hello world
frame-allocator-diagnostic: process-teardown pid=889 reclaimed_frames=818 before_free=169205 before_allocated=27001 after_free=170023 after_allocated=26183
frame-allocator-diagnostic: process-teardown pid=886 reclaimed_frames=820 before_free=170023 before_allocated=26183 after_free=170843 after_allocated=25363
testcase busybox sort test.txt | /glibc/busybox uniq success
frame-allocator-diagnostic: process-teardown pid=891 reclaimed_frames=817 before_free=169192 before_allocated=27014 after_free=170009 after_allocated=26197
  File: test.txt
  Size: 60        	Blocks: 0          IO Block: 512    regular file
Device: 1h/1d	Inode: 4368043057645409086  Links: 1
Access: (0666/-rw-rw-rw-)  Uid: (    0/    root)   Gid: (    0/    root)
Access: 1970-01-01 00:00:00.000000000 +0000
Modify: 1970-01-01 00:00:00.000000000 +0000
Change: 1970-01-01 00:00:00.000000000 +0000
frame-allocator-diagnostic: process-teardown pid=892 reclaimed_frames=818 before_free=169184 before_allocated=27022 after_free=170002 after_allocated=26204
frame-allocator-diagnostic: process-teardown pid=890 reclaimed_frames=820 before_free=170002 before_allocated=26204 after_free=170822 after_allocated=25384
testcase busybox stat test.txt success
frame-allocator-diagnostic: process-teardown pid=894 reclaimed_frames=817 before_free=169171 before_allocated=27035 after_free=169988 after_allocated=26218
hello world
ccccccc
bbbbbbb
aaaaaaa
2222222
1111111
bbbbbbb
frame-allocator-diagnostic: process-teardown pid=895 reclaimed_frames=818 before_free=169163 before_allocated=27043 after_free=169981 after_allocated=26225
frame-allocator-diagnostic: process-teardown pid=893 reclaimed_frames=820 before_free=169981 before_allocated=26225 after_free=170801 after_allocated=25405
testcase busybox strings test.txt success
frame-allocator-diagnostic: process-teardown pid=897 reclaimed_frames=817 before_free=169150 before_allocated=27056 after_free=169967 after_allocated=26239
        7         8        60 test.txt
frame-allocator-diagnostic: process-teardown pid=898 reclaimed_frames=818 before_free=169142 before_allocated=27064 after_free=169960 after_allocated=26246
frame-allocator-diagnostic: process-teardown pid=896 reclaimed_frames=820 before_free=169960 before_allocated=26246 after_free=170780 after_allocated=25426
testcase busybox wc test.txt success
frame-allocator-diagnostic: process-teardown pid=900 reclaimed_frames=817 before_free=169129 before_allocated=27077 after_free=169946 after_allocated=26260
frame-allocator-diagnostic: process-teardown pid=901 reclaimed_frames=818 before_free=169119 before_allocated=27087 after_free=169937 after_allocated=26269
frame-allocator-diagnostic: process-teardown pid=899 reclaimed_frames=822 before_free=169937 before_allocated=26269 after_free=170759 after_allocated=25447
testcase busybox [ -f test.txt ] success
frame-allocator-diagnostic: process-teardown pid=903 reclaimed_frames=817 before_free=169108 before_allocated=27098 after_free=169925 after_allocated=26281
hello world
ccccccc
bbbbbbb
aaaaaaa
2222222
1111111
bbbbbbb
frame-allocator-diagnostic: process-teardown pid=904 reclaimed_frames=817 before_free=169101 before_allocated=27105 after_free=169918 after_allocated=26288
frame-allocator-diagnostic: process-teardown pid=902 reclaimed_frames=820 before_free=169918 before_allocated=26288 after_free=170738 after_allocated=25468
testcase busybox more test.txt success
frame-allocator-diagnostic: process-teardown pid=906 reclaimed_frames=817 before_free=169087 before_allocated=27119 after_free=169904 after_allocated=26302
frame-allocator-diagnostic: process-teardown pid=907 reclaimed_frames=817 before_free=169080 before_allocated=27126 after_free=169897 after_allocated=26309
frame-allocator-diagnostic: process-teardown pid=905 reclaimed_frames=820 before_free=169897 before_allocated=26309 after_free=170717 after_allocated=25489
testcase busybox rm test.txt success
frame-allocator-diagnostic: process-teardown pid=909 reclaimed_frames=817 before_free=169066 before_allocated=27140 after_free=169883 after_allocated=26323
frame-allocator-diagnostic: process-teardown pid=910 reclaimed_frames=817 before_free=169059 before_allocated=27147 after_free=169876 after_allocated=26330
frame-allocator-diagnostic: process-teardown pid=908 reclaimed_frames=820 before_free=169876 before_allocated=26330 after_free=170696 after_allocated=25510
testcase busybox mkdir test_dir success
frame-allocator-diagnostic: process-teardown pid=912 reclaimed_frames=817 before_free=169045 before_allocated=27161 after_free=169862 after_allocated=26344
frame-allocator-diagnostic: process-teardown pid=913 reclaimed_frames=817 before_free=169038 before_allocated=27168 after_free=169855 after_allocated=26351
frame-allocator-diagnostic: process-teardown pid=911 reclaimed_frames=820 before_free=169855 before_allocated=26351 after_free=170675 after_allocated=25531
testcase busybox mv test_dir test success
frame-allocator-diagnostic: process-teardown pid=915 reclaimed_frames=817 before_free=169024 before_allocated=27182 after_free=169841 after_allocated=26365
frame-allocator-diagnostic: process-teardown pid=916 reclaimed_frames=817 before_free=169017 before_allocated=27189 after_free=169834 after_allocated=26372
frame-allocator-diagnostic: process-teardown pid=914 reclaimed_frames=820 before_free=169834 before_allocated=26372 after_free=170654 after_allocated=25552
testcase busybox rmdir test success
frame-allocator-diagnostic: process-teardown pid=918 reclaimed_frames=817 before_free=169003 before_allocated=27203 after_free=169820 after_allocated=26386
echo "hello world" > test.txt
grep hello busybox_cmd.txt
frame-allocator-diagnostic: process-teardown pid=919 reclaimed_frames=826 before_free=168987 before_allocated=27219 after_free=169813 after_allocated=26393
frame-allocator-diagnostic: process-teardown pid=917 reclaimed_frames=820 before_free=169813 before_allocated=26393 after_free=170633 after_allocated=25573
testcase busybox grep hello busybox_cmd.txt success
frame-allocator-diagnostic: process-teardown pid=921 reclaimed_frames=817 before_free=168982 before_allocated=27224 after_free=169799 after_allocated=26407
frame-allocator-diagnostic: process-teardown pid=922 reclaimed_frames=817 before_free=168975 before_allocated=27231 after_free=169792 after_allocated=26414
frame-allocator-diagnostic: process-teardown pid=920 reclaimed_frames=820 before_free=169792 before_allocated=26414 after_free=170612 after_allocated=25594
testcase busybox cp busybox_cmd.txt busybox_cmd.bak success
frame-allocator-diagnostic: process-teardown pid=924 reclaimed_frames=817 before_free=168961 before_allocated=27245 after_free=169778 after_allocated=26428
frame-allocator-diagnostic: process-teardown pid=925 reclaimed_frames=817 before_free=168954 before_allocated=27252 after_free=169771 after_allocated=26435
frame-allocator-diagnostic: process-teardown pid=923 reclaimed_frames=820 before_free=169771 before_allocated=26435 after_free=170591 after_allocated=25615
testcase busybox rm busybox_cmd.bak success
frame-allocator-diagnostic: process-teardown pid=927 reclaimed_frames=817 before_free=168940 before_allocated=27266 after_free=169757 after_allocated=26449
./busybox_cmd.txt
frame-allocator-diagnostic: process-teardown pid=928 reclaimed_frames=819 before_free=168931 before_allocated=27275 after_free=169750 after_allocated=26456
frame-allocator-diagnostic: process-teardown pid=926 reclaimed_frames=820 before_free=169750 before_allocated=26456 after_free=170570 after_allocated=25636
testcase busybox find -name "busybox_cmd.txt" success
#### OS COMP TEST GROUP END busybox-glibc ####
frame-allocator-diagnostic: process-teardown pid=930 reclaimed_frames=817 before_free=168919 before_allocated=27287 after_free=169736 after_allocated=26470
#### OS COMP TEST GROUP START cyclictest-glibc ####
frame-allocator-diagnostic: process-teardown pid=932 reclaimed_frames=818 before_free=168085 before_allocated=28121 after_free=168903 after_allocated=27303
====== cyclictest NO_STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 (  934) P:99 I:1000 C:      7 Min: 139621 Act:141101 Avg:160992 Max:  245703
frame-allocator-diagnostic: process-teardown pid=933 reclaimed_frames=564 before_free=168325 before_allocated=27881 after_free=168889 after_allocated=27317
====== cyclictest NO_STRESS_P1 end: success ======
====== cyclictest NO_STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 (  936) P:99 I:1000 C:      7 Min: 138678 Act:143004 Avg:159382 Max:  240381
T: 1 (  937) P:99 I:1500 C:      7 Min: 138694 Act:143414 Avg:159290 Max:  240277
T: 2 (  938) P:99 I:2000 C:      7 Min: 137986 Act:143418 Avg:159064 Max:  239263
T: 3 (  939) P:99 I:2500 C:      8 Min:      7 Act:143911 Avg:138977 Max:  238747
T: 4 (  940) P:99 I:3000 C:      7 Min: 137099 Act:143378 Avg:158577 Max:  240210
T: 5 (  941) P:99 I:3500 C:      7 Min: 136388 Act:141372 Avg:158631 Max:  241201
T: 6 (  942) P:99 I:4000 C:      7 Min: 135862 Act:141374 Avg:158113 Max:  241187
T: 7 (  943) P:99 I:4500 C:      7 Min: 135527 Act:140368 Avg:156383 Max:  238679
frame-allocator-diagnostic: process-teardown pid=935 reclaimed_frames=570 before_free=168296 before_allocated=27910 after_free=168866 after_allocated=27340
====== cyclictest NO_STRESS_P8 end: success ======
====== start hackbench ======
Running in process mode with 10 groups using 40 file descriptors each (== 400 tasks)
Each sender will pass 100000000 messages of 100 bytes
frame-allocator-diagnostic: process-teardown pid=945 reclaimed_frames=817 before_free=144294 before_allocated=51912 after_free=145111 after_allocated=51095
====== cyclictest STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 1347) P:99 I:1000 C:      4 Min: 270592 Act:310965 Avg:310415 Max:  388428
frame-allocator-diagnostic: process-teardown pid=1346 reclaimed_frames=564 before_free=144534 before_allocated=51672 after_free=145098 after_allocated=51108
====== cyclictest STRESS_P1 end: success ======
====== cyclictest STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 1349) P:99 I:1000 C:      4 Min: 280890 Act:411013 Avg:345765 Max:  411013
T: 1 ( 1350) P:99 I:1500 C:      4 Min: 280736 Act:410745 Avg:345277 Max:  410745
T: 2 ( 1351) P:99 I:2000 C:      4 Min: 279754 Act:411794 Avg:345397 Max:  411794
T: 3 ( 1352) P:99 I:2500 C:      4 Min: 279231 Act:410806 Avg:344758 Max:  410806
T: 4 ( 1353) P:99 I:3000 C:      4 Min: 280667 Act:411028 Avg:344921 Max:  411028
T: 5 ( 1354) P:99 I:3500 C:      4 Min: 278687 Act:409548 Avg:343910 Max:  409548
T: 6 ( 1355) P:99 I:4000 C:      4 Min: 279664 Act:410057 Avg:344396 Max:  410057
T: 7 ( 1356) P:99 I:4500 C:      4 Min: 277644 Act:409571 Avg:343384 Max:  409571
frame-allocator-diagnostic: process-teardown pid=1348 reclaimed_frames=570 before_free=144508 before_allocated=51698 after_free=145078 after_allocated=51128
====== cyclictest STRESS_P8 end: success ======
frame-allocator-diagnostic: process-teardown pid=1357 reclaimed_frames=817 before_free=144254 before_allocated=51952 after_free=145071 after_allocated=51135
frame-allocator-diagnostic: process-teardown pid=1358 reclaimed_frames=817 before_free=144247 before_allocated=51959 after_free=145064 after_allocated=51142
====== kill hackbench: success ======
#### OS COMP TEST GROUP END cyclictest-glibc ####
frame-allocator-diagnostic: process-teardown pid=1359 reclaimed_frames=818 before_free=144239 before_allocated=51967 after_free=145057 after_allocated=51149
frame-allocator-diagnostic: process-teardown pid=931 reclaimed_frames=824 before_free=145057 before_allocated=51149 after_free=145881 after_allocated=50325
frame-allocator-diagnostic: process-teardown pid=929 reclaimed_frames=820 before_free=145881 before_allocated=50325 after_free=146701 after_allocated=49505
#### OS COMP TEST GROUP START iozone-glibc ####
SKIP: iozone throughput mode currently hangs in the evaluator environment
#### OS COMP TEST GROUP END iozone-glibc ####
frame-allocator-diagnostic: process-teardown pid=1361 reclaimed_frames=817 before_free=145050 before_allocated=51156 after_free=145867 after_allocated=50339
#### OS COMP TEST GROUP START iperf-glibc ####
frame-allocator-diagnostic: process-teardown pid=1363 reclaimed_frames=818 before_free=144216 before_allocated=51990 after_free=145034 after_allocated=51172
frame-allocator-diagnostic: process-teardown pid=1364 reclaimed_frames=2 before_free=144676 before_allocated=51530 after_free=144678 after_allocated=51528
frame-allocator-diagnostic: process-teardown pid=1365 reclaimed_frames=342 before_free=144678 before_allocated=51528 after_free=145020 after_allocated=51186
====== iperf BASIC_UDP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 0.0.0.0 port 49193 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  5]   0.00-2.00   sec  4.28 KBytes  17.5 Kbits/sec  3  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.00   sec  4.28 KBytes  17.5 Kbits/sec  0.000 ms  0/3 (0%)  sender
[  5]   0.00-3.02   sec  4.28 KBytes  11.6 Kbits/sec  11.660 ms  0/3 (0%)  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=1366 reclaimed_frames=343 before_free=144665 before_allocated=51541 after_free=145008 after_allocated=51198
====== iperf BASIC_UDP end: success ======

====== iperf BASIC_TCP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 127.0.0.1 port 49177 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.01   sec  1.50 MBytes  6.27 Mbits/sec    0   0.00 Bytes       
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.01   sec  1.50 MBytes  6.27 Mbits/sec    0             sender
[  5]   0.00-2.99   sec   640 KBytes  1.75 Mbits/sec                  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=1367 reclaimed_frames=343 before_free=144622 before_allocated=51584 after_free=144965 after_allocated=51241
====== iperf BASIC_TCP end: success ======

====== iperf PARALLEL_UDP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 0.0.0.0 port 49194 connected to 127.0.0.1 port 5001
[  7] local 0.0.0.0 port 49195 connected to 127.0.0.1 port 5001
[  9] local 0.0.0.0 port 49196 connected to 127.0.0.1 port 5001
[ 11] local 0.0.0.0 port 49197 connected to 127.0.0.1 port 5001
[ 13] local 0.0.0.0 port 49198 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  5]   0.00-2.10   sec  1.43 KBytes  5.55 Kbits/sec  1  
[  7]   0.00-2.11   sec  1.43 KBytes  5.55 Kbits/sec  1  
[  9]   0.00-2.11   sec  1.43 KBytes  5.55 Kbits/sec  1  
[ 11]   0.00-2.11   sec  1.43 KBytes  5.55 Kbits/sec  1  
[ 13]   0.00-2.11   sec  1.43 KBytes  5.55 Kbits/sec  1  
[SUM]   0.00-2.10   sec  7.13 KBytes  27.7 Kbits/sec  5  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.10   sec  1.43 KBytes  5.55 Kbits/sec  0.000 ms  0/1 (0%)  sender
[  5]   0.00-3.12   sec  1.43 KBytes  3.74 Kbits/sec  0.000 ms  0/1 (0%)  receiver
[  7]   0.00-2.10   sec  1.43 KBytes  5.55 Kbits/sec  0.000 ms  0/1 (0%)  sender
[  7]   0.00-3.12   sec  1.43 KBytes  3.74 Kbits/sec  0.000 ms  0/1 (0%)  receiver
[  9]   0.00-2.10   sec  1.43 KBytes  5.55 Kbits/sec  0.000 ms  0/1 (0%)  sender
[  9]   0.00-3.12   sec  1.43 KBytes  3.74 Kbits/sec  0.000 ms  0/1 (0%)  receiver
[ 11]   0.00-2.10   sec  1.43 KBytes  5.55 Kbits/sec  0.000 ms  0/1 (0%)  sender
[ 11]   0.00-3.12   sec  1.43 KBytes  3.74 Kbits/sec  0.000 ms  0/1 (0%)  receiver
[ 13]   0.00-2.10   sec  1.43 KBytes  5.55 Kbits/sec  0.000 ms  0/1 (0%)  sender
[ 13]   0.00-3.12   sec  1.43 KBytes  3.74 Kbits/sec  0.000 ms  0/1 (0%)  receiver
[SUM]   0.00-2.10   sec  7.13 KBytes  27.7 Kbits/sec  0.000 ms  0/5 (0%)  sender
[SUM]   0.00-3.12   sec  7.13 KBytes  18.7 Kbits/sec  0.000 ms  0/5 (0%)  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=1368 reclaimed_frames=346 before_free=144634 before_allocated=51572 after_free=144980 after_allocated=51226
====== iperf PARALLEL_UDP end: success ======

====== iperf PARALLEL_TCP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 127.0.0.1 port 49180 connected to 127.0.0.1 port 5001
[  7] local 127.0.0.1 port 49181 connected to 127.0.0.1 port 5001
[  9] local 127.0.0.1 port 49182 connected to 127.0.0.1 port 5001
[ 11] local 127.0.0.1 port 49183 connected to 127.0.0.1 port 5001
[ 13] local 127.0.0.1 port 49184 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.18   sec  1.50 MBytes  5.77 Mbits/sec    0   0.00 Bytes       
[  7]   0.00-2.18   sec  1.50 MBytes  5.77 Mbits/sec    0   0.00 Bytes       
[  9]   0.00-2.18   sec  1.50 MBytes  5.77 Mbits/sec    0   0.00 Bytes       
[ 11]   0.00-2.18   sec  1.50 MBytes  5.77 Mbits/sec    0   0.00 Bytes       
[ 13]   0.00-2.18   sec  1.50 MBytes  5.77 Mbits/sec    0   0.00 Bytes       
[SUM]   0.00-2.18   sec  7.50 MBytes  28.9 Mbits/sec    0             
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.18   sec  1.50 MBytes  5.77 Mbits/sec    0             sender
[  5]   0.00-3.29   sec   640 KBytes  1.59 Mbits/sec                  receiver
[  7]   0.00-2.18   sec  1.50 MBytes  5.77 Mbits/sec    0             sender
[  7]   0.00-3.29   sec   640 KBytes  1.59 Mbits/sec                  receiver
[  9]   0.00-2.18   sec  1.50 MBytes  5.77 Mbits/sec    0             sender
[  9]   0.00-3.29   sec   640 KBytes  1.59 Mbits/sec                  receiver
[ 11]   0.00-2.18   sec  1.50 MBytes  5.77 Mbits/sec    0             sender
[ 11]   0.00-3.29   sec   640 KBytes  1.59 Mbits/sec                  receiver
[ 13]   0.00-2.18   sec  1.50 MBytes  5.77 Mbits/sec    0             sender
[ 13]   0.00-3.29   sec   640 KBytes  1.59 Mbits/sec                  receiver
[SUM]   0.00-2.18   sec  7.50 MBytes  28.9 Mbits/sec    0             sender
[SUM]   0.00-3.29   sec  3.12 MBytes  7.96 Mbits/sec                  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=1369 reclaimed_frames=346 before_free=144468 before_allocated=51738 after_free=144814 after_allocated=51392
====== iperf PARALLEL_TCP end: success ======

====== iperf REVERSE_UDP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 0.0.0.0 port 49199 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.38   sec  5.70 KBytes  19.6 Kbits/sec  14.329 ms  0/4 (0%)  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-3.53   sec  7.13 KBytes  16.5 Kbits/sec  0.000 ms  0/5 (0%)  sender
[  5]   0.00-2.38   sec  5.70 KBytes  19.6 Kbits/sec  14.329 ms  0/4 (0%)  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=1370 reclaimed_frames=343 before_free=144624 before_allocated=51582 after_free=144967 after_allocated=51239
====== iperf REVERSE_UDP end: success ======

====== iperf REVERSE_TCP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 127.0.0.1 port 49187 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.28   sec   896 KBytes  3.22 Mbits/sec                  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-3.43   sec  1.75 MBytes  4.28 Mbits/sec    0             sender
[  5]   0.00-2.28   sec   896 KBytes  3.22 Mbits/sec                  receiver

iperf Done.
frame-allocator-diagnostic: process-teardown pid=1371 reclaimed_frames=343 before_free=144584 before_allocated=51622 after_free=144927 after_allocated=51279
====== iperf REVERSE_TCP end: success ======

#### OS COMP TEST GROUP END iperf-glibc ####
frame-allocator-diagnostic: process-teardown pid=1372 reclaimed_frames=818 before_free=144135 before_allocated=52071 after_free=144953 after_allocated=51253
frame-allocator-diagnostic: process-teardown pid=1362 reclaimed_frames=825 before_free=144953 before_allocated=51253 after_free=145778 after_allocated=50428
frame-allocator-diagnostic: process-teardown pid=1360 reclaimed_frames=820 before_free=145778 before_allocated=50428 after_free=146598 after_allocated=49608
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
frame-allocator-diagnostic: process-teardown pid=1374 reclaimed_frames=817 before_free=144947 before_allocated=51259 after_free=145764 after_allocated=50442
#### OS COMP TEST GROUP START lua-glibc ####
frame-allocator-diagnostic: process-teardown pid=1376 reclaimed_frames=818 before_free=144113 before_allocated=52093 after_free=144931 after_allocated=51275
frame-allocator-diagnostic: process-teardown pid=1378 reclaimed_frames=355 before_free=143743 before_allocated=52463 after_free=144098 after_allocated=52108
testcase lua date.lua success
frame-allocator-diagnostic: process-teardown pid=1377 reclaimed_frames=821 before_free=144096 before_allocated=52110 after_free=144917 after_allocated=51289
frame-allocator-diagnostic: process-teardown pid=1380 reclaimed_frames=355 before_free=143729 before_allocated=52477 after_free=144084 after_allocated=52122
testcase lua file_io.lua success
frame-allocator-diagnostic: process-teardown pid=1379 reclaimed_frames=821 before_free=144082 before_allocated=52124 after_free=144903 after_allocated=51303
frame-allocator-diagnostic: process-teardown pid=1382 reclaimed_frames=355 before_free=143715 before_allocated=52491 after_free=144070 after_allocated=52136
testcase lua max_min.lua success
frame-allocator-diagnostic: process-teardown pid=1381 reclaimed_frames=821 before_free=144068 before_allocated=52138 after_free=144889 after_allocated=51317
frame-allocator-diagnostic: process-teardown pid=1384 reclaimed_frames=355 before_free=143701 before_allocated=52505 after_free=144056 after_allocated=52150
testcase lua random.lua success
frame-allocator-diagnostic: process-teardown pid=1383 reclaimed_frames=821 before_free=144054 before_allocated=52152 after_free=144875 after_allocated=51331
frame-allocator-diagnostic: process-teardown pid=1386 reclaimed_frames=355 before_free=143686 before_allocated=52520 after_free=144041 after_allocated=52165
testcase lua remove.lua success
frame-allocator-diagnostic: process-teardown pid=1385 reclaimed_frames=821 before_free=144039 before_allocated=52167 after_free=144860 after_allocated=51346
frame-allocator-diagnostic: process-teardown pid=1388 reclaimed_frames=355 before_free=143672 before_allocated=52534 after_free=144027 after_allocated=52179
testcase lua round_num.lua success
frame-allocator-diagnostic: process-teardown pid=1387 reclaimed_frames=821 before_free=144025 before_allocated=52181 after_free=144846 after_allocated=51360
frame-allocator-diagnostic: process-teardown pid=1390 reclaimed_frames=355 before_free=143658 before_allocated=52548 after_free=144013 after_allocated=52193
testcase lua sin30.lua success
frame-allocator-diagnostic: process-teardown pid=1389 reclaimed_frames=821 before_free=144011 before_allocated=52195 after_free=144832 after_allocated=51374
frame-allocator-diagnostic: process-teardown pid=1392 reclaimed_frames=355 before_free=143644 before_allocated=52562 after_free=143999 after_allocated=52207
testcase lua sort.lua success
frame-allocator-diagnostic: process-teardown pid=1391 reclaimed_frames=821 before_free=143997 before_allocated=52209 after_free=144818 after_allocated=51388
frame-allocator-diagnostic: process-teardown pid=1394 reclaimed_frames=355 before_free=143630 before_allocated=52576 after_free=143985 after_allocated=52221
testcase lua strings.lua success
frame-allocator-diagnostic: process-teardown pid=1393 reclaimed_frames=821 before_free=143983 before_allocated=52223 after_free=144804 after_allocated=51402
#### OS COMP TEST GROUP END lua-glibc ####
frame-allocator-diagnostic: process-teardown pid=1395 reclaimed_frames=818 before_free=143979 before_allocated=52227 after_free=144797 after_allocated=51409
frame-allocator-diagnostic: process-teardown pid=1375 reclaimed_frames=820 before_free=144797 before_allocated=51409 after_free=145617 after_allocated=50589
frame-allocator-diagnostic: process-teardown pid=1373 reclaimed_frames=820 before_free=145617 before_allocated=50589 after_free=146437 after_allocated=49769
frame-allocator-diagnostic: process-teardown pid=1397 reclaimed_frames=817 before_free=144786 before_allocated=51420 after_free=145603 after_allocated=50603
#### OS COMP TEST GROUP START netperf-glibc ####
frame-allocator-diagnostic: process-teardown pid=1399 reclaimed_frames=818 before_free=143952 before_allocated=52254 after_free=144770 after_allocated=51436
====== netperf UDP_STREAM begin ======
Starting netserver with host '127.0.0.1' port '12865' and family AF_UNSPEC
MIGRATED UDP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET
enable_enobufs failed: getprotobyname
Socket  Message  Elapsed      Messages                
Size    Size     Time         Okay Errors   Throughput
bytes   bytes    secs            #      #   10^6bits/sec

 65536    1000   2.28            4      0       0.01
 65536           2.28            4              0.01

frame-allocator-diagnostic: process-teardown pid=1402 reclaimed_frames=42 before_free=143160 before_allocated=53046 after_free=143202 after_allocated=53004
frame-allocator-diagnostic: process-teardown pid=1401 reclaimed_frames=776 before_free=143202 before_allocated=53004 after_free=143978 after_allocated=52228
====== netperf UDP_STREAM end: success ======
====== netperf TCP_STREAM begin ======
frame-allocator-diagnostic: process-teardown pid=1406 reclaimed_frames=42 before_free=143138 before_allocated=53068 after_free=143180 after_allocated=53026
MIGRATED TCP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET
Recv   Send    Send                          
Socket Socket  Message  Elapsed              
Size   Size    Size     Time     Throughput  
bytes  bytes   bytes    secs.    10^6bits/sec  

 65536  65536   1000    0.46       18.11   
catcher: timer popped with times_up != 0
frame-allocator-diagnostic: process-teardown pid=1405 reclaimed_frames=775 before_free=143180 before_allocated=53026 after_free=143955 after_allocated=52251
====== netperf TCP_STREAM end: success ======
====== netperf UDP_RR begin ======
MIGRATED UDP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      2.29        1.75   
65536  65536 
frame-allocator-diagnostic: process-teardown pid=1410 reclaimed_frames=24 before_free=143152 before_allocated=53054 after_free=143176 after_allocated=53030
frame-allocator-diagnostic: process-teardown pid=1409 reclaimed_frames=757 before_free=143176 before_allocated=53030 after_free=143933 after_allocated=52273
====== netperf UDP_RR end: success ======
====== netperf TCP_RR begin ======
frame-allocator-diagnostic: process-teardown pid=1414 reclaimed_frames=24 before_free=143130 before_allocated=53076 after_free=143154 after_allocated=53052
MIGRATED TCP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.66        2.40   
65536  65536 
frame-allocator-diagnostic: process-teardown pid=1413 reclaimed_frames=757 before_free=143154 before_allocated=53052 after_free=143911 after_allocated=52295
====== netperf TCP_RR end: success ======
====== netperf TCP_CRR begin ======
frame-allocator-diagnostic: process-teardown pid=1418 reclaimed_frames=24 before_free=143109 before_allocated=53097 after_free=143133 after_allocated=53073
MIGRATED TCP Connect/Request/Response TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00        2.99   
65536  65536 
frame-allocator-diagnostic: process-teardown pid=1417 reclaimed_frames=757 before_free=143133 before_allocated=53073 after_free=143890 after_allocated=52316
====== netperf TCP_CRR end: success ======
frame-allocator-diagnostic: process-teardown pid=1421 reclaimed_frames=817 before_free=143066 before_allocated=53140 after_free=143883 after_allocated=52323
frame-allocator-diagnostic: process-teardown pid=1400 reclaimed_frames=757 before_free=143883 before_allocated=52323 after_free=144640 after_allocated=51566
#### OS COMP TEST GROUP END netperf-glibc ####
frame-allocator-diagnostic: process-teardown pid=1422 reclaimed_frames=818 before_free=143815 before_allocated=52391 after_free=144633 after_allocated=51573
frame-allocator-diagnostic: process-teardown pid=1398 reclaimed_frames=825 before_free=144633 before_allocated=51573 after_free=145458 after_allocated=50748
frame-allocator-diagnostic: process-teardown pid=1396 reclaimed_frames=820 before_free=145458 before_allocated=50748 after_free=146278 after_allocated=49928
#### OS COMP TEST GROUP START unixbench-glibc ####
SKIP: unixbench currently blocks on unresolved executable/runtime compatibility
#### OS COMP TEST GROUP END unixbench-glibc ####
[572.672057 0:2 axplat_loongarch64_qemu_virt::power:23] Shutting down...
===== LTP loongarch64 evaluation end: 2026-05-19T12:42:47+08:00 status=0 =====
```
