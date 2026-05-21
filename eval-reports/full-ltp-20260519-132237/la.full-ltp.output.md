# Full LTP attempt output: loongarch64

- Command: `OSKERNEL_FULL_LTP=1 OSKERNEL_LTP_CASE_TIMEOUT_SECS=30 timeout 7200 ./run-eval la`
- Exit/status captured: `0`
- Start: `2026-05-19T13:36:02+08:00`
- End: `2026-05-19T13:48:20+08:00`
- LTP cases started: `220`; last case: `crash01`
- Group starts: `ltp-musl`; group ends seen: `none`
- TPASS/TFAIL/TBROK/TCONF/TWARN markers: `302/20/48/90/10`
- SKIP LTP CASE markers: `45`

## Full console output (ANSI stripped)

```text
===== FULL LTP loongarch64 start: 2026-05-19T13:36:02+08:00 =====
cwd=/root/oskernel2026-orays
command=OSKERNEL_FULL_LTP=1 OSKERNEL_LTP_CASE_TIMEOUT_SECS=30 timeout 7200 ./run-eval la
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

warning: `axnet` (lib) generated 1 warning
warning: arceos_posix_api@0.2.0: using checked-in src/ctypes_gen.rs; libclang may not support target loongarch64-unknown-none-softfloat
   Compiling arceos-shell v0.1.0 (/root/oskernel2026-orays/examples/shell)
    Finished `release` profile [optimized] target(s) in 15.85s
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

[  0.004729 0 axruntime:135] Logging is enabled.
[  0.006518 0 axruntime:136] Primary CPU 0 started, arg = 0x0.
[  0.008317 0 axruntime:139] Found physcial memory regions:
[  0.008927 0 axruntime:141]   [PA:0x100d0000, PA:0x100d1000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.009964 0 axruntime:141]   [PA:0x100e0000, PA:0x100e1000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.010579 0 axruntime:141]   [PA:0x1fe00000, PA:0x1fe01000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.011199 0 axruntime:141]   [PA:0x20000000, PA:0x30000000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.011861 0 axruntime:141]   [PA:0x40000000, PA:0x40020000) mmio (READ | WRITE | DEVICE | RESERVED)
[  0.012451 0 axruntime:141]   [PA:0x80000000, PA:0x800fc000) .text (READ | EXECUTE | RESERVED)
[  0.013014 0 axruntime:141]   [PA:0x800fc000, PA:0x80125000) .rodata (READ | RESERVED)
[  0.013559 0 axruntime:141]   [PA:0x80125000, PA:0x8012a000) .data .tdata .tbss .percpu (READ | WRITE | RESERVED)
[  0.014413 0 axruntime:141]   [PA:0x8012a000, PA:0x8016a000) boot stack (READ | WRITE | RESERVED)
[  0.014921 0 axruntime:141]   [PA:0x8016a000, PA:0x80191000) .bss (READ | WRITE | RESERVED)
[  0.015273 0 axruntime:141]   [PA:0x80191000, PA:0xb0000000) free memory (READ | WRITE | FREE)
[  0.015722 0 axruntime:216] Initialize global memory allocator...
[  0.016104 0 axruntime:217]   use TLSF allocator.
[  0.018863 0 axmm:103] Initialize virtual memory management...
[  0.078715 0 axruntime:156] Initialize platform devices...
smp = 1
[  0.080587 0 axtask::api:73] Initialize scheduling...
[  0.083520 0 axtask::api:83]   use FIFO scheduler.
[  0.084083 0 axdriver:152] Initialize device drivers...
[  0.084572 0 axdriver:153]   device model: static
[  0.094735 0 virtio_drivers::device::blk:63] found a block device of size 4194304KB
[  0.097566 0 axdriver::bus::pci:107] registered a new Block device at 00:01.0: "virtio-blk"
[  0.105395 0 virtio_drivers::device::net::dev_raw:33] negotiated_features Features(MAC | STATUS | RING_INDIRECT_DESC | RING_EVENT_IDX | VERSION_1)
[  0.111493 0 axdriver::bus::pci:107] registered a new Net device at 00:02.0: "virtio-net"
[  0.186048 0 axfs:44] Initialize filesystems...
[  0.186707 0 axfs:47]   use block device 0: "virtio-blk"
[  0.188780 0 axfs::root:336]   detected root filesystem: Ext4
[  0.215601 0 axnet:42] Initialize network subsystem...
[  0.216287 0 axnet:45]   use NIC 0: "virtio-net"
[  0.222053 0 axnet::smoltcp_impl:335] created net interface "eth0":
[  0.222756 0 axnet::smoltcp_impl:336]   ether:    52-54-00-12-34-56
[  0.223461 0 axnet::smoltcp_impl:337]   ip:       10.0.2.15/24
[  0.224099 0 axnet::smoltcp_impl:338]   gateway:  10.0.2.2
[  0.224708 0 axruntime:182] Initialize interrupt handlers...
[  0.226644 0 axruntime:194] Primary CPU 0 init OK.
frame-allocator-diagnostic: process-teardown pid=5 reclaimed_frames=509 before_free=191371 before_allocated=4836 after_free=191880 after_allocated=4327
#### OS COMP TEST GROUP START ltp-musl ####
RUN LTP CASE abort01
frame-allocator-diagnostic: process-teardown pid=8 reclaimed_frames=511 before_free=190780 before_allocated=5427 after_free=191291 after_allocated=4916
frame-allocator-diagnostic: process-teardown pid=10 reclaimed_frames=1 before_free=190758 before_allocated=5449 after_free=190759 after_allocated=5448
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
frame-allocator-diagnostic: process-teardown pid=14 reclaimed_frames=3 before_free=190469 before_allocated=5738 after_free=190472 after_allocated=5735
abort01.c:51: TFAIL: Child exited with 139, expected SIGIOT
frame-allocator-diagnostic: process-teardown pid=13 reclaimed_frames=12 before_free=190469 before_allocated=5738 after_free=190481 after_allocated=5726

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[  5.884232 0:9 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=9 reclaimed_frames=262 before_free=190482 before_allocated=5725 after_free=190744 after_allocated=5463
frame-allocator-diagnostic: process-teardown pid=7 reclaimed_frames=12 before_free=190744 before_allocated=5463 after_free=190756 after_allocated=5451
FAIL LTP CASE abort01 : 0
RUN LTP CASE abs01
frame-allocator-diagnostic: process-teardown pid=17 reclaimed_frames=511 before_free=190212 before_allocated=5995 after_free=190723 after_allocated=5484
frame-allocator-diagnostic: process-teardown pid=11 reclaimed_frames=508 before_free=190723 before_allocated=5484 after_free=191231 after_allocated=4976
frame-allocator-diagnostic: process-teardown pid=19 reclaimed_frames=1 before_free=190699 before_allocated=5508 after_free=190700 after_allocated=5507
abs01       1  TPASS  :  Test passed
abs01       2  TPASS  :  Test passed
abs01       3  TPASS  :  Test passed
frame-allocator-diagnostic: process-teardown pid=18 reclaimed_frames=263 before_free=190438 before_allocated=5769 after_free=190701 after_allocated=5506
frame-allocator-diagnostic: process-teardown pid=16 reclaimed_frames=12 before_free=190701 before_allocated=5506 after_free=190713 after_allocated=5494
FAIL LTP CASE abs01 : 0
RUN LTP CASE accept01
frame-allocator-diagnostic: process-teardown pid=22 reclaimed_frames=511 before_free=190171 before_allocated=6036 after_free=190682 after_allocated=5525
frame-allocator-diagnostic: process-teardown pid=20 reclaimed_frames=508 before_free=190682 before_allocated=5525 after_free=191190 after_allocated=5017
frame-allocator-diagnostic: process-teardown pid=24 reclaimed_frames=1 before_free=190657 before_allocated=5550 after_free=190658 after_allocated=5549
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
accept01.c:92: TPASS: bad file descriptor : EBADF (9)
[ 13.908162 0:27 axnet::smoltcp_impl::tcp:284] [AxError::InvalidInput] socket accept() failed: not listen
[ 13.908732 0:27 arceos_posix_api::imp::net:567] sys_accept => Err(EINVAL)
accept01.c:92: TPASS: invalid socket buffer : EINVAL (22)
[ 13.909686 0:27 axnet::smoltcp_impl::tcp:284] [AxError::InvalidInput] socket accept() failed: not listen
[ 13.910119 0:27 arceos_posix_api::imp::net:567] sys_accept => Err(EINVAL)
accept01.c:92: TPASS: invalid salen : EINVAL (22)
[ 13.910676 0:27 axnet::smoltcp_impl::tcp:284] [AxError::InvalidInput] socket accept() failed: not listen
[ 13.911074 0:27 arceos_posix_api::imp::net:567] sys_accept => Err(EINVAL)
accept01.c:92: TPASS: no queued connections : EINVAL (22)
[ 13.911839 0:27 arceos_posix_api::imp::net:567] sys_accept => Err(EOPNOTSUPP)
accept01.c:92: TPASS: UDP accept : EOPNOTSUPP (95)
frame-allocator-diagnostic: process-teardown pid=27 reclaimed_frames=11 before_free=190374 before_allocated=5833 after_free=190385 after_allocated=5822

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[ 13.924396 0:23 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=23 reclaimed_frames=265 before_free=190386 before_allocated=5821 after_free=190651 after_allocated=5556
frame-allocator-diagnostic: process-teardown pid=21 reclaimed_frames=12 before_free=190651 before_allocated=5556 after_free=190663 after_allocated=5544
FAIL LTP CASE accept01 : 0
RUN LTP CASE accept02
frame-allocator-diagnostic: process-teardown pid=30 reclaimed_frames=511 before_free=190120 before_allocated=6087 after_free=190631 after_allocated=5576
frame-allocator-diagnostic: process-teardown pid=25 reclaimed_frames=508 before_free=190631 before_allocated=5576 after_free=191139 after_allocated=5068
frame-allocator-diagnostic: process-teardown pid=32 reclaimed_frames=1 before_free=190607 before_allocated=5600 after_free=190608 after_allocated=5599
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_buffers.c:57: TINFO: Test is using guarded buffers
accept02.c:131: TINFO: Starting listener on port: 49153
accept02.c:75: TPASS: Multicast group was not copied: EADDRNOTAVAIL (99)
frame-allocator-diagnostic: process-teardown pid=35 reclaimed_frames=12 before_free=190322 before_allocated=5885 after_free=190334 after_allocated=5873

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[ 18.196495 0:31 axfs::fops:297] [AxError::NotADirectory]
[ 18.202763 0:31 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=31 reclaimed_frames=266 before_free=190335 before_allocated=5872 after_free=190601 after_allocated=5606
frame-allocator-diagnostic: process-teardown pid=29 reclaimed_frames=12 before_free=190601 before_allocated=5606 after_free=190613 after_allocated=5594
FAIL LTP CASE accept02 : 0
RUN LTP CASE accept03
frame-allocator-diagnostic: process-teardown pid=40 reclaimed_frames=511 before_free=190070 before_allocated=6137 after_free=190581 after_allocated=5626
frame-allocator-diagnostic: process-teardown pid=33 reclaimed_frames=508 before_free=190581 before_allocated=5626 after_free=191089 after_allocated=5118
frame-allocator-diagnostic: process-teardown pid=42 reclaimed_frames=1 before_free=190557 before_allocated=5650 after_free=190558 after_allocated=5649
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
accept03.c:48: TPASS: accept() on file : ENOTSOCK (88)
accept03.c:48: TPASS: accept() on O_PATH file : EBADF (9)
accept03.c:48: TPASS: accept() on directory : ENOTSOCK (88)
accept03.c:48: TPASS: accept() on /dev/zero : ENOTSOCK (88)
accept03.c:48: TPASS: accept() on /proc/self/maps : ENOTSOCK (88)
accept03.c:48: TPASS: accept() on pipe read end : ENOTSOCK (88)
accept03.c:48: TPASS: accept() on pipe write end : ENOTSOCK (88)
tst_fd.c:106: TCONF: epoll_create(): ENOSYS (38)
tst_fd.c:114: TCONF: Skipping eventfd: ENOSYS (38)
tst_fd.c:125: TCONF: Skipping signalfd: ENOSYS (38)
tst_fd.c:135: TCONF: Skipping timerfd: ENOSYS (38)
tst_fd.c:144: TCONF: pidfd_open(): ENOSYS (38)
tst_fd.c:151: TCONF: Skipping fanotify: ENOSYS (38)
tst_fd.c:160: TCONF: Skipping inotify: ENOSYS (38)
tst_fd.c:170: TCONF: Skipping userfaultfd: ENOSYS (38)
tst_fd.c:188: TCONF: Skipping perf event: ENOSYS (38)
tst_fd.c:199: TCONF: Skipping io uring: ENOSYS (38)
tst_fd.c:215: TCONF: Skipping bpf map: ENOSYS (38)
tst_fd.c:224: TCONF: Skipping fsopen: ENOSYS (38)
tst_fd.c:233: TCONF: Skipping fspick: ENOSYS (38)
tst_fd.c:242: TCONF: Skipping open_tree: ENOSYS (38)
tst_fd.c:251: TCONF: Skipping memfd: ENOSYS (38)
tst_fd.c:260: TCONF: Skipping memfd secret: ENOSYS (38)
frame-allocator-diagnostic: process-teardown pid=45 reclaimed_frames=11 before_free=190273 before_allocated=5934 after_free=190284 after_allocated=5923

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[ 22.524210 0:41 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=41 reclaimed_frames=266 before_free=190285 before_allocated=5922 after_free=190551 after_allocated=5656
frame-allocator-diagnostic: process-teardown pid=39 reclaimed_frames=12 before_free=190551 before_allocated=5656 after_free=190563 after_allocated=5644
FAIL LTP CASE accept03 : 0
RUN LTP CASE accept4_01
frame-allocator-diagnostic: process-teardown pid=48 reclaimed_frames=511 before_free=190021 before_allocated=6186 after_free=190532 after_allocated=5675
frame-allocator-diagnostic: process-teardown pid=43 reclaimed_frames=508 before_free=190532 before_allocated=5675 after_free=191040 after_allocated=5167
frame-allocator-diagnostic: process-teardown pid=50 reclaimed_frames=1 before_free=190507 before_allocated=5700 after_free=190508 after_allocated=5699
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
accept4_01.c:71: TINFO: Testing variant: libc accept4()
accept4_01.c:78: TINFO: server listening on: 49155
accept4_01.c:151: TPASS: Close-on-exec 0, nonblock 0
accept4_01.c:151: TPASS: Close-on-exec 1, nonblock 0
accept4_01.c:151: TPASS: Close-on-exec 0, nonblock 1
accept4_01.c:151: TPASS: Close-on-exec 1, nonblock 1
frame-allocator-diagnostic: process-teardown pid=53 reclaimed_frames=12 before_free=190221 before_allocated=5986 after_free=190233 after_allocated=5974
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
accept4_01.c:71: TINFO: Testing variant: __NR_accept4 syscall
accept4_01.c:78: TINFO: server listening on: 49160
accept4_01.c:151: TPASS: Close-on-exec 0, nonblock 0
accept4_01.c:151: TPASS: Close-on-exec 1, nonblock 0
accept4_01.c:151: TPASS: Close-on-exec 0, nonblock 1
accept4_01.c:151: TPASS: Close-on-exec 1, nonblock 1
frame-allocator-diagnostic: process-teardown pid=56 reclaimed_frames=12 before_free=190213 before_allocated=5994 after_free=190225 after_allocated=5982
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
accept4_01.c:71: TINFO: Testing variant: __NR_socketcall SYS_ACCEPT4 syscall
accept4_01.c:78: TINFO: server listening on: 49165
accept4_01.c:43: TCONF: syscall(-1) __NR_socketcall not supported on your arch
frame-allocator-diagnostic: process-teardown pid=59 reclaimed_frames=12 before_free=190205 before_allocated=6002 after_free=190217 after_allocated=5990

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[ 26.702061 0:49 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=49 reclaimed_frames=267 before_free=190218 before_allocated=5989 after_free=190485 after_allocated=5722
frame-allocator-diagnostic: process-teardown pid=47 reclaimed_frames=12 before_free=190485 before_allocated=5722 after_free=190497 after_allocated=5710
FAIL LTP CASE accept4_01 : 32
RUN LTP CASE access01
frame-allocator-diagnostic: process-teardown pid=62 reclaimed_frames=511 before_free=189953 before_allocated=6254 after_free=190464 after_allocated=5743
frame-allocator-diagnostic: process-teardown pid=51 reclaimed_frames=508 before_free=190464 before_allocated=5743 after_free=190972 after_allocated=5235
frame-allocator-diagnostic: process-teardown pid=64 reclaimed_frames=1 before_free=190440 before_allocated=5767 after_free=190441 after_allocated=5766
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
access01.c:245: TPASS: access(accessfile_rwx, F_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, F_OK) as nobody passed
frame-allocator-diagnostic: process-teardown pid=68 reclaimed_frames=8 before_free=190143 before_allocated=6064 after_free=190151 after_allocated=6056
access01.c:245: TPASS: access(accessfile_rwx, X_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, X_OK) as nobody passed
frame-allocator-diagnostic: process-teardown pid=69 reclaimed_frames=8 before_free=190135 before_allocated=6072 after_free=190143 after_allocated=6064
access01.c:245: TPASS: access(accessfile_rwx, W_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, W_OK) as nobody passed
frame-allocator-diagnostic: process-teardown pid=70 reclaimed_frames=8 before_free=190127 before_allocated=6080 after_free=190135 after_allocated=6072
access01.c:245: TPASS: access(accessfile_rwx, R_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, R_OK) as nobody passed
frame-allocator-diagnostic: process-teardown pid=71 reclaimed_frames=8 before_free=190119 before_allocated=6088 after_free=190127 after_allocated=6080
access01.c:245: TPASS: access(accessfile_rwx, R_OK|W_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, R_OK|W_OK) as nobody passed
frame-allocator-diagnostic: process-teardown pid=72 reclaimed_frames=8 before_free=190111 before_allocated=6096 after_free=190119 after_allocated=6088
access01.c:245: TPASS: access(accessfile_rwx, R_OK|X_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, R_OK|X_OK) as nobody passed
frame-allocator-diagnostic: process-teardown pid=73 reclaimed_frames=8 before_free=190103 before_allocated=6104 after_free=190111 after_allocated=6096
access01.c:245: TPASS: access(accessfile_rwx, W_OK|X_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, W_OK|X_OK) as nobody passed
frame-allocator-diagnostic: process-teardown pid=74 reclaimed_frames=8 before_free=190095 before_allocated=6112 after_free=190103 after_allocated=6104
access01.c:245: TPASS: access(accessfile_rwx, R_OK|W_OK|X_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, R_OK|W_OK|X_OK) as nobody passed
frame-allocator-diagnostic: process-teardown pid=75 reclaimed_frames=8 before_free=190087 before_allocated=6120 after_free=190095 after_allocated=6112
access01.c:245: TPASS: access(accessfile_x, X_OK) as root passed
access01.c:245: TPASS: access(accessfile_x, X_OK) as nobody passed
frame-allocator-diagnostic: process-teardown pid=76 reclaimed_frames=8 before_free=190079 before_allocated=6128 after_free=190087 after_allocated=6120
access01.c:245: TPASS: access(accessfile_w, W_OK) as root passed
access01.c:245: TPASS: access(accessfile_w, W_OK) as nobody passed
frame-allocator-diagnostic: process-teardown pid=77 reclaimed_frames=8 before_free=190071 before_allocated=6136 after_free=190079 after_allocated=6128
access01.c:245: TPASS: access(accessfile_r, R_OK) as root passed
access01.c:245: TPASS: access(accessfile_r, R_OK) as nobody passed
frame-allocator-diagnostic: process-teardown pid=78 reclaimed_frames=8 before_free=190063 before_allocated=6144 after_free=190071 after_allocated=6136
access01.c:242: TPASS: access(accessfile_r, X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessfile_r, X_OK) as nobody : EACCES (13)
frame-allocator-diagnostic: process-teardown pid=79 reclaimed_frames=8 before_free=190055 before_allocated=6152 after_free=190063 after_allocated=6144
access01.c:242: TPASS: access(accessfile_r, W_OK) as nobody : EACCES (13)
frame-allocator-diagnostic: process-teardown pid=80 reclaimed_frames=8 before_free=190047 before_allocated=6160 after_free=190055 after_allocated=6152
tst_test.c:1464: TBROK: Test 12 haven't reported results!
frame-allocator-diagnostic: process-teardown pid=67 reclaimed_frames=12 before_free=190054 before_allocated=6153 after_free=190066 after_allocated=6141

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[ 30.832088 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.832629 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.833029 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.833768 0:63 axfs::root:433] [AxError::IsADirectory]
[ 30.834595 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.835008 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.835396 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.835836 0:63 axfs::root:433] [AxError::IsADirectory]
[ 30.836386 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.836778 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.837162 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.837594 0:63 axfs::root:433] [AxError::IsADirectory]
[ 30.838162 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.838554 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.838936 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.839370 0:63 axfs::root:433] [AxError::IsADirectory]
[ 30.839958 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.840357 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.840743 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.841173 0:63 axfs::root:433] [AxError::IsADirectory]
[ 30.841714 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.842189 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.842582 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.843028 0:63 axfs::root:433] [AxError::IsADirectory]
[ 30.843401 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.843769 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.844259 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.844659 0:63 axfs::fops:297] [AxError::NotADirectory]
[ 30.845203 0:63 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=63 reclaimed_frames=264 before_free=190066 before_allocated=6141 after_free=190330 after_allocated=5877
frame-allocator-diagnostic: process-teardown pid=61 reclaimed_frames=13 before_free=190330 before_allocated=5877 after_free=190343 after_allocated=5864
FAIL LTP CASE access01 : 2
RUN LTP CASE access02
frame-allocator-diagnostic: process-teardown pid=83 reclaimed_frames=511 before_free=189800 before_allocated=6407 after_free=190311 after_allocated=5896
frame-allocator-diagnostic: process-teardown pid=65 reclaimed_frames=508 before_free=190311 before_allocated=5896 after_free=190819 after_allocated=5388
frame-allocator-diagnostic: process-teardown pid=85 reclaimed_frames=1 before_free=190287 before_allocated=5920 after_free=190288 after_allocated=5919
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
access02.c:175: TBROK: symlink(file_f,symlink_f) failed: ENOSYS (38)
frame-allocator-diagnostic: process-teardown pid=88 reclaimed_frames=13 before_free=190005 before_allocated=6202 after_free=190018 after_allocated=6189

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[ 34.825878 0:84 axfs::fops:297] [AxError::NotADirectory]
[ 34.826384 0:84 axfs::fops:297] [AxError::NotADirectory]
[ 34.826747 0:84 axfs::fops:297] [AxError::NotADirectory]
[ 34.827094 0:84 axfs::fops:297] [AxError::NotADirectory]
[ 34.827934 0:84 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=84 reclaimed_frames=262 before_free=190019 before_allocated=6188 after_free=190281 after_allocated=5926
frame-allocator-diagnostic: process-teardown pid=82 reclaimed_frames=12 before_free=190281 before_allocated=5926 after_free=190293 after_allocated=5914
FAIL LTP CASE access02 : 2
RUN LTP CASE access03
frame-allocator-diagnostic: process-teardown pid=91 reclaimed_frames=511 before_free=185653 before_allocated=10554 after_free=186164 after_allocated=10043
frame-allocator-diagnostic: process-teardown pid=86 reclaimed_frames=508 before_free=186164 before_allocated=10043 after_free=186672 after_allocated=9535
frame-allocator-diagnostic: process-teardown pid=93 reclaimed_frames=1 before_free=186140 before_allocated=10067 after_free=186141 after_allocated=10066
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
read-cstr-efault: pid=96 tid=96 ptr=0xffffffffffffffff fault_addr=0xffffffffffffffff pc=0x100006d950 reason="pointer overflow" aspace=AddrSpaceQuery { contains: false, area_found: false, area_start: 0, area_end: 0, area_flags: 0x0, backend: "none", pte_mapped: false, paddr: 0, pte_flags: 0x0, page_size: None, shared_metadata: false } created_by_fork=true credential_generation=0 uid=0 gid=0
access03.c:37: TPASS: invalid address as root : EFAULT (14)
read-cstr-efault: pid=97 tid=97 ptr=0xffffffffffffffff fault_addr=0xffffffffffffffff pc=0x100006d950 reason="pointer overflow" aspace=AddrSpaceQuery { contains: false, area_found: false, area_start: 0, area_end: 0, area_flags: 0x0, backend: "none", pte_mapped: false, paddr: 0, pte_flags: 0x0, page_size: None, shared_metadata: false } created_by_fork=true credential_generation=1 uid=65534 gid=0
access03.c:46: TPASS: invalid address as nobody : EFAULT (14)
frame-allocator-diagnostic: process-teardown pid=97 reclaimed_frames=8 before_free=185843 before_allocated=10364 after_free=185851 after_allocated=10356
read-cstr-efault: pid=96 tid=96 ptr=0xffffffffffffffff fault_addr=0xffffffffffffffff pc=0x100006d950 reason="pointer overflow" aspace=AddrSpaceQuery { contains: false, area_found: false, area_start: 0, area_end: 0, area_flags: 0x0, backend: "none", pte_mapped: false, paddr: 0, pte_flags: 0x0, page_size: None, shared_metadata: false } created_by_fork=true credential_generation=0 uid=0 gid=0
access03.c:37: TPASS: invalid address as root : EFAULT (14)
read-cstr-efault: pid=98 tid=98 ptr=0xffffffffffffffff fault_addr=0xffffffffffffffff pc=0x100006d950 reason="pointer overflow" aspace=AddrSpaceQuery { contains: false, area_found: false, area_start: 0, area_end: 0, area_flags: 0x0, backend: "none", pte_mapped: false, paddr: 0, pte_flags: 0x0, page_size: None, shared_metadata: false } created_by_fork=true credential_generation=1 uid=65534 gid=0
access03.c:46: TPASS: invalid address as nobody : EFAULT (14)
frame-allocator-diagnostic: process-teardown pid=98 reclaimed_frames=8 before_free=185835 before_allocated=10372 after_free=185843 after_allocated=10364
read-cstr-efault: pid=96 tid=96 ptr=0xffffffffffffffff fault_addr=0xffffffffffffffff pc=0x100006d950 reason="pointer overflow" aspace=AddrSpaceQuery { contains: false, area_found: false, area_start: 0, area_end: 0, area_flags: 0x0, backend: "none", pte_mapped: false, paddr: 0, pte_flags: 0x0, page_size: None, shared_metadata: false } created_by_fork=true credential_generation=0 uid=0 gid=0
access03.c:37: TPASS: invalid address as root : EFAULT (14)
read-cstr-efault: pid=99 tid=99 ptr=0xffffffffffffffff fault_addr=0xffffffffffffffff pc=0x100006d950 reason="pointer overflow" aspace=AddrSpaceQuery { contains: false, area_found: false, area_start: 0, area_end: 0, area_flags: 0x0, backend: "none", pte_mapped: false, paddr: 0, pte_flags: 0x0, page_size: None, shared_metadata: false } created_by_fork=true credential_generation=1 uid=65534 gid=0
access03.c:46: TPASS: invalid address as nobody : EFAULT (14)
frame-allocator-diagnostic: process-teardown pid=99 reclaimed_frames=8 before_free=185827 before_allocated=10380 after_free=185835 after_allocated=10372
read-cstr-efault: pid=96 tid=96 ptr=0xffffffffffffffff fault_addr=0xffffffffffffffff pc=0x100006d950 reason="pointer overflow" aspace=AddrSpaceQuery { contains: false, area_found: false, area_start: 0, area_end: 0, area_flags: 0x0, backend: "none", pte_mapped: false, paddr: 0, pte_flags: 0x0, page_size: None, shared_metadata: false } created_by_fork=true credential_generation=0 uid=0 gid=0
access03.c:37: TPASS: invalid address as root : EFAULT (14)
read-cstr-efault: pid=100 tid=100 ptr=0xffffffffffffffff fault_addr=0xffffffffffffffff pc=0x100006d950 reason="pointer overflow" aspace=AddrSpaceQuery { contains: false, area_found: false, area_start: 0, area_end: 0, area_flags: 0x0, backend: "none", pte_mapped: false, paddr: 0, pte_flags: 0x0, page_size: None, shared_metadata: false } created_by_fork=true credential_generation=1 uid=65534 gid=0
access03.c:46: TPASS: invalid address as nobody : EFAULT (14)
frame-allocator-diagnostic: process-teardown pid=100 reclaimed_frames=8 before_free=185819 before_allocated=10388 after_free=185827 after_allocated=10380
frame-allocator-diagnostic: process-teardown pid=96 reclaimed_frames=13 before_free=185826 before_allocated=10381 after_free=185839 after_allocated=10368

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[ 38.875316 0:92 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=92 reclaimed_frames=262 before_free=185840 before_allocated=10367 after_free=186102 after_allocated=10105
frame-allocator-diagnostic: process-teardown pid=90 reclaimed_frames=13 before_free=186102 before_allocated=10105 after_free=186115 after_allocated=10092
FAIL LTP CASE access03 : 0
RUN LTP CASE access04
frame-allocator-diagnostic: process-teardown pid=103 reclaimed_frames=511 before_free=185572 before_allocated=10635 after_free=186083 after_allocated=10124
frame-allocator-diagnostic: process-teardown pid=94 reclaimed_frames=508 before_free=186083 before_allocated=10124 after_free=186591 after_allocated=9616
frame-allocator-diagnostic: process-teardown pid=105 reclaimed_frames=1 before_free=186059 before_allocated=10148 after_free=186060 after_allocated=10147
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_test.c:1017: TINFO: Mounting (null) to /tmp/LTP_acckiDENg/mntpoint fstyp=tmpfs flags=21
tst_test.c:1017: TBROK: mount((null), mntpoint, tmpfs, 33, 0) failed: EINVAL (22)

Summary:
passed   0
failed   0
broken   1
skipped  0
warnings 0
[ 42.759961 0:104 axfs::root:433] [AxError::IsADirectory]
[ 42.760728 0:104 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=104 reclaimed_frames=271 before_free=185790 before_allocated=10417 after_free=186061 after_allocated=10146
frame-allocator-diagnostic: process-teardown pid=102 reclaimed_frames=12 before_free=186061 before_allocated=10146 after_free=186073 after_allocated=10134
FAIL LTP CASE access04 : 2
RUN LTP CASE acct01
frame-allocator-diagnostic: process-teardown pid=108 reclaimed_frames=511 before_free=185530 before_allocated=10677 after_free=186041 after_allocated=10166
frame-allocator-diagnostic: process-teardown pid=106 reclaimed_frames=508 before_free=186041 before_allocated=10166 after_free=186549 after_allocated=9658
frame-allocator-diagnostic: process-teardown pid=110 reclaimed_frames=1 before_free=186017 before_allocated=10190 after_free=186018 after_allocated=10189
tst_buffers.c:57: TINFO: Test is using guarded buffers
[ 46.679439 0:109 axfs::root:423] [AxError::AlreadyExists]
tst_test.c:1011: TBROK: mkdir(ro_mntpoint/dir/, 0777) failed: EEXIST (17)

Summary:
passed   0
failed   0
broken   1
skipped  0
warnings 0
[ 46.687922 0:109 axfs::root:433] [AxError::IsADirectory]
[ 46.689639 0:109 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=109 reclaimed_frames=272 before_free=185747 before_allocated=10460 after_free=186019 after_allocated=10188
frame-allocator-diagnostic: process-teardown pid=107 reclaimed_frames=12 before_free=186019 before_allocated=10188 after_free=186031 after_allocated=10176
FAIL LTP CASE acct01 : 2
RUN LTP CASE acct02
frame-allocator-diagnostic: process-teardown pid=113 reclaimed_frames=511 before_free=185488 before_allocated=10719 after_free=185999 after_allocated=10208
frame-allocator-diagnostic: process-teardown pid=111 reclaimed_frames=508 before_free=185999 before_allocated=10208 after_free=186507 after_allocated=9700
frame-allocator-diagnostic: process-teardown pid=115 reclaimed_frames=1 before_free=185975 before_allocated=10232 after_free=185976 after_allocated=10231
tst_kconfig.c:71: TINFO: Couldn't locate kernel config!
tst_kconfig.c:207: TBROK: Cannot parse kernel .config
frame-allocator-diagnostic: process-teardown pid=114 reclaimed_frames=263 before_free=185714 before_allocated=10493 after_free=185977 after_allocated=10230
frame-allocator-diagnostic: process-teardown pid=112 reclaimed_frames=12 before_free=185977 before_allocated=10230 after_free=185989 after_allocated=10218
FAIL LTP CASE acct02 : 2
RUN LTP CASE acct02_helper
frame-allocator-diagnostic: process-teardown pid=118 reclaimed_frames=511 before_free=185446 before_allocated=10761 after_free=185957 after_allocated=10250
frame-allocator-diagnostic: process-teardown pid=116 reclaimed_frames=508 before_free=185957 before_allocated=10250 after_free=186465 after_allocated=9742
frame-allocator-diagnostic: process-teardown pid=120 reclaimed_frames=1 before_free=185933 before_allocated=10274 after_free=185934 after_allocated=10273
frame-allocator-diagnostic: process-teardown pid=119 reclaimed_frames=184 before_free=185751 before_allocated=10456 after_free=185935 after_allocated=10272
frame-allocator-diagnostic: process-teardown pid=117 reclaimed_frames=12 before_free=185935 before_allocated=10272 after_free=185947 after_allocated=10260
FAIL LTP CASE acct02_helper : 128
RUN LTP CASE acl1
frame-allocator-diagnostic: process-teardown pid=123 reclaimed_frames=511 before_free=185403 before_allocated=10804 after_free=185914 after_allocated=10293
frame-allocator-diagnostic: process-teardown pid=121 reclaimed_frames=508 before_free=185914 before_allocated=10293 after_free=186422 after_allocated=9785
frame-allocator-diagnostic: process-teardown pid=125 reclaimed_frames=1 before_free=185890 before_allocated=10317 after_free=185891 after_allocated=10316
The acl library was missing upon compilation.
frame-allocator-diagnostic: process-teardown pid=124 reclaimed_frames=184 before_free=185708 before_allocated=10499 after_free=185892 after_allocated=10315
frame-allocator-diagnostic: process-teardown pid=122 reclaimed_frames=13 before_free=185892 before_allocated=10315 after_free=185905 after_allocated=10302
FAIL LTP CASE acl1 : 32
RUN LTP CASE add_ipv6addr
frame-allocator-diagnostic: process-teardown pid=128 reclaimed_frames=512 before_free=185360 before_allocated=10847 after_free=185872 after_allocated=10335
SKIP LTP CASE add_ipv6addr : requires LTP network environment
frame-allocator-diagnostic: process-teardown pid=127 reclaimed_frames=13 before_free=185872 before_allocated=10335 after_free=185885 after_allocated=10322
FAIL LTP CASE add_ipv6addr : 32
RUN LTP CASE add_key01
frame-allocator-diagnostic: process-teardown pid=130 reclaimed_frames=511 before_free=185342 before_allocated=10865 after_free=185853 after_allocated=10354
frame-allocator-diagnostic: process-teardown pid=126 reclaimed_frames=508 before_free=185853 before_allocated=10354 after_free=186361 after_allocated=9846
frame-allocator-diagnostic: process-teardown pid=132 reclaimed_frames=1 before_free=185829 before_allocated=10378 after_free=185830 after_allocated=10377
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
../../../../include/lapi/keyctl.h:29: TCONF: syscall(217) __NR_add_key not supported on your arch
frame-allocator-diagnostic: process-teardown pid=135 reclaimed_frames=10 before_free=185548 before_allocated=10659 after_free=185558 after_allocated=10649

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[ 59.567244 0:131 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=131 reclaimed_frames=264 before_free=185558 before_allocated=10649 after_free=185822 after_allocated=10385
frame-allocator-diagnostic: process-teardown pid=129 reclaimed_frames=12 before_free=185822 before_allocated=10385 after_free=185834 after_allocated=10373
FAIL LTP CASE add_key01 : 32
RUN LTP CASE add_key02
frame-allocator-diagnostic: process-teardown pid=138 reclaimed_frames=511 before_free=185291 before_allocated=10916 after_free=185802 after_allocated=10405
frame-allocator-diagnostic: process-teardown pid=133 reclaimed_frames=508 before_free=185802 before_allocated=10405 after_free=186310 after_allocated=9897
frame-allocator-diagnostic: process-teardown pid=140 reclaimed_frames=1 before_free=185778 before_allocated=10429 after_free=185779 after_allocated=10428
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
../../../../include/lapi/keyctl.h:29: TCONF: syscall(217) __NR_add_key not supported on your arch
frame-allocator-diagnostic: process-teardown pid=143 reclaimed_frames=11 before_free=185498 before_allocated=10709 after_free=185509 after_allocated=10698

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[ 63.601020 0:139 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=139 reclaimed_frames=262 before_free=185510 before_allocated=10697 after_free=185772 after_allocated=10435
frame-allocator-diagnostic: process-teardown pid=137 reclaimed_frames=12 before_free=185772 before_allocated=10435 after_free=185784 after_allocated=10423
FAIL LTP CASE add_key02 : 32
RUN LTP CASE add_key03
frame-allocator-diagnostic: process-teardown pid=146 reclaimed_frames=511 before_free=185241 before_allocated=10966 after_free=185752 after_allocated=10455
frame-allocator-diagnostic: process-teardown pid=141 reclaimed_frames=508 before_free=185752 before_allocated=10455 after_free=186260 after_allocated=9947
frame-allocator-diagnostic: process-teardown pid=148 reclaimed_frames=1 before_free=185728 before_allocated=10479 after_free=185729 after_allocated=10478
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
../../../../include/lapi/keyctl.h:29: TCONF: syscall(217) __NR_add_key not supported on your arch
frame-allocator-diagnostic: process-teardown pid=151 reclaimed_frames=13 before_free=185446 before_allocated=10761 after_free=185459 after_allocated=10748

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[ 67.727243 0:147 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=147 reclaimed_frames=262 before_free=185460 before_allocated=10747 after_free=185722 after_allocated=10485
frame-allocator-diagnostic: process-teardown pid=145 reclaimed_frames=12 before_free=185722 before_allocated=10485 after_free=185734 after_allocated=10473
FAIL LTP CASE add_key03 : 32
RUN LTP CASE add_key04
frame-allocator-diagnostic: process-teardown pid=154 reclaimed_frames=511 before_free=185191 before_allocated=11016 after_free=185702 after_allocated=10505
frame-allocator-diagnostic: process-teardown pid=149 reclaimed_frames=508 before_free=185702 before_allocated=10505 after_free=186210 after_allocated=9997
frame-allocator-diagnostic: process-teardown pid=156 reclaimed_frames=1 before_free=185678 before_allocated=10529 after_free=185679 after_allocated=10528
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_buffers.c:57: TINFO: Test is using guarded buffers
../../../../include/lapi/keyctl.h:54: TCONF: syscall(219) __NR_keyctl not supported on your arch
frame-allocator-diagnostic: process-teardown pid=159 reclaimed_frames=11 before_free=185398 before_allocated=10809 after_free=185409 after_allocated=10798

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[ 71.750859 0:155 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=155 reclaimed_frames=262 before_free=185410 before_allocated=10797 after_free=185672 after_allocated=10535
frame-allocator-diagnostic: process-teardown pid=153 reclaimed_frames=12 before_free=185672 before_allocated=10535 after_free=185684 after_allocated=10523
FAIL LTP CASE add_key04 : 32
RUN LTP CASE add_key05
frame-allocator-diagnostic: process-teardown pid=162 reclaimed_frames=511 before_free=185141 before_allocated=11066 after_free=185652 after_allocated=10555
frame-allocator-diagnostic: process-teardown pid=157 reclaimed_frames=508 before_free=185652 before_allocated=10555 after_free=186160 after_allocated=10047
frame-allocator-diagnostic: process-teardown pid=164 reclaimed_frames=1 before_free=185628 before_allocated=10579 after_free=185629 after_allocated=10578
tst_cmd.c:257: TCONF: Couldn't find 'useradd' in $PATH
frame-allocator-diagnostic: process-teardown pid=163 reclaimed_frames=263 before_free=185367 before_allocated=10840 after_free=185630 after_allocated=10577
frame-allocator-diagnostic: process-teardown pid=161 reclaimed_frames=12 before_free=185630 before_allocated=10577 after_free=185642 after_allocated=10565
FAIL LTP CASE add_key05 : 32
RUN LTP CASE adjtimex01
frame-allocator-diagnostic: process-teardown pid=167 reclaimed_frames=511 before_free=185099 before_allocated=11108 after_free=185610 after_allocated=10597
frame-allocator-diagnostic: process-teardown pid=165 reclaimed_frames=508 before_free=185610 before_allocated=10597 after_free=186118 after_allocated=10089
frame-allocator-diagnostic: process-teardown pid=169 reclaimed_frames=1 before_free=185585 before_allocated=10622 after_free=185586 after_allocated=10621
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
adjtimex01.c:24: TPASS: adjtimex() with mode 0x403f 
adjtimex01.c:33: TPASS: adjtimex() with mode 0x8001 
frame-allocator-diagnostic: process-teardown pid=172 reclaimed_frames=11 before_free=185303 before_allocated=10904 after_free=185314 after_allocated=10893

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[ 79.816825 0:168 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=168 reclaimed_frames=264 before_free=185315 before_allocated=10892 after_free=185579 after_allocated=10628
frame-allocator-diagnostic: process-teardown pid=166 reclaimed_frames=13 before_free=185579 before_allocated=10628 after_free=185592 after_allocated=10615
FAIL LTP CASE adjtimex01 : 0
RUN LTP CASE adjtimex02
frame-allocator-diagnostic: process-teardown pid=175 reclaimed_frames=511 before_free=185048 before_allocated=11159 after_free=185559 after_allocated=10648
frame-allocator-diagnostic: process-teardown pid=170 reclaimed_frames=508 before_free=185559 before_allocated=10648 after_free=186067 after_allocated=10140
frame-allocator-diagnostic: process-teardown pid=177 reclaimed_frames=1 before_free=185535 before_allocated=10672 after_free=185536 after_allocated=10671
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
adjtimex02.c:111: TINFO: Testing variant: libc adjtimex()
adjtimex02.c:100: TPASS: adjtimex() error : EPERM (1)
adjtimex02.c:95: TCONF: EFAULT is skipped for libc variant
adjtimex02.c:100: TPASS: adjtimex() error : EINVAL (22)
adjtimex02.c:100: TPASS: adjtimex() error : EINVAL (22)
frame-allocator-diagnostic: process-teardown pid=180 reclaimed_frames=13 before_free=185251 before_allocated=10956 after_free=185264 after_allocated=10943
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
adjtimex02.c:111: TINFO: Testing variant: __NR_adjtimex syscall
adjtimex02.c:100: TPASS: adjtimex() error : EPERM (1)
adjtimex02.c:100: TPASS: adjtimex() error : EFAULT (14)
adjtimex02.c:100: TPASS: adjtimex() error : EINVAL (22)
adjtimex02.c:100: TPASS: adjtimex() error : EINVAL (22)
frame-allocator-diagnostic: process-teardown pid=183 reclaimed_frames=13 before_free=185243 before_allocated=10964 after_free=185256 after_allocated=10951

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[ 83.890975 0:176 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=176 reclaimed_frames=264 before_free=185257 before_allocated=10950 after_free=185521 after_allocated=10686
frame-allocator-diagnostic: process-teardown pid=174 reclaimed_frames=13 before_free=185521 before_allocated=10686 after_free=185534 after_allocated=10673
FAIL LTP CASE adjtimex02 : 0
RUN LTP CASE adjtimex03
frame-allocator-diagnostic: process-teardown pid=186 reclaimed_frames=511 before_free=184991 before_allocated=11216 after_free=185502 after_allocated=10705
frame-allocator-diagnostic: process-teardown pid=178 reclaimed_frames=508 before_free=185502 before_allocated=10705 after_free=186010 after_allocated=10197
frame-allocator-diagnostic: process-teardown pid=188 reclaimed_frames=1 before_free=185478 before_allocated=10729 after_free=185479 after_allocated=10728
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
adjtimex03.c:53: TINFO: expecting adjtimex() to fail with EINVAL with mode 0x8000
adjtimex03.c:62: TINFO: tai : 0x00000000
adjtimex03.c:53: TINFO: expecting adjtimex() to fail with EINVAL with mode 0x8000
adjtimex03.c:62: TINFO: tai : 0x00000000
adjtimex03.c:53: TINFO: expecting adjtimex() to fail with EINVAL with mode 0x8000
adjtimex03.c:62: TINFO: tai : 0x00000000
adjtimex03.c:53: TINFO: expecting adjtimex() to fail with EINVAL with mode 0x8000
adjtimex03.c:62: TINFO: tai : 0x00000000
adjtimex03.c:53: TINFO: expecting adjtimex() to fail with EINVAL with mode 0x8000
adjtimex03.c:62: TINFO: tai : 0x00000000
adjtimex03.c:53: TINFO: expecting adjtimex() to fail with EINVAL with mode 0x8000
adjtimex03.c:62: TINFO: tai : 0x00000000
adjtimex03.c:53: TINFO: expecting adjtimex() to fail with EINVAL with mode 0x8000
adjtimex03.c:62: TINFO: tai : 0x00000000
adjtimex03.c:53: TINFO: expecting adjtimex() to fail with EINVAL with mode 0x8000
adjtimex03.c:62: TINFO: tai : 0x00000000
adjtimex03.c:53: TINFO: expecting adjtimex() to fail with EINVAL with mode 0x8000
adjtimex03.c:62: TINFO: tai : 0x00000000
adjtimex03.c:53: TINFO: expecting adjtimex() to fail with EINVAL with mode 0x8000
adjtimex03.c:62: TINFO: tai : 0x00000000
adjtimex03.c:73: TPASS: Data leak not observed
frame-allocator-diagnostic: process-teardown pid=191 reclaimed_frames=11 before_free=185197 before_allocated=11010 after_free=185208 after_allocated=10999

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[ 87.811838 0:187 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=187 reclaimed_frames=263 before_free=185209 before_allocated=10998 after_free=185472 after_allocated=10735
frame-allocator-diagnostic: process-teardown pid=185 reclaimed_frames=12 before_free=185472 before_allocated=10735 after_free=185484 after_allocated=10723
FAIL LTP CASE adjtimex03 : 0
RUN LTP CASE af_alg01
SKIP LTP CASE af_alg01 : AF_ALG unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=193 reclaimed_frames=13 before_free=185461 before_allocated=10746 after_free=185474 after_allocated=10733
FAIL LTP CASE af_alg01 : 32
RUN LTP CASE af_alg02
SKIP LTP CASE af_alg02 : AF_ALG unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=194 reclaimed_frames=13 before_free=185451 before_allocated=10756 after_free=185464 after_allocated=10743
FAIL LTP CASE af_alg02 : 32
RUN LTP CASE af_alg03
SKIP LTP CASE af_alg03 : AF_ALG unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=195 reclaimed_frames=12 before_free=185442 before_allocated=10765 after_free=185454 after_allocated=10753
FAIL LTP CASE af_alg03 : 32
RUN LTP CASE af_alg04
SKIP LTP CASE af_alg04 : AF_ALG unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=196 reclaimed_frames=12 before_free=185432 before_allocated=10775 after_free=185444 after_allocated=10763
FAIL LTP CASE af_alg04 : 32
RUN LTP CASE af_alg05
SKIP LTP CASE af_alg05 : AF_ALG unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=197 reclaimed_frames=13 before_free=185421 before_allocated=10786 after_free=185434 after_allocated=10773
FAIL LTP CASE af_alg05 : 32
RUN LTP CASE af_alg06
SKIP LTP CASE af_alg06 : AF_ALG unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=198 reclaimed_frames=13 before_free=185411 before_allocated=10796 after_free=185424 after_allocated=10783
FAIL LTP CASE af_alg06 : 32
RUN LTP CASE af_alg07
SKIP LTP CASE af_alg07 : AF_ALG unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=199 reclaimed_frames=13 before_free=185401 before_allocated=10806 after_free=185414 after_allocated=10793
FAIL LTP CASE af_alg07 : 32
RUN LTP CASE aio-stress
frame-allocator-diagnostic: process-teardown pid=201 reclaimed_frames=511 before_free=184870 before_allocated=11337 after_free=185381 after_allocated=10826
frame-allocator-diagnostic: process-teardown pid=189 reclaimed_frames=508 before_free=185381 before_allocated=10826 after_free=185889 after_allocated=10318
frame-allocator-diagnostic: process-teardown pid=203 reclaimed_frames=1 before_free=185357 before_allocated=10850 after_free=185358 after_allocated=10849
tst_test.c:1175: TCONF: test requires libaio and its development packages
frame-allocator-diagnostic: process-teardown pid=202 reclaimed_frames=261 before_free=185098 before_allocated=11109 after_free=185359 after_allocated=10848
frame-allocator-diagnostic: process-teardown pid=200 reclaimed_frames=13 before_free=185359 before_allocated=10848 after_free=185372 after_allocated=10835
FAIL LTP CASE aio-stress : 32
RUN LTP CASE aio01
frame-allocator-diagnostic: process-teardown pid=206 reclaimed_frames=511 before_free=184829 before_allocated=11378 after_free=185340 after_allocated=10867
frame-allocator-diagnostic: process-teardown pid=204 reclaimed_frames=508 before_free=185340 before_allocated=10867 after_free=185848 after_allocated=10359
frame-allocator-diagnostic: process-teardown pid=208 reclaimed_frames=1 before_free=185316 before_allocated=10891 after_free=185317 after_allocated=10890
aio01       1  TCONF  :  aio01.c:421: test requires libaio and it's development packages
aio01       2  TCONF  :  aio01.c:421: Remaining cases not appropriate for configuration
frame-allocator-diagnostic: process-teardown pid=207 reclaimed_frames=263 before_free=185055 before_allocated=11152 after_free=185318 after_allocated=10889
frame-allocator-diagnostic: process-teardown pid=205 reclaimed_frames=12 before_free=185318 before_allocated=10889 after_free=185330 after_allocated=10877
FAIL LTP CASE aio01 : 32
RUN LTP CASE aio02
frame-allocator-diagnostic: process-teardown pid=211 reclaimed_frames=511 before_free=184787 before_allocated=11420 after_free=185298 after_allocated=10909
frame-allocator-diagnostic: process-teardown pid=209 reclaimed_frames=508 before_free=185298 before_allocated=10909 after_free=185806 after_allocated=10401
frame-allocator-diagnostic: process-teardown pid=213 reclaimed_frames=1 before_free=185274 before_allocated=10933 after_free=185275 after_allocated=10932
tst_test.c:1175: TCONF: test requires libaio and its development packages
frame-allocator-diagnostic: process-teardown pid=212 reclaimed_frames=261 before_free=185015 before_allocated=11192 after_free=185276 after_allocated=10931
frame-allocator-diagnostic: process-teardown pid=210 reclaimed_frames=12 before_free=185276 before_allocated=10931 after_free=185288 after_allocated=10919
FAIL LTP CASE aio02 : 32
RUN LTP CASE aiocp
frame-allocator-diagnostic: process-teardown pid=216 reclaimed_frames=511 before_free=184745 before_allocated=11462 after_free=185256 after_allocated=10951
frame-allocator-diagnostic: process-teardown pid=214 reclaimed_frames=508 before_free=185256 before_allocated=10951 after_free=185764 after_allocated=10443
frame-allocator-diagnostic: process-teardown pid=218 reclaimed_frames=1 before_free=185232 before_allocated=10975 after_free=185233 after_allocated=10974
tst_test.c:1175: TCONF: test requires libaio and its development packages
frame-allocator-diagnostic: process-teardown pid=217 reclaimed_frames=261 before_free=184973 before_allocated=11234 after_free=185234 after_allocated=10973
frame-allocator-diagnostic: process-teardown pid=215 reclaimed_frames=12 before_free=185234 before_allocated=10973 after_free=185246 after_allocated=10961
FAIL LTP CASE aiocp : 32
RUN LTP CASE aiodio_append
frame-allocator-diagnostic: process-teardown pid=221 reclaimed_frames=511 before_free=184703 before_allocated=11504 after_free=185214 after_allocated=10993
frame-allocator-diagnostic: process-teardown pid=219 reclaimed_frames=508 before_free=185214 before_allocated=10993 after_free=185722 after_allocated=10485
frame-allocator-diagnostic: process-teardown pid=223 reclaimed_frames=1 before_free=185190 before_allocated=11017 after_free=185191 after_allocated=11016
tst_test.c:1175: TCONF: test requires libaio and its development packages
frame-allocator-diagnostic: process-teardown pid=222 reclaimed_frames=261 before_free=184931 before_allocated=11276 after_free=185192 after_allocated=11015
frame-allocator-diagnostic: process-teardown pid=220 reclaimed_frames=12 before_free=185192 before_allocated=11015 after_free=185204 after_allocated=11003
FAIL LTP CASE aiodio_append : 32
RUN LTP CASE aiodio_sparse
frame-allocator-diagnostic: process-teardown pid=226 reclaimed_frames=511 before_free=184661 before_allocated=11546 after_free=185172 after_allocated=11035
frame-allocator-diagnostic: process-teardown pid=224 reclaimed_frames=508 before_free=185172 before_allocated=11035 after_free=185680 after_allocated=10527
frame-allocator-diagnostic: process-teardown pid=228 reclaimed_frames=1 before_free=185148 before_allocated=11059 after_free=185149 after_allocated=11058
tst_test.c:1175: TCONF: test requires libaio and its development packages
frame-allocator-diagnostic: process-teardown pid=227 reclaimed_frames=261 before_free=184889 before_allocated=11318 after_free=185150 after_allocated=11057
frame-allocator-diagnostic: process-teardown pid=225 reclaimed_frames=12 before_free=185150 before_allocated=11057 after_free=185162 after_allocated=11045
FAIL LTP CASE aiodio_sparse : 32
RUN LTP CASE alarm02
frame-allocator-diagnostic: process-teardown pid=231 reclaimed_frames=511 before_free=184618 before_allocated=11589 after_free=185129 after_allocated=11078
frame-allocator-diagnostic: process-teardown pid=229 reclaimed_frames=508 before_free=185129 before_allocated=11078 after_free=185637 after_allocated=10570
frame-allocator-diagnostic: process-teardown pid=233 reclaimed_frames=1 before_free=185105 before_allocated=11102 after_free=185106 after_allocated=11101
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
alarm02.c:36: TPASS: alarm(2147483647) passed
alarm02.c:38: TPASS: alarm(0) passed
alarm02.c:36: TPASS: alarm(2147483647) passed
alarm02.c:38: TPASS: alarm(0) passed
alarm02.c:36: TPASS: alarm(1073741823) passed
alarm02.c:38: TPASS: alarm(0) passed
frame-allocator-diagnostic: process-teardown pid=236 reclaimed_frames=12 before_free=184824 before_allocated=11383 after_free=184836 after_allocated=11371

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[116.883013 0:232 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=232 reclaimed_frames=262 before_free=184837 before_allocated=11370 after_free=185099 after_allocated=11108
frame-allocator-diagnostic: process-teardown pid=230 reclaimed_frames=13 before_free=185099 before_allocated=11108 after_free=185112 after_allocated=11095
FAIL LTP CASE alarm02 : 0
RUN LTP CASE alarm03
frame-allocator-diagnostic: process-teardown pid=242 reclaimed_frames=511 before_free=184569 before_allocated=11638 after_free=185080 after_allocated=11127
frame-allocator-diagnostic: process-teardown pid=234 reclaimed_frames=508 before_free=185080 before_allocated=11127 after_free=185588 after_allocated=10619
frame-allocator-diagnostic: process-teardown pid=244 reclaimed_frames=1 before_free=185056 before_allocated=11151 after_free=185057 after_allocated=11150
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
alarm03.c:30: TPASS: alarm(0) in parent process passed
alarm03.c:26: TPASS: alarm(0) in child process passed
frame-allocator-diagnostic: process-teardown pid=249 reclaimed_frames=7 before_free=184762 before_allocated=11445 after_free=184769 after_allocated=11438
frame-allocator-diagnostic: process-teardown pid=247 reclaimed_frames=11 before_free=184768 before_allocated=11439 after_free=184779 after_allocated=11428

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[120.973041 0:243 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=243 reclaimed_frames=262 before_free=184780 before_allocated=11427 after_free=185042 after_allocated=11165
frame-allocator-diagnostic: process-teardown pid=241 reclaimed_frames=12 before_free=185042 before_allocated=11165 after_free=185054 after_allocated=11153
FAIL LTP CASE alarm03 : 0
RUN LTP CASE alarm05
frame-allocator-diagnostic: process-teardown pid=252 reclaimed_frames=511 before_free=184511 before_allocated=11696 after_free=185022 after_allocated=11185
frame-allocator-diagnostic: process-teardown pid=245 reclaimed_frames=508 before_free=185022 before_allocated=11185 after_free=185530 after_allocated=10677
frame-allocator-diagnostic: process-teardown pid=254 reclaimed_frames=1 before_free=184998 before_allocated=11209 after_free=184999 after_allocated=11208
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
alarm05.c:28: TPASS: alarm(10) passed
alarm05.c:30: TPASS: alarm(1) passed
alarm05.c:32: TPASS: alarms_fired == 1 (1)
frame-allocator-diagnostic: process-teardown pid=257 reclaimed_frames=12 before_free=184717 before_allocated=11490 after_free=184729 after_allocated=11478

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[128.078739 0:253 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=253 reclaimed_frames=262 before_free=184730 before_allocated=11477 after_free=184992 after_allocated=11215
frame-allocator-diagnostic: process-teardown pid=251 reclaimed_frames=12 before_free=184992 before_allocated=11215 after_free=185004 after_allocated=11203
FAIL LTP CASE alarm05 : 0
RUN LTP CASE alarm06
frame-allocator-diagnostic: process-teardown pid=262 reclaimed_frames=511 before_free=184460 before_allocated=11747 after_free=184971 after_allocated=11236
frame-allocator-diagnostic: process-teardown pid=255 reclaimed_frames=508 before_free=184971 before_allocated=11236 after_free=185479 after_allocated=10728
frame-allocator-diagnostic: process-teardown pid=264 reclaimed_frames=1 before_free=184947 before_allocated=11260 after_free=184948 after_allocated=11259
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
alarm06.c:35: TPASS: alarm(0) passed
alarm06.c:40: TPASS: alarms_received == 0 (0)
frame-allocator-diagnostic: process-teardown pid=267 reclaimed_frames=12 before_free=184666 before_allocated=11541 after_free=184678 after_allocated=11529

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[135.268556 0:263 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=263 reclaimed_frames=262 before_free=184679 before_allocated=11528 after_free=184941 after_allocated=11266
frame-allocator-diagnostic: process-teardown pid=261 reclaimed_frames=13 before_free=184941 before_allocated=11266 after_free=184954 after_allocated=11253
FAIL LTP CASE alarm06 : 0
RUN LTP CASE alarm07
frame-allocator-diagnostic: process-teardown pid=271 reclaimed_frames=511 before_free=184410 before_allocated=11797 after_free=184921 after_allocated=11286
frame-allocator-diagnostic: process-teardown pid=265 reclaimed_frames=508 before_free=184921 before_allocated=11286 after_free=185429 after_allocated=10778
frame-allocator-diagnostic: process-teardown pid=273 reclaimed_frames=1 before_free=184897 before_allocated=11310 after_free=184898 after_allocated=11309
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
alarm07.c:36: TPASS: alarm_cnt == 1 (1)
alarm07.c:32: TPASS: alarm_cnt == 0 (0)
frame-allocator-diagnostic: process-teardown pid=278 reclaimed_frames=6 before_free=184603 before_allocated=11604 after_free=184609 after_allocated=11598
frame-allocator-diagnostic: process-teardown pid=276 reclaimed_frames=12 before_free=184608 before_allocated=11599 after_free=184620 after_allocated=11587

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[142.387080 0:272 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=272 reclaimed_frames=262 before_free=184621 before_allocated=11586 after_free=184883 after_allocated=11324
frame-allocator-diagnostic: process-teardown pid=270 reclaimed_frames=13 before_free=184883 before_allocated=11324 after_free=184896 after_allocated=11311
FAIL LTP CASE alarm07 : 0
RUN LTP CASE ar01.sh
frame-allocator-diagnostic: process-teardown pid=281 reclaimed_frames=511 before_free=184353 before_allocated=11854 after_free=184864 after_allocated=11343
frame-allocator-diagnostic: process-teardown pid=274 reclaimed_frames=508 before_free=184864 before_allocated=11343 after_free=185372 after_allocated=10835
frame-allocator-diagnostic: process-teardown pid=283 reclaimed_frames=1 before_free=184840 before_allocated=11367 after_free=184841 after_allocated=11366
sh: tst_test.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=285 reclaimed_frames=8 before_free=184290 before_allocated=11917 after_free=184298 after_allocated=11909
frame-allocator-diagnostic: process-teardown pid=282 reclaimed_frames=535 before_free=184298 before_allocated=11909 after_free=184833 after_allocated=11374
frame-allocator-diagnostic: process-teardown pid=280 reclaimed_frames=12 before_free=184833 before_allocated=11374 after_free=184845 after_allocated=11362
FAIL LTP CASE ar01.sh : 2
RUN LTP CASE arch_prctl01
frame-allocator-diagnostic: process-teardown pid=287 reclaimed_frames=511 before_free=184302 before_allocated=11905 after_free=184813 after_allocated=11394
frame-allocator-diagnostic: process-teardown pid=284 reclaimed_frames=508 before_free=184813 before_allocated=11394 after_free=185321 after_allocated=10886
frame-allocator-diagnostic: process-teardown pid=289 reclaimed_frames=1 before_free=184789 before_allocated=11418 after_free=184790 after_allocated=11417
tst_test.c:1201: TCONF: This arch 'unknown' is not supported for test!
frame-allocator-diagnostic: process-teardown pid=288 reclaimed_frames=261 before_free=184530 before_allocated=11677 after_free=184791 after_allocated=11416
frame-allocator-diagnostic: process-teardown pid=286 reclaimed_frames=12 before_free=184791 before_allocated=11416 after_free=184803 after_allocated=11404
FAIL LTP CASE arch_prctl01 : 32
RUN LTP CASE arping01.sh
frame-allocator-diagnostic: process-teardown pid=292 reclaimed_frames=511 before_free=184261 before_allocated=11946 after_free=184772 after_allocated=11435
frame-allocator-diagnostic: process-teardown pid=290 reclaimed_frames=508 before_free=184755 before_allocated=11452 after_free=185263 after_allocated=10944
frame-allocator-diagnostic: process-teardown pid=294 reclaimed_frames=1 before_free=184747 before_allocated=11460 after_free=184748 after_allocated=11459
sh: tst_net.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=296 reclaimed_frames=6 before_free=184223 before_allocated=11984 after_free=184229 after_allocated=11978
frame-allocator-diagnostic: process-teardown pid=293 reclaimed_frames=512 before_free=184229 before_allocated=11978 after_free=184741 after_allocated=11466
frame-allocator-diagnostic: process-teardown pid=291 reclaimed_frames=12 before_free=184741 before_allocated=11466 after_free=184753 after_allocated=11454
FAIL LTP CASE arping01.sh : 2
RUN LTP CASE asapi_01
frame-allocator-diagnostic: process-teardown pid=298 reclaimed_frames=511 before_free=184210 before_allocated=11997 after_free=184721 after_allocated=11486
frame-allocator-diagnostic: process-teardown pid=295 reclaimed_frames=508 before_free=184721 before_allocated=11486 after_free=185229 after_allocated=10978
frame-allocator-diagnostic: process-teardown pid=300 reclaimed_frames=1 before_free=184697 before_allocated=11510 after_free=184698 after_allocated=11509
asapi_01    1  TPASS  :  IN6_ARE_ADDR_EQUAL
asapi_01    2  TFAIL  :  asapi_01.c:119: "hopopt" protocols entry
asapi_01    3  TPASS  :  "ipv6" protocols entry
asapi_01    4  TPASS  :  "ipv6-route" protocols entry
asapi_01    5  TPASS  :  "ipv6-frag" protocols entry
asapi_01    6  TPASS  :  "esp" protocols entry
asapi_01    7  TPASS  :  "ah" protocols entry
asapi_01    8  TPASS  :  "ipv6-icmp" protocols entry
asapi_01    9  TPASS  :  "ipv6-nonxt" protocols entry
asapi_01   10  TPASS  :  "ipv6-opts" protocols entry
[153.496107 0:299 arceos_posix_api::imp::net:403] sys_bind => Err(EINVAL)
asapi_01   11  TBROK  :  asapi_01.c:355: bind(3, sock_ntop: unknown AF_xxx: 0, len: 16, 16) failed: errno=EINVAL(22): Invalid argument
asapi_01   12  TBROK  :  asapi_01.c:355: Remaining cases broken
frame-allocator-diagnostic: process-teardown pid=299 reclaimed_frames=271 before_free=184428 before_allocated=11779 after_free=184699 after_allocated=11508
frame-allocator-diagnostic: process-teardown pid=297 reclaimed_frames=12 before_free=184699 before_allocated=11508 after_free=184711 after_allocated=11496
FAIL LTP CASE asapi_01 : 3
RUN LTP CASE asapi_02
frame-allocator-diagnostic: process-teardown pid=303 reclaimed_frames=511 before_free=184168 before_allocated=12039 after_free=184679 after_allocated=11528
frame-allocator-diagnostic: process-teardown pid=301 reclaimed_frames=508 before_free=184679 before_allocated=11528 after_free=185187 after_allocated=11020
frame-allocator-diagnostic: process-teardown pid=305 reclaimed_frames=1 before_free=184655 before_allocated=11552 after_free=184656 after_allocated=11551
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
asapi_02.c:219: TCONF: socket(10, 3, 58) failed: EAFNOSUPPORT (97)
frame-allocator-diagnostic: process-teardown pid=308 reclaimed_frames=12 before_free=184371 before_allocated=11836 after_free=184383 after_allocated=11824

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[157.750271 0:304 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=304 reclaimed_frames=265 before_free=184384 before_allocated=11823 after_free=184649 after_allocated=11558
frame-allocator-diagnostic: process-teardown pid=302 reclaimed_frames=12 before_free=184649 before_allocated=11558 after_free=184661 after_allocated=11546
FAIL LTP CASE asapi_02 : 32
RUN LTP CASE asapi_03
frame-allocator-diagnostic: process-teardown pid=311 reclaimed_frames=511 before_free=184119 before_allocated=12088 after_free=184630 after_allocated=11577
frame-allocator-diagnostic: process-teardown pid=306 reclaimed_frames=508 before_free=184630 before_allocated=11577 after_free=185138 after_allocated=11069
frame-allocator-diagnostic: process-teardown pid=313 reclaimed_frames=1 before_free=184605 before_allocated=11602 after_free=184606 after_allocated=11601
asapi_03    1  TCONF  :  asapi_03.c:255: socket(10, 3, 159) failed: errno=EAFNOSUPPORT(97): Address family not supported by protocol
asapi_03    2  TCONF  :  asapi_03.c:255: Remaining cases not appropriate for configuration: errno=EAFNOSUPPORT(97): Address family not supported by protocol
frame-allocator-diagnostic: process-teardown pid=312 reclaimed_frames=270 before_free=184337 before_allocated=11870 after_free=184607 after_allocated=11600
frame-allocator-diagnostic: process-teardown pid=310 reclaimed_frames=12 before_free=184607 before_allocated=11600 after_free=184619 after_allocated=11588
FAIL LTP CASE asapi_03 : 32
RUN LTP CASE ask_password.sh
frame-allocator-diagnostic: process-teardown pid=316 reclaimed_frames=511 before_free=184075 before_allocated=12132 after_free=184586 after_allocated=11621
frame-allocator-diagnostic: process-teardown pid=314 reclaimed_frames=508 before_free=184586 before_allocated=11621 after_free=185094 after_allocated=11113
frame-allocator-diagnostic: process-teardown pid=318 reclaimed_frames=1 before_free=184562 before_allocated=11645 after_free=184563 after_allocated=11644
timeout: can't execute 'ltp/testcases/bin/ask_password.sh': Exec format error
frame-allocator-diagnostic: process-teardown pid=317 reclaimed_frames=3 before_free=184561 before_allocated=11646 after_free=184564 after_allocated=11643
frame-allocator-diagnostic: process-teardown pid=315 reclaimed_frames=13 before_free=184564 before_allocated=11643 after_free=184577 after_allocated=11630
FAIL LTP CASE ask_password.sh : 126
RUN LTP CASE aslr01
frame-allocator-diagnostic: process-teardown pid=321 reclaimed_frames=511 before_free=184034 before_allocated=12173 after_free=184545 after_allocated=11662
frame-allocator-diagnostic: process-teardown pid=319 reclaimed_frames=508 before_free=184545 before_allocated=11662 after_free=185053 after_allocated=11154
frame-allocator-diagnostic: process-teardown pid=323 reclaimed_frames=1 before_free=184521 before_allocated=11686 after_free=184522 after_allocated=11685
tst_kconfig.c:71: TINFO: Couldn't locate kernel config!
tst_kconfig.c:207: TBROK: Cannot parse kernel .config
frame-allocator-diagnostic: process-teardown pid=322 reclaimed_frames=264 before_free=184259 before_allocated=11948 after_free=184523 after_allocated=11684
frame-allocator-diagnostic: process-teardown pid=320 reclaimed_frames=12 before_free=184523 before_allocated=11684 after_free=184535 after_allocated=11672
FAIL LTP CASE aslr01 : 2
RUN LTP CASE assign_password.sh
frame-allocator-diagnostic: process-teardown pid=326 reclaimed_frames=511 before_free=183991 before_allocated=12216 after_free=184502 after_allocated=11705
frame-allocator-diagnostic: process-teardown pid=324 reclaimed_frames=508 before_free=183977 before_allocated=12230 after_free=184485 after_allocated=11722
frame-allocator-diagnostic: process-teardown pid=328 reclaimed_frames=1 before_free=184478 before_allocated=11729 after_free=184479 after_allocated=11728
timeout: can't execute 'ltp/testcases/bin/assign_password.sh': Exec format error
frame-allocator-diagnostic: process-teardown pid=327 reclaimed_frames=4 before_free=184476 before_allocated=11731 after_free=184480 after_allocated=11727
frame-allocator-diagnostic: process-teardown pid=325 reclaimed_frames=13 before_free=184480 before_allocated=11727 after_free=184493 after_allocated=11714
FAIL LTP CASE assign_password.sh : 126
RUN LTP CASE atof01
frame-allocator-diagnostic: process-teardown pid=331 reclaimed_frames=511 before_free=183950 before_allocated=12257 after_free=184461 after_allocated=11746
frame-allocator-diagnostic: process-teardown pid=329 reclaimed_frames=508 before_free=184461 before_allocated=11746 after_free=184969 after_allocated=11238
frame-allocator-diagnostic: process-teardown pid=333 reclaimed_frames=1 before_free=184437 before_allocated=11770 after_free=184438 after_allocated=11769
atof01      1  TPASS  :  Test passed
atof01      2  TPASS  :  Test passed
atof01      3  TPASS  :  Test passed
atof01      4  TPASS  :  Test passed
frame-allocator-diagnostic: process-teardown pid=332 reclaimed_frames=263 before_free=184176 before_allocated=12031 after_free=184439 after_allocated=11768
frame-allocator-diagnostic: process-teardown pid=330 reclaimed_frames=12 before_free=184439 before_allocated=11768 after_free=184451 after_allocated=11756
FAIL LTP CASE atof01 : 0
RUN LTP CASE autogroup01
frame-allocator-diagnostic: process-teardown pid=336 reclaimed_frames=511 before_free=183908 before_allocated=12299 after_free=184419 after_allocated=11788
frame-allocator-diagnostic: process-teardown pid=334 reclaimed_frames=508 before_free=184419 before_allocated=11788 after_free=184927 after_allocated=11280
frame-allocator-diagnostic: process-teardown pid=338 reclaimed_frames=1 before_free=184395 before_allocated=11812 after_free=184396 after_allocated=11811
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
autogroup01.c:65: TCONF: autogroup not supported
frame-allocator-diagnostic: process-teardown pid=341 reclaimed_frames=12 before_free=184114 before_allocated=12093 after_free=184126 after_allocated=12081

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[176.702313 0:337 axfs::fops:297] [AxError::NotADirectory]
[176.703294 0:337 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=337 reclaimed_frames=262 before_free=184127 before_allocated=12080 after_free=184389 after_allocated=11818
frame-allocator-diagnostic: process-teardown pid=335 reclaimed_frames=12 before_free=184389 before_allocated=11818 after_free=184401 after_allocated=11806
FAIL LTP CASE autogroup01 : 32
RUN LTP CASE bbr01.sh
frame-allocator-diagnostic: process-teardown pid=344 reclaimed_frames=511 before_free=183858 before_allocated=12349 after_free=184369 after_allocated=11838
frame-allocator-diagnostic: process-teardown pid=339 reclaimed_frames=508 before_free=184369 before_allocated=11838 after_free=184877 after_allocated=11330
frame-allocator-diagnostic: process-teardown pid=346 reclaimed_frames=1 before_free=184345 before_allocated=11862 after_free=184346 after_allocated=11861
sh: tcp_cc_lib.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=348 reclaimed_frames=7 before_free=183819 before_allocated=12388 after_free=183826 after_allocated=12381
frame-allocator-diagnostic: process-teardown pid=345 reclaimed_frames=513 before_free=183826 before_allocated=12381 after_free=184339 after_allocated=11868
frame-allocator-diagnostic: process-teardown pid=343 reclaimed_frames=12 before_free=184339 before_allocated=11868 after_free=184351 after_allocated=11856
FAIL LTP CASE bbr01.sh : 2
RUN LTP CASE bbr02.sh
frame-allocator-diagnostic: process-teardown pid=350 reclaimed_frames=511 before_free=183808 before_allocated=12399 after_free=184319 after_allocated=11888
frame-allocator-diagnostic: process-teardown pid=347 reclaimed_frames=508 before_free=183794 before_allocated=12413 after_free=184302 after_allocated=11905
frame-allocator-diagnostic: process-teardown pid=352 reclaimed_frames=1 before_free=184295 before_allocated=11912 after_free=184296 after_allocated=11911
sh: tcp_cc_lib.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=354 reclaimed_frames=7 before_free=183769 before_allocated=12438 after_free=183776 after_allocated=12431
frame-allocator-diagnostic: process-teardown pid=351 reclaimed_frames=513 before_free=183776 before_allocated=12431 after_free=184289 after_allocated=11918
frame-allocator-diagnostic: process-teardown pid=349 reclaimed_frames=12 before_free=184289 before_allocated=11918 after_free=184301 after_allocated=11906
FAIL LTP CASE bbr02.sh : 2
RUN LTP CASE bind01
frame-allocator-diagnostic: process-teardown pid=356 reclaimed_frames=511 before_free=183757 before_allocated=12450 after_free=184268 after_allocated=11939
frame-allocator-diagnostic: process-teardown pid=353 reclaimed_frames=508 before_free=184268 before_allocated=11939 after_free=184776 after_allocated=11431
frame-allocator-diagnostic: process-teardown pid=358 reclaimed_frames=1 before_free=184244 before_allocated=11963 after_free=184245 after_allocated=11962
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
bind01.c:60: TPASS: invalid salen : EINVAL (22)
bind01.c:60: TPASS: invalid socket : ENOTSOCK (88)
bind01.c:63: TPASS: INADDR_ANYPORT passed
bind01.c:60: TFAIL: UNIX-domain of current directory expected EAFNOSUPPORT: EINVAL (22)
bind01.c:60: TFAIL: non-local address succeeded
bind01.c:60: TPASS: sockfd is not a valid file descriptor : EBADF (9)
bind01.c:60: TFAIL: a component of addr prefix is not a directory expected ENOTDIR: ENOTSOCK (88)
frame-allocator-diagnostic: process-teardown pid=361 reclaimed_frames=11 before_free=183961 before_allocated=12246 after_free=183972 after_allocated=12235

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[183.613781 0:357 axfs::fops:297] [AxError::NotADirectory]
[183.614760 0:357 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=357 reclaimed_frames=265 before_free=183973 before_allocated=12234 after_free=184238 after_allocated=11969
frame-allocator-diagnostic: process-teardown pid=355 reclaimed_frames=13 before_free=184238 before_allocated=11969 after_free=184251 after_allocated=11956
FAIL LTP CASE bind01 : 0
RUN LTP CASE bind02
frame-allocator-diagnostic: process-teardown pid=364 reclaimed_frames=511 before_free=183707 before_allocated=12500 after_free=184218 after_allocated=11989
frame-allocator-diagnostic: process-teardown pid=359 reclaimed_frames=508 before_free=184218 before_allocated=11989 after_free=184726 after_allocated=11481
frame-allocator-diagnostic: process-teardown pid=366 reclaimed_frames=1 before_free=184194 before_allocated=12013 after_free=184195 after_allocated=12012
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
bind02.c:52: TINFO: Switching credentials to user: nobody, group: nogroup
bind02.c:39: TFAIL: bind() succeeded
frame-allocator-diagnostic: process-teardown pid=369 reclaimed_frames=13 before_free=183909 before_allocated=12298 after_free=183922 after_allocated=12285

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[187.909753 0:365 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=365 reclaimed_frames=265 before_free=183923 before_allocated=12284 after_free=184188 after_allocated=12019
frame-allocator-diagnostic: process-teardown pid=363 reclaimed_frames=13 before_free=184188 before_allocated=12019 after_free=184201 after_allocated=12006
FAIL LTP CASE bind02 : 0
RUN LTP CASE bind03
frame-allocator-diagnostic: process-teardown pid=372 reclaimed_frames=511 before_free=183658 before_allocated=12549 after_free=184169 after_allocated=12038
frame-allocator-diagnostic: process-teardown pid=367 reclaimed_frames=508 before_free=184169 before_allocated=12038 after_free=184677 after_allocated=11530
frame-allocator-diagnostic: process-teardown pid=374 reclaimed_frames=1 before_free=184145 before_allocated=12062 after_free=184146 after_allocated=12061
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
bind03.c:72: TBROK: bind(3, socket.1, 110) failed: ENOTSOCK (88)
frame-allocator-diagnostic: process-teardown pid=377 reclaimed_frames=11 before_free=183862 before_allocated=12345 after_free=183873 after_allocated=12334

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[192.233628 0:373 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=373 reclaimed_frames=265 before_free=183874 before_allocated=12333 after_free=184139 after_allocated=12068
frame-allocator-diagnostic: process-teardown pid=371 reclaimed_frames=12 before_free=184139 before_allocated=12068 after_free=184151 after_allocated=12056
FAIL LTP CASE bind03 : 2
RUN LTP CASE bind04
frame-allocator-diagnostic: process-teardown pid=380 reclaimed_frames=511 before_free=183608 before_allocated=12599 after_free=184119 after_allocated=12088
frame-allocator-diagnostic: process-teardown pid=375 reclaimed_frames=508 before_free=184119 before_allocated=12088 after_free=184627 after_allocated=11580
frame-allocator-diagnostic: process-teardown pid=382 reclaimed_frames=1 before_free=184095 before_allocated=12112 after_free=184096 after_allocated=12111
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
bind04.c:117: TINFO: Testing AF_UNIX pathname stream
bind04.c:121: TFAIL: bind() failed: ENOTSOCK (88)
bind04.c:117: TINFO: Testing AF_UNIX pathname seqpacket
bind04.c:118: TCONF: socket(1, 5, 0) failed: ESOCKTNOSUPPORT (94)
frame-allocator-diagnostic: process-teardown pid=385 reclaimed_frames=13 before_free=183808 before_allocated=12399 after_free=183821 after_allocated=12386

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[196.632763 0:381 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=381 reclaimed_frames=267 before_free=183822 before_allocated=12385 after_free=184089 after_allocated=12118
frame-allocator-diagnostic: process-teardown pid=379 reclaimed_frames=12 before_free=184089 before_allocated=12118 after_free=184101 after_allocated=12106
FAIL LTP CASE bind04 : 32
RUN LTP CASE bind05
frame-allocator-diagnostic: process-teardown pid=388 reclaimed_frames=511 before_free=183558 before_allocated=12649 after_free=184069 after_allocated=12138
frame-allocator-diagnostic: process-teardown pid=383 reclaimed_frames=508 before_free=184069 before_allocated=12138 after_free=184577 after_allocated=11630
frame-allocator-diagnostic: process-teardown pid=390 reclaimed_frames=1 before_free=184045 before_allocated=12162 after_free=184046 after_allocated=12161
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
bind05.c:131: TINFO: Testing AF_UNIX pathname datagram
bind05.c:134: TFAIL: bind() failed: ENOTSOCK (88)
bind05.c:131: TINFO: Testing AF_UNIX abstract datagram
bind05.c:134: TFAIL: bind() failed: ENOTSOCK (88)
bind05.c:131: TINFO: Testing IPv4 loop UDP variant 1
bind05.c:167: TPASS: Communication successful
bind05.c:131: TINFO: Testing IPv4 loop UDP variant 2
bind05.c:167: TPASS: Communication successful
bind05.c:131: TINFO: Testing IPv4 loop UDP-Lite
bind05.c:132: TCONF: socket(2, 2, 136) failed: EPROTONOSUPPORT (93)
frame-allocator-diagnostic: process-teardown pid=393 reclaimed_frames=13 before_free=183758 before_allocated=12449 after_free=183771 after_allocated=12436

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[200.868685 0:389 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=389 reclaimed_frames=267 before_free=183772 before_allocated=12435 after_free=184039 after_allocated=12168
frame-allocator-diagnostic: process-teardown pid=387 reclaimed_frames=12 before_free=184039 before_allocated=12168 after_free=184051 after_allocated=12156
FAIL LTP CASE bind05 : 32
RUN LTP CASE bind06
frame-allocator-diagnostic: process-teardown pid=398 reclaimed_frames=511 before_free=183508 before_allocated=12699 after_free=184019 after_allocated=12188
frame-allocator-diagnostic: process-teardown pid=391 reclaimed_frames=508 before_free=184019 before_allocated=12188 after_free=184527 after_allocated=11680
frame-allocator-diagnostic: process-teardown pid=400 reclaimed_frames=1 before_free=183995 before_allocated=12212 after_free=183996 after_allocated=12211
tst_kconfig.c:71: TINFO: Couldn't locate kernel config!
tst_kconfig.c:207: TBROK: Cannot parse kernel .config
frame-allocator-diagnostic: process-teardown pid=399 reclaimed_frames=269 before_free=183728 before_allocated=12479 after_free=183997 after_allocated=12210
frame-allocator-diagnostic: process-teardown pid=397 reclaimed_frames=12 before_free=183997 before_allocated=12210 after_free=184009 after_allocated=12198
FAIL LTP CASE bind06 : 2
RUN LTP CASE bind_noport01.sh
frame-allocator-diagnostic: process-teardown pid=403 reclaimed_frames=511 before_free=183466 before_allocated=12741 after_free=183977 after_allocated=12230
frame-allocator-diagnostic: process-teardown pid=401 reclaimed_frames=508 before_free=183977 before_allocated=12230 after_free=184485 after_allocated=11722
frame-allocator-diagnostic: process-teardown pid=405 reclaimed_frames=1 before_free=183953 before_allocated=12254 after_free=183954 after_allocated=12253
sh: tst_net.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=407 reclaimed_frames=7 before_free=183427 before_allocated=12780 after_free=183434 after_allocated=12773
frame-allocator-diagnostic: process-teardown pid=404 reclaimed_frames=513 before_free=183434 before_allocated=12773 after_free=183947 after_allocated=12260
frame-allocator-diagnostic: process-teardown pid=402 reclaimed_frames=12 before_free=183947 before_allocated=12260 after_free=183959 after_allocated=12248
FAIL LTP CASE bind_noport01.sh : 2
RUN LTP CASE binfmt_misc01.sh
frame-allocator-diagnostic: process-teardown pid=409 reclaimed_frames=511 before_free=183416 before_allocated=12791 after_free=183927 after_allocated=12280
frame-allocator-diagnostic: process-teardown pid=406 reclaimed_frames=508 before_free=183910 before_allocated=12297 after_free=184418 after_allocated=11789
frame-allocator-diagnostic: process-teardown pid=411 reclaimed_frames=1 before_free=183902 before_allocated=12305 after_free=183903 after_allocated=12304
sh: binfmt_misc_lib.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=413 reclaimed_frames=8 before_free=183372 before_allocated=12835 after_free=183380 after_allocated=12827
frame-allocator-diagnostic: process-teardown pid=410 reclaimed_frames=516 before_free=183380 before_allocated=12827 after_free=183896 after_allocated=12311
frame-allocator-diagnostic: process-teardown pid=408 reclaimed_frames=13 before_free=183896 before_allocated=12311 after_free=183909 after_allocated=12298
FAIL LTP CASE binfmt_misc01.sh : 2
RUN LTP CASE binfmt_misc02.sh
frame-allocator-diagnostic: process-teardown pid=415 reclaimed_frames=511 before_free=183365 before_allocated=12842 after_free=183876 after_allocated=12331
frame-allocator-diagnostic: process-teardown pid=412 reclaimed_frames=508 before_free=183351 before_allocated=12856 after_free=183859 after_allocated=12348
frame-allocator-diagnostic: process-teardown pid=417 reclaimed_frames=1 before_free=183852 before_allocated=12355 after_free=183853 after_allocated=12354
sh: binfmt_misc_lib.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=419 reclaimed_frames=8 before_free=183319 before_allocated=12888 after_free=183327 after_allocated=12880
frame-allocator-diagnostic: process-teardown pid=416 reclaimed_frames=519 before_free=183327 before_allocated=12880 after_free=183846 after_allocated=12361
frame-allocator-diagnostic: process-teardown pid=414 reclaimed_frames=13 before_free=183846 before_allocated=12361 after_free=183859 after_allocated=12348
FAIL LTP CASE binfmt_misc02.sh : 2
RUN LTP CASE binfmt_misc_lib.sh
SKIP LTP CASE binfmt_misc_lib.sh : LTP shell helper library is not a standalone test
frame-allocator-diagnostic: process-teardown pid=420 reclaimed_frames=12 before_free=183837 before_allocated=12370 after_free=183849 after_allocated=12358
FAIL LTP CASE binfmt_misc_lib.sh : 32
RUN LTP CASE block_dev
frame-allocator-diagnostic: process-teardown pid=422 reclaimed_frames=511 before_free=183306 before_allocated=12901 after_free=183817 after_allocated=12390
frame-allocator-diagnostic: process-teardown pid=418 reclaimed_frames=508 before_free=183817 before_allocated=12390 after_free=184325 after_allocated=11882
frame-allocator-diagnostic: process-teardown pid=424 reclaimed_frames=1 before_free=183792 before_allocated=12415 after_free=183793 after_allocated=12414
block_dev    1  TCONF  :  tst_module.c:69: Failed to find module 'ltp_block_dev.ko'
block_dev    2  TCONF  :  tst_module.c:69: Remaining cases not appropriate for configuration
frame-allocator-diagnostic: process-teardown pid=423 reclaimed_frames=265 before_free=183529 before_allocated=12678 after_free=183794 after_allocated=12413
frame-allocator-diagnostic: process-teardown pid=421 reclaimed_frames=13 before_free=183794 before_allocated=12413 after_free=183807 after_allocated=12400
FAIL LTP CASE block_dev : 32
RUN LTP CASE bpf_map01
frame-allocator-diagnostic: process-teardown pid=427 reclaimed_frames=511 before_free=183263 before_allocated=12944 after_free=183774 after_allocated=12433
frame-allocator-diagnostic: process-teardown pid=425 reclaimed_frames=508 before_free=183774 before_allocated=12433 after_free=184282 after_allocated=11925
frame-allocator-diagnostic: process-teardown pid=429 reclaimed_frames=1 before_free=183750 before_allocated=12457 after_free=183751 after_allocated=12456
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
bpf_common.c:16: TINFO: Raising RLIMIT_MEMLOCK to 2097151
../../../../include/lapi/bpf.h:623: TCONF: syscall(280) __NR_bpf not supported on your arch
frame-allocator-diagnostic: process-teardown pid=432 reclaimed_frames=10 before_free=183462 before_allocated=12745 after_free=183472 after_allocated=12735

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[218.184488 0:428 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=428 reclaimed_frames=271 before_free=183473 before_allocated=12734 after_free=183744 after_allocated=12463
frame-allocator-diagnostic: process-teardown pid=426 reclaimed_frames=13 before_free=183744 before_allocated=12463 after_free=183757 after_allocated=12450
FAIL LTP CASE bpf_map01 : 32
RUN LTP CASE bpf_prog01
frame-allocator-diagnostic: process-teardown pid=435 reclaimed_frames=511 before_free=183214 before_allocated=12993 after_free=183725 after_allocated=12482
frame-allocator-diagnostic: process-teardown pid=430 reclaimed_frames=508 before_free=183725 before_allocated=12482 after_free=184233 after_allocated=11974
frame-allocator-diagnostic: process-teardown pid=437 reclaimed_frames=1 before_free=183701 before_allocated=12506 after_free=183702 after_allocated=12505
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
bpf_common.c:16: TINFO: Raising RLIMIT_MEMLOCK to 2097151
../../../../include/lapi/bpf.h:623: TCONF: syscall(280) __NR_bpf not supported on your arch
frame-allocator-diagnostic: process-teardown pid=440 reclaimed_frames=11 before_free=183415 before_allocated=12792 after_free=183426 after_allocated=12781

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[222.559466 0:436 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=436 reclaimed_frames=268 before_free=183427 before_allocated=12780 after_free=183695 after_allocated=12512
frame-allocator-diagnostic: process-teardown pid=434 reclaimed_frames=12 before_free=183695 before_allocated=12512 after_free=183707 after_allocated=12500
FAIL LTP CASE bpf_prog01 : 32
RUN LTP CASE bpf_prog02
frame-allocator-diagnostic: process-teardown pid=443 reclaimed_frames=511 before_free=183164 before_allocated=13043 after_free=183675 after_allocated=12532
frame-allocator-diagnostic: process-teardown pid=438 reclaimed_frames=508 before_free=183675 before_allocated=12532 after_free=184183 after_allocated=12024
frame-allocator-diagnostic: process-teardown pid=445 reclaimed_frames=1 before_free=183651 before_allocated=12556 after_free=183652 after_allocated=12555
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
bpf_common.c:16: TINFO: Raising RLIMIT_MEMLOCK to 2097151
tst_capability.c:17: TCONF: syscall(90) __NR_capget not supported on your arch
frame-allocator-diagnostic: process-teardown pid=448 reclaimed_frames=10 before_free=183365 before_allocated=12842 after_free=183375 after_allocated=12832

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[226.867013 0:444 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=444 reclaimed_frames=269 before_free=183376 before_allocated=12831 after_free=183645 after_allocated=12562
frame-allocator-diagnostic: process-teardown pid=442 reclaimed_frames=12 before_free=183645 before_allocated=12562 after_free=183657 after_allocated=12550
FAIL LTP CASE bpf_prog02 : 32
RUN LTP CASE bpf_prog03
frame-allocator-diagnostic: process-teardown pid=451 reclaimed_frames=511 before_free=183114 before_allocated=13093 after_free=183625 after_allocated=12582
frame-allocator-diagnostic: process-teardown pid=446 reclaimed_frames=508 before_free=183625 before_allocated=12582 after_free=184133 after_allocated=12074
frame-allocator-diagnostic: process-teardown pid=453 reclaimed_frames=1 before_free=183600 before_allocated=12607 after_free=183601 after_allocated=12606
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
bpf_common.c:16: TINFO: Raising RLIMIT_MEMLOCK to 2097151
tst_capability.c:17: TCONF: syscall(90) __NR_capget not supported on your arch
frame-allocator-diagnostic: process-teardown pid=456 reclaimed_frames=10 before_free=183313 before_allocated=12894 after_free=183323 after_allocated=12884

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[231.279029 0:452 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=452 reclaimed_frames=270 before_free=183324 before_allocated=12883 after_free=183594 after_allocated=12613
frame-allocator-diagnostic: process-teardown pid=450 reclaimed_frames=13 before_free=183594 before_allocated=12613 after_free=183607 after_allocated=12600
FAIL LTP CASE bpf_prog03 : 32
RUN LTP CASE bpf_prog04
frame-allocator-diagnostic: process-teardown pid=459 reclaimed_frames=511 before_free=183063 before_allocated=13144 after_free=183574 after_allocated=12633
frame-allocator-diagnostic: process-teardown pid=454 reclaimed_frames=508 before_free=183574 before_allocated=12633 after_free=184082 after_allocated=12125
frame-allocator-diagnostic: process-teardown pid=461 reclaimed_frames=1 before_free=183550 before_allocated=12657 after_free=183551 after_allocated=12656
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
bpf_common.c:16: TINFO: Raising RLIMIT_MEMLOCK to 2097151
tst_capability.c:17: TCONF: syscall(90) __NR_capget not supported on your arch
frame-allocator-diagnostic: process-teardown pid=464 reclaimed_frames=11 before_free=183264 before_allocated=12943 after_free=183275 after_allocated=12932

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[235.565369 0:460 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=460 reclaimed_frames=268 before_free=183276 before_allocated=12931 after_free=183544 after_allocated=12663
frame-allocator-diagnostic: process-teardown pid=458 reclaimed_frames=13 before_free=183544 before_allocated=12663 after_free=183557 after_allocated=12650
FAIL LTP CASE bpf_prog04 : 32
RUN LTP CASE bpf_prog05
frame-allocator-diagnostic: process-teardown pid=467 reclaimed_frames=511 before_free=183014 before_allocated=13193 after_free=183525 after_allocated=12682
frame-allocator-diagnostic: process-teardown pid=462 reclaimed_frames=508 before_free=183525 before_allocated=12682 after_free=184033 after_allocated=12174
frame-allocator-diagnostic: process-teardown pid=469 reclaimed_frames=1 before_free=183500 before_allocated=12707 after_free=183501 after_allocated=12706
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
bpf_common.c:16: TINFO: Raising RLIMIT_MEMLOCK to 2097151
tst_capability.c:17: TCONF: syscall(90) __NR_capget not supported on your arch
frame-allocator-diagnostic: process-teardown pid=472 reclaimed_frames=10 before_free=183213 before_allocated=12994 after_free=183223 after_allocated=12984

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[239.906292 0:468 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=468 reclaimed_frames=270 before_free=183224 before_allocated=12983 after_free=183494 after_allocated=12713
frame-allocator-diagnostic: process-teardown pid=466 reclaimed_frames=13 before_free=183494 before_allocated=12713 after_free=183507 after_allocated=12700
FAIL LTP CASE bpf_prog05 : 32
RUN LTP CASE bpf_prog06
frame-allocator-diagnostic: process-teardown pid=475 reclaimed_frames=511 before_free=182963 before_allocated=13244 after_free=183474 after_allocated=12733
frame-allocator-diagnostic: process-teardown pid=470 reclaimed_frames=508 before_free=183474 before_allocated=12733 after_free=183982 after_allocated=12225
frame-allocator-diagnostic: process-teardown pid=477 reclaimed_frames=1 before_free=183450 before_allocated=12757 after_free=183451 after_allocated=12756
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
bpf_common.c:16: TINFO: Raising RLIMIT_MEMLOCK to 2097151
tst_capability.c:17: TCONF: syscall(90) __NR_capget not supported on your arch
frame-allocator-diagnostic: process-teardown pid=480 reclaimed_frames=10 before_free=183163 before_allocated=13044 after_free=183173 after_allocated=13034

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[244.316086 0:476 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=476 reclaimed_frames=270 before_free=183174 before_allocated=13033 after_free=183444 after_allocated=12763
frame-allocator-diagnostic: process-teardown pid=474 reclaimed_frames=13 before_free=183444 before_allocated=12763 after_free=183457 after_allocated=12750
FAIL LTP CASE bpf_prog06 : 32
RUN LTP CASE bpf_prog07
frame-allocator-diagnostic: process-teardown pid=483 reclaimed_frames=511 before_free=182914 before_allocated=13293 after_free=183425 after_allocated=12782
frame-allocator-diagnostic: process-teardown pid=478 reclaimed_frames=508 before_free=183425 before_allocated=12782 after_free=183933 after_allocated=12274
frame-allocator-diagnostic: process-teardown pid=485 reclaimed_frames=1 before_free=183401 before_allocated=12806 after_free=183402 after_allocated=12805
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
bpf_common.c:16: TINFO: Raising RLIMIT_MEMLOCK to 2097151
tst_capability.c:17: TCONF: syscall(90) __NR_capget not supported on your arch
frame-allocator-diagnostic: process-teardown pid=488 reclaimed_frames=10 before_free=183114 before_allocated=13093 after_free=183124 after_allocated=13083

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[248.655383 0:484 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=484 reclaimed_frames=270 before_free=183125 before_allocated=13082 after_free=183395 after_allocated=12812
frame-allocator-diagnostic: process-teardown pid=482 reclaimed_frames=12 before_free=183395 before_allocated=12812 after_free=183407 after_allocated=12800
FAIL LTP CASE bpf_prog07 : 32
RUN LTP CASE brk01
frame-allocator-diagnostic: process-teardown pid=491 reclaimed_frames=511 before_free=182864 before_allocated=13343 after_free=183375 after_allocated=12832
frame-allocator-diagnostic: process-teardown pid=486 reclaimed_frames=508 before_free=183375 before_allocated=12832 after_free=183883 after_allocated=12324
frame-allocator-diagnostic: process-teardown pid=493 reclaimed_frames=1 before_free=183351 before_allocated=12856 after_free=183352 after_allocated=12855
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
brk01.c:24: TINFO: Testing libc variant
brk01.c:35: TCONF: brk() not implemented
frame-allocator-diagnostic: process-teardown pid=496 reclaimed_frames=11 before_free=183071 before_allocated=13136 after_free=183082 after_allocated=13125
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
brk01.c:21: TINFO: Testing syscall variant
brk01.c:70: TPASS: brk() works fine
frame-allocator-diagnostic: process-teardown pid=499 reclaimed_frames=12 before_free=183062 before_allocated=13145 after_free=183074 after_allocated=13133

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[252.722820 0:492 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=492 reclaimed_frames=262 before_free=183075 before_allocated=13132 after_free=183337 after_allocated=12870
frame-allocator-diagnostic: process-teardown pid=490 reclaimed_frames=12 before_free=183337 before_allocated=12870 after_free=183349 after_allocated=12858
FAIL LTP CASE brk01 : 32
RUN LTP CASE brk02
frame-allocator-diagnostic: process-teardown pid=502 reclaimed_frames=511 before_free=182806 before_allocated=13401 after_free=183317 after_allocated=12890
frame-allocator-diagnostic: process-teardown pid=494 reclaimed_frames=508 before_free=183317 before_allocated=12890 after_free=183825 after_allocated=12382
frame-allocator-diagnostic: process-teardown pid=504 reclaimed_frames=1 before_free=183293 before_allocated=12914 after_free=183294 after_allocated=12913
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
brk02.c:42: TINFO: Testing libc variant
brk02.c:53: TCONF: brk() not implemented
frame-allocator-diagnostic: process-teardown pid=507 reclaimed_frames=11 before_free=183013 before_allocated=13194 after_free=183024 after_allocated=13183
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
brk02.c:39: TINFO: Testing syscall variant
brk02.c:86: TPASS: munmap at least two VMAs of brk() passed
frame-allocator-diagnostic: process-teardown pid=510 reclaimed_frames=11 before_free=183005 before_allocated=13202 after_free=183016 after_allocated=13191

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[256.799414 0:503 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=503 reclaimed_frames=262 before_free=183017 before_allocated=13190 after_free=183279 after_allocated=12928
frame-allocator-diagnostic: process-teardown pid=501 reclaimed_frames=12 before_free=183279 before_allocated=12928 after_free=183291 after_allocated=12916
FAIL LTP CASE brk02 : 32
RUN LTP CASE broken_ip-checksum.sh
frame-allocator-diagnostic: process-teardown pid=513 reclaimed_frames=511 before_free=182748 before_allocated=13459 after_free=183259 after_allocated=12948
frame-allocator-diagnostic: process-teardown pid=505 reclaimed_frames=508 before_free=183259 before_allocated=12948 after_free=183767 after_allocated=12440
frame-allocator-diagnostic: process-teardown pid=515 reclaimed_frames=1 before_free=183235 before_allocated=12972 after_free=183236 after_allocated=12971
sh: tst_net.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=517 reclaimed_frames=6 before_free=182711 before_allocated=13496 after_free=182717 after_allocated=13490
frame-allocator-diagnostic: process-teardown pid=514 reclaimed_frames=512 before_free=182717 before_allocated=13490 after_free=183229 after_allocated=12978
frame-allocator-diagnostic: process-teardown pid=512 reclaimed_frames=12 before_free=183229 before_allocated=12978 after_free=183241 after_allocated=12966
FAIL LTP CASE broken_ip-checksum.sh : 2
RUN LTP CASE broken_ip-dstaddr.sh
frame-allocator-diagnostic: process-teardown pid=519 reclaimed_frames=511 before_free=182698 before_allocated=13509 after_free=183209 after_allocated=12998
frame-allocator-diagnostic: process-teardown pid=516 reclaimed_frames=508 before_free=182683 before_allocated=13524 after_free=183191 after_allocated=13016
frame-allocator-diagnostic: process-teardown pid=521 reclaimed_frames=1 before_free=183184 before_allocated=13023 after_free=183185 after_allocated=13022
sh: tst_net.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=523 reclaimed_frames=6 before_free=182660 before_allocated=13547 after_free=182666 after_allocated=13541
frame-allocator-diagnostic: process-teardown pid=520 reclaimed_frames=512 before_free=182666 before_allocated=13541 after_free=183178 after_allocated=13029
frame-allocator-diagnostic: process-teardown pid=518 reclaimed_frames=13 before_free=183178 before_allocated=13029 after_free=183191 after_allocated=13016
FAIL LTP CASE broken_ip-dstaddr.sh : 2
RUN LTP CASE broken_ip-fragment.sh
frame-allocator-diagnostic: process-teardown pid=525 reclaimed_frames=511 before_free=182647 before_allocated=13560 after_free=183158 after_allocated=13049
frame-allocator-diagnostic: process-teardown pid=522 reclaimed_frames=508 before_free=182633 before_allocated=13574 after_free=183141 after_allocated=13066
frame-allocator-diagnostic: process-teardown pid=527 reclaimed_frames=1 before_free=183134 before_allocated=13073 after_free=183135 after_allocated=13072
sh: tst_net.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=529 reclaimed_frames=6 before_free=182610 before_allocated=13597 after_free=182616 after_allocated=13591
frame-allocator-diagnostic: process-teardown pid=526 reclaimed_frames=512 before_free=182616 before_allocated=13591 after_free=183128 after_allocated=13079
frame-allocator-diagnostic: process-teardown pid=524 reclaimed_frames=13 before_free=183128 before_allocated=13079 after_free=183141 after_allocated=13066
FAIL LTP CASE broken_ip-fragment.sh : 2
RUN LTP CASE broken_ip-ihl.sh
frame-allocator-diagnostic: process-teardown pid=531 reclaimed_frames=511 before_free=182598 before_allocated=13609 after_free=183109 after_allocated=13098
frame-allocator-diagnostic: process-teardown pid=528 reclaimed_frames=508 before_free=182584 before_allocated=13623 after_free=183092 after_allocated=13115
frame-allocator-diagnostic: process-teardown pid=533 reclaimed_frames=1 before_free=183085 before_allocated=13122 after_free=183086 after_allocated=13121
sh: tst_net.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=535 reclaimed_frames=6 before_free=182561 before_allocated=13646 after_free=182567 after_allocated=13640
frame-allocator-diagnostic: process-teardown pid=532 reclaimed_frames=512 before_free=182567 before_allocated=13640 after_free=183079 after_allocated=13128
frame-allocator-diagnostic: process-teardown pid=530 reclaimed_frames=12 before_free=183079 before_allocated=13128 after_free=183091 after_allocated=13116
FAIL LTP CASE broken_ip-ihl.sh : 2
RUN LTP CASE broken_ip-nexthdr.sh
frame-allocator-diagnostic: process-teardown pid=537 reclaimed_frames=511 before_free=182548 before_allocated=13659 after_free=183059 after_allocated=13148
frame-allocator-diagnostic: process-teardown pid=534 reclaimed_frames=508 before_free=182533 before_allocated=13674 after_free=183041 after_allocated=13166
frame-allocator-diagnostic: process-teardown pid=539 reclaimed_frames=1 before_free=183034 before_allocated=13173 after_free=183035 after_allocated=13172
sh: tst_net.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=541 reclaimed_frames=6 before_free=182510 before_allocated=13697 after_free=182516 after_allocated=13691
frame-allocator-diagnostic: process-teardown pid=538 reclaimed_frames=512 before_free=182516 before_allocated=13691 after_free=183028 after_allocated=13179
frame-allocator-diagnostic: process-teardown pid=536 reclaimed_frames=13 before_free=183028 before_allocated=13179 after_free=183041 after_allocated=13166
FAIL LTP CASE broken_ip-nexthdr.sh : 2
RUN LTP CASE broken_ip-plen.sh
frame-allocator-diagnostic: process-teardown pid=543 reclaimed_frames=511 before_free=182498 before_allocated=13709 after_free=183009 after_allocated=13198
frame-allocator-diagnostic: process-teardown pid=540 reclaimed_frames=508 before_free=182483 before_allocated=13724 after_free=182991 after_allocated=13216
frame-allocator-diagnostic: process-teardown pid=545 reclaimed_frames=1 before_free=182984 before_allocated=13223 after_free=182985 after_allocated=13222
sh: tst_net.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=547 reclaimed_frames=6 before_free=182460 before_allocated=13747 after_free=182466 after_allocated=13741
frame-allocator-diagnostic: process-teardown pid=544 reclaimed_frames=512 before_free=182466 before_allocated=13741 after_free=182978 after_allocated=13229
frame-allocator-diagnostic: process-teardown pid=542 reclaimed_frames=13 before_free=182978 before_allocated=13229 after_free=182991 after_allocated=13216
FAIL LTP CASE broken_ip-plen.sh : 2
RUN LTP CASE broken_ip-protcol.sh
frame-allocator-diagnostic: process-teardown pid=549 reclaimed_frames=511 before_free=182448 before_allocated=13759 after_free=182959 after_allocated=13248
frame-allocator-diagnostic: process-teardown pid=546 reclaimed_frames=508 before_free=182434 before_allocated=13773 after_free=182942 after_allocated=13265
frame-allocator-diagnostic: process-teardown pid=551 reclaimed_frames=1 before_free=182935 before_allocated=13272 after_free=182936 after_allocated=13271
sh: tst_net.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=553 reclaimed_frames=6 before_free=182411 before_allocated=13796 after_free=182417 after_allocated=13790
frame-allocator-diagnostic: process-teardown pid=550 reclaimed_frames=512 before_free=182417 before_allocated=13790 after_free=182929 after_allocated=13278
frame-allocator-diagnostic: process-teardown pid=548 reclaimed_frames=12 before_free=182929 before_allocated=13278 after_free=182941 after_allocated=13266
FAIL LTP CASE broken_ip-protcol.sh : 2
RUN LTP CASE broken_ip-version.sh
frame-allocator-diagnostic: process-teardown pid=555 reclaimed_frames=511 before_free=182398 before_allocated=13809 after_free=182909 after_allocated=13298
frame-allocator-diagnostic: process-teardown pid=552 reclaimed_frames=508 before_free=182384 before_allocated=13823 after_free=182892 after_allocated=13315
frame-allocator-diagnostic: process-teardown pid=557 reclaimed_frames=1 before_free=182885 before_allocated=13322 after_free=182886 after_allocated=13321
sh: tst_net.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=559 reclaimed_frames=6 before_free=182361 before_allocated=13846 after_free=182367 after_allocated=13840
frame-allocator-diagnostic: process-teardown pid=556 reclaimed_frames=512 before_free=182367 before_allocated=13840 after_free=182879 after_allocated=13328
frame-allocator-diagnostic: process-teardown pid=554 reclaimed_frames=12 before_free=182879 before_allocated=13328 after_free=182891 after_allocated=13316
FAIL LTP CASE broken_ip-version.sh : 2
RUN LTP CASE busy_poll01.sh
frame-allocator-diagnostic: process-teardown pid=561 reclaimed_frames=511 before_free=182348 before_allocated=13859 after_free=182859 after_allocated=13348
frame-allocator-diagnostic: process-teardown pid=558 reclaimed_frames=508 before_free=182334 before_allocated=13873 after_free=182842 after_allocated=13365
frame-allocator-diagnostic: process-teardown pid=563 reclaimed_frames=1 before_free=182835 before_allocated=13372 after_free=182836 after_allocated=13371
sh: busy_poll_lib.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=565 reclaimed_frames=8 before_free=182307 before_allocated=13900 after_free=182315 after_allocated=13892
frame-allocator-diagnostic: process-teardown pid=562 reclaimed_frames=514 before_free=182315 before_allocated=13892 after_free=182829 after_allocated=13378
frame-allocator-diagnostic: process-teardown pid=560 reclaimed_frames=12 before_free=182829 before_allocated=13378 after_free=182841 after_allocated=13366
FAIL LTP CASE busy_poll01.sh : 2
RUN LTP CASE busy_poll02.sh
frame-allocator-diagnostic: process-teardown pid=567 reclaimed_frames=511 before_free=182298 before_allocated=13909 after_free=182809 after_allocated=13398
frame-allocator-diagnostic: process-teardown pid=564 reclaimed_frames=508 before_free=182793 before_allocated=13414 after_free=183301 after_allocated=12906
frame-allocator-diagnostic: process-teardown pid=569 reclaimed_frames=1 before_free=182785 before_allocated=13422 after_free=182786 after_allocated=13421
sh: busy_poll_lib.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=571 reclaimed_frames=7 before_free=182258 before_allocated=13949 after_free=182265 after_allocated=13942
frame-allocator-diagnostic: process-teardown pid=568 reclaimed_frames=514 before_free=182265 before_allocated=13942 after_free=182779 after_allocated=13428
frame-allocator-diagnostic: process-teardown pid=566 reclaimed_frames=12 before_free=182779 before_allocated=13428 after_free=182791 after_allocated=13416
FAIL LTP CASE busy_poll02.sh : 2
RUN LTP CASE busy_poll03.sh
frame-allocator-diagnostic: process-teardown pid=573 reclaimed_frames=511 before_free=182248 before_allocated=13959 after_free=182759 after_allocated=13448
frame-allocator-diagnostic: process-teardown pid=570 reclaimed_frames=508 before_free=182234 before_allocated=13973 after_free=182742 after_allocated=13465
frame-allocator-diagnostic: process-teardown pid=575 reclaimed_frames=1 before_free=182735 before_allocated=13472 after_free=182736 after_allocated=13471
sh: busy_poll_lib.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=577 reclaimed_frames=7 before_free=182208 before_allocated=13999 after_free=182215 after_allocated=13992
frame-allocator-diagnostic: process-teardown pid=574 reclaimed_frames=514 before_free=182215 before_allocated=13992 after_free=182729 after_allocated=13478
frame-allocator-diagnostic: process-teardown pid=572 reclaimed_frames=12 before_free=182729 before_allocated=13478 after_free=182741 after_allocated=13466
FAIL LTP CASE busy_poll03.sh : 2
RUN LTP CASE busy_poll_lib.sh
SKIP LTP CASE busy_poll_lib.sh : LTP shell helper library is not a standalone test
frame-allocator-diagnostic: process-teardown pid=578 reclaimed_frames=12 before_free=182719 before_allocated=13488 after_free=182731 after_allocated=13476
FAIL LTP CASE busy_poll_lib.sh : 32
RUN LTP CASE cacheflush01
frame-allocator-diagnostic: process-teardown pid=580 reclaimed_frames=511 before_free=182188 before_allocated=14019 after_free=182699 after_allocated=13508
frame-allocator-diagnostic: process-teardown pid=576 reclaimed_frames=508 before_free=182699 before_allocated=13508 after_free=183207 after_allocated=13000
frame-allocator-diagnostic: process-teardown pid=582 reclaimed_frames=1 before_free=182674 before_allocated=13533 after_free=182675 after_allocated=13532
tst_test.c:1175: TCONF: system doesn't support cacheflush()
frame-allocator-diagnostic: process-teardown pid=581 reclaimed_frames=261 before_free=182415 before_allocated=13792 after_free=182676 after_allocated=13531
frame-allocator-diagnostic: process-teardown pid=579 reclaimed_frames=13 before_free=182676 before_allocated=13531 after_free=182689 after_allocated=13518
FAIL LTP CASE cacheflush01 : 32
RUN LTP CASE can_bcm01
frame-allocator-diagnostic: process-teardown pid=585 reclaimed_frames=511 before_free=182145 before_allocated=14062 after_free=182656 after_allocated=13551
frame-allocator-diagnostic: process-teardown pid=583 reclaimed_frames=508 before_free=182656 before_allocated=13551 after_free=183164 after_allocated=13043
frame-allocator-diagnostic: process-teardown pid=587 reclaimed_frames=1 before_free=182632 before_allocated=13575 after_free=182633 after_allocated=13574
tst_kernel.c:90: TINFO: uname.machine=loongarch64 kernel is 64bit
tst_kernel.c:126: TWARN: expected file /lib/modules/6.0.0/modules.dep does not exist or not a file
tst_kernel.c:126: TWARN: expected file /lib/modules/6.0.0/modules.builtin does not exist or not a file
tst_test.c:1229: TCONF: vcan driver not available
frame-allocator-diagnostic: process-teardown pid=586 reclaimed_frames=272 before_free=182362 before_allocated=13845 after_free=182634 after_allocated=13573
frame-allocator-diagnostic: process-teardown pid=584 reclaimed_frames=13 before_free=182634 before_allocated=13573 after_free=182647 after_allocated=13560
FAIL LTP CASE can_bcm01 : 32
RUN LTP CASE can_filter
frame-allocator-diagnostic: process-teardown pid=590 reclaimed_frames=511 before_free=182105 before_allocated=14102 after_free=182616 after_allocated=13591
frame-allocator-diagnostic: process-teardown pid=588 reclaimed_frames=508 before_free=182616 before_allocated=13591 after_free=183124 after_allocated=13083
frame-allocator-diagnostic: process-teardown pid=592 reclaimed_frames=1 before_free=182590 before_allocated=13617 after_free=182591 after_allocated=13616
tst_kernel.c:126: TWARN: expected file /lib/modules/6.0.0/modules.dep does not exist or not a file
tst_kernel.c:126: TWARN: expected file /lib/modules/6.0.0/modules.builtin does not exist or not a file
tst_test.c:1229: TCONF: vcan driver not available
frame-allocator-diagnostic: process-teardown pid=591 reclaimed_frames=266 before_free=182326 before_allocated=13881 after_free=182592 after_allocated=13615
frame-allocator-diagnostic: process-teardown pid=589 reclaimed_frames=13 before_free=182592 before_allocated=13615 after_free=182605 after_allocated=13602
FAIL LTP CASE can_filter : 32
RUN LTP CASE can_rcv_own_msgs
frame-allocator-diagnostic: process-teardown pid=595 reclaimed_frames=511 before_free=182061 before_allocated=14146 after_free=182572 after_allocated=13635
frame-allocator-diagnostic: process-teardown pid=593 reclaimed_frames=508 before_free=182572 before_allocated=13635 after_free=183080 after_allocated=13127
frame-allocator-diagnostic: process-teardown pid=597 reclaimed_frames=1 before_free=182548 before_allocated=13659 after_free=182549 after_allocated=13658
tst_kernel.c:126: TWARN: expected file /lib/modules/6.0.0/modules.dep does not exist or not a file
tst_kernel.c:126: TWARN: expected file /lib/modules/6.0.0/modules.builtin does not exist or not a file
tst_test.c:1229: TCONF: vcan driver not available
frame-allocator-diagnostic: process-teardown pid=596 reclaimed_frames=266 before_free=182284 before_allocated=13923 after_free=182550 after_allocated=13657
frame-allocator-diagnostic: process-teardown pid=594 reclaimed_frames=13 before_free=182550 before_allocated=13657 after_free=182563 after_allocated=13644
FAIL LTP CASE can_rcv_own_msgs : 32
RUN LTP CASE cap_bounds_r
frame-allocator-diagnostic: process-teardown pid=600 reclaimed_frames=511 before_free=182020 before_allocated=14187 after_free=182531 after_allocated=13676
frame-allocator-diagnostic: process-teardown pid=598 reclaimed_frames=508 before_free=182531 before_allocated=13676 after_free=183039 after_allocated=13168
frame-allocator-diagnostic: process-teardown pid=602 reclaimed_frames=1 before_free=182507 before_allocated=13700 after_free=182508 after_allocated=13699
cap_bounds_r    1  TCONF  :  cap_bounds_r.c:103: System doesn't have POSIX capabilities.
frame-allocator-diagnostic: process-teardown pid=601 reclaimed_frames=263 before_free=182246 before_allocated=13961 after_free=182509 after_allocated=13698
frame-allocator-diagnostic: process-teardown pid=599 reclaimed_frames=12 before_free=182509 before_allocated=13698 after_free=182521 after_allocated=13686
FAIL LTP CASE cap_bounds_r : 32
RUN LTP CASE cap_bounds_rw
frame-allocator-diagnostic: process-teardown pid=605 reclaimed_frames=511 before_free=181978 before_allocated=14229 after_free=182489 after_allocated=13718
frame-allocator-diagnostic: process-teardown pid=603 reclaimed_frames=508 before_free=182489 before_allocated=13718 after_free=182997 after_allocated=13210
frame-allocator-diagnostic: process-teardown pid=607 reclaimed_frames=1 before_free=182465 before_allocated=13742 after_free=182466 after_allocated=13741
cap_bounds_rw    1  TCONF  :  cap_bounds_rw.c:161: System doesn't have POSIX capabilities.
frame-allocator-diagnostic: process-teardown pid=606 reclaimed_frames=263 before_free=182204 before_allocated=14003 after_free=182467 after_allocated=13740
frame-allocator-diagnostic: process-teardown pid=604 reclaimed_frames=12 before_free=182467 before_allocated=13740 after_free=182479 after_allocated=13728
FAIL LTP CASE cap_bounds_rw : 32
RUN LTP CASE cap_bset_inh_bounds
frame-allocator-diagnostic: process-teardown pid=610 reclaimed_frames=511 before_free=181936 before_allocated=14271 after_free=182447 after_allocated=13760
frame-allocator-diagnostic: process-teardown pid=608 reclaimed_frames=508 before_free=182447 before_allocated=13760 after_free=182955 after_allocated=13252
frame-allocator-diagnostic: process-teardown pid=612 reclaimed_frames=1 before_free=182423 before_allocated=13784 after_free=182424 after_allocated=13783
cap_bounds_r    1  TCONF  :  cap_bset_inh_bounds.c:132: System doesn't have sys/capability.h.
frame-allocator-diagnostic: process-teardown pid=611 reclaimed_frames=263 before_free=182162 before_allocated=14045 after_free=182425 after_allocated=13782
frame-allocator-diagnostic: process-teardown pid=609 reclaimed_frames=12 before_free=182425 before_allocated=13782 after_free=182437 after_allocated=13770
FAIL LTP CASE cap_bset_inh_bounds : 32
RUN LTP CASE capget01
frame-allocator-diagnostic: process-teardown pid=615 reclaimed_frames=511 before_free=181894 before_allocated=14313 after_free=182405 after_allocated=13802
frame-allocator-diagnostic: process-teardown pid=613 reclaimed_frames=508 before_free=182405 before_allocated=13802 after_free=182913 after_allocated=13294
frame-allocator-diagnostic: process-teardown pid=617 reclaimed_frames=1 before_free=182380 before_allocated=13827 after_free=182381 after_allocated=13826
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_capability.c:17: TCONF: syscall(90) __NR_capget not supported on your arch
frame-allocator-diagnostic: process-teardown pid=620 reclaimed_frames=11 before_free=182098 before_allocated=14109 after_free=182109 after_allocated=14098

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[305.912357 0:616 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=616 reclaimed_frames=264 before_free=182110 before_allocated=14097 after_free=182374 after_allocated=13833
frame-allocator-diagnostic: process-teardown pid=614 reclaimed_frames=13 before_free=182374 before_allocated=13833 after_free=182387 after_allocated=13820
FAIL LTP CASE capget01 : 32
RUN LTP CASE capget02
frame-allocator-diagnostic: process-teardown pid=623 reclaimed_frames=511 before_free=181843 before_allocated=14364 after_free=182354 after_allocated=13853
frame-allocator-diagnostic: process-teardown pid=618 reclaimed_frames=508 before_free=182354 before_allocated=13853 after_free=182862 after_allocated=13345
frame-allocator-diagnostic: process-teardown pid=625 reclaimed_frames=1 before_free=182330 before_allocated=13877 after_free=182331 after_allocated=13876
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
capget02.c:57: TCONF: syscall(90) __NR_capget not supported on your arch
frame-allocator-diagnostic: process-teardown pid=628 reclaimed_frames=12 before_free=182046 before_allocated=14161 after_free=182058 after_allocated=14149

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[310.047484 0:624 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=624 reclaimed_frames=265 before_free=182059 before_allocated=14148 after_free=182324 after_allocated=13883
frame-allocator-diagnostic: process-teardown pid=622 reclaimed_frames=13 before_free=182324 before_allocated=13883 after_free=182337 after_allocated=13870
FAIL LTP CASE capget02 : 32
RUN LTP CASE capset01
frame-allocator-diagnostic: process-teardown pid=631 reclaimed_frames=511 before_free=181793 before_allocated=14414 after_free=182304 after_allocated=13903
frame-allocator-diagnostic: process-teardown pid=626 reclaimed_frames=508 before_free=182304 before_allocated=13903 after_free=182812 after_allocated=13395
frame-allocator-diagnostic: process-teardown pid=633 reclaimed_frames=1 before_free=182280 before_allocated=13927 after_free=182281 after_allocated=13926
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
capset01.c:43: TCONF: syscall(90) __NR_capget not supported on your arch
frame-allocator-diagnostic: process-teardown pid=636 reclaimed_frames=11 before_free=181998 before_allocated=14209 after_free=182009 after_allocated=14198

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[314.171170 0:632 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=632 reclaimed_frames=264 before_free=182010 before_allocated=14197 after_free=182274 after_allocated=13933
frame-allocator-diagnostic: process-teardown pid=630 reclaimed_frames=13 before_free=182274 before_allocated=13933 after_free=182287 after_allocated=13920
FAIL LTP CASE capset01 : 32
RUN LTP CASE capset02
frame-allocator-diagnostic: process-teardown pid=639 reclaimed_frames=511 before_free=181744 before_allocated=14463 after_free=182255 after_allocated=13952
frame-allocator-diagnostic: process-teardown pid=634 reclaimed_frames=508 before_free=182255 before_allocated=13952 after_free=182763 after_allocated=13444
frame-allocator-diagnostic: process-teardown pid=641 reclaimed_frames=1 before_free=182231 before_allocated=13976 after_free=182232 after_allocated=13975
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
capset02.c:91: TCONF: syscall(91) __NR_capset not supported on your arch
frame-allocator-diagnostic: process-teardown pid=644 reclaimed_frames=11 before_free=181949 before_allocated=14258 after_free=181960 after_allocated=14247

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[318.446503 0:640 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=640 reclaimed_frames=264 before_free=181961 before_allocated=14246 after_free=182225 after_allocated=13982
frame-allocator-diagnostic: process-teardown pid=638 reclaimed_frames=12 before_free=182225 before_allocated=13982 after_free=182237 after_allocated=13970
FAIL LTP CASE capset02 : 32
RUN LTP CASE capset03
frame-allocator-diagnostic: process-teardown pid=647 reclaimed_frames=511 before_free=181694 before_allocated=14513 after_free=182205 after_allocated=14002
frame-allocator-diagnostic: process-teardown pid=642 reclaimed_frames=508 before_free=182205 before_allocated=14002 after_free=182713 after_allocated=13494
frame-allocator-diagnostic: process-teardown pid=649 reclaimed_frames=1 before_free=182181 before_allocated=14026 after_free=182182 after_allocated=14025
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
capset03.c:43: TCONF: syscall(91) __NR_capset not supported on your arch
frame-allocator-diagnostic: process-teardown pid=652 reclaimed_frames=11 before_free=181899 before_allocated=14308 after_free=181910 after_allocated=14297

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[322.526516 0:648 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=648 reclaimed_frames=264 before_free=181911 before_allocated=14296 after_free=182175 after_allocated=14032
frame-allocator-diagnostic: process-teardown pid=646 reclaimed_frames=12 before_free=182175 before_allocated=14032 after_free=182187 after_allocated=14020
FAIL LTP CASE capset03 : 32
RUN LTP CASE capset04
frame-allocator-diagnostic: process-teardown pid=655 reclaimed_frames=511 before_free=181644 before_allocated=14563 after_free=182155 after_allocated=14052
frame-allocator-diagnostic: process-teardown pid=650 reclaimed_frames=508 before_free=182155 before_allocated=14052 after_free=182663 after_allocated=13544
frame-allocator-diagnostic: process-teardown pid=657 reclaimed_frames=1 before_free=182131 before_allocated=14076 after_free=182132 after_allocated=14075
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
capset04.c:46: TCONF: syscall(90) __NR_capget not supported on your arch
frame-allocator-diagnostic: process-teardown pid=660 reclaimed_frames=12 before_free=181848 before_allocated=14359 after_free=181860 after_allocated=14347

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[326.607449 0:656 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=656 reclaimed_frames=264 before_free=181861 before_allocated=14346 after_free=182125 after_allocated=14082
frame-allocator-diagnostic: process-teardown pid=654 reclaimed_frames=12 before_free=182125 before_allocated=14082 after_free=182137 after_allocated=14070
FAIL LTP CASE capset04 : 32
RUN LTP CASE cfs_bandwidth01
frame-allocator-diagnostic: process-teardown pid=663 reclaimed_frames=511 before_free=181594 before_allocated=14613 after_free=182105 after_allocated=14102
frame-allocator-diagnostic: process-teardown pid=658 reclaimed_frames=508 before_free=182105 before_allocated=14102 after_free=182613 after_allocated=13594
frame-allocator-diagnostic: process-teardown pid=665 reclaimed_frames=1 before_free=182081 before_allocated=14126 after_free=182082 after_allocated=14125
tst_kconfig.c:71: TINFO: Couldn't locate kernel config!
tst_kconfig.c:207: TBROK: Cannot parse kernel .config
frame-allocator-diagnostic: process-teardown pid=664 reclaimed_frames=262 before_free=181821 before_allocated=14386 after_free=182083 after_allocated=14124
frame-allocator-diagnostic: process-teardown pid=662 reclaimed_frames=12 before_free=182083 before_allocated=14124 after_free=182095 after_allocated=14112
FAIL LTP CASE cfs_bandwidth01 : 2
RUN LTP CASE cgroup_core01
SKIP LTP CASE cgroup_core01 : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=667 reclaimed_frames=12 before_free=182073 before_allocated=14134 after_free=182085 after_allocated=14122
FAIL LTP CASE cgroup_core01 : 32
RUN LTP CASE cgroup_core02
SKIP LTP CASE cgroup_core02 : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=668 reclaimed_frames=13 before_free=182062 before_allocated=14145 after_free=182075 after_allocated=14132
FAIL LTP CASE cgroup_core02 : 32
RUN LTP CASE cgroup_core03
SKIP LTP CASE cgroup_core03 : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=669 reclaimed_frames=12 before_free=182053 before_allocated=14154 after_free=182065 after_allocated=14142
FAIL LTP CASE cgroup_core03 : 32
RUN LTP CASE cgroup_fj_common.sh
SKIP LTP CASE cgroup_fj_common.sh : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=670 reclaimed_frames=12 before_free=182043 before_allocated=14164 after_free=182055 after_allocated=14152
FAIL LTP CASE cgroup_fj_common.sh : 32
RUN LTP CASE cgroup_fj_function.sh
SKIP LTP CASE cgroup_fj_function.sh : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=671 reclaimed_frames=13 before_free=182032 before_allocated=14175 after_free=182045 after_allocated=14162
FAIL LTP CASE cgroup_fj_function.sh : 32
RUN LTP CASE cgroup_fj_proc
SKIP LTP CASE cgroup_fj_proc : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=672 reclaimed_frames=12 before_free=182023 before_allocated=14184 after_free=182035 after_allocated=14172
FAIL LTP CASE cgroup_fj_proc : 32
RUN LTP CASE cgroup_fj_stress.sh
SKIP LTP CASE cgroup_fj_stress.sh : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=673 reclaimed_frames=12 before_free=182013 before_allocated=14194 after_free=182025 after_allocated=14182
FAIL LTP CASE cgroup_fj_stress.sh : 32
RUN LTP CASE cgroup_lib.sh
SKIP LTP CASE cgroup_lib.sh : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=674 reclaimed_frames=13 before_free=182002 before_allocated=14205 after_free=182015 after_allocated=14192
FAIL LTP CASE cgroup_lib.sh : 32
RUN LTP CASE cgroup_regression_3_1.sh
SKIP LTP CASE cgroup_regression_3_1.sh : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=675 reclaimed_frames=13 before_free=181992 before_allocated=14215 after_free=182005 after_allocated=14202
FAIL LTP CASE cgroup_regression_3_1.sh : 32
RUN LTP CASE cgroup_regression_3_2.sh
SKIP LTP CASE cgroup_regression_3_2.sh : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=676 reclaimed_frames=12 before_free=181983 before_allocated=14224 after_free=181995 after_allocated=14212
FAIL LTP CASE cgroup_regression_3_2.sh : 32
RUN LTP CASE cgroup_regression_5_1.sh
SKIP LTP CASE cgroup_regression_5_1.sh : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=677 reclaimed_frames=13 before_free=181972 before_allocated=14235 after_free=181985 after_allocated=14222
FAIL LTP CASE cgroup_regression_5_1.sh : 32
RUN LTP CASE cgroup_regression_5_2.sh
SKIP LTP CASE cgroup_regression_5_2.sh : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=678 reclaimed_frames=12 before_free=181963 before_allocated=14244 after_free=181975 after_allocated=14232
FAIL LTP CASE cgroup_regression_5_2.sh : 32
RUN LTP CASE cgroup_regression_6_1.sh
SKIP LTP CASE cgroup_regression_6_1.sh : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=679 reclaimed_frames=12 before_free=181953 before_allocated=14254 after_free=181965 after_allocated=14242
FAIL LTP CASE cgroup_regression_6_1.sh : 32
RUN LTP CASE cgroup_regression_6_2.sh
SKIP LTP CASE cgroup_regression_6_2.sh : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=680 reclaimed_frames=12 before_free=181943 before_allocated=14264 after_free=181955 after_allocated=14252
FAIL LTP CASE cgroup_regression_6_2.sh : 32
RUN LTP CASE cgroup_regression_fork_processes
SKIP LTP CASE cgroup_regression_fork_processes : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=681 reclaimed_frames=12 before_free=181933 before_allocated=14274 after_free=181945 after_allocated=14262
FAIL LTP CASE cgroup_regression_fork_processes : 32
RUN LTP CASE cgroup_regression_getdelays
SKIP LTP CASE cgroup_regression_getdelays : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=682 reclaimed_frames=12 before_free=181923 before_allocated=14284 after_free=181935 after_allocated=14272
FAIL LTP CASE cgroup_regression_getdelays : 32
RUN LTP CASE cgroup_regression_test.sh
SKIP LTP CASE cgroup_regression_test.sh : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=683 reclaimed_frames=13 before_free=181912 before_allocated=14295 after_free=181925 after_allocated=14282
FAIL LTP CASE cgroup_regression_test.sh : 32
RUN LTP CASE cgroup_xattr
SKIP LTP CASE cgroup_xattr : cgroup unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=684 reclaimed_frames=12 before_free=181903 before_allocated=14304 after_free=181915 after_allocated=14292
FAIL LTP CASE cgroup_xattr : 32
RUN LTP CASE change_password.sh
frame-allocator-diagnostic: process-teardown pid=666 reclaimed_frames=508 before_free=181900 before_allocated=14307 after_free=182408 after_allocated=13799
frame-allocator-diagnostic: process-teardown pid=686 reclaimed_frames=511 before_free=181880 before_allocated=14327 after_free=182391 after_allocated=13816
frame-allocator-diagnostic: process-teardown pid=688 reclaimed_frames=1 before_free=181859 before_allocated=14348 after_free=181860 after_allocated=14347
timeout: can't execute 'ltp/testcases/bin/change_password.sh': Exec format error
frame-allocator-diagnostic: process-teardown pid=687 reclaimed_frames=4 before_free=181857 before_allocated=14350 after_free=181861 after_allocated=14346
frame-allocator-diagnostic: process-teardown pid=685 reclaimed_frames=12 before_free=181861 before_allocated=14346 after_free=181873 after_allocated=14334
FAIL LTP CASE change_password.sh : 126
RUN LTP CASE chdir01
frame-allocator-diagnostic: process-teardown pid=691 reclaimed_frames=511 before_free=181329 before_allocated=14878 after_free=181840 after_allocated=14367
frame-allocator-diagnostic: process-teardown pid=689 reclaimed_frames=508 before_free=181840 before_allocated=14367 after_free=182348 after_allocated=13859
frame-allocator-diagnostic: process-teardown pid=693 reclaimed_frames=1 before_free=181816 before_allocated=14391 after_free=181817 after_allocated=14390
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_device.c:293: TWARN: Failed to create test_dev.img: ENOSPC (28)
tst_device.c:354: TBROK: Failed to acquire device

Summary:
passed   0
failed   0
broken   1
skipped  0
warnings 1
[338.062637 0:692 axfs::root:433] [AxError::IsADirectory]
[338.063805 0:692 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=692 reclaimed_frames=271 before_free=58667 before_allocated=137540 after_free=58938 after_allocated=137269
frame-allocator-diagnostic: process-teardown pid=694 reclaimed_frames=508 before_free=58938 before_allocated=137269 after_free=59446 after_allocated=136761
frame-allocator-diagnostic: process-teardown pid=690 reclaimed_frames=13 before_free=59446 before_allocated=136761 after_free=59459 after_allocated=136748
FAIL LTP CASE chdir01 : 6
RUN LTP CASE chdir04
frame-allocator-diagnostic: process-teardown pid=696 reclaimed_frames=511 before_free=58915 before_allocated=137292 after_free=59426 after_allocated=136781
frame-allocator-diagnostic: process-teardown pid=698 reclaimed_frames=1 before_free=58894 before_allocated=137313 after_free=58895 after_allocated=137312
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
chdir04.c:29: TFAIL: chdir() expected ENAMETOOLONG: ENOENT (2)
chdir04.c:29: TPASS: chdir() : ENOENT (2)
read-cstr-efault: pid=701 tid=701 ptr=0x10000be000 fault_addr=0x10000be000 pc=0x100006d9f0 reason="range is not readable" aspace=AddrSpaceQuery { contains: true, area_found: true, area_start: 68720254976, area_end: 68720259072, area_flags: USER, backend: "alloc-lazy", pte_mapped: false, paddr: 0, pte_flags: 0x0, page_size: None, shared_metadata: false } created_by_fork=true credential_generation=0 uid=0 gid=0
chdir04.c:29: TPASS: chdir() : EFAULT (14)
frame-allocator-diagnostic: process-teardown pid=701 reclaimed_frames=11 before_free=58614 before_allocated=137593 after_free=58625 after_allocated=137582

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[342.206980 0:697 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=697 reclaimed_frames=262 before_free=58626 before_allocated=137581 after_free=58888 after_allocated=137319
frame-allocator-diagnostic: process-teardown pid=695 reclaimed_frames=13 before_free=58888 before_allocated=137319 after_free=58901 after_allocated=137306
FAIL LTP CASE chdir04 : 0
RUN LTP CASE check_envval
frame-allocator-diagnostic: process-teardown pid=704 reclaimed_frames=512 before_free=58357 before_allocated=137850 after_free=58869 after_allocated=137338
frame-allocator-diagnostic: process-teardown pid=699 reclaimed_frames=508 before_free=58869 before_allocated=137338 after_free=59377 after_allocated=136830
SKIP LTP CASE check_envval : requires LTP network environment
frame-allocator-diagnostic: process-teardown pid=703 reclaimed_frames=12 before_free=59377 before_allocated=136830 after_free=59389 after_allocated=136818
FAIL LTP CASE check_envval : 32
RUN LTP CASE check_icmpv4_connectivity
frame-allocator-diagnostic: process-teardown pid=706 reclaimed_frames=511 before_free=58846 before_allocated=137361 after_free=59357 after_allocated=136850
frame-allocator-diagnostic: process-teardown pid=708 reclaimed_frames=1 before_free=58825 before_allocated=137382 after_free=58826 after_allocated=137381
Usage: /musl/ltp/testcases/bin/check_icmpv4_connectivity source_interface_name destionation_ipv4_address
frame-allocator-diagnostic: process-teardown pid=707 reclaimed_frames=514 before_free=58313 before_allocated=137894 after_free=58827 after_allocated=137380
frame-allocator-diagnostic: process-teardown pid=705 reclaimed_frames=12 before_free=58827 before_allocated=137380 after_free=58839 after_allocated=137368
FAIL LTP CASE check_icmpv4_connectivity : 1
RUN LTP CASE check_icmpv6_connectivity
frame-allocator-diagnostic: process-teardown pid=711 reclaimed_frames=511 before_free=58296 before_allocated=137911 after_free=58807 after_allocated=137400
frame-allocator-diagnostic: process-teardown pid=709 reclaimed_frames=508 before_free=58282 before_allocated=137925 after_free=58790 after_allocated=137417
frame-allocator-diagnostic: process-teardown pid=713 reclaimed_frames=1 before_free=58783 before_allocated=137424 after_free=58784 after_allocated=137423
Usage: /musl/ltp/testcases/bin/check_icmpv6_connectivity source_interface_name destionation_ipv6_address
frame-allocator-diagnostic: process-teardown pid=712 reclaimed_frames=514 before_free=58271 before_allocated=137936 after_free=58785 after_allocated=137422
frame-allocator-diagnostic: process-teardown pid=710 reclaimed_frames=12 before_free=58785 before_allocated=137422 after_free=58797 after_allocated=137410
FAIL LTP CASE check_icmpv6_connectivity : 1
RUN LTP CASE check_keepcaps
frame-allocator-diagnostic: process-teardown pid=716 reclaimed_frames=511 before_free=58254 before_allocated=137953 after_free=58765 after_allocated=137442
frame-allocator-diagnostic: process-teardown pid=714 reclaimed_frames=508 before_free=58765 before_allocated=137442 after_free=59273 after_allocated=136934
frame-allocator-diagnostic: process-teardown pid=718 reclaimed_frames=1 before_free=58741 before_allocated=137466 after_free=58742 after_allocated=137465
keepcaps    1  TCONF  :  check_keepcaps.c:152: linux/securebits.h or libcap does not exist.
keepcaps    2  TCONF  :  check_keepcaps.c:152: Remaining cases not appropriate for configuration
frame-allocator-diagnostic: process-teardown pid=717 reclaimed_frames=263 before_free=58480 before_allocated=137727 after_free=58743 after_allocated=137464
frame-allocator-diagnostic: process-teardown pid=715 reclaimed_frames=12 before_free=58743 before_allocated=137464 after_free=58755 after_allocated=137452
FAIL LTP CASE check_keepcaps : 32
RUN LTP CASE check_netem
frame-allocator-diagnostic: process-teardown pid=721 reclaimed_frames=512 before_free=58211 before_allocated=137996 after_free=58723 after_allocated=137484
SKIP LTP CASE check_netem : requires LTP network environment
frame-allocator-diagnostic: process-teardown pid=720 reclaimed_frames=12 before_free=58723 before_allocated=137484 after_free=58735 after_allocated=137472
FAIL LTP CASE check_netem : 32
RUN LTP CASE check_pe
frame-allocator-diagnostic: process-teardown pid=719 reclaimed_frames=508 before_free=58720 before_allocated=137487 after_free=59228 after_allocated=136979
frame-allocator-diagnostic: process-teardown pid=723 reclaimed_frames=511 before_free=58700 before_allocated=137507 after_free=59211 after_allocated=136996
frame-allocator-diagnostic: process-teardown pid=725 reclaimed_frames=1 before_free=58678 before_allocated=137529 after_free=58679 after_allocated=137528
check_pe    1  TCONF  :  check_pe.c:83: System doesn't have sys/capability.h
frame-allocator-diagnostic: process-teardown pid=724 reclaimed_frames=263 before_free=58417 before_allocated=137790 after_free=58680 after_allocated=137527
frame-allocator-diagnostic: process-teardown pid=722 reclaimed_frames=13 before_free=58680 before_allocated=137527 after_free=58693 after_allocated=137514
FAIL LTP CASE check_pe : 32
RUN LTP CASE check_setkey
frame-allocator-diagnostic: process-teardown pid=728 reclaimed_frames=512 before_free=58148 before_allocated=138059 after_free=58660 after_allocated=137547
SKIP LTP CASE check_setkey : requires LTP network environment
frame-allocator-diagnostic: process-teardown pid=727 reclaimed_frames=13 before_free=58660 before_allocated=137547 after_free=58673 after_allocated=137534
FAIL LTP CASE check_setkey : 32
RUN LTP CASE check_simple_capset
frame-allocator-diagnostic: process-teardown pid=726 reclaimed_frames=508 before_free=58634 before_allocated=137573 after_free=59142 after_allocated=137065
frame-allocator-diagnostic: process-teardown pid=730 reclaimed_frames=511 before_free=58638 before_allocated=137569 after_free=59149 after_allocated=137058
frame-allocator-diagnostic: process-teardown pid=732 reclaimed_frames=1 before_free=58617 before_allocated=137590 after_free=58618 after_allocated=137589
System doesn't support full POSIX capabilities.
frame-allocator-diagnostic: process-teardown pid=731 reclaimed_frames=184 before_free=58435 before_allocated=137772 after_free=58619 after_allocated=137588
frame-allocator-diagnostic: process-teardown pid=729 reclaimed_frames=12 before_free=58619 before_allocated=137588 after_free=58631 after_allocated=137576
FAIL LTP CASE check_simple_capset : 1
RUN LTP CASE chmod01
frame-allocator-diagnostic: process-teardown pid=735 reclaimed_frames=511 before_free=58087 before_allocated=138120 after_free=58598 after_allocated=137609
frame-allocator-diagnostic: process-teardown pid=733 reclaimed_frames=508 before_free=58598 before_allocated=137609 after_free=59106 after_allocated=137101
frame-allocator-diagnostic: process-teardown pid=737 reclaimed_frames=1 before_free=58574 before_allocated=137633 after_free=58575 after_allocated=137632
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
chmod01.c:60: TINFO: Testing variant: verify permissions of file
chmod01.c:40: TPASS: chmod(testfile, 0000) passed
chmod01.c:50: TPASS: stat(testfile) mode=0000
chmod01.c:40: TPASS: chmod(testfile, 0007) passed
chmod01.c:50: TPASS: stat(testfile) mode=0007
chmod01.c:40: TPASS: chmod(testfile, 0070) passed
chmod01.c:50: TPASS: stat(testfile) mode=0070
chmod01.c:40: TPASS: chmod(testfile, 0700) passed
chmod01.c:50: TPASS: stat(testfile) mode=0700
chmod01.c:40: TPASS: chmod(testfile, 0777) passed
chmod01.c:50: TPASS: stat(testfile) mode=0777
chmod01.c:40: TPASS: chmod(testfile, 2777) passed
chmod01.c:50: TPASS: stat(testfile) mode=2777
chmod01.c:40: TPASS: chmod(testfile, 4777) passed
chmod01.c:50: TPASS: stat(testfile) mode=4777
chmod01.c:40: TPASS: chmod(testfile, 6777) passed
chmod01.c:50: TPASS: stat(testfile) mode=6777
frame-allocator-diagnostic: process-teardown pid=740 reclaimed_frames=12 before_free=58291 before_allocated=137916 after_free=58303 after_allocated=137904
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
chmod01.c:60: TINFO: Testing variant: verify permissions of directory
chmod01.c:40: TPASS: chmod(testdir_1, 0000) passed
chmod01.c:50: TPASS: stat(testdir_1) mode=0000
chmod01.c:40: TPASS: chmod(testdir_1, 0007) passed
chmod01.c:50: TPASS: stat(testdir_1) mode=0007
chmod01.c:40: TPASS: chmod(testdir_1, 0070) passed
chmod01.c:50: TPASS: stat(testdir_1) mode=0070
chmod01.c:40: TPASS: chmod(testdir_1, 0700) passed
chmod01.c:50: TPASS: stat(testdir_1) mode=0700
chmod01.c:40: TPASS: chmod(testdir_1, 0777) passed
chmod01.c:50: TPASS: stat(testdir_1) mode=0777
chmod01.c:40: TPASS: chmod(testdir_1, 2777) passed
chmod01.c:50: TPASS: stat(testdir_1) mode=2777
chmod01.c:40: TPASS: chmod(testdir_1, 4777) passed
chmod01.c:50: TPASS: stat(testdir_1) mode=4777
chmod01.c:40: TPASS: chmod(testdir_1, 6777) passed
chmod01.c:50: TPASS: stat(testdir_1) mode=6777
frame-allocator-diagnostic: process-teardown pid=743 reclaimed_frames=12 before_free=58283 before_allocated=137924 after_free=58295 after_allocated=137912

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[360.615941 0:736 axfs::root:433] [AxError::IsADirectory]
[360.616497 0:736 axfs::fops:297] [AxError::NotADirectory]
[360.617077 0:736 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=736 reclaimed_frames=265 before_free=58295 before_allocated=137912 after_free=58560 after_allocated=137647
frame-allocator-diagnostic: process-teardown pid=734 reclaimed_frames=13 before_free=58560 before_allocated=137647 after_free=58573 after_allocated=137634
FAIL LTP CASE chmod01 : 0
RUN LTP CASE chmod03
frame-allocator-diagnostic: process-teardown pid=746 reclaimed_frames=511 before_free=58030 before_allocated=138177 after_free=58541 after_allocated=137666
frame-allocator-diagnostic: process-teardown pid=738 reclaimed_frames=508 before_free=58541 before_allocated=137666 after_free=59049 after_allocated=137158
frame-allocator-diagnostic: process-teardown pid=748 reclaimed_frames=1 before_free=58517 before_allocated=137690 after_free=58518 after_allocated=137689
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
chmod03.c:42: TPASS: chmod(testfile, 1777) passed
chmod03.c:54: TPASS: stat(testfile) mode=101777
chmod03.c:42: TPASS: chmod(testdir_3, 1777) passed
chmod03.c:54: TPASS: stat(testdir_3) mode=41777
frame-allocator-diagnostic: process-teardown pid=751 reclaimed_frames=13 before_free=58235 before_allocated=137972 after_free=58248 after_allocated=137959

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[365.561292 0:747 axfs::root:433] [AxError::IsADirectory]
[365.561856 0:747 axfs::fops:297] [AxError::NotADirectory]
[365.562428 0:747 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=747 reclaimed_frames=263 before_free=58248 before_allocated=137959 after_free=58511 after_allocated=137696
frame-allocator-diagnostic: process-teardown pid=745 reclaimed_frames=12 before_free=58511 before_allocated=137696 after_free=58523 after_allocated=137684
FAIL LTP CASE chmod03 : 0
RUN LTP CASE chmod05
frame-allocator-diagnostic: process-teardown pid=754 reclaimed_frames=511 before_free=57980 before_allocated=138227 after_free=58491 after_allocated=137716
frame-allocator-diagnostic: process-teardown pid=749 reclaimed_frames=508 before_free=58491 before_allocated=137716 after_free=58999 after_allocated=137208
frame-allocator-diagnostic: process-teardown pid=756 reclaimed_frames=1 before_free=58467 before_allocated=137740 after_free=58468 after_allocated=137739
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
chmod05.c:76: TBROK: Group ID lookup failed: ENOTSOCK (88)
frame-allocator-diagnostic: process-teardown pid=759 reclaimed_frames=13 before_free=58185 before_allocated=138022 after_free=58198 after_allocated=138009

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[369.900411 0:755 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=755 reclaimed_frames=262 before_free=58199 before_allocated=138008 after_free=58461 after_allocated=137746
frame-allocator-diagnostic: process-teardown pid=753 reclaimed_frames=12 before_free=58461 before_allocated=137746 after_free=58473 after_allocated=137734
FAIL LTP CASE chmod05 : 2
RUN LTP CASE chmod06
frame-allocator-diagnostic: process-teardown pid=762 reclaimed_frames=511 before_free=57930 before_allocated=138277 after_free=58441 after_allocated=137766
frame-allocator-diagnostic: process-teardown pid=757 reclaimed_frames=508 before_free=58441 before_allocated=137766 after_free=58949 after_allocated=137258
frame-allocator-diagnostic: process-teardown pid=764 reclaimed_frames=1 before_free=58417 before_allocated=137790 after_free=58418 after_allocated=137789
[373.929498 0:763 axfs::root:423] [AxError::AlreadyExists]
tst_test.c:1011: TBROK: mkdir(mntpoint/dir/, 0777) failed: EEXIST (17)

Summary:
passed   0
failed   0
broken   1
skipped  0
warnings 0
[373.934862 0:763 axfs::root:433] [AxError::IsADirectory]
[373.935544 0:763 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=763 reclaimed_frames=265 before_free=58154 before_allocated=138053 after_free=58419 after_allocated=137788
frame-allocator-diagnostic: process-teardown pid=761 reclaimed_frames=12 before_free=58419 before_allocated=137788 after_free=58431 after_allocated=137776
FAIL LTP CASE chmod06 : 2
RUN LTP CASE chmod07
frame-allocator-diagnostic: process-teardown pid=767 reclaimed_frames=511 before_free=57888 before_allocated=138319 after_free=58399 after_allocated=137808
frame-allocator-diagnostic: process-teardown pid=765 reclaimed_frames=508 before_free=58399 before_allocated=137808 after_free=58907 after_allocated=137300
frame-allocator-diagnostic: process-teardown pid=769 reclaimed_frames=1 before_free=58374 before_allocated=137833 after_free=58375 after_allocated=137832
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
chmod07.c:77: TINFO: getgrnam(users) failed - try fallback daemon
chmod07.c:77: TBROK: getgrnam(daemon) failed: ENOTSOCK (88)
frame-allocator-diagnostic: process-teardown pid=772 reclaimed_frames=13 before_free=58092 before_allocated=138115 after_free=58105 after_allocated=138102

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[378.107432 0:768 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=768 reclaimed_frames=262 before_free=58106 before_allocated=138101 after_free=58368 after_allocated=137839
frame-allocator-diagnostic: process-teardown pid=766 reclaimed_frames=13 before_free=58368 before_allocated=137839 after_free=58381 after_allocated=137826
FAIL LTP CASE chmod07 : 2
RUN LTP CASE chown01
frame-allocator-diagnostic: process-teardown pid=775 reclaimed_frames=511 before_free=57837 before_allocated=138370 after_free=58348 after_allocated=137859
frame-allocator-diagnostic: process-teardown pid=770 reclaimed_frames=508 before_free=58348 before_allocated=137859 after_free=58856 after_allocated=137351
frame-allocator-diagnostic: process-teardown pid=777 reclaimed_frames=1 before_free=58324 before_allocated=137883 after_free=58325 after_allocated=137882
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
chown01.c:24: TPASS: chown(chown01_testfile,0,0) passed
frame-allocator-diagnostic: process-teardown pid=780 reclaimed_frames=12 before_free=58043 before_allocated=138164 after_free=58055 after_allocated=138152

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[382.416789 0:776 axfs::fops:297] [AxError::NotADirectory]
[382.417709 0:776 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=776 reclaimed_frames=262 before_free=58056 before_allocated=138151 after_free=58318 after_allocated=137889
frame-allocator-diagnostic: process-teardown pid=774 reclaimed_frames=13 before_free=58318 before_allocated=137889 after_free=58331 after_allocated=137876
FAIL LTP CASE chown01 : 0
RUN LTP CASE chown01_16
frame-allocator-diagnostic: process-teardown pid=783 reclaimed_frames=511 before_free=57787 before_allocated=138420 after_free=58298 after_allocated=137909
frame-allocator-diagnostic: process-teardown pid=778 reclaimed_frames=508 before_free=58298 before_allocated=137909 after_free=58806 after_allocated=137401
frame-allocator-diagnostic: process-teardown pid=785 reclaimed_frames=1 before_free=58274 before_allocated=137933 after_free=58275 after_allocated=137932
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
/code/ltp-full-20240524/testcases/kernel/syscalls/chown/../utils/compat_tst_16.h:153: TCONF: 16-bit version of chown() is not supported on your platform
frame-allocator-diagnostic: process-teardown pid=788 reclaimed_frames=12 before_free=57993 before_allocated=138214 after_free=58005 after_allocated=138202

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[386.530971 0:784 axfs::fops:297] [AxError::NotADirectory]
[386.531880 0:784 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=784 reclaimed_frames=262 before_free=58006 before_allocated=138201 after_free=58268 after_allocated=137939
frame-allocator-diagnostic: process-teardown pid=782 reclaimed_frames=13 before_free=58268 before_allocated=137939 after_free=58281 after_allocated=137926
FAIL LTP CASE chown01_16 : 32
RUN LTP CASE chown02
frame-allocator-diagnostic: process-teardown pid=791 reclaimed_frames=511 before_free=57738 before_allocated=138469 after_free=58249 after_allocated=137958
frame-allocator-diagnostic: process-teardown pid=786 reclaimed_frames=508 before_free=58249 before_allocated=137958 after_free=58757 after_allocated=137450
frame-allocator-diagnostic: process-teardown pid=793 reclaimed_frames=1 before_free=58225 before_allocated=137982 after_free=58226 after_allocated=137981
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
chown02.c:46: TPASS: chown(testfile1, 0, 0) passed
chown02.c:46: TPASS: chown(testfile2, 0, 0) passed
frame-allocator-diagnostic: process-teardown pid=796 reclaimed_frames=12 before_free=57944 before_allocated=138263 after_free=57956 after_allocated=138251

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[390.628600 0:792 axfs::fops:297] [AxError::NotADirectory]
[390.629097 0:792 axfs::fops:297] [AxError::NotADirectory]
[390.629775 0:792 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=792 reclaimed_frames=262 before_free=57957 before_allocated=138250 after_free=58219 after_allocated=137988
frame-allocator-diagnostic: process-teardown pid=790 reclaimed_frames=12 before_free=58219 before_allocated=137988 after_free=58231 after_allocated=137976
FAIL LTP CASE chown02 : 0
RUN LTP CASE chown02_16
frame-allocator-diagnostic: process-teardown pid=799 reclaimed_frames=511 before_free=57688 before_allocated=138519 after_free=58199 after_allocated=138008
frame-allocator-diagnostic: process-teardown pid=794 reclaimed_frames=508 before_free=58199 before_allocated=138008 after_free=58707 after_allocated=137500
frame-allocator-diagnostic: process-teardown pid=801 reclaimed_frames=1 before_free=58175 before_allocated=138032 after_free=58176 after_allocated=138031
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
/code/ltp-full-20240524/testcases/kernel/syscalls/chown/../utils/compat_tst_16.h:153: TCONF: 16-bit version of chown() is not supported on your platform
frame-allocator-diagnostic: process-teardown pid=804 reclaimed_frames=12 before_free=57894 before_allocated=138313 after_free=57906 after_allocated=138301

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[394.741258 0:800 axfs::fops:297] [AxError::NotADirectory]
[394.741752 0:800 axfs::fops:297] [AxError::NotADirectory]
[394.742629 0:800 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=800 reclaimed_frames=262 before_free=57907 before_allocated=138300 after_free=58169 after_allocated=138038
frame-allocator-diagnostic: process-teardown pid=798 reclaimed_frames=12 before_free=58169 before_allocated=138038 after_free=58181 after_allocated=138026
FAIL LTP CASE chown02_16 : 32
RUN LTP CASE chown03
frame-allocator-diagnostic: process-teardown pid=807 reclaimed_frames=511 before_free=57638 before_allocated=138569 after_free=58149 after_allocated=138058
frame-allocator-diagnostic: process-teardown pid=802 reclaimed_frames=508 before_free=58149 before_allocated=138058 after_free=58657 after_allocated=137550
frame-allocator-diagnostic: process-teardown pid=809 reclaimed_frames=1 before_free=58125 before_allocated=138082 after_free=58126 after_allocated=138081
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
chown03.c:63: TPASS: chown(chown03_testfile, -1, 65534) passed
frame-allocator-diagnostic: process-teardown pid=812 reclaimed_frames=13 before_free=57843 before_allocated=138364 after_free=57856 after_allocated=138351

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[398.880647 0:808 axfs::fops:297] [AxError::NotADirectory]
[398.881477 0:808 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=808 reclaimed_frames=262 before_free=57857 before_allocated=138350 after_free=58119 after_allocated=138088
frame-allocator-diagnostic: process-teardown pid=806 reclaimed_frames=12 before_free=58119 before_allocated=138088 after_free=58131 after_allocated=138076
FAIL LTP CASE chown03 : 0
RUN LTP CASE chown03_16
frame-allocator-diagnostic: process-teardown pid=815 reclaimed_frames=511 before_free=57588 before_allocated=138619 after_free=58099 after_allocated=138108
frame-allocator-diagnostic: process-teardown pid=810 reclaimed_frames=508 before_free=58099 before_allocated=138108 after_free=58607 after_allocated=137600
frame-allocator-diagnostic: process-teardown pid=817 reclaimed_frames=1 before_free=58075 before_allocated=138132 after_free=58076 after_allocated=138131
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
/code/ltp-full-20240524/testcases/kernel/syscalls/chown/../utils/compat_tst_16.h:153: TCONF: 16-bit version of chown() is not supported on your platform
frame-allocator-diagnostic: process-teardown pid=820 reclaimed_frames=13 before_free=57792 before_allocated=138415 after_free=57805 after_allocated=138402

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[402.988874 0:816 axfs::fops:297] [AxError::NotADirectory]
[402.990836 0:816 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=816 reclaimed_frames=263 before_free=57806 before_allocated=138401 after_free=58069 after_allocated=138138
frame-allocator-diagnostic: process-teardown pid=814 reclaimed_frames=12 before_free=58069 before_allocated=138138 after_free=58081 after_allocated=138126
FAIL LTP CASE chown03_16 : 32
RUN LTP CASE chown04
frame-allocator-diagnostic: process-teardown pid=823 reclaimed_frames=511 before_free=57538 before_allocated=138669 after_free=58049 after_allocated=138158
frame-allocator-diagnostic: process-teardown pid=818 reclaimed_frames=508 before_free=58049 before_allocated=138158 after_free=58557 after_allocated=137650
frame-allocator-diagnostic: process-teardown pid=825 reclaimed_frames=1 before_free=58025 before_allocated=138182 after_free=58026 after_allocated=138181
[407.143454 0:824 axfs::root:423] [AxError::AlreadyExists]
tst_test.c:1011: TBROK: mkdir(mntpoint/dir/, 0777) failed: EEXIST (17)

Summary:
passed   0
failed   0
broken   1
skipped  0
warnings 0
[407.152164 0:824 axfs::root:433] [AxError::IsADirectory]
[407.153372 0:824 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=824 reclaimed_frames=265 before_free=57762 before_allocated=138445 after_free=58027 after_allocated=138180
frame-allocator-diagnostic: process-teardown pid=822 reclaimed_frames=12 before_free=58027 before_allocated=138180 after_free=58039 after_allocated=138168
FAIL LTP CASE chown04 : 2
RUN LTP CASE chown04_16
frame-allocator-diagnostic: process-teardown pid=828 reclaimed_frames=511 before_free=57495 before_allocated=138712 after_free=58006 after_allocated=138201
frame-allocator-diagnostic: process-teardown pid=826 reclaimed_frames=508 before_free=58006 before_allocated=138201 after_free=58514 after_allocated=137693
frame-allocator-diagnostic: process-teardown pid=830 reclaimed_frames=1 before_free=57982 before_allocated=138225 after_free=57983 after_allocated=138224
[411.213063 0:829 axfs::root:423] [AxError::AlreadyExists]
tst_test.c:1011: TBROK: mkdir(mntpoint/dir/, 0777) failed: EEXIST (17)

Summary:
passed   0
failed   0
broken   1
skipped  0
warnings 0
[411.220937 0:829 axfs::root:433] [AxError::IsADirectory]
[411.222147 0:829 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=829 reclaimed_frames=266 before_free=57718 before_allocated=138489 after_free=57984 after_allocated=138223
frame-allocator-diagnostic: process-teardown pid=827 reclaimed_frames=13 before_free=57984 before_allocated=138223 after_free=57997 after_allocated=138210
FAIL LTP CASE chown04_16 : 2
RUN LTP CASE chown05
frame-allocator-diagnostic: process-teardown pid=833 reclaimed_frames=511 before_free=57454 before_allocated=138753 after_free=57965 after_allocated=138242
frame-allocator-diagnostic: process-teardown pid=831 reclaimed_frames=508 before_free=57965 before_allocated=138242 after_free=58473 after_allocated=137734
frame-allocator-diagnostic: process-teardown pid=835 reclaimed_frames=1 before_free=57941 before_allocated=138266 after_free=57942 after_allocated=138265
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
chown05.c:42: TPASS: chown(testfile, 700, 701), change owner/group ids passed
chown05.c:42: TPASS: chown(testfile, 702, -1), change owner id only passed
chown05.c:42: TPASS: chown(testfile, 703, 701), change owner id only passed
chown05.c:42: TPASS: chown(testfile, -1, 704), change group id only passed
chown05.c:42: TPASS: chown(testfile, 703, 705), change group id only passed
chown05.c:42: TPASS: chown(testfile, -1, -1), no change passed
frame-allocator-diagnostic: process-teardown pid=838 reclaimed_frames=11 before_free=57661 before_allocated=138546 after_free=57672 after_allocated=138535

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[415.411641 0:834 axfs::fops:297] [AxError::NotADirectory]
[415.412437 0:834 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=834 reclaimed_frames=262 before_free=57673 before_allocated=138534 after_free=57935 after_allocated=138272
frame-allocator-diagnostic: process-teardown pid=832 reclaimed_frames=12 before_free=57935 before_allocated=138272 after_free=57947 after_allocated=138260
FAIL LTP CASE chown05 : 0
RUN LTP CASE chown05_16
frame-allocator-diagnostic: process-teardown pid=841 reclaimed_frames=511 before_free=57404 before_allocated=138803 after_free=57915 after_allocated=138292
frame-allocator-diagnostic: process-teardown pid=836 reclaimed_frames=508 before_free=57915 before_allocated=138292 after_free=58423 after_allocated=137784
frame-allocator-diagnostic: process-teardown pid=843 reclaimed_frames=1 before_free=57891 before_allocated=138316 after_free=57892 after_allocated=138315
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
/code/ltp-full-20240524/testcases/kernel/syscalls/chown/../utils/compat_tst_16.h:153: TCONF: 16-bit version of chown() is not supported on your platform
frame-allocator-diagnostic: process-teardown pid=846 reclaimed_frames=11 before_free=57611 before_allocated=138596 after_free=57622 after_allocated=138585

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[419.564192 0:842 axfs::fops:297] [AxError::NotADirectory]
[419.565114 0:842 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=842 reclaimed_frames=262 before_free=57623 before_allocated=138584 after_free=57885 after_allocated=138322
frame-allocator-diagnostic: process-teardown pid=840 reclaimed_frames=12 before_free=57885 before_allocated=138322 after_free=57897 after_allocated=138310
FAIL LTP CASE chown05_16 : 32
RUN LTP CASE chroot01
frame-allocator-diagnostic: process-teardown pid=849 reclaimed_frames=511 before_free=57353 before_allocated=138854 after_free=57864 after_allocated=138343
frame-allocator-diagnostic: process-teardown pid=844 reclaimed_frames=508 before_free=57864 before_allocated=138343 after_free=58372 after_allocated=137835
frame-allocator-diagnostic: process-teardown pid=851 reclaimed_frames=1 before_free=57840 before_allocated=138367 after_free=57841 after_allocated=138366
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
chroot01.c:23: TFAIL: unprivileged chroot() expected EPERM: ENOSYS (38)
frame-allocator-diagnostic: process-teardown pid=854 reclaimed_frames=13 before_free=57558 before_allocated=138649 after_free=57571 after_allocated=138636

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[423.703539 0:850 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=850 reclaimed_frames=262 before_free=57572 before_allocated=138635 after_free=57834 after_allocated=138373
frame-allocator-diagnostic: process-teardown pid=848 reclaimed_frames=13 before_free=57834 before_allocated=138373 after_free=57847 after_allocated=138360
FAIL LTP CASE chroot01 : 0
RUN LTP CASE chroot02
frame-allocator-diagnostic: process-teardown pid=857 reclaimed_frames=511 before_free=57304 before_allocated=138903 after_free=57815 after_allocated=138392
frame-allocator-diagnostic: process-teardown pid=852 reclaimed_frames=508 before_free=57815 before_allocated=138392 after_free=58323 after_allocated=137884
frame-allocator-diagnostic: process-teardown pid=859 reclaimed_frames=1 before_free=57791 before_allocated=138416 after_free=57792 after_allocated=138415
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
chroot02.c:30: TFAIL: chroot(/tmp/LTP_chrnfJkoi) failed: ENOSYS (38)
frame-allocator-diagnostic: process-teardown pid=863 reclaimed_frames=7 before_free=57498 before_allocated=138709 after_free=57505 after_allocated=138702
tst_test.c:1449: TBROK: Test haven't reported results!
frame-allocator-diagnostic: process-teardown pid=862 reclaimed_frames=11 before_free=57503 before_allocated=138704 after_free=57514 after_allocated=138693

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[427.814818 0:858 axfs::fops:297] [AxError::NotADirectory]
[427.815848 0:858 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=858 reclaimed_frames=262 before_free=57515 before_allocated=138692 after_free=57777 after_allocated=138430
frame-allocator-diagnostic: process-teardown pid=856 reclaimed_frames=12 before_free=57777 before_allocated=138430 after_free=57789 after_allocated=138418
FAIL LTP CASE chroot02 : 2
RUN LTP CASE chroot03
frame-allocator-diagnostic: process-teardown pid=866 reclaimed_frames=511 before_free=57246 before_allocated=138961 after_free=57757 after_allocated=138450
frame-allocator-diagnostic: process-teardown pid=860 reclaimed_frames=508 before_free=57757 before_allocated=138450 after_free=58265 after_allocated=137942
frame-allocator-diagnostic: process-teardown pid=868 reclaimed_frames=1 before_free=57733 before_allocated=138474 after_free=57734 after_allocated=138473
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
chroot03.c:65: TBROK: symlink(sym_dir1/,sym_dir2) failed: ENOSYS (38)
frame-allocator-diagnostic: process-teardown pid=871 reclaimed_frames=12 before_free=57448 before_allocated=138759 after_free=57460 after_allocated=138747

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[431.969328 0:867 axfs::fops:297] [AxError::NotADirectory]
[431.970326 0:867 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=867 reclaimed_frames=266 before_free=57461 before_allocated=138746 after_free=57727 after_allocated=138480
frame-allocator-diagnostic: process-teardown pid=865 reclaimed_frames=12 before_free=57727 before_allocated=138480 after_free=57739 after_allocated=138468
FAIL LTP CASE chroot03 : 2
RUN LTP CASE chroot04
frame-allocator-diagnostic: process-teardown pid=874 reclaimed_frames=511 before_free=57196 before_allocated=139011 after_free=57707 after_allocated=138500
frame-allocator-diagnostic: process-teardown pid=869 reclaimed_frames=508 before_free=57707 before_allocated=138500 after_free=58215 after_allocated=137992
frame-allocator-diagnostic: process-teardown pid=876 reclaimed_frames=1 before_free=57683 before_allocated=138524 after_free=57684 after_allocated=138523
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
chroot04.c:27: TFAIL: no search permission chroot() expected EACCES: ENOSYS (38)
frame-allocator-diagnostic: process-teardown pid=879 reclaimed_frames=13 before_free=57401 before_allocated=138806 after_free=57414 after_allocated=138793

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[436.131209 0:875 axfs::root:433] [AxError::IsADirectory]
[436.132386 0:875 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=875 reclaimed_frames=263 before_free=57414 before_allocated=138793 after_free=57677 after_allocated=138530
frame-allocator-diagnostic: process-teardown pid=873 reclaimed_frames=12 before_free=57677 before_allocated=138530 after_free=57689 after_allocated=138518
FAIL LTP CASE chroot04 : 0
RUN LTP CASE cleanup_lvm.sh
frame-allocator-diagnostic: process-teardown pid=882 reclaimed_frames=511 before_free=57145 before_allocated=139062 after_free=57656 after_allocated=138551
frame-allocator-diagnostic: process-teardown pid=877 reclaimed_frames=508 before_free=57656 before_allocated=138551 after_free=58164 after_allocated=138043
frame-allocator-diagnostic: process-teardown pid=884 reclaimed_frames=1 before_free=57632 before_allocated=138575 after_free=57633 after_allocated=138574
sh: tst_test.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=886 reclaimed_frames=7 before_free=57105 before_allocated=139102 after_free=57112 after_allocated=139095
frame-allocator-diagnostic: process-teardown pid=883 reclaimed_frames=514 before_free=57112 before_allocated=139095 after_free=57626 after_allocated=138581
frame-allocator-diagnostic: process-teardown pid=881 reclaimed_frames=13 before_free=57626 before_allocated=138581 after_free=57639 after_allocated=138568
FAIL LTP CASE cleanup_lvm.sh : 2
RUN LTP CASE clock_adjtime01
frame-allocator-diagnostic: process-teardown pid=888 reclaimed_frames=511 before_free=57096 before_allocated=139111 after_free=57607 after_allocated=138600
frame-allocator-diagnostic: process-teardown pid=885 reclaimed_frames=508 before_free=57607 before_allocated=138600 after_free=58115 after_allocated=138092
frame-allocator-diagnostic: process-teardown pid=890 reclaimed_frames=1 before_free=57583 before_allocated=138624 after_free=57584 after_allocated=138623
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_adjtime01.c:186: TINFO: Testing variant: syscall with old kernel spec
clock_adjtime.h:123: TCONF: syscall(266) __NR_clock_adjtime not supported on your arch
frame-allocator-diagnostic: process-teardown pid=893 reclaimed_frames=11 before_free=57302 before_allocated=138905 after_free=57313 after_allocated=138894

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[441.606716 0:889 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=889 reclaimed_frames=263 before_free=57314 before_allocated=138893 after_free=57577 after_allocated=138630
frame-allocator-diagnostic: process-teardown pid=887 reclaimed_frames=12 before_free=57577 before_allocated=138630 after_free=57589 after_allocated=138618
FAIL LTP CASE clock_adjtime01 : 32
RUN LTP CASE clock_adjtime02
frame-allocator-diagnostic: process-teardown pid=896 reclaimed_frames=511 before_free=57045 before_allocated=139162 after_free=57556 after_allocated=138651
frame-allocator-diagnostic: process-teardown pid=891 reclaimed_frames=508 before_free=57556 before_allocated=138651 after_free=58064 after_allocated=138143
frame-allocator-diagnostic: process-teardown pid=898 reclaimed_frames=1 before_free=57532 before_allocated=138675 after_free=57533 after_allocated=138674
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_adjtime02.c:197: TINFO: Testing variant: syscall with old kernel spec
clock_adjtime.h:123: TCONF: syscall(266) __NR_clock_adjtime not supported on your arch
frame-allocator-diagnostic: process-teardown pid=901 reclaimed_frames=11 before_free=57251 before_allocated=138956 after_free=57262 after_allocated=138945

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[445.788104 0:897 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=897 reclaimed_frames=263 before_free=57263 before_allocated=138944 after_free=57526 after_allocated=138681
frame-allocator-diagnostic: process-teardown pid=895 reclaimed_frames=13 before_free=57526 before_allocated=138681 after_free=57539 after_allocated=138668
FAIL LTP CASE clock_adjtime02 : 32
RUN LTP CASE clock_getres01
frame-allocator-diagnostic: process-teardown pid=904 reclaimed_frames=511 before_free=56996 before_allocated=139211 after_free=57507 after_allocated=138700
frame-allocator-diagnostic: process-teardown pid=899 reclaimed_frames=508 before_free=57507 before_allocated=138700 after_free=58015 after_allocated=138192
frame-allocator-diagnostic: process-teardown pid=906 reclaimed_frames=1 before_free=57483 before_allocated=138724 after_free=57484 after_allocated=138723
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_getres01.c:62: TINFO: Testing variant: vDSO or syscall with libc spec
clock_getres01.c:88: TPASS: clock_getres(REALTIME, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(MONOTONIC, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(PROCESS_CPUTIME_ID, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(THREAD_CPUTIME_ID, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_MONOTONIC_RAW, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_REALTIME_COARSE, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_MONOTONIC_COARSE, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_BOOTTIME, ...) succeeded
clock_getres01.c:73: TCONF: clock_getres(CLOCK_REALTIME_ALARM, ...) NO SUPPORTED
clock_getres01.c:73: TCONF: clock_getres(CLOCK_BOOTTIME_ALARM, ...) NO SUPPORTED
clock_getres01.c:88: TPASS: clock_getres(-1, ...) succeeded
frame-allocator-diagnostic: process-teardown pid=909 reclaimed_frames=11 before_free=57202 before_allocated=139005 after_free=57213 after_allocated=138994
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_getres01.c:62: TINFO: Testing variant: vDSO or syscall with libc spec with NULL res
clock_getres01.c:88: TPASS: clock_getres(REALTIME, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(MONOTONIC, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(PROCESS_CPUTIME_ID, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(THREAD_CPUTIME_ID, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_MONOTONIC_RAW, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_REALTIME_COARSE, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_MONOTONIC_COARSE, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_BOOTTIME, ...) succeeded
clock_getres01.c:73: TCONF: clock_getres(CLOCK_REALTIME_ALARM, ...) NO SUPPORTED
clock_getres01.c:73: TCONF: clock_getres(CLOCK_BOOTTIME_ALARM, ...) NO SUPPORTED
clock_getres01.c:88: TPASS: clock_getres(-1, ...) succeeded
frame-allocator-diagnostic: process-teardown pid=912 reclaimed_frames=11 before_free=57194 before_allocated=139013 after_free=57205 after_allocated=139002
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_getres01.c:62: TINFO: Testing variant: syscall with old kernel spec
clock_getres01.c:88: TPASS: clock_getres(REALTIME, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(MONOTONIC, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(PROCESS_CPUTIME_ID, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(THREAD_CPUTIME_ID, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_MONOTONIC_RAW, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_REALTIME_COARSE, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_MONOTONIC_COARSE, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_BOOTTIME, ...) succeeded
clock_getres01.c:73: TCONF: clock_getres(CLOCK_REALTIME_ALARM, ...) NO SUPPORTED
clock_getres01.c:73: TCONF: clock_getres(CLOCK_BOOTTIME_ALARM, ...) NO SUPPORTED
clock_getres01.c:88: TPASS: clock_getres(-1, ...) succeeded
frame-allocator-diagnostic: process-teardown pid=915 reclaimed_frames=11 before_free=57186 before_allocated=139021 after_free=57197 after_allocated=139010
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_getres01.c:62: TINFO: Testing variant: syscall with old kernel spec with NULL res
clock_getres01.c:88: TPASS: clock_getres(REALTIME, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(MONOTONIC, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(PROCESS_CPUTIME_ID, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(THREAD_CPUTIME_ID, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_MONOTONIC_RAW, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_REALTIME_COARSE, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_MONOTONIC_COARSE, ...) succeeded
clock_getres01.c:88: TPASS: clock_getres(CLOCK_BOOTTIME, ...) succeeded
clock_getres01.c:73: TCONF: clock_getres(CLOCK_REALTIME_ALARM, ...) NO SUPPORTED
clock_getres01.c:73: TCONF: clock_getres(CLOCK_BOOTTIME_ALARM, ...) NO SUPPORTED
clock_getres01.c:88: TPASS: clock_getres(-1, ...) succeeded
frame-allocator-diagnostic: process-teardown pid=918 reclaimed_frames=11 before_free=57178 before_allocated=139029 after_free=57189 after_allocated=139018

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[449.957670 0:905 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=905 reclaimed_frames=263 before_free=57190 before_allocated=139017 after_free=57453 after_allocated=138754
frame-allocator-diagnostic: process-teardown pid=903 reclaimed_frames=12 before_free=57453 before_allocated=138754 after_free=57465 after_allocated=138742
FAIL LTP CASE clock_getres01 : 0
RUN LTP CASE clock_gettime01
frame-allocator-diagnostic: process-teardown pid=921 reclaimed_frames=511 before_free=56922 before_allocated=139285 after_free=57433 after_allocated=138774
frame-allocator-diagnostic: process-teardown pid=907 reclaimed_frames=508 before_free=57433 before_allocated=138774 after_free=57941 after_allocated=138266
frame-allocator-diagnostic: process-teardown pid=923 reclaimed_frames=1 before_free=57409 before_allocated=138798 after_free=57410 after_allocated=138797
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_gettime01.c:78: TINFO: Testing variant: vDSO or syscall with libc spec
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_REALTIME passed
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_MONOTONIC passed
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_PROCESS_CPUTIME_ID passed
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_THREAD_CPUTIME_ID passed
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_REALTIME_COARSE passed
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_MONOTONIC_COARSE passed
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_MONOTONIC_RAW passed
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_BOOTTIME passed
frame-allocator-diagnostic: process-teardown pid=926 reclaimed_frames=11 before_free=57129 before_allocated=139078 after_free=57140 after_allocated=139067
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_gettime01.c:78: TINFO: Testing variant: syscall with old kernel spec
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_REALTIME passed
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_MONOTONIC passed
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_PROCESS_CPUTIME_ID passed
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_THREAD_CPUTIME_ID passed
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_REALTIME_COARSE passed
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_MONOTONIC_COARSE passed
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_MONOTONIC_RAW passed
clock_gettime01.c:111: TPASS: clock_gettime(2): clock CLOCK_BOOTTIME passed
frame-allocator-diagnostic: process-teardown pid=929 reclaimed_frames=11 before_free=57121 before_allocated=139086 after_free=57132 after_allocated=139075

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[454.255613 0:922 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=922 reclaimed_frames=262 before_free=57133 before_allocated=139074 after_free=57395 after_allocated=138812
frame-allocator-diagnostic: process-teardown pid=920 reclaimed_frames=12 before_free=57395 before_allocated=138812 after_free=57407 after_allocated=138800
FAIL LTP CASE clock_gettime01 : 0
RUN LTP CASE clock_gettime02
frame-allocator-diagnostic: process-teardown pid=932 reclaimed_frames=511 before_free=56864 before_allocated=139343 after_free=57375 after_allocated=138832
frame-allocator-diagnostic: process-teardown pid=924 reclaimed_frames=508 before_free=57375 before_allocated=138832 after_free=57883 after_allocated=138324
frame-allocator-diagnostic: process-teardown pid=934 reclaimed_frames=1 before_free=57351 before_allocated=138856 after_free=57352 after_allocated=138855
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_gettime02.c:102: TINFO: Testing variant: 0: syscall with old kernel spec
clock_gettime02.c:130: TPASS: clock_gettime(2): clock INVALID/UNKNOWN CLOCK failed as expected: EINVAL (22)
clock_gettime02.c:130: TPASS: clock_gettime(2): clock INVALID/UNKNOWN CLOCK failed as expected: EINVAL (22)
clock_gettime02.c:130: TPASS: clock_gettime(2): clock CLOCK_REALTIME failed as expected: EFAULT (14)
clock_gettime02.c:130: TPASS: clock_gettime(2): clock CLOCK_MONOTONIC failed as expected: EFAULT (14)
clock_gettime02.c:130: TPASS: clock_gettime(2): clock CLOCK_PROCESS_CPUTIME_ID failed as expected: EFAULT (14)
clock_gettime02.c:130: TPASS: clock_gettime(2): clock CLOCK_THREAD_CPUTIME_ID failed as expected: EFAULT (14)
clock_gettime02.c:130: TPASS: clock_gettime(2): clock CLOCK_REALTIME_COARSE failed as expected: EFAULT (14)
clock_gettime02.c:130: TPASS: clock_gettime(2): clock CLOCK_MONOTONIC_COARSE failed as expected: EFAULT (14)
clock_gettime02.c:130: TPASS: clock_gettime(2): clock CLOCK_MONOTONIC_RAW failed as expected: EFAULT (14)
clock_gettime02.c:130: TPASS: clock_gettime(2): clock CLOCK_BOOTTIME failed as expected: EFAULT (14)
frame-allocator-diagnostic: process-teardown pid=937 reclaimed_frames=10 before_free=57072 before_allocated=139135 after_free=57082 after_allocated=139125

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[458.466162 0:933 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=933 reclaimed_frames=262 before_free=57083 before_allocated=139124 after_free=57345 after_allocated=138862
frame-allocator-diagnostic: process-teardown pid=931 reclaimed_frames=12 before_free=57345 before_allocated=138862 after_free=57357 after_allocated=138850
FAIL LTP CASE clock_gettime02 : 0
RUN LTP CASE clock_gettime03
frame-allocator-diagnostic: process-teardown pid=940 reclaimed_frames=511 before_free=56814 before_allocated=139393 after_free=57325 after_allocated=138882
frame-allocator-diagnostic: process-teardown pid=935 reclaimed_frames=508 before_free=57325 before_allocated=138882 after_free=57833 after_allocated=138374
frame-allocator-diagnostic: process-teardown pid=942 reclaimed_frames=1 before_free=57301 before_allocated=138906 after_free=57302 after_allocated=138905
tst_kconfig.c:71: TINFO: Couldn't locate kernel config!
tst_kconfig.c:207: TBROK: Cannot parse kernel .config
frame-allocator-diagnostic: process-teardown pid=941 reclaimed_frames=262 before_free=57041 before_allocated=139166 after_free=57303 after_allocated=138904
frame-allocator-diagnostic: process-teardown pid=939 reclaimed_frames=12 before_free=57303 before_allocated=138904 after_free=57315 after_allocated=138892
FAIL LTP CASE clock_gettime03 : 2
RUN LTP CASE clock_gettime04
frame-allocator-diagnostic: process-teardown pid=945 reclaimed_frames=511 before_free=56772 before_allocated=139435 after_free=57283 after_allocated=138924
frame-allocator-diagnostic: process-teardown pid=943 reclaimed_frames=508 before_free=57283 before_allocated=138924 after_free=57791 after_allocated=138416
frame-allocator-diagnostic: process-teardown pid=947 reclaimed_frames=1 before_free=57259 before_allocated=138948 after_free=57260 after_allocated=138947
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
frame-allocator-diagnostic: process-teardown pid=951 reclaimed_frames=3 before_free=56967 before_allocated=139240 after_free=56970 after_allocated=139237
vdso_helpers.c:27: TINFO: Couldn't find AT_SYSINFO_EHDR
vdso_helpers.c:72: TINFO: Couldn't find vdso_gettime()
vdso_helpers.c:76: TINFO: Couldn't find vdso_gettime64()
clock_gettime04.c:183: TPASS: CLOCK_REALTIME: Difference between successive readings is reasonable for following variants:
clock_gettime04.c:188: TINFO: 	- vDSO or syscall with libc spec
clock_gettime04.c:188: TINFO: 	- syscall with old kernel spec
clock_gettime04.c:188: TINFO: 	- vDSO with old kernel spec
clock_gettime04.c:188: TINFO: 	- gettimeofday
clock_gettime04.c:183: TPASS: CLOCK_REALTIME_COARSE: Difference between successive readings is reasonable for following variants:
clock_gettime04.c:188: TINFO: 	- vDSO or syscall with libc spec
clock_gettime04.c:188: TINFO: 	- syscall with old kernel spec
clock_gettime04.c:188: TINFO: 	- vDSO with old kernel spec
clock_gettime04.c:183: TPASS: CLOCK_MONOTONIC: Difference between successive readings is reasonable for following variants:
clock_gettime04.c:188: TINFO: 	- vDSO or syscall with libc spec
clock_gettime04.c:188: TINFO: 	- syscall with old kernel spec
clock_gettime04.c:188: TINFO: 	- vDSO with old kernel spec
clock_gettime04.c:183: TPASS: CLOCK_MONOTONIC_COARSE: Difference between successive readings is reasonable for following variants:
clock_gettime04.c:188: TINFO: 	- vDSO or syscall with libc spec
clock_gettime04.c:188: TINFO: 	- syscall with old kernel spec
clock_gettime04.c:188: TINFO: 	- vDSO with old kernel spec
clock_gettime04.c:176: TFAIL: CLOCK_MONOTONIC_RAW(syscall with old kernel spec): Difference between successive readings greater than 5 ms (1): 5
clock_gettime04.c:183: TPASS: CLOCK_BOOTTIME: Difference between successive readings is reasonable for following variants:
clock_gettime04.c:188: TINFO: 	- vDSO or syscall with libc spec
clock_gettime04.c:188: TINFO: 	- syscall with old kernel spec
clock_gettime04.c:188: TINFO: 	- vDSO with old kernel spec
frame-allocator-diagnostic: process-teardown pid=950 reclaimed_frames=12 before_free=56969 before_allocated=139238 after_free=56981 after_allocated=139226

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[467.223189 0:946 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=946 reclaimed_frames=263 before_free=56982 before_allocated=139225 after_free=57245 after_allocated=138962
frame-allocator-diagnostic: process-teardown pid=948 reclaimed_frames=508 before_free=57245 before_allocated=138962 after_free=57753 after_allocated=138454
frame-allocator-diagnostic: process-teardown pid=944 reclaimed_frames=12 before_free=57753 before_allocated=138454 after_free=57765 after_allocated=138442
FAIL LTP CASE clock_gettime04 : 0
RUN LTP CASE clock_nanosleep01
frame-allocator-diagnostic: process-teardown pid=954 reclaimed_frames=511 before_free=57221 before_allocated=138986 after_free=57732 after_allocated=138475
frame-allocator-diagnostic: process-teardown pid=956 reclaimed_frames=1 before_free=57200 before_allocated=139007 after_free=57201 after_allocated=139006
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_nanosleep01.c:124: TINFO: Testing variant: vDSO or syscall with libc spec
clock_nanosleep01.c:139: TINFO: case NORMAL
clock_nanosleep01.c:218: TPASS: clock_nanosleep() failed with: EINVAL (22)
clock_nanosleep01.c:139: TINFO: case NORMAL
clock_nanosleep01.c:218: TPASS: clock_nanosleep() failed with: EINVAL (22)
clock_nanosleep01.c:139: TINFO: case NORMAL
clock_nanosleep01.c:218: TPASS: clock_nanosleep() failed with: EINVAL (22)
clock_nanosleep01.c:139: TINFO: case SEND_SIGINT
frame-allocator-diagnostic: process-teardown pid=960 reclaimed_frames=8 before_free=56899 before_allocated=139308 after_free=56907 after_allocated=139300
clock_nanosleep01.c:195: TFAIL: The clock_nanosleep() haven't updated timespec or it's not valid: SUCCESS (0)
clock_nanosleep01.c:139: TINFO: case BAD_TS_ADDR_REQ
clock_nanosleep01.c:143: TCONF: The libc wrapper may dereference req or rem
clock_nanosleep01.c:139: TINFO: case BAD_TS_ADDR_REM
clock_nanosleep01.c:143: TCONF: The libc wrapper may dereference req or rem
frame-allocator-diagnostic: process-teardown pid=959 reclaimed_frames=12 before_free=56908 before_allocated=139299 after_free=56920 after_allocated=139287
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_nanosleep01.c:124: TINFO: Testing variant: syscall with old kernel spec
clock_nanosleep01.c:139: TINFO: case NORMAL
clock_nanosleep01.c:218: TPASS: clock_nanosleep() failed with: EINVAL (22)
clock_nanosleep01.c:139: TINFO: case NORMAL
clock_nanosleep01.c:218: TPASS: clock_nanosleep() failed with: EINVAL (22)
clock_nanosleep01.c:139: TINFO: case NORMAL
clock_nanosleep01.c:212: TFAIL: returned 0, expected -1, expected errno: EOPNOTSUPP (95): SUCCESS (0)
clock_nanosleep01.c:139: TINFO: case SEND_SIGINT
frame-allocator-diagnostic: process-teardown pid=957 reclaimed_frames=508 before_free=56887 before_allocated=139320 after_free=57395 after_allocated=138812
frame-allocator-diagnostic: process-teardown pid=964 reclaimed_frames=8 before_free=57391 before_allocated=138816 after_free=57399 after_allocated=138808
clock_nanosleep01.c:195: TFAIL: The clock_nanosleep() haven't updated timespec or it's not valid: SUCCESS (0)
clock_nanosleep01.c:139: TINFO: case BAD_TS_ADDR_REQ
clock_nanosleep01.c:218: TPASS: clock_nanosleep() failed with: EFAULT (14)
clock_nanosleep01.c:139: TINFO: case BAD_TS_ADDR_REM
frame-allocator-diagnostic: process-teardown pid=965 reclaimed_frames=8 before_free=57383 before_allocated=138824 after_free=57391 after_allocated=138816
clock_nanosleep01.c:218: TPASS: clock_nanosleep() failed with: EFAULT (14)
frame-allocator-diagnostic: process-teardown pid=963 reclaimed_frames=12 before_free=57392 before_allocated=138815 after_free=57404 after_allocated=138803

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[532.072106 0:955 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=955 reclaimed_frames=265 before_free=57405 before_allocated=138802 after_free=57670 after_allocated=138537
frame-allocator-diagnostic: process-teardown pid=953 reclaimed_frames=13 before_free=57670 before_allocated=138537 after_free=57683 after_allocated=138524
FAIL LTP CASE clock_nanosleep01 : 0
RUN LTP CASE clock_nanosleep02
frame-allocator-diagnostic: process-teardown pid=968 reclaimed_frames=511 before_free=57140 before_allocated=139067 after_free=57651 after_allocated=138556
frame-allocator-diagnostic: process-teardown pid=970 reclaimed_frames=1 before_free=57119 before_allocated=139088 after_free=57120 after_allocated=139087
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
frame-allocator-diagnostic: process-teardown pid=974 reclaimed_frames=3 before_free=56827 before_allocated=139380 after_free=56830 after_allocated=139377
tst_timer_test.c:357: TINFO: CLOCK_MONOTONIC resolution 1ns
tst_timer_test.c:369: TINFO: prctl(PR_GET_TIMERSLACK) = 0us
tst_test.c:1625: TINFO: Updating max runtime to 0h 00m 09s
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 39s
tst_timer_test.c:379: TINFO: Failed to set zero latency constraint: No such file or directory
tst_timer_test.c:263: TINFO: clock_nanosleep() sleeping for 1000us 500 iterations, threshold 401.01us
tst_timer_test.c:305: TINFO: min 1072us, max 7972us, median 1105us, trunc mean 1115.63us (discarded 25)
tst_timer_test.c:326: TPASS: Measured times are within thresholds
tst_timer_test.c:263: TINFO: clock_nanosleep() sleeping for 2000us 500 iterations, threshold 402.01us
tst_timer_test.c:305: TINFO: min 2072us, max 8905us, median 2120us, trunc mean 2127.77us (discarded 25)
tst_timer_test.c:326: TPASS: Measured times are within thresholds
tst_timer_test.c:263: TINFO: clock_nanosleep() sleeping for 5000us 300 iterations, threshold 405.04us
tst_timer_test.c:305: TINFO: min 5080us, max 11978us, median 5167us, trunc mean 5166.44us (discarded 15)
tst_timer_test.c:326: TPASS: Measured times are within thresholds
tst_timer_test.c:263: TINFO: clock_nanosleep() sleeping for 10000us 100 iterations, threshold 410.33us
tst_timer_test.c:305: TINFO: min 10144us, max 15666us, median 10197us, trunc mean 10211.55us (discarded 5)
tst_timer_test.c:326: TPASS: Measured times are within thresholds
tst_timer_test.c:263: TINFO: clock_nanosleep() sleeping for 25000us 50 iterations, threshold 426.29us
tst_timer_test.c:305: TINFO: min 25167us, max 26533us, median 25235us, trunc mean 25244.67us (discarded 2)
tst_timer_test.c:326: TPASS: Measured times are within thresholds
tst_timer_test.c:263: TINFO: clock_nanosleep() sleeping for 100000us 10 iterations, threshold 537.00us
tst_timer_test.c:305: TINFO: min 100217us, max 100310us, median 100249us, trunc mean 100262.44us (discarded 1)
tst_timer_test.c:326: TPASS: Measured times are within thresholds
tst_timer_test.c:263: TINFO: clock_nanosleep() sleeping for 1000000us 2 iterations, threshold 4400.00us
tst_timer_test.c:305: TINFO: min 1000223us, max 1000247us, median 1000223us, trunc mean 1000223.00us (discarded 1)
tst_timer_test.c:326: TPASS: Measured times are within thresholds
frame-allocator-diagnostic: process-teardown pid=973 reclaimed_frames=14 before_free=56828 before_allocated=139379 after_free=56842 after_allocated=139365

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[544.763764 0:969 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=969 reclaimed_frames=262 before_free=56843 before_allocated=139364 after_free=57105 after_allocated=139102
frame-allocator-diagnostic: process-teardown pid=967 reclaimed_frames=12 before_free=57105 before_allocated=139102 after_free=57117 after_allocated=139090
FAIL LTP CASE clock_nanosleep02 : 0
RUN LTP CASE clock_nanosleep03
frame-allocator-diagnostic: process-teardown pid=977 reclaimed_frames=511 before_free=56574 before_allocated=139633 after_free=57085 after_allocated=139122
frame-allocator-diagnostic: process-teardown pid=971 reclaimed_frames=508 before_free=57085 before_allocated=139122 after_free=57593 after_allocated=138614
frame-allocator-diagnostic: process-teardown pid=979 reclaimed_frames=1 before_free=57061 before_allocated=139146 after_free=57062 after_allocated=139145
tst_kconfig.c:71: TINFO: Couldn't locate kernel config!
tst_kconfig.c:207: TBROK: Cannot parse kernel .config
frame-allocator-diagnostic: process-teardown pid=978 reclaimed_frames=262 before_free=56801 before_allocated=139406 after_free=57063 after_allocated=139144
frame-allocator-diagnostic: process-teardown pid=976 reclaimed_frames=12 before_free=57063 before_allocated=139144 after_free=57075 after_allocated=139132
FAIL LTP CASE clock_nanosleep03 : 2
RUN LTP CASE clock_nanosleep04
frame-allocator-diagnostic: process-teardown pid=982 reclaimed_frames=511 before_free=56532 before_allocated=139675 after_free=57043 after_allocated=139164
frame-allocator-diagnostic: process-teardown pid=980 reclaimed_frames=508 before_free=57043 before_allocated=139164 after_free=57551 after_allocated=138656
frame-allocator-diagnostic: process-teardown pid=984 reclaimed_frames=1 before_free=57019 before_allocated=139188 after_free=57020 after_allocated=139187
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_nanosleep04.c:33: TINFO: Testing variant: vDSO or syscall with libc spec
clock_nanosleep04.c:57: TPASS: clock_nanosleep(2) passed for clock CLOCK_MONOTONIC
clock_nanosleep04.c:57: TPASS: clock_nanosleep(2) passed for clock CLOCK_REALTIME
frame-allocator-diagnostic: process-teardown pid=987 reclaimed_frames=10 before_free=56740 before_allocated=139467 after_free=56750 after_allocated=139457
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_nanosleep04.c:33: TINFO: Testing variant: syscall with old kernel spec
clock_nanosleep04.c:57: TPASS: clock_nanosleep(2) passed for clock CLOCK_MONOTONIC
clock_nanosleep04.c:57: TPASS: clock_nanosleep(2) passed for clock CLOCK_REALTIME
frame-allocator-diagnostic: process-teardown pid=990 reclaimed_frames=10 before_free=56732 before_allocated=139475 after_free=56742 after_allocated=139465

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[553.165773 0:983 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=983 reclaimed_frames=262 before_free=56743 before_allocated=139464 after_free=57005 after_allocated=139202
frame-allocator-diagnostic: process-teardown pid=981 reclaimed_frames=12 before_free=57005 before_allocated=139202 after_free=57017 after_allocated=139190
FAIL LTP CASE clock_nanosleep04 : 0
RUN LTP CASE clock_settime01
frame-allocator-diagnostic: process-teardown pid=993 reclaimed_frames=511 before_free=56473 before_allocated=139734 after_free=56984 after_allocated=139223
frame-allocator-diagnostic: process-teardown pid=985 reclaimed_frames=508 before_free=56984 before_allocated=139223 after_free=57492 after_allocated=138715
frame-allocator-diagnostic: process-teardown pid=995 reclaimed_frames=1 before_free=56960 before_allocated=139247 after_free=56961 after_allocated=139246
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_settime01.c:42: TINFO: Testing variant: vDSO or syscall with libc spec
clock_settime01.c:76: TPASS: clock_settime(2): was able to advance time
clock_settime01.c:97: TPASS: clock_settime(2): was able to recede time
frame-allocator-diagnostic: process-teardown pid=998 reclaimed_frames=9 before_free=56678 before_allocated=139529 after_free=56687 after_allocated=139520
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_settime01.c:42: TINFO: Testing variant: syscall with old kernel spec
clock_settime01.c:76: TPASS: clock_settime(2): was able to advance time
clock_settime01.c:97: TPASS: clock_settime(2): was able to recede time
frame-allocator-diagnostic: process-teardown pid=1001 reclaimed_frames=9 before_free=56670 before_allocated=139537 after_free=56679 after_allocated=139528

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[558.167221 0:994 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=994 reclaimed_frames=266 before_free=56680 before_allocated=139527 after_free=56946 after_allocated=139261
frame-allocator-diagnostic: process-teardown pid=992 reclaimed_frames=13 before_free=56946 before_allocated=139261 after_free=56959 after_allocated=139248
FAIL LTP CASE clock_settime01 : 0
RUN LTP CASE clock_settime02
frame-allocator-diagnostic: process-teardown pid=1004 reclaimed_frames=511 before_free=56415 before_allocated=139792 after_free=56926 after_allocated=139281
frame-allocator-diagnostic: process-teardown pid=996 reclaimed_frames=508 before_free=56926 before_allocated=139281 after_free=57434 after_allocated=138773
frame-allocator-diagnostic: process-teardown pid=1006 reclaimed_frames=1 before_free=56902 before_allocated=139305 after_free=56903 after_allocated=139304
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_settime02.c:106: TINFO: Testing variant: syscall with old kernel spec
clock_settime02.c:150: TPASS: clock_settime(CLOCK_REALTIME): failed as expected: EFAULT (14)
clock_settime02.c:150: TPASS: clock_settime(CLOCK_REALTIME): failed as expected: EINVAL (22)
clock_settime02.c:150: TPASS: clock_settime(CLOCK_REALTIME): failed as expected: EINVAL (22)
clock_settime02.c:150: TPASS: clock_settime(CLOCK_REALTIME): failed as expected: EINVAL (22)
clock_settime02.c:150: TPASS: clock_settime(CLOCK_MONOTONIC): failed as expected: EINVAL (22)
clock_settime02.c:150: TPASS: clock_settime(INVALID/UNKNOWN CLOCK): failed as expected: EINVAL (22)
clock_settime02.c:150: TPASS: clock_settime(INVALID/UNKNOWN CLOCK): failed as expected: EINVAL (22)
clock_settime02.c:150: TPASS: clock_settime(CLOCK_MONOTONIC_COARSE): failed as expected: EINVAL (22)
clock_settime02.c:150: TPASS: clock_settime(CLOCK_MONOTONIC_RAW): failed as expected: EINVAL (22)
clock_settime02.c:150: TPASS: clock_settime(CLOCK_BOOTTIME): failed as expected: EINVAL (22)
clock_settime02.c:150: TPASS: clock_settime(CLOCK_PROCESS_CPUTIME_ID): failed as expected: EINVAL (22)
clock_settime02.c:150: TPASS: clock_settime(CLOCK_THREAD_CPUTIME_ID): failed as expected: EINVAL (22)
frame-allocator-diagnostic: process-teardown pid=1009 reclaimed_frames=9 before_free=56624 before_allocated=139583 after_free=56633 after_allocated=139574

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[562.544067 0:1005 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1005 reclaimed_frames=262 before_free=56634 before_allocated=139573 after_free=56896 after_allocated=139311
frame-allocator-diagnostic: process-teardown pid=1003 reclaimed_frames=13 before_free=56896 before_allocated=139311 after_free=56909 after_allocated=139298
FAIL LTP CASE clock_settime02 : 0
RUN LTP CASE clock_settime03
frame-allocator-diagnostic: process-teardown pid=1012 reclaimed_frames=511 before_free=56366 before_allocated=139841 after_free=56877 after_allocated=139330
frame-allocator-diagnostic: process-teardown pid=1007 reclaimed_frames=508 before_free=56877 before_allocated=139330 after_free=57385 after_allocated=138822
frame-allocator-diagnostic: process-teardown pid=1014 reclaimed_frames=1 before_free=56853 before_allocated=139354 after_free=56854 after_allocated=139353
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clock_settime03.c:35: TINFO: Testing variant: syscall with old kernel spec
clock_settime03.c:62: TCONF: syscall(107) __NR_timer_create not supported on your arch
frame-allocator-diagnostic: process-teardown pid=1017 reclaimed_frames=11 before_free=56573 before_allocated=139634 after_free=56584 after_allocated=139623

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[566.682760 0:1013 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1013 reclaimed_frames=262 before_free=56585 before_allocated=139622 after_free=56847 after_allocated=139360
frame-allocator-diagnostic: process-teardown pid=1011 reclaimed_frames=12 before_free=56847 before_allocated=139360 after_free=56859 after_allocated=139348
FAIL LTP CASE clock_settime03 : 32
RUN LTP CASE clone01
frame-allocator-diagnostic: process-teardown pid=1020 reclaimed_frames=511 before_free=56316 before_allocated=139891 after_free=56827 after_allocated=139380
frame-allocator-diagnostic: process-teardown pid=1015 reclaimed_frames=508 before_free=56827 before_allocated=139380 after_free=57335 after_allocated=138872
frame-allocator-diagnostic: process-teardown pid=1022 reclaimed_frames=1 before_free=56803 before_allocated=139404 after_free=56804 after_allocated=139403
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
frame-allocator-diagnostic: process-teardown pid=1026 reclaimed_frames=6 before_free=56513 before_allocated=139694 after_free=56519 after_allocated=139688
clone01.c:37: TPASS: clone returned 1026
clone01.c:43: TPASS: Child exited with 0
frame-allocator-diagnostic: process-teardown pid=1025 reclaimed_frames=10 before_free=56517 before_allocated=139690 after_free=56527 after_allocated=139680

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[570.853862 0:1021 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1021 reclaimed_frames=261 before_free=56528 before_allocated=139679 after_free=56789 after_allocated=139418
frame-allocator-diagnostic: process-teardown pid=1019 reclaimed_frames=12 before_free=56789 before_allocated=139418 after_free=56801 after_allocated=139406
FAIL LTP CASE clone01 : 0
RUN LTP CASE clone02
frame-allocator-diagnostic: process-teardown pid=1029 reclaimed_frames=511 before_free=56258 before_allocated=139949 after_free=56769 after_allocated=139438
frame-allocator-diagnostic: process-teardown pid=1023 reclaimed_frames=508 before_free=56769 before_allocated=139438 after_free=57277 after_allocated=138930
frame-allocator-diagnostic: process-teardown pid=1031 reclaimed_frames=1 before_free=56745 before_allocated=139462 after_free=56746 after_allocated=139461
clone02     1  TFAIL  :  clone02.c:139: clone() failed: TEST_ERRNO=ENOSYS(38): Function not implemented
frame-allocator-diagnostic: process-teardown pid=1033 reclaimed_frames=10 before_free=56461 before_allocated=139746 after_free=56471 after_allocated=139736
clone02     2  TPASS  :  Test Passed
[574.870676 0:1030 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1030 reclaimed_frames=266 before_free=56473 before_allocated=139734 after_free=56739 after_allocated=139468
frame-allocator-diagnostic: process-teardown pid=1028 reclaimed_frames=12 before_free=56739 before_allocated=139468 after_free=56751 after_allocated=139456
FAIL LTP CASE clone02 : 1
RUN LTP CASE clone03
frame-allocator-diagnostic: process-teardown pid=1035 reclaimed_frames=511 before_free=56208 before_allocated=139999 after_free=56719 after_allocated=139488
frame-allocator-diagnostic: process-teardown pid=1032 reclaimed_frames=508 before_free=56719 before_allocated=139488 after_free=57227 after_allocated=138980
frame-allocator-diagnostic: process-teardown pid=1037 reclaimed_frames=1 before_free=56695 before_allocated=139512 after_free=56696 after_allocated=139511
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
frame-allocator-diagnostic: process-teardown pid=1041 reclaimed_frames=7 before_free=56403 before_allocated=139804 after_free=56410 after_allocated=139797
clone03.c:38: TFAIL: pid(0) retval 1041 != 0: SUCCESS (0)
frame-allocator-diagnostic: process-teardown pid=1040 reclaimed_frames=10 before_free=56409 before_allocated=139798 after_free=56419 after_allocated=139788

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[578.896354 0:1036 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1036 reclaimed_frames=261 before_free=56420 before_allocated=139787 after_free=56681 after_allocated=139526
frame-allocator-diagnostic: process-teardown pid=1034 reclaimed_frames=12 before_free=56681 before_allocated=139526 after_free=56693 after_allocated=139514
FAIL LTP CASE clone03 : 0
RUN LTP CASE clone04
frame-allocator-diagnostic: process-teardown pid=1044 reclaimed_frames=511 before_free=56150 before_allocated=140057 after_free=56661 after_allocated=139546
frame-allocator-diagnostic: process-teardown pid=1038 reclaimed_frames=508 before_free=56661 before_allocated=139546 after_free=57169 after_allocated=139038
frame-allocator-diagnostic: process-teardown pid=1046 reclaimed_frames=1 before_free=56637 before_allocated=139570 after_free=56638 after_allocated=139569
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clone04.c:40: TPASS: NULL stack : EINVAL (22)
frame-allocator-diagnostic: process-teardown pid=1049 reclaimed_frames=9 before_free=56360 before_allocated=139847 after_free=56369 after_allocated=139838

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[582.911117 0:1045 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1045 reclaimed_frames=261 before_free=56370 before_allocated=139837 after_free=56631 after_allocated=139576
frame-allocator-diagnostic: process-teardown pid=1043 reclaimed_frames=12 before_free=56631 before_allocated=139576 after_free=56643 after_allocated=139564
FAIL LTP CASE clone04 : 0
RUN LTP CASE clone05
frame-allocator-diagnostic: process-teardown pid=1052 reclaimed_frames=511 before_free=56100 before_allocated=140107 after_free=56611 after_allocated=139596
frame-allocator-diagnostic: process-teardown pid=1047 reclaimed_frames=508 before_free=56611 before_allocated=139596 after_free=57119 after_allocated=139088
frame-allocator-diagnostic: process-teardown pid=1054 reclaimed_frames=1 before_free=56586 before_allocated=139621 after_free=56587 after_allocated=139620
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clone05.c:48: TFAIL: child_exited retval 0 != 1: SUCCESS (0)
frame-allocator-diagnostic: process-teardown pid=1058 reclaimed_frames=4 before_free=56297 before_allocated=139910 after_free=56301 after_allocated=139906
frame-allocator-diagnostic: process-teardown pid=1057 reclaimed_frames=9 before_free=56301 before_allocated=139906 after_free=56310 after_allocated=139897

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[587.201789 0:1053 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1053 reclaimed_frames=261 before_free=56311 before_allocated=139896 after_free=56572 after_allocated=139635
frame-allocator-diagnostic: process-teardown pid=1051 reclaimed_frames=13 before_free=56572 before_allocated=139635 after_free=56585 after_allocated=139622
FAIL LTP CASE clone05 : 0
RUN LTP CASE clone06
frame-allocator-diagnostic: process-teardown pid=1061 reclaimed_frames=511 before_free=56041 before_allocated=140166 after_free=56552 after_allocated=139655
frame-allocator-diagnostic: process-teardown pid=1055 reclaimed_frames=508 before_free=56552 before_allocated=139655 after_free=57060 after_allocated=139147
frame-allocator-diagnostic: process-teardown pid=1063 reclaimed_frames=1 before_free=56528 before_allocated=139679 after_free=56529 after_allocated=139678
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clone06.c:38: TPASS: The environment variables of the child and the parent are the same 
frame-allocator-diagnostic: process-teardown pid=1067 reclaimed_frames=6 before_free=56238 before_allocated=139969 after_free=56244 after_allocated=139963
tst_test.c:1449: TBROK: Test haven't reported results!
frame-allocator-diagnostic: process-teardown pid=1066 reclaimed_frames=9 before_free=56243 before_allocated=139964 after_free=56252 after_allocated=139955

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[591.355176 0:1062 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1062 reclaimed_frames=261 before_free=56253 before_allocated=139954 after_free=56514 after_allocated=139693
frame-allocator-diagnostic: process-teardown pid=1060 reclaimed_frames=13 before_free=56514 before_allocated=139693 after_free=56527 after_allocated=139680
FAIL LTP CASE clone06 : 2
RUN LTP CASE clone07
frame-allocator-diagnostic: process-teardown pid=1070 reclaimed_frames=511 before_free=55984 before_allocated=140223 after_free=56495 after_allocated=139712
frame-allocator-diagnostic: process-teardown pid=1064 reclaimed_frames=508 before_free=56495 before_allocated=139712 after_free=57003 after_allocated=139204
frame-allocator-diagnostic: process-teardown pid=1072 reclaimed_frames=1 before_free=56471 before_allocated=139736 after_free=56472 after_allocated=139735
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clone07.c:39: TBROK: waitpid(0,0x1ffffffc5c,0) failed: EINVAL (22)
frame-allocator-diagnostic: process-teardown pid=1075 reclaimed_frames=9 before_free=56178 before_allocated=140029 after_free=56187 after_allocated=140020
frame-allocator-diagnostic: process-teardown pid=1076 reclaimed_frames=8 before_free=56187 before_allocated=140020 after_free=56195 after_allocated=140012

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[595.338367 0:1071 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1071 reclaimed_frames=261 before_free=56196 before_allocated=140011 after_free=56457 after_allocated=139750
frame-allocator-diagnostic: process-teardown pid=1069 reclaimed_frames=12 before_free=56457 before_allocated=139750 after_free=56469 after_allocated=139738
FAIL LTP CASE clone07 : 2
RUN LTP CASE clone08
frame-allocator-diagnostic: process-teardown pid=1079 reclaimed_frames=511 before_free=55925 before_allocated=140282 after_free=56436 after_allocated=139771
frame-allocator-diagnostic: process-teardown pid=1073 reclaimed_frames=508 before_free=56436 before_allocated=139771 after_free=56944 after_allocated=139263
frame-allocator-diagnostic: process-teardown pid=1081 reclaimed_frames=1 before_free=56412 before_allocated=139795 after_free=56413 after_allocated=139794
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clone08.c:62: TINFO: running CLONE_PARENT
clone08.c:85: TBROK: CLONE_PARENT clone() failed: ENOSYS (38)
frame-allocator-diagnostic: process-teardown pid=1085 reclaimed_frames=8 before_free=56118 before_allocated=140089 after_free=56126 after_allocated=140081
tst_test.c:1464: TBROK: Test 0 haven't reported results!
frame-allocator-diagnostic: process-teardown pid=1084 reclaimed_frames=10 before_free=56126 before_allocated=140081 after_free=56136 after_allocated=140071

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[599.420097 0:1080 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1080 reclaimed_frames=261 before_free=56137 before_allocated=140070 after_free=56398 after_allocated=139809
frame-allocator-diagnostic: process-teardown pid=1078 reclaimed_frames=13 before_free=56398 before_allocated=139809 after_free=56411 after_allocated=139796
FAIL LTP CASE clone08 : 2
RUN LTP CASE clone09
frame-allocator-diagnostic: process-teardown pid=1088 reclaimed_frames=511 before_free=55868 before_allocated=140339 after_free=56379 after_allocated=139828
frame-allocator-diagnostic: process-teardown pid=1082 reclaimed_frames=508 before_free=56379 before_allocated=139828 after_free=56887 after_allocated=139320
frame-allocator-diagnostic: process-teardown pid=1090 reclaimed_frames=1 before_free=56355 before_allocated=139852 after_free=56356 after_allocated=139851
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clone09.c:59: TINFO: create clone in a new netns with 'CLONE_NEWNET' flag
clone09.c:50: TBROK: clone(CLONE_NEWNET) failed: ENOSYS (38)
frame-allocator-diagnostic: process-teardown pid=1093 reclaimed_frames=11 before_free=56076 before_allocated=140131 after_free=56087 after_allocated=140120

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[603.581006 0:1089 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1089 reclaimed_frames=261 before_free=56088 before_allocated=140119 after_free=56349 after_allocated=139858
frame-allocator-diagnostic: process-teardown pid=1087 reclaimed_frames=12 before_free=56349 before_allocated=139858 after_free=56361 after_allocated=139846
FAIL LTP CASE clone09 : 2
RUN LTP CASE clone301
frame-allocator-diagnostic: process-teardown pid=1096 reclaimed_frames=511 before_free=55818 before_allocated=140389 after_free=56329 after_allocated=139878
frame-allocator-diagnostic: process-teardown pid=1091 reclaimed_frames=508 before_free=56329 before_allocated=139878 after_free=56837 after_allocated=139370
frame-allocator-diagnostic: process-teardown pid=1098 reclaimed_frames=1 before_free=56305 before_allocated=139902 after_free=56306 after_allocated=139901
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
../../../../include/lapi/sched.h:76: TCONF: syscall(435) __NR_clone3 not supported on your arch
frame-allocator-diagnostic: process-teardown pid=1101 reclaimed_frames=12 before_free=56023 before_allocated=140184 after_free=56035 after_allocated=140172

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[607.639977 0:1097 axfs::fops:297] [AxError::NotADirectory]
[607.640931 0:1097 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1097 reclaimed_frames=263 before_free=56036 before_allocated=140171 after_free=56299 after_allocated=139908
frame-allocator-diagnostic: process-teardown pid=1095 reclaimed_frames=12 before_free=56299 before_allocated=139908 after_free=56311 after_allocated=139896
FAIL LTP CASE clone301 : 32
RUN LTP CASE clone302
frame-allocator-diagnostic: process-teardown pid=1104 reclaimed_frames=511 before_free=55768 before_allocated=140439 after_free=56279 after_allocated=139928
frame-allocator-diagnostic: process-teardown pid=1099 reclaimed_frames=508 before_free=56279 before_allocated=139928 after_free=56787 after_allocated=139420
frame-allocator-diagnostic: process-teardown pid=1106 reclaimed_frames=1 before_free=56255 before_allocated=139952 after_free=56256 after_allocated=139951
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clone302.c:63: TPASS: sizeof(struct clone_args_minimal) == 64 (64)
../../../../include/lapi/sched.h:76: TCONF: syscall(435) __NR_clone3 not supported on your arch
frame-allocator-diagnostic: process-teardown pid=1109 reclaimed_frames=11 before_free=55974 before_allocated=140233 after_free=55985 after_allocated=140222

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[611.751882 0:1105 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1105 reclaimed_frames=263 before_free=55986 before_allocated=140221 after_free=56249 after_allocated=139958
frame-allocator-diagnostic: process-teardown pid=1103 reclaimed_frames=12 before_free=56249 before_allocated=139958 after_free=56261 after_allocated=139946
FAIL LTP CASE clone302 : 32
RUN LTP CASE clone303
frame-allocator-diagnostic: process-teardown pid=1112 reclaimed_frames=511 before_free=55718 before_allocated=140489 after_free=56229 after_allocated=139978
frame-allocator-diagnostic: process-teardown pid=1107 reclaimed_frames=508 before_free=56229 before_allocated=139978 after_free=56737 after_allocated=139470
frame-allocator-diagnostic: process-teardown pid=1114 reclaimed_frames=1 before_free=56204 before_allocated=140003 after_free=56205 after_allocated=140002
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_cgroup.c:712: TINFO: Could not mount V2 CGroups on /tmp/cgroup_unified: ENODEV (19)
tst_cgroup.c:880: TCONF: V2 'base' controller required, but it's mounted on V1

Summary:
passed   0
failed   0
broken   0
skipped  1
warnings 0
[615.746052 0:1113 axfs::fops:297] [AxError::NotADirectory]
[615.747016 0:1113 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1113 reclaimed_frames=264 before_free=55942 before_allocated=140265 after_free=56206 after_allocated=140001
frame-allocator-diagnostic: process-teardown pid=1111 reclaimed_frames=13 before_free=56206 before_allocated=140001 after_free=56219 after_allocated=139988
FAIL LTP CASE clone303 : 32
RUN LTP CASE close01
frame-allocator-diagnostic: process-teardown pid=1117 reclaimed_frames=511 before_free=55675 before_allocated=140532 after_free=56186 after_allocated=140021
frame-allocator-diagnostic: process-teardown pid=1115 reclaimed_frames=508 before_free=56186 before_allocated=140021 after_free=56694 after_allocated=139513
frame-allocator-diagnostic: process-teardown pid=1119 reclaimed_frames=1 before_free=56162 before_allocated=140045 after_free=56163 after_allocated=140044
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
close01.c:50: TPASS: close a file fd passed
close01.c:50: TPASS: close a pipe fd passed
close01.c:50: TPASS: close a socket fd passed
frame-allocator-diagnostic: process-teardown pid=1122 reclaimed_frames=12 before_free=55878 before_allocated=140329 after_free=55890 after_allocated=140317

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[619.969086 0:1118 axfs::fops:297] [AxError::NotADirectory]
[619.970072 0:1118 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1118 reclaimed_frames=265 before_free=55891 before_allocated=140316 after_free=56156 after_allocated=140051
frame-allocator-diagnostic: process-teardown pid=1116 reclaimed_frames=13 before_free=56156 before_allocated=140051 after_free=56169 after_allocated=140038
FAIL LTP CASE close01 : 0
RUN LTP CASE close02
frame-allocator-diagnostic: process-teardown pid=1125 reclaimed_frames=511 before_free=55625 before_allocated=140582 after_free=56136 after_allocated=140071
frame-allocator-diagnostic: process-teardown pid=1120 reclaimed_frames=508 before_free=56136 before_allocated=140071 after_free=56644 after_allocated=139563
frame-allocator-diagnostic: process-teardown pid=1127 reclaimed_frames=1 before_free=56112 before_allocated=140095 after_free=56113 after_allocated=140094
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
close02.c:20: TPASS: close(-1) : EBADF (9)
frame-allocator-diagnostic: process-teardown pid=1130 reclaimed_frames=11 before_free=55832 before_allocated=140375 after_free=55843 after_allocated=140364

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[624.097896 0:1126 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1126 reclaimed_frames=262 before_free=55844 before_allocated=140363 after_free=56106 after_allocated=140101
frame-allocator-diagnostic: process-teardown pid=1124 reclaimed_frames=13 before_free=56106 before_allocated=140101 after_free=56119 after_allocated=140088
FAIL LTP CASE close02 : 0
RUN LTP CASE close_range01
frame-allocator-diagnostic: process-teardown pid=1133 reclaimed_frames=511 before_free=55576 before_allocated=140631 after_free=56087 after_allocated=140120
frame-allocator-diagnostic: process-teardown pid=1128 reclaimed_frames=508 before_free=56087 before_allocated=140120 after_free=56595 after_allocated=139612
frame-allocator-diagnostic: process-teardown pid=1135 reclaimed_frames=1 before_free=56063 before_allocated=140144 after_free=56064 after_allocated=140143
tst_device.c:293: TWARN: Failed to create test_dev.img: ENOSPC (28)
tst_device.c:354: TBROK: Failed to acquire device

HINT: You _MAY_ be missing kernel fixes:

https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=fec8a6a69103

Summary:
passed   0
failed   0
broken   1
skipped  0
warnings 1
[629.896556 0:1134 axfs::root:433] [AxError::IsADirectory]
[629.897272 0:1134 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1134 reclaimed_frames=264 before_free=55801 before_allocated=140406 after_free=56065 after_allocated=140142
frame-allocator-diagnostic: process-teardown pid=1136 reclaimed_frames=508 before_free=56065 before_allocated=140142 after_free=56573 after_allocated=139634
frame-allocator-diagnostic: process-teardown pid=1132 reclaimed_frames=12 before_free=56573 before_allocated=139634 after_free=56585 after_allocated=139622
FAIL LTP CASE close_range01 : 6
RUN LTP CASE close_range02
frame-allocator-diagnostic: process-teardown pid=1138 reclaimed_frames=511 before_free=56042 before_allocated=140165 after_free=56553 after_allocated=139654
frame-allocator-diagnostic: process-teardown pid=1140 reclaimed_frames=1 before_free=56021 before_allocated=140186 after_free=56022 after_allocated=140185
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
../../../../include/lapi/close_range.h:25: TCONF: syscall(436) __NR_close_range not supported on your arch
frame-allocator-diagnostic: process-teardown pid=1143 reclaimed_frames=12 before_free=55739 before_allocated=140468 after_free=55751 after_allocated=140456

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[633.933462 0:1139 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1139 reclaimed_frames=263 before_free=55752 before_allocated=140455 after_free=56015 after_allocated=140192
frame-allocator-diagnostic: process-teardown pid=1137 reclaimed_frames=12 before_free=56015 before_allocated=140192 after_free=56027 after_allocated=140180
FAIL LTP CASE close_range02 : 32
RUN LTP CASE cmdlib.sh
SKIP LTP CASE cmdlib.sh : LTP shell helper library is not a standalone test
frame-allocator-diagnostic: process-teardown pid=1145 reclaimed_frames=12 before_free=56005 before_allocated=140202 after_free=56017 after_allocated=140190
FAIL LTP CASE cmdlib.sh : 32
RUN LTP CASE cn_pec.sh
frame-allocator-diagnostic: process-teardown pid=1147 reclaimed_frames=511 before_free=55474 before_allocated=140733 after_free=55985 after_allocated=140222
frame-allocator-diagnostic: process-teardown pid=1141 reclaimed_frames=508 before_free=55985 before_allocated=140222 after_free=56493 after_allocated=139714
frame-allocator-diagnostic: process-teardown pid=1149 reclaimed_frames=1 before_free=55961 before_allocated=140246 after_free=55962 after_allocated=140245
sh: ambiguous redirect
sh: 1: unknown operand
sh: syntax error at 'done'
frame-allocator-diagnostic: process-teardown pid=1148 reclaimed_frames=518 before_free=55445 before_allocated=140762 after_free=55963 after_allocated=140244
frame-allocator-diagnostic: process-teardown pid=1146 reclaimed_frames=12 before_free=55963 before_allocated=140244 after_free=55975 after_allocated=140232
FAIL LTP CASE cn_pec.sh : 1
RUN LTP CASE confstr01
frame-allocator-diagnostic: process-teardown pid=1152 reclaimed_frames=511 before_free=55432 before_allocated=140775 after_free=55943 after_allocated=140264
frame-allocator-diagnostic: process-teardown pid=1150 reclaimed_frames=508 before_free=55943 before_allocated=140264 after_free=56451 after_allocated=139756
frame-allocator-diagnostic: process-teardown pid=1154 reclaimed_frames=1 before_free=55919 before_allocated=140288 after_free=55920 after_allocated=140287
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 14
confstr01.c:75: TPASS: confstr PATH = '/bin:/usr/bin'
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 0
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 0
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr POSIX_V7_ILP32_OFF32_CFLAGS = ''
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr POSIX_V7_ILP32_OFF32_LDFLAGS = ''
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr POSIX_V7_ILP32_OFF32_LIBS = ''
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr POSIX_V7_ILP32_OFFBIG_CFLAGS = ''
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr POSIX_V7_ILP32_OFFBIG_LDFLAGS = ''
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr POSIX_V7_ILP32_OFFBIG_LIBS = ''
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr POSIX_V7_LP64_OFF64_CFLAGS = ''
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr POSIX_V7_LP64_OFF64_LDFLAGS = ''
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr POSIX_V7_LP64_OFF64_LIBS = ''
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr POSIX_V7_LPBIG_OFFBIG_CFLAGS = ''
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr POSIX_V7_LPBIG_OFFBIG_LDFLAGS = ''
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr POSIX_V7_LPBIG_OFFBIG_LIBS = ''
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr POSIX_V7_THREADS_CFLAGS = ''
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr POSIX_V7_THREADS_LDFLAGS = ''
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr POSIX_V7_WIDTH_RESTRICTED_ENVS = ''
confstr01.c:61: TPASS: confstr(test_cases[i].value, NULL, (size_t)0) returned 1
confstr01.c:75: TPASS: confstr V7_ENV = ''
frame-allocator-diagnostic: process-teardown pid=1157 reclaimed_frames=12 before_free=55638 before_allocated=140569 after_free=55650 after_allocated=140557

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[639.515713 0:1153 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1153 reclaimed_frames=262 before_free=55651 before_allocated=140556 after_free=55913 after_allocated=140294
frame-allocator-diagnostic: process-teardown pid=1151 reclaimed_frames=12 before_free=55913 before_allocated=140294 after_free=55925 after_allocated=140282
FAIL LTP CASE confstr01 : 0
RUN LTP CASE connect01
frame-allocator-diagnostic: process-teardown pid=1160 reclaimed_frames=511 before_free=55381 before_allocated=140826 after_free=55892 after_allocated=140315
frame-allocator-diagnostic: process-teardown pid=1155 reclaimed_frames=508 before_free=55892 before_allocated=140315 after_free=56400 after_allocated=139807
frame-allocator-diagnostic: process-teardown pid=1162 reclaimed_frames=1 before_free=55868 before_allocated=140339 after_free=55869 after_allocated=140338
connect01    1  TPASS  :  bad file descriptor successful
connect01    2  TPASS  :  invalid socket buffer successful
connect01    3  TPASS  :  invalid salen successful
connect01    4  TPASS  :  invalid socket successful
[643.813618 0:1161 axnet::smoltcp_impl::tcp:197] [AxError::ConnectionRefused] socket connect() failed
[643.814233 0:1161 arceos_posix_api::imp::net:422] sys_connect => Err(ECONNREFUSED)
connect01    5  TBROK  :  connect01.c:226: connect(3, 0.0.0.0:49170, 16) failed: errno=ECONNREFUSED(111): Connection refused
connect01    6  TBROK  :  connect01.c:226: Remaining cases broken
frame-allocator-diagnostic: process-teardown pid=1161 reclaimed_frames=8 before_free=55589 before_allocated=140618 after_free=55597 after_allocated=140610
frame-allocator-diagnostic: process-teardown pid=1164 reclaimed_frames=265 before_free=55597 before_allocated=140610 after_free=55862 after_allocated=140345
frame-allocator-diagnostic: process-teardown pid=1159 reclaimed_frames=13 before_free=55862 before_allocated=140345 after_free=55875 after_allocated=140332
FAIL LTP CASE connect01 : 2
RUN LTP CASE connect02
frame-allocator-diagnostic: process-teardown pid=1166 reclaimed_frames=511 before_free=55332 before_allocated=140875 after_free=55843 after_allocated=140364
frame-allocator-diagnostic: process-teardown pid=1163 reclaimed_frames=508 before_free=55843 before_allocated=140364 after_free=56351 after_allocated=139856
frame-allocator-diagnostic: process-teardown pid=1168 reclaimed_frames=1 before_free=55819 before_allocated=140388 after_free=55820 after_allocated=140387
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
connect02.c:53: TCONF: socket(10, 1, 6) failed: EAFNOSUPPORT (97)
frame-allocator-diagnostic: process-teardown pid=1171 reclaimed_frames=12 before_free=55534 before_allocated=140673 after_free=55546 after_allocated=140661

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[648.145976 0:1167 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1167 reclaimed_frames=266 before_free=55547 before_allocated=140660 after_free=55813 after_allocated=140394
frame-allocator-diagnostic: process-teardown pid=1165 reclaimed_frames=12 before_free=55813 before_allocated=140394 after_free=55825 after_allocated=140382
FAIL LTP CASE connect02 : 32
RUN LTP CASE copy_file_range01
frame-allocator-diagnostic: process-teardown pid=1174 reclaimed_frames=511 before_free=55282 before_allocated=140925 after_free=55793 after_allocated=140414
frame-allocator-diagnostic: process-teardown pid=1169 reclaimed_frames=508 before_free=55793 before_allocated=140414 after_free=56301 after_allocated=139906
frame-allocator-diagnostic: process-teardown pid=1176 reclaimed_frames=1 before_free=55769 before_allocated=140438 after_free=55770 after_allocated=140437
tst_device.c:293: TWARN: Failed to create test_dev.img: ENOSPC (28)
tst_device.c:354: TBROK: Failed to acquire device

Summary:
passed   0
failed   0
broken   1
skipped  0
warnings 1
[653.952786 0:1175 axfs::root:433] [AxError::IsADirectory]
[653.953697 0:1175 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1175 reclaimed_frames=264 before_free=55507 before_allocated=140700 after_free=55771 after_allocated=140436
frame-allocator-diagnostic: process-teardown pid=1177 reclaimed_frames=508 before_free=55771 before_allocated=140436 after_free=56279 after_allocated=139928
frame-allocator-diagnostic: process-teardown pid=1173 reclaimed_frames=12 before_free=56279 before_allocated=139928 after_free=56291 after_allocated=139916
FAIL LTP CASE copy_file_range01 : 6
RUN LTP CASE copy_file_range02
frame-allocator-diagnostic: process-teardown pid=1179 reclaimed_frames=511 before_free=55747 before_allocated=140460 after_free=56258 after_allocated=139949
frame-allocator-diagnostic: process-teardown pid=1181 reclaimed_frames=1 before_free=55726 before_allocated=140481 after_free=55727 after_allocated=140480
tst_device.c:293: TWARN: Failed to create test_dev.img: ENOSPC (28)
tst_device.c:354: TBROK: Failed to acquire device

Summary:
passed   0
failed   0
broken   1
skipped  0
warnings 1
[659.733575 0:1180 axfs::root:433] [AxError::IsADirectory]
[659.734315 0:1180 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1180 reclaimed_frames=264 before_free=55464 before_allocated=140743 after_free=55728 after_allocated=140479
frame-allocator-diagnostic: process-teardown pid=1182 reclaimed_frames=508 before_free=55728 before_allocated=140479 after_free=56236 after_allocated=139971
frame-allocator-diagnostic: process-teardown pid=1178 reclaimed_frames=13 before_free=56236 before_allocated=139971 after_free=56249 after_allocated=139958
FAIL LTP CASE copy_file_range02 : 6
RUN LTP CASE copy_file_range03
frame-allocator-diagnostic: process-teardown pid=1184 reclaimed_frames=511 before_free=55706 before_allocated=140501 after_free=56217 after_allocated=139990
frame-allocator-diagnostic: process-teardown pid=1186 reclaimed_frames=1 before_free=55685 before_allocated=140522 after_free=55686 after_allocated=140521
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
copy_file_range.h:36: TINFO: Testing libc copy_file_range()
copy_file_range03.c:42: TBROK: copy_file_range unexpectedly failed: ENOSYS (38)
frame-allocator-diagnostic: process-teardown pid=1189 reclaimed_frames=12 before_free=55404 before_allocated=140803 after_free=55416 after_allocated=140791

Summary:
passed   0
failed   0
broken   0
skipped  0
warnings 0
[665.307509 0:1185 axfs::fops:297] [AxError::NotADirectory]
[665.308153 0:1185 axfs::fops:297] [AxError::NotADirectory]
[665.308886 0:1185 axfs::root:433] [AxError::IsADirectory]
frame-allocator-diagnostic: process-teardown pid=1185 reclaimed_frames=262 before_free=55417 before_allocated=140790 after_free=55679 after_allocated=140528
frame-allocator-diagnostic: process-teardown pid=1183 reclaimed_frames=12 before_free=55679 before_allocated=140528 after_free=55691 after_allocated=140516
FAIL LTP CASE copy_file_range03 : 2
RUN LTP CASE cp_tests.sh
frame-allocator-diagnostic: process-teardown pid=1192 reclaimed_frames=511 before_free=55148 before_allocated=141059 after_free=55659 after_allocated=140548
frame-allocator-diagnostic: process-teardown pid=1194 reclaimed_frames=1 before_free=55127 before_allocated=141080 after_free=55128 after_allocated=141079
sh: tst_test.sh: No such file or directory
frame-allocator-diagnostic: process-teardown pid=1187 reclaimed_frames=508 before_free=54596 before_allocated=141611 after_free=55104 after_allocated=141103
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=1196 reclaimed_frames=8 before_free=55103 before_allocated=141104 after_free=55111 after_allocated=141096
frame-allocator-diagnostic: process-teardown pid=1193 reclaimed_frames=518 before_free=55111 before_allocated=141096 after_free=55629 after_allocated=140578
frame-allocator-diagnostic: process-teardown pid=1191 reclaimed_frames=12 before_free=55629 before_allocated=140578 after_free=55641 after_allocated=140566
FAIL LTP CASE cp_tests.sh : 2
RUN LTP CASE cpio_tests.sh
frame-allocator-diagnostic: process-teardown pid=1198 reclaimed_frames=511 before_free=55098 before_allocated=141109 after_free=55609 after_allocated=140598
frame-allocator-diagnostic: process-teardown pid=1195 reclaimed_frames=508 before_free=55084 before_allocated=141123 after_free=55592 after_allocated=140615
frame-allocator-diagnostic: process-teardown pid=1200 reclaimed_frames=1 before_free=55585 before_allocated=140622 after_free=55586 after_allocated=140621
sh: tst_test.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=1202 reclaimed_frames=8 before_free=55056 before_allocated=141151 after_free=55064 after_allocated=141143
frame-allocator-diagnostic: process-teardown pid=1199 reclaimed_frames=515 before_free=55064 before_allocated=141143 after_free=55579 after_allocated=140628
frame-allocator-diagnostic: process-teardown pid=1197 reclaimed_frames=12 before_free=55579 before_allocated=140628 after_free=55591 after_allocated=140616
FAIL LTP CASE cpio_tests.sh : 2
RUN LTP CASE cpuacct.sh
frame-allocator-diagnostic: process-teardown pid=1204 reclaimed_frames=511 before_free=55047 before_allocated=141160 after_free=55558 after_allocated=140649
frame-allocator-diagnostic: process-teardown pid=1201 reclaimed_frames=508 before_free=55033 before_allocated=141174 after_free=55541 after_allocated=140666
frame-allocator-diagnostic: process-teardown pid=1206 reclaimed_frames=1 before_free=55534 before_allocated=140673 after_free=55535 after_allocated=140672
sh: tst_test.sh: No such file or directory
sh: can't execute 'tst_run': Exec format error
frame-allocator-diagnostic: process-teardown pid=1208 reclaimed_frames=8 before_free=54993 before_allocated=141214 after_free=55001 after_allocated=141206
frame-allocator-diagnostic: process-teardown pid=1205 reclaimed_frames=526 before_free=55001 before_allocated=141206 after_free=55527 after_allocated=140680
frame-allocator-diagnostic: process-teardown pid=1203 reclaimed_frames=13 before_free=55527 before_allocated=140680 after_free=55540 after_allocated=140667
FAIL LTP CASE cpuacct.sh : 2
RUN LTP CASE cpuacct_task
frame-allocator-diagnostic: process-teardown pid=1210 reclaimed_frames=511 before_free=54997 before_allocated=141210 after_free=55508 after_allocated=140699
frame-allocator-diagnostic: process-teardown pid=1207 reclaimed_frames=508 before_free=55508 before_allocated=140699 after_free=56016 after_allocated=140191
frame-allocator-diagnostic: process-teardown pid=1212 reclaimed_frames=1 before_free=55484 before_allocated=140723 after_free=55485 after_allocated=140722
Usage: ltp/testcases/bin/cpuacct_task /cgroup/.../tasks
frame-allocator-diagnostic: process-teardown pid=1211 reclaimed_frames=184 before_free=55302 before_allocated=140905 after_free=55486 after_allocated=140721
frame-allocator-diagnostic: process-teardown pid=1209 reclaimed_frames=12 before_free=55486 before_allocated=140721 after_free=55498 after_allocated=140709
FAIL LTP CASE cpuacct_task : 1
RUN LTP CASE cpuctl_def_task01
frame-allocator-diagnostic: process-teardown pid=1215 reclaimed_frames=511 before_free=54954 before_allocated=141253 after_free=55465 after_allocated=140742
frame-allocator-diagnostic: process-teardown pid=1213 reclaimed_frames=508 before_free=55465 before_allocated=140742 after_free=55973 after_allocated=140234
frame-allocator-diagnostic: process-teardown pid=1217 reclaimed_frames=1 before_free=55441 before_allocated=140766 after_free=55442 after_allocated=140765
cpu_controller_tests    1  TBROK  :  cpuctl_def_task01.c:120: Invalid input parameters
cpu_controller_tests    2  TBROK  :  cpuctl_def_task01.c:120: Remaining cases broken
frame-allocator-diagnostic: process-teardown pid=1216 reclaimed_frames=267 before_free=55176 before_allocated=141031 after_free=55443 after_allocated=140764
frame-allocator-diagnostic: process-teardown pid=1214 reclaimed_frames=13 before_free=55443 before_allocated=140764 after_free=55456 after_allocated=140751
FAIL LTP CASE cpuctl_def_task01 : 2
RUN LTP CASE cpuctl_def_task02
frame-allocator-diagnostic: process-teardown pid=1220 reclaimed_frames=511 before_free=54913 before_allocated=141294 after_free=55424 after_allocated=140783
frame-allocator-diagnostic: process-teardown pid=1218 reclaimed_frames=508 before_free=55424 before_allocated=140783 after_free=55932 after_allocated=140275
frame-allocator-diagnostic: process-teardown pid=1222 reclaimed_frames=1 before_free=55400 before_allocated=140807 after_free=55401 after_allocated=140806
cpu_controller_test04    1  TBROK  :  cpuctl_def_task02.c:140: Invalid test number passed
cpu_controller_test04    2  TBROK  :  cpuctl_def_task02.c:140: Remaining cases broken
frame-allocator-diagnostic: process-teardown pid=1221 reclaimed_frames=266 before_free=55136 before_allocated=141071 after_free=55402 after_allocated=140805
frame-allocator-diagnostic: process-teardown pid=1219 reclaimed_frames=12 before_free=55402 before_allocated=140805 after_free=55414 after_allocated=140793
FAIL LTP CASE cpuctl_def_task02 : 2
RUN LTP CASE cpuctl_def_task03
frame-allocator-diagnostic: process-teardown pid=1225 reclaimed_frames=511 before_free=54871 before_allocated=141336 after_free=55382 after_allocated=140825
frame-allocator-diagnostic: process-teardown pid=1223 reclaimed_frames=508 before_free=55382 before_allocated=140825 after_free=55890 after_allocated=140317
frame-allocator-diagnostic: process-teardown pid=1227 reclaimed_frames=1 before_free=55358 before_allocated=140849 after_free=55359 after_allocated=140848
cpu_controller_test06    1  TBROK  :  cpuctl_def_task03.c:136: Invalid test number passed
cpu_controller_test06    2  TBROK  :  cpuctl_def_task03.c:136: Remaining cases broken
frame-allocator-diagnostic: process-teardown pid=1226 reclaimed_frames=266 before_free=55094 before_allocated=141113 after_free=55360 after_allocated=140847
frame-allocator-diagnostic: process-teardown pid=1224 reclaimed_frames=12 before_free=55360 before_allocated=140847 after_free=55372 after_allocated=140835
FAIL LTP CASE cpuctl_def_task03 : 2
RUN LTP CASE cpuctl_def_task04
frame-allocator-diagnostic: process-teardown pid=1230 reclaimed_frames=511 before_free=54829 before_allocated=141378 after_free=55340 after_allocated=140867
frame-allocator-diagnostic: process-teardown pid=1228 reclaimed_frames=508 before_free=55340 before_allocated=140867 after_free=55848 after_allocated=140359
frame-allocator-diagnostic: process-teardown pid=1232 reclaimed_frames=1 before_free=55316 before_allocated=140891 after_free=55317 after_allocated=140890
cpu_controller_test06    1  TBROK  :  cpuctl_def_task04.c:139: Invalid test number passed
cpu_controller_test06    2  TBROK  :  cpuctl_def_task04.c:139: Remaining cases broken
frame-allocator-diagnostic: process-teardown pid=1231 reclaimed_frames=266 before_free=55052 before_allocated=141155 after_free=55318 after_allocated=140889
frame-allocator-diagnostic: process-teardown pid=1229 reclaimed_frames=12 before_free=55318 before_allocated=140889 after_free=55330 after_allocated=140877
FAIL LTP CASE cpuctl_def_task04 : 2
RUN LTP CASE cpuctl_fj_cpu-hog
frame-allocator-diagnostic: process-teardown pid=1235 reclaimed_frames=511 before_free=54787 before_allocated=141420 after_free=55298 after_allocated=140909
frame-allocator-diagnostic: process-teardown pid=1233 reclaimed_frames=508 before_free=55298 before_allocated=140909 after_free=55806 after_allocated=140401
frame-allocator-diagnostic: process-teardown pid=1237 reclaimed_frames=1 before_free=55274 before_allocated=140933 after_free=55275 after_allocated=140932
cpuctl_fj_cpu-hog: sigsuspend(): Function not implemented
frame-allocator-diagnostic: process-teardown pid=1236 reclaimed_frames=184 before_free=55092 before_allocated=141115 after_free=55276 after_allocated=140931
frame-allocator-diagnostic: process-teardown pid=1234 reclaimed_frames=12 before_free=55276 before_allocated=140931 after_free=55288 after_allocated=140919
FAIL LTP CASE cpuctl_fj_cpu-hog : 1
RUN LTP CASE cpuctl_fj_simple_echo
frame-allocator-diagnostic: process-teardown pid=1240 reclaimed_frames=511 before_free=54745 before_allocated=141462 after_free=55256 after_allocated=140951
frame-allocator-diagnostic: process-teardown pid=1238 reclaimed_frames=508 before_free=54731 before_allocated=141476 after_free=55239 after_allocated=140968
frame-allocator-diagnostic: process-teardown pid=1242 reclaimed_frames=1 before_free=55232 before_allocated=140975 after_free=55233 after_allocated=140974
usage: cpuctl_fj_simple_echo STRING [ostream]
frame-allocator-diagnostic: process-teardown pid=1241 reclaimed_frames=184 before_free=55050 before_allocated=141157 after_free=55234 after_allocated=140973
frame-allocator-diagnostic: process-teardown pid=1239 reclaimed_frames=12 before_free=55234 before_allocated=140973 after_free=55246 after_allocated=140961
FAIL LTP CASE cpuctl_fj_simple_echo : 1
RUN LTP CASE cpuctl_latency_check_task
frame-allocator-diagnostic: process-teardown pid=1245 reclaimed_frames=511 before_free=54702 before_allocated=141505 after_free=55213 after_allocated=140994
frame-allocator-diagnostic: process-teardown pid=1243 reclaimed_frames=508 before_free=54688 before_allocated=141519 after_free=55196 after_allocated=141011
frame-allocator-diagnostic: process-teardown pid=1247 reclaimed_frames=1 before_free=55189 before_allocated=141018 after_free=55190 after_allocated=141017
Invalid #args received from script. Exiting test..
frame-allocator-diagnostic: process-teardown pid=1246 reclaimed_frames=187 before_free=55004 before_allocated=141203 after_free=55191 after_allocated=141016
frame-allocator-diagnostic: process-teardown pid=1244 reclaimed_frames=13 before_free=55191 before_allocated=141016 after_free=55204 after_allocated=141003
FAIL LTP CASE cpuctl_latency_check_task : 1
RUN LTP CASE cpuctl_latency_test
frame-allocator-diagnostic: process-teardown pid=1250 reclaimed_frames=511 before_free=54661 before_allocated=141546 after_free=55172 after_allocated=141035
frame-allocator-diagnostic: process-teardown pid=1248 reclaimed_frames=508 before_free=54647 before_allocated=141560 after_free=55155 after_allocated=141052
frame-allocator-diagnostic: process-teardown pid=1252 reclaimed_frames=1 before_free=55148 before_allocated=141059 after_free=55149 after_allocated=141058
cpuctl_latency_test: TBROK	 Invalid #args received from script The test will run without any cpu load
frame-allocator-diagnostic: process-teardown pid=1251 reclaimed_frames=187 before_free=54963 before_allocated=141244 after_free=55150 after_allocated=141057
frame-allocator-diagnostic: process-teardown pid=1249 reclaimed_frames=12 before_free=55150 before_allocated=141057 after_free=55162 after_allocated=141045
FAIL LTP CASE cpuctl_latency_test : 22
RUN LTP CASE cpuctl_test01
frame-allocator-diagnostic: process-teardown pid=1255 reclaimed_frames=511 before_free=54619 before_allocated=141588 after_free=55130 after_allocated=141077
frame-allocator-diagnostic: process-teardown pid=1253 reclaimed_frames=508 before_free=55130 before_allocated=141077 after_free=55638 after_allocated=140569
frame-allocator-diagnostic: process-teardown pid=1257 reclaimed_frames=1 before_free=55106 before_allocated=141101 after_free=55107 after_allocated=141100
cpuctl_test01    1  TBROK  :  cpuctl_test01.c:120: Invalid input parameters
cpuctl_test01    2  TBROK  :  cpuctl_test01.c:120: Remaining cases broken
frame-allocator-diagnostic: process-teardown pid=1256 reclaimed_frames=266 before_free=54842 before_allocated=141365 after_free=55108 after_allocated=141099
frame-allocator-diagnostic: process-teardown pid=1254 reclaimed_frames=12 before_free=55108 before_allocated=141099 after_free=55120 after_allocated=141087
FAIL LTP CASE cpuctl_test01 : 2
RUN LTP CASE cpuctl_test02
frame-allocator-diagnostic: process-teardown pid=1260 reclaimed_frames=511 before_free=54577 before_allocated=141630 after_free=55088 after_allocated=141119
frame-allocator-diagnostic: process-teardown pid=1258 reclaimed_frames=508 before_free=55088 before_allocated=141119 after_free=55596 after_allocated=140611
frame-allocator-diagnostic: process-teardown pid=1262 reclaimed_frames=1 before_free=55063 before_allocated=141144 after_free=55064 after_allocated=141143
cpuctl_test02    1  TBROK  :  cpuctl_test02.c:144: Invalid test number passed
cpuctl_test02    2  TBROK  :  cpuctl_test02.c:144: Remaining cases broken
frame-allocator-diagnostic: process-teardown pid=1261 reclaimed_frames=266 before_free=54799 before_allocated=141408 after_free=55065 after_allocated=141142
frame-allocator-diagnostic: process-teardown pid=1259 reclaimed_frames=13 before_free=55065 before_allocated=141142 after_free=55078 after_allocated=141129
FAIL LTP CASE cpuctl_test02 : 2
RUN LTP CASE cpuctl_test03
frame-allocator-diagnostic: process-teardown pid=1265 reclaimed_frames=511 before_free=54535 before_allocated=141672 after_free=55046 after_allocated=141161
frame-allocator-diagnostic: process-teardown pid=1263 reclaimed_frames=508 before_free=55046 before_allocated=141161 after_free=55554 after_allocated=140653
frame-allocator-diagnostic: process-teardown pid=1267 reclaimed_frames=1 before_free=55021 before_allocated=141186 after_free=55022 after_allocated=141185
cpuctl_test03    1  TBROK  :  cpuctl_test03.c:139: Invalid test number passed
cpuctl_test03    2  TBROK  :  cpuctl_test03.c:139: Remaining cases broken
frame-allocator-diagnostic: process-teardown pid=1266 reclaimed_frames=266 before_free=54757 before_allocated=141450 after_free=55023 after_allocated=141184
frame-allocator-diagnostic: process-teardown pid=1264 reclaimed_frames=13 before_free=55023 before_allocated=141184 after_free=55036 after_allocated=141171
FAIL LTP CASE cpuctl_test03 : 2
RUN LTP CASE cpuctl_test04
frame-allocator-diagnostic: process-teardown pid=1270 reclaimed_frames=511 before_free=54493 before_allocated=141714 after_free=55004 after_allocated=141203
frame-allocator-diagnostic: process-teardown pid=1268 reclaimed_frames=508 before_free=55004 before_allocated=141203 after_free=55512 after_allocated=140695
frame-allocator-diagnostic: process-teardown pid=1272 reclaimed_frames=1 before_free=54980 before_allocated=141227 after_free=54981 after_allocated=141226
cpuctl_test04    1  TBROK  :  cpuctl_test04.c:140: Invalid test number passed
cpuctl_test04    2  TBROK  :  cpuctl_test04.c:140: Remaining cases broken
frame-allocator-diagnostic: process-teardown pid=1271 reclaimed_frames=266 before_free=54716 before_allocated=141491 after_free=54982 after_allocated=141225
frame-allocator-diagnostic: process-teardown pid=1269 reclaimed_frames=12 before_free=54982 before_allocated=141225 after_free=54994 after_allocated=141213
FAIL LTP CASE cpuctl_test04 : 2
RUN LTP CASE cpufreq_boost
frame-allocator-diagnostic: process-teardown pid=1275 reclaimed_frames=511 before_free=54451 before_allocated=141756 after_free=54962 after_allocated=141245
frame-allocator-diagnostic: process-teardown pid=1273 reclaimed_frames=508 before_free=54962 before_allocated=141245 after_free=55470 after_allocated=140737
frame-allocator-diagnostic: process-teardown pid=1277 reclaimed_frames=1 before_free=54938 before_allocated=141269 after_free=54939 after_allocated=141268
frame-allocator-diagnostic: process-teardown pid=1279 reclaimed_frames=3 before_free=54664 before_allocated=141543 after_free=54667 after_allocated=141540
cpufreq_boost    1  TCONF  :  cpufreq_boost.c:107: overclock not supported
cpufreq_boost    2  TCONF  :  cpufreq_boost.c:107: Remaining cases not appropriate for configuration
frame-allocator-diagnostic: process-teardown pid=1276 reclaimed_frames=265 before_free=54667 before_allocated=141540 after_free=54932 after_allocated=141275
frame-allocator-diagnostic: process-teardown pid=1274 reclaimed_frames=12 before_free=54932 before_allocated=141275 after_free=54944 after_allocated=141263
FAIL LTP CASE cpufreq_boost : 32
RUN LTP CASE cpuhotplug01.sh
SKIP LTP CASE cpuhotplug01.sh : CPU hotplug unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=1280 reclaimed_frames=12 before_free=54922 before_allocated=141285 after_free=54934 after_allocated=141273
FAIL LTP CASE cpuhotplug01.sh : 32
RUN LTP CASE cpuhotplug02.sh
SKIP LTP CASE cpuhotplug02.sh : CPU hotplug unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=1281 reclaimed_frames=12 before_free=54912 before_allocated=141295 after_free=54924 after_allocated=141283
FAIL LTP CASE cpuhotplug02.sh : 32
RUN LTP CASE cpuhotplug03.sh
SKIP LTP CASE cpuhotplug03.sh : CPU hotplug unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=1282 reclaimed_frames=12 before_free=54902 before_allocated=141305 after_free=54914 after_allocated=141293
FAIL LTP CASE cpuhotplug03.sh : 32
RUN LTP CASE cpuhotplug04.sh
SKIP LTP CASE cpuhotplug04.sh : CPU hotplug unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=1283 reclaimed_frames=12 before_free=54892 before_allocated=141315 after_free=54904 after_allocated=141303
FAIL LTP CASE cpuhotplug04.sh : 32
RUN LTP CASE cpuhotplug05.sh
SKIP LTP CASE cpuhotplug05.sh : CPU hotplug unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=1284 reclaimed_frames=13 before_free=54881 before_allocated=141326 after_free=54894 after_allocated=141313
FAIL LTP CASE cpuhotplug05.sh : 32
RUN LTP CASE cpuhotplug06.sh
SKIP LTP CASE cpuhotplug06.sh : CPU hotplug unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=1285 reclaimed_frames=13 before_free=54871 before_allocated=141336 after_free=54884 after_allocated=141323
FAIL LTP CASE cpuhotplug06.sh : 32
RUN LTP CASE cpuhotplug07.sh
SKIP LTP CASE cpuhotplug07.sh : CPU hotplug unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=1286 reclaimed_frames=12 before_free=54862 before_allocated=141345 after_free=54874 after_allocated=141333
FAIL LTP CASE cpuhotplug07.sh : 32
RUN LTP CASE cpuhotplug_do_disk_write_loop
SKIP LTP CASE cpuhotplug_do_disk_write_loop : CPU hotplug unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=1287 reclaimed_frames=13 before_free=54851 before_allocated=141356 after_free=54864 after_allocated=141343
FAIL LTP CASE cpuhotplug_do_disk_write_loop : 32
RUN LTP CASE cpuhotplug_do_kcompile_loop
SKIP LTP CASE cpuhotplug_do_kcompile_loop : CPU hotplug unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=1288 reclaimed_frames=12 before_free=54842 before_allocated=141365 after_free=54854 after_allocated=141353
FAIL LTP CASE cpuhotplug_do_kcompile_loop : 32
RUN LTP CASE cpuhotplug_do_spin_loop
SKIP LTP CASE cpuhotplug_do_spin_loop : CPU hotplug unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=1289 reclaimed_frames=12 before_free=54832 before_allocated=141375 after_free=54844 after_allocated=141363
FAIL LTP CASE cpuhotplug_do_spin_loop : 32
RUN LTP CASE cpuhotplug_hotplug.sh
SKIP LTP CASE cpuhotplug_hotplug.sh : CPU hotplug unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=1290 reclaimed_frames=12 before_free=54822 before_allocated=141385 after_free=54834 after_allocated=141373
FAIL LTP CASE cpuhotplug_hotplug.sh : 32
RUN LTP CASE cpuhotplug_report_proc_interrupts
SKIP LTP CASE cpuhotplug_report_proc_interrupts : CPU hotplug unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=1291 reclaimed_frames=13 before_free=54811 before_allocated=141396 after_free=54824 after_allocated=141383
FAIL LTP CASE cpuhotplug_report_proc_interrupts : 32
RUN LTP CASE cpuhotplug_testsuite.sh
SKIP LTP CASE cpuhotplug_testsuite.sh : CPU hotplug unsupported by kernel
frame-allocator-diagnostic: process-teardown pid=1292 reclaimed_frames=13 before_free=54801 before_allocated=141406 after_free=54814 after_allocated=141393
FAIL LTP CASE cpuhotplug_testsuite.sh : 32
RUN LTP CASE cpuset01
frame-allocator-diagnostic: process-teardown pid=1294 reclaimed_frames=511 before_free=54271 before_allocated=141936 after_free=54782 after_allocated=141425
frame-allocator-diagnostic: process-teardown pid=1278 reclaimed_frames=508 before_free=54782 before_allocated=141425 after_free=55290 after_allocated=140917
frame-allocator-diagnostic: process-teardown pid=1296 reclaimed_frames=1 before_free=54758 before_allocated=141449 after_free=54759 after_allocated=141448
tst_test.c:1175: TCONF: test requires libnuma development packages with LIBNUMA_API_VERSION >= 2
frame-allocator-diagnostic: process-teardown pid=1295 reclaimed_frames=261 before_free=54499 before_allocated=141708 after_free=54760 after_allocated=141447
frame-allocator-diagnostic: process-teardown pid=1293 reclaimed_frames=12 before_free=54760 before_allocated=141447 after_free=54772 after_allocated=141435
FAIL LTP CASE cpuset01 : 32
RUN LTP CASE crash01
frame-allocator-diagnostic: process-teardown pid=1299 reclaimed_frames=511 before_free=54229 before_allocated=141978 after_free=54740 after_allocated=141467
frame-allocator-diagnostic: process-teardown pid=1297 reclaimed_frames=508 before_free=54740 before_allocated=141467 after_free=55248 after_allocated=140959
frame-allocator-diagnostic: process-teardown pid=1301 reclaimed_frames=1 before_free=54716 before_allocated=141491 after_free=54717 after_allocated=141490
crash01     0  TINFO  :  crashme +2000.80 721 100
[721.909709 0:1303 axruntime::lang_items:5] panicked at vendor/axcpu/src/loongarch64/trap.rs:65:13:
Unhandled trap Exception(InstructionNotExist) @ 0x10000b3020:
TrapFrame {
    regs: GeneralRegisters {
        zero: 0x5601000000012001,
        ra: 0x120005168,
        tp: 0x10000b6dc0,
        sp: 0x1ffffffc90,
        a0: 0x0,
        a1: 0x1ffffffc40,
        a2: 0x1ffffffc60,
        a3: 0x8,
        a4: 0x1ffffffbd8,
        a5: 0x10000b7000,
        a6: 0x73,
        a7: 0x67,
        t0: 0x10000b3020,
        t1: 0x120003e1c,
        t2: 0x0,
        t3: 0x100006d98c,
        t4: 0x100000001,
        t5: 0x1ffffffbe8,
        t6: 0x1ffffffb80,
        t7: 0x0,
        t8: 0x1fffffe450,
        u0: 0x0,
        fp: 0x0,
        s0: 0x120038b78,
        s1: 0x120038b78,
        s2: 0x1200386a0,
        s3: 0x1,
        s4: 0x1200237c0,
        s5: 0x1200237d0,
        s6: 0x120025970,
        s7: 0x120023790,
        s8: 0x120038000,
    },
    prmd: 0x7,
    era: 0x10000b3020,
}
[721.914566 0:1303 axplat_loongarch64_qemu_virt::power:23] Shutting down...
===== FULL LTP loongarch64 end: 2026-05-19T13:48:20+08:00 status=0 =====
```
