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
[37maxconfig-gen[0m [90mconfigs/defconfig.toml /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/axplat-loongarch64-qemu-virt-0.4.2/axconfig.toml  -w arch=loongarch64 -w platform=loongarch64-qemu-virt -o "/root/oskernel2026-orays/build/kernels/loongarch64.axconfig.toml" -w plat.phys-memory-size=0x3000_0000 -w plat.max-cpu-num=1 -c "/root/oskernel2026-orays/build/kernels/loongarch64.axconfig.toml"[0m
    [92;1mBuilding[0m App: shell, Arch: loongarch64, Platform: loongarch64-qemu-virt, App type: rust
[37mcargo -C examples/shell build[0m [90m-Z unstable-options --target loongarch64-unknown-none-softfloat --target-dir /root/oskernel2026-orays/build/kernels/target/loongarch64 --release  --features "axstd/defplat axstd/log-level-info axstd/alloc axstd/paging axstd/irq axstd/multitask axstd/fs axstd/net auto-run-tests uspace"[0m
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
    Finished `release` profile [optimized] target(s) in 23.35s
[37mrust-objcopy --binary-architecture=loongarch64[0m [90m/root/oskernel2026-orays/build/kernels/loongarch64/shell_loongarch64-qemu-virt.elf --strip-all -O binary /root/oskernel2026-orays/build/kernels/loongarch64/shell_loongarch64-qemu-virt.bin[0m
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

[37m[  0.004790 0 axruntime:135] [32mLogging is enabled.[m
[m[37m[  0.006444 0 axruntime:136] [32mPrimary CPU 0 started, arg = 0x0.[m
[m[37m[  0.008007 0 axruntime:139] [32mFound physcial memory regions:[m
[m[37m[  0.008517 0 axruntime:141] [32m  [PA:0x100d0000, PA:0x100d1000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.009258 0 axruntime:141] [32m  [PA:0x100e0000, PA:0x100e1000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.009653 0 axruntime:141] [32m  [PA:0x1fe00000, PA:0x1fe01000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.010089 0 axruntime:141] [32m  [PA:0x20000000, PA:0x30000000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.010554 0 axruntime:141] [32m  [PA:0x40000000, PA:0x40020000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.011024 0 axruntime:141] [32m  [PA:0x80000000, PA:0x80120000) .text (READ | EXECUTE | RESERVED)[m
[m[37m[  0.011572 0 axruntime:141] [32m  [PA:0x80120000, PA:0x8014b000) .rodata (READ | RESERVED)[m
[m[37m[  0.012050 0 axruntime:141] [32m  [PA:0x8014b000, PA:0x80150000) .data .tdata .tbss .percpu (READ | WRITE | RESERVED)[m
[m[37m[  0.012545 0 axruntime:141] [32m  [PA:0x80150000, PA:0x80190000) boot stack (READ | WRITE | RESERVED)[m
[m[37m[  0.012993 0 axruntime:141] [32m  [PA:0x80190000, PA:0x801b7000) .bss (READ | WRITE | RESERVED)[m
[m[37m[  0.013421 0 axruntime:141] [32m  [PA:0x801b7000, PA:0xb0000000) free memory (READ | WRITE | FREE)[m
[m[37m[  0.013990 0 axruntime:216] [32mInitialize global memory allocator...[m
[m[37m[  0.014394 0 axruntime:217] [32m  use TLSF allocator.[m
[m[37m[  0.018054 0 axmm:103] [32mInitialize virtual memory management...[m
[m[37m[  0.080188 0 axruntime:156] [32mInitialize platform devices...[m
[msmp = 1
[37m[  0.092832 0 axtask::api:73] [32mInitialize scheduling...[m
[m[37m[  0.107436 0 axtask::api:83] [32m  use Round-robin scheduler.[m
[m[37m[  0.117010 0 axdriver:152] [32mInitialize device drivers...[m
[m[37m[  0.125995 0 axdriver:153] [32m  device model: static[m
[m[37m[  0.147053 0 virtio_drivers::device::blk:63] [32mfound a block device of size 4194304KB[m
[m[37m[  0.163456 0 axdriver::bus::pci:107] [32mregistered a new Block device at 00:01.0: "virtio-blk"[m
[m[37m[  0.186908 0 virtio_drivers::device::net::dev_raw:33] [32mnegotiated_features Features(MAC | STATUS | RING_INDIRECT_DESC | RING_EVENT_IDX | VERSION_1)[m
[m[37m[  0.215221 0 axdriver::bus::pci:107] [32mregistered a new Net device at 00:02.0: "virtio-net"[m
[m[37m[  0.327197 0 axfs:44] [32mInitialize filesystems...[m
[m[37m[  0.340431 0 axfs:47] [32m  use block device 0: "virtio-blk"[m
[m[37m[  0.351771 0 axfs::root:336] [32m  detected root filesystem: Ext4[m
[m[37m[  0.389674 0 axnet:42] [32mInitialize network subsystem...[m
[m[37m[  0.390765 0 axnet:45] [32m  use NIC 0: "virtio-net"[m
[m[37m[  0.403549 0 axnet::smoltcp_impl:335] [32mcreated net interface "eth0":[m
[m[37m[  0.417533 0 axnet::smoltcp_impl:336] [32m  ether:    52-54-00-12-34-56[m
[m[37m[  0.427430 0 axnet::smoltcp_impl:337] [32m  ip:       10.0.2.15/24[m
[m[37m[  0.435405 0 axnet::smoltcp_impl:338] [32m  gateway:  10.0.2.2[m
[m[37m[  0.445641 0 axruntime:182] [32mInitialize interrupt handlers...[m
[m[37m[  0.450854 0 axruntime:194] [32mPrimary CPU 0 init OK.[m
[m#### OS COMP TEST GROUP START libctest-musl ####
========== START entry-static.exe argv ==========
Pass!
========== END entry-static.exe argv ==========
========== START entry-static.exe basename ==========
Pass!
========== END entry-static.exe basename ==========
========== START entry-static.exe clocale_mbfuncs ==========
Pass!
========== END entry-static.exe clocale_mbfuncs ==========
========== START entry-static.exe clock_gettime ==========
Pass!
========== END entry-static.exe clock_gettime ==========
========== START entry-static.exe dirname ==========
Pass!
========== END entry-static.exe dirname ==========
========== START entry-static.exe env ==========
Pass!
========== END entry-static.exe env ==========
========== START entry-static.exe fdopen ==========
Pass!
========== END entry-static.exe fdopen ==========
========== START entry-static.exe fnmatch ==========
Pass!
========== END entry-static.exe fnmatch ==========
========== START entry-static.exe fscanf ==========
Pass!
========== END entry-static.exe fscanf ==========
========== START entry-static.exe fwscanf ==========
Pass!
========== END entry-static.exe fwscanf ==========
========== START entry-static.exe iconv_open ==========
Pass!
========== END entry-static.exe iconv_open ==========
========== START entry-static.exe inet_pton ==========
Pass!
========== END entry-static.exe inet_pton ==========
========== START entry-static.exe mbc ==========
Pass!
========== END entry-static.exe mbc ==========
========== START entry-static.exe memstream ==========
Pass!
========== END entry-static.exe memstream ==========
========== START entry-static.exe pthread_cancel_points ==========
FAIL libctest entry-static.exe pthread_cancel_points: timeout
========== END entry-static.exe pthread_cancel_points ==========
========== START entry-static.exe pthread_cancel ==========
FAIL libctest entry-static.exe pthread_cancel: timeout
========== END entry-static.exe pthread_cancel ==========
========== START entry-static.exe pthread_cond ==========
FAIL libctest entry-static.exe pthread_cond: timeout
========== END entry-static.exe pthread_cond ==========
========== START entry-static.exe pthread_tsd ==========
Pass!
========== END entry-static.exe pthread_tsd ==========
========== START entry-static.exe qsort ==========
autorun: /tmp/testsuite/musl/libctest/libctest_testcode.sh timed out after 120s
#### OS COMP TEST GROUP START libctest-glibc ####
========== START entry-static.exe argv ==========
Pass!
========== END entry-static.exe argv ==========
========== START entry-static.exe basename ==========
Pass!
========== END entry-static.exe basename ==========
========== START entry-static.exe clocale_mbfuncs ==========
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 80 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 80 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 81 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 81 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 82 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 82 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 83 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 83 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 84 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 84 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 85 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 85 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 86 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 86 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 87 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 87 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 88 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 88 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 89 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 89 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 8a to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 8a to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 8b to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 8b to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 8c to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 8c to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 8d to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 8d to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 8e to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 8e to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 8f to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 8f to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 90 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 90 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 91 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 91 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 92 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 92 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 93 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 93 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 94 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 94 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 95 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 95 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 96 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 96 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 97 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 97 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 98 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 98 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 99 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 99 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 9a to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 9a to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 9b to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 9b to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 9c to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 9c to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 9d to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 9d to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 9e to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 9e to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte 9f to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte 9f to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte a0 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte a0 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte a1 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte a1 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte a2 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte a2 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte a3 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte a3 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte a4 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte a4 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte a5 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte a5 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte a6 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte a6 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte a7 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte a7 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte a8 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte a8 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte a9 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte a9 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte aa to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte aa to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte ab to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte ab to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte ac to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte ac to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte ad to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte ad to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte ae to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte ae to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte af to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte af to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte b0 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte b0 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte b1 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte b1 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte b2 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte b2 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte b3 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte b3 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte b4 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte b4 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte b5 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte b5 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte b6 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte b6 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte b7 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte b7 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte b8 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte b8 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte b9 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte b9 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte ba to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte ba to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte bb to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte bb to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte bc to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte bc to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte bd to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte bd to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte be to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte be to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte bf to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte bf to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte c0 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte c0 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte c1 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte c1 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte c2 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte c2 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte c3 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte c3 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte c4 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte c4 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte c5 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte c5 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte c6 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte c6 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte c7 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte c7 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte c8 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte c8 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte c9 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte c9 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte ca to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte ca to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte cb to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte cb to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte cc to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte cc to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte cd to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte cd to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte ce to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte ce to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte cf to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte cf to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte d0 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte d0 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte d1 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte d1 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte d2 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte d2 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte d3 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte d3 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte d4 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte d4 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte d5 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte d5 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte d6 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte d6 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte d7 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte d7 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte d8 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte d8 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte d9 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte d9 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte da to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte da to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte db to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte db to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte dc to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte dc to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte dd to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte dd to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte de to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte de to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte df to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte df to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte e0 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte e0 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte e1 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte e1 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte e2 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte e2 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte e3 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte e3 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte e4 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte e4 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte e5 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte e5 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte e6 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte e6 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte e7 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte e7 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte e8 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte e8 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte e9 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte e9 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte ea to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte ea to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte eb to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte eb to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte ec to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte ec to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte ed to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte ed to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte ee to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte ee to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte ef to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte ef to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte f0 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte f0 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte f1 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte f1 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte f2 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte f2 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte f3 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte f3 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte f4 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte f4 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte f5 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte f5 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte f6 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte f6 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte f7 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte f7 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte f8 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte f8 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte f9 to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte f9 to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte fa to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte fa to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte fb to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte fb to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte fc to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte fc to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte fd to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte fd to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte fe to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte fe to wchar_t
src/functional/clocale_mbfuncs.c:28: mbrtowc failed to convert byte ff to wchar_t
src/functional/clocale_mbfuncs.c:30: btowc failed to convert byte ff to wchar_t
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0000
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0001
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0002
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0003
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0004
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0005
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0006
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0007
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0008
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0009
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e000a
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e000b
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e000c
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e000d
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e000e
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e000f
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0010
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0011
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0012
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0013
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0014
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0015
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0016
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0017
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0018
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0019
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e001a
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e001b
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e001c
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e001d
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e001e
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e001f
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0020
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0021
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0022
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0023
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0024
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0025
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0026
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0027
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0028
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0029
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e002a
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e002b
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e002c
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e002d
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e002e
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e002f
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0030
src/functional/clocale_mbfuncs.c:52: wcrtomb accepted non-image wchar_t e0031
src/functional/clocale_mbfuncs.c:55: additional 128 non-image errors (not printed)
src/functional/clocale_mbfuncs.c:60: wcsrtombs returned -1, expected 255
src/functional/clocale_mbfuncs.c:62: mbsrtowcs returned 127, expected 255
src/functional/clocale_mbfuncs.c:64: wcsrtombs/mbsrtowcs round trip failed
FAIL libctest entry-static.exe clocale_mbfuncs: 1
========== END entry-static.exe clocale_mbfuncs ==========
========== START entry-static.exe clock_gettime ==========
Pass!
========== END entry-static.exe clock_gettime ==========
========== START entry-static.exe dirname ==========
Pass!
========== END entry-static.exe dirname ==========
========== START entry-static.exe env ==========
Pass!
========== END entry-static.exe env ==========
========== START entry-static.exe fdopen ==========
Pass!
========== END entry-static.exe fdopen ==========
========== START entry-static.exe fnmatch ==========
src/functional/fnmatch.c:156: fnmatch("[[?*\]", "\", 0) failed, got 1 want 0
src/functional/fnmatch.c:156: fnmatch("[]?*\]", "]", 0) failed, got 1 want 0
src/functional/fnmatch.c:156: fnmatch("*[![:digit:]]*/[![:d-d]", "a/b", FNM_PATHNAME) failed, got 0 want -1
src/functional/fnmatch.c:156: fnmatch("*[![:digit:]]*/[[:d-d]", "a/[", FNM_PATHNAME) failed, got 0 want -1
src/functional/fnmatch.c:156: fnmatch("\", "\", 0) failed, got 1 want 0
FAIL libctest entry-static.exe fnmatch: 1
========== END entry-static.exe fnmatch ==========
========== START entry-static.exe fscanf ==========
src/functional/fscanf.c:52: fscanf(f, "ld %5i%2i", &x, &y) failed (got 2 fields, expected 1)
src/functional/fscanf.c:59: !!(f=writetemp("      42")) failed (failed to make temp file)
src/functional/fscanf.c:70: !!(f=writetemp("[abc123]....x")) failed (failed to make temp file)
src/functional/fscanf.c:84: !!(f=writetemp("0x1p 12")) failed (failed to make temp file)
src/functional/fscanf.c:108: !!(f=writetemp("0x.1p4    012")) failed (failed to make temp file)
src/functional/fscanf.c:121: !!(f=writetemp("0xx")) failed (failed to make temp file)
FAIL libctest entry-static.exe fscanf: 1
========== END entry-static.exe fscanf ==========
========== START entry-static.exe fwscanf ==========
src/functional/fwscanf.c:42: !!(f=writetemp("      42")) failed (failed to make temp file)
src/functional/fwscanf.c:53: !!(f=writetemp("[abc123]....x")) failed (failed to make temp file)
src/functional/fwscanf.c:67: !!(f=writetemp("0x1p 12")) failed (failed to make temp file)
src/functional/fwscanf.c:91: !!(f=writetemp("0x.1p4    012")) failed (failed to make temp file)
src/functional/fwscanf.c:104: !!(f=writetemp("0xx")) failed (failed to make temp file)
FAIL libctest entry-static.exe fwscanf: 1
========== END entry-static.exe fwscanf ==========
========== START entry-static.exe iconv_open ==========
Pass!
========== END entry-static.exe iconv_open ==========
========== START entry-static.exe inet_pton ==========
Pass!
========== END entry-static.exe inet_pton ==========
========== START entry-static.exe mbc ==========
src/functional/mbc.c:44: cannot set UTF-8 locale for test (codeset=ANSI_X3.4-1968)
FAIL libctest entry-static.exe mbc: 83
========== END entry-static.exe mbc ==========
========== START entry-static.exe memstream ==========
Pass!
========== END entry-static.exe memstream ==========
========== START entry-static.exe pthread_cancel_points ==========
The futex facility returned an unexpected error code.
FAIL libctest entry-static.exe pthread_cancel_points: 134
========== END entry-static.exe pthread_cancel_points ==========
========== START entry-static.exe pthread_cancel ==========
The futex facility returned an unexpected error code.
FAIL libctest entry-static.exe pthread_cancel: 134
========== END entry-static.exe pthread_cancel ==========
========== START entry-static.exe pthread_cond ==========
The futex facility returned an unexpected error code.
FAIL libctest entry-static.exe pthread_cond: 134
========== END entry-static.exe pthread_cond ==========
========== START entry-static.exe pthread_tsd ==========
The futex facility returned an unexpected error code.
FAIL libctest entry-static.exe pthread_tsd: 134
========== END entry-static.exe pthread_tsd ==========
========== START entry-static.exe qsort ==========
Pass!
========== END entry-static.exe qsort ==========
========== START entry-static.exe random ==========
Pass!
========== END entry-static.exe random ==========
========== START entry-static.exe search_hsearch ==========
autorun: /tmp/testsuite/glibc/libctest/libctest_testcode.sh timed out after 120s
#### OS COMP TEST GROUP START basic-musl ####
Testing brk :
========== START test_brk ==========
Before alloc,heap pos: 77824
After alloc,heap pos: 77888
Alloc again,heap pos: 77952
========== END test_brk ==========
Testing chdir :
========== START test_chdir ==========
chdir ret: 0
  current working dir : 
========== END test_chdir ==========
Testing clone :
========== START test_clone ==========
  Child says successfully!
clone process successfully.
pid:324
========== END test_clone ==========
Testing close :
========== START test_close ==========
  close 3 success.
========== END test_close ==========
Testing dup2 :
========== START test_dup2 ==========
  from fd 100
========== END test_dup2 ==========
Testing dup :
========== START test_dup ==========
  new fd is 3.
========== END test_dup ==========
Testing execve :
========== START test_execve ==========
  I am test_echo.
execve success.
========== END main ==========
Testing exit :
========== START test_exit ==========
exit OK.
========== END test_exit ==========
Testing fork :
========== START test_fork ==========
  child process.
  parent process. wstatus:0
========== END test_fork ==========
Testing fstat :
========== START test_fstat ==========
fstat ret: 0
fstat: dev: 1, inode: 1012599416, mode: 33206, nlink: 1, size: 52, atime: 0, mtime: 0, ctime: 0
========== END test_fstat ==========
Testing getcwd :
========== START test_getcwd ==========
getcwd: /tmp/testsuite/musl/basic/basic successfully!
========== END test_getcwd ==========
Testing getdents :
========== START test_getdents ==========
open fd:3
getdents fd:-20
getdents success.


========== END test_getdents ==========
Testing getpid :
========== START test_getpid ==========
getpid success.
pid = 336
========== END test_getpid ==========
Testing getppid :
========== START test_getppid ==========
  getppid success. ppid : 320
========== END test_getppid ==========
Testing gettimeofday :
========== START test_gettimeofday ==========
gettimeofday success.
start:263361, end:263431
interval: 70
========== END test_gettimeofday ==========
Testing mkdir_ :
========== START test_mkdir ==========
mkdir ret: 0
  mkdir success.
========== END test_mkdir ==========
Testing mmap :
========== START test_mmap ==========
file len: 27
mmap content:   Hello, mmap successfully!
========== END test_mmap ==========
Testing mount :
========== START test_mount ==========
Mounting dev:/dev/vda2 to ./mnt
mount return: 0
mount successfully
umount return: 0
========== END test_mount ==========
Testing munmap :
========== START test_munmap ==========
file len: 27
munmap return: 0
munmap successfully!
========== END test_munmap ==========
Testing openat :
========== START test_openat ==========
open dir fd: 3
openat fd: 4
openat success.
========== END test_openat ==========
Testing open :
========== START test_open ==========
Hi, this is a text file.
syscalls testing success!

========== END test_open ==========
Testing pipe :
========== START test_pipe ==========
cpid: 346
cpid: 0
  Write to pipe successfully.

========== END test_pipe ==========
Testing read :
========== START test_read ==========
Hi, this is a text file.
syscalls testing success!

========== END test_read ==========
Testing sleep :
========== START test_sleep ==========
sleep success.
========== END test_sleep ==========
Testing times :
========== START test_times ==========
mytimes success
{tms_utime:0, tms_stime:0, tms_cutime:0, tms_cstime:0}
========== END test_times ==========
Testing umount :
========== START test_umount ==========
Mounting dev:/dev/vda2 to ./mnt
mount return: 0
umount success.
return: 0
========== END test_umount ==========
Testing uname :
========== START test_uname ==========
Uname: Linux arceos 6.0.0 ArceOS loongarch64 localdomain
========== END test_uname ==========
Testing unlink :
========== START test_unlink ==========
  unlink success!
========== END test_unlink ==========
Testing wait :
========== START test_wait ==========
This is child process
wait child success.
wstatus: 0
========== END test_wait ==========
Testing waitpid :
========== START test_waitpid ==========
This is child process
waitpid successfully.
wstatus: 3
========== END test_waitpid ==========
Testing write :
========== START test_write ==========
Hello operating system contest.
========== END test_write ==========
Testing yield :
========== START test_yield ==========
  I am child process: 359. iteration 0.
  I am child process: 360. iteration 1.
  I am child process: 361. iteration 2.
  I am child process: 359. iteration 0.
  I am child process: 360. iteration 1.
  I am child process: 361. iteration 2.
  I am child process: 359. iteration 0.
  I am child process: 360. iteration 1.
  I am child process: 361. iteration 2.
  I am child process: 359. iteration 0.
  I am child process: 360. iteration 1.
  I am child process: 361. iteration 2.
  I am child process: 359. iteration 0.
  I am child process: 360. iteration 1.
  I am child process: 361. iteration 2.
========== END test_yield ==========
#### OS COMP TEST GROUP END basic-musl ####
#### OS COMP TEST GROUP START basic-glibc ####
Testing brk :
========== START test_brk ==========
Before alloc,heap pos: 77824
After alloc,heap pos: 77888
Alloc again,heap pos: 77952
========== END test_brk ==========
Testing chdir :
========== START test_chdir ==========
chdir ret: 0
  current working dir : 
========== END test_chdir ==========
Testing clone :
========== START test_clone ==========
  Child says successfully!
clone process successfully.
pid:371
========== END test_clone ==========
Testing close :
========== START test_close ==========
  close 3 success.
========== END test_close ==========
Testing dup2 :
========== START test_dup2 ==========
  from fd 100
========== END test_dup2 ==========
Testing dup :
========== START test_dup ==========
  new fd is 3.
========== END test_dup ==========
Testing execve :
========== START test_execve ==========
  I am test_echo.
execve success.
========== END main ==========
Testing exit :
========== START test_exit ==========
exit OK.
========== END test_exit ==========
Testing fork :
========== START test_fork ==========
  child process.
  parent process. wstatus:0
========== END test_fork ==========
Testing fstat :
========== START test_fstat ==========
fstat ret: 0
fstat: dev: 1, inode: 1612857110, mode: 33206, nlink: 1, size: 52, atime: 0, mtime: 0, ctime: 0
========== END test_fstat ==========
Testing getcwd :
========== START test_getcwd ==========
getcwd: /tmp/testsuite/glibc/basic/basic successfully!
========== END test_getcwd ==========
Testing getdents :
========== START test_getdents ==========
open fd:3
getdents fd:-20
getdents success.


========== END test_getdents ==========
Testing getpid :
========== START test_getpid ==========
getpid success.
pid = 383
========== END test_getpid ==========
Testing getppid :
========== START test_getppid ==========
  getppid success. ppid : 367
========== END test_getppid ==========
Testing gettimeofday :
========== START test_gettimeofday ==========
gettimeofday success.
start:294163, end:294247
interval: 84
========== END test_gettimeofday ==========
Testing mkdir_ :
========== START test_mkdir ==========
mkdir ret: 0
  mkdir success.
========== END test_mkdir ==========
Testing mmap :
========== START test_mmap ==========
file len: 27
mmap content:   Hello, mmap successfully!
========== END test_mmap ==========
Testing mount :
========== START test_mount ==========
Mounting dev:/dev/vda2 to ./mnt
mount return: 0
mount successfully
umount return: 0
========== END test_mount ==========
Testing munmap :
========== START test_munmap ==========
file len: 27
munmap return: 0
munmap successfully!
========== END test_munmap ==========
Testing openat :
========== START test_openat ==========
open dir fd: 3
openat fd: 4
openat success.
========== END test_openat ==========
Testing open :
========== START test_open ==========
Hi, this is a text file.
syscalls testing success!

========== END test_open ==========
Testing pipe :
========== START test_pipe ==========
cpid: 393
cpid: 0
  Write to pipe successfully.

========== END test_pipe ==========
Testing read :
========== START test_read ==========
Hi, this is a text file.
syscalls testing success!

========== END test_read ==========
Testing sleep :
========== START test_sleep ==========
sleep success.
========== END test_sleep ==========
Testing times :
========== START test_times ==========
mytimes success
{tms_utime:0, tms_stime:0, tms_cutime:0, tms_cstime:0}
========== END test_times ==========
Testing umount :
========== START test_umount ==========
Mounting dev:/dev/vda2 to ./mnt
mount return: 0
umount success.
return: 0
========== END test_umount ==========
Testing uname :
========== START test_uname ==========
Uname: Linux arceos 6.0.0 ArceOS loongarch64 localdomain
========== END test_uname ==========
Testing unlink :
========== START test_unlink ==========
  unlink success!
========== END test_unlink ==========
Testing wait :
========== START test_wait ==========
This is child process
wait child success.
wstatus: 0
========== END test_wait ==========
Testing waitpid :
========== START test_waitpid ==========
This is child process
waitpid successfully.
wstatus: 3
========== END test_waitpid ==========
Testing write :
========== START test_write ==========
Hello operating system contest.
========== END test_write ==========
Testing yield :
========== START test_yield ==========
  I am child process: 406. iteration 0.
  I am child process: 407. iteration 1.
  I am child process: 408. iteration 2.
  I am child process: 406. iteration 0.
  I am child process: 407. iteration 1.
  I am child process: 408. iteration 2.
  I am child process: 406. iteration 0.
  I am child process: 407. iteration 1.
  I am child process: 408. iteration 2.
  I am child process: 406. iteration 0.
  I am child process: 407. iteration 1.
  I am child process: 408. iteration 2.
  I am child process: 406. iteration 0.
  I am child process: 407. iteration 1.
  I am child process: 408. iteration 2.
========== END test_yield ==========
#### OS COMP TEST GROUP END basic-glibc ####
#### OS COMP TEST GROUP START busybox-musl ####
#### independent command test
testcase busybox echo "#### independent command test" success
testcase busybox ash -c exit success
testcase busybox sh -c exit success
bbb
testcase busybox basename /aaa/bbb success
    January 1970
Su Mo Tu We Th Fr Sa
             1  2  3
 4  5  6  7  8  9 10
11 12 13 14 15 16 17
18 19 20 21 22 23 24
25 26 27 28 29 30 31
                     
testcase busybox cal success
[H[Jtestcase busybox clear success
Thu Jan  1 00:05:18 UTC 1970
testcase busybox date success
Filesystem           1K-blocks      Used Available Use% Mounted on
devfs                   784676     81804    702872  10% /dev
tmpfs                   784676     81804    702872  10% /tmp
tmpfs                   784676     81804    702872  10% /var
proc                    784676     81804    702872  10% /proc
sysfs                   784676     81804    702872  10% /sys
testcase busybox df success
/aaa
testcase busybox dirname /aaa/bbb success
testcase busybox dmesg success
0	./lib
0	.
testcase busybox du success
2
testcase busybox expr 1 + 1 success
testcase busybox false success
testcase busybox true success
testcase busybox which ls fail
return: 1, cmd: which ls
Linux
testcase busybox uname success
 00:05:36 up 0 min,  0 users,  load average: 0.00, 0.00, 0.00
testcase busybox uptime success
abc
testcase busybox printf "abc\n" success
PID   USER     TIME  COMMAND
testcase busybox ps success
/tmp/testsuite/musl/busybox
testcase busybox pwd success
              total        used        free      shared  buff/cache   available
Mem:              0           0           0           0           0      781818
-/+ buffers/cache:            0           0
Swap:             0           0           0
testcase busybox free success
Thu Jan  1 00:05:45 1970  0.000000 seconds
testcase busybox hwclock success
testcase busybox sh -c 'sleep 5' & /musl/busybox kill $! success
[0;0mawk[m                  [0;0mkill[m                 [0;0mseq[m
[0;0mbasename[m             [1;34mlib[m                  [0;0msetsid[m
[0;0mbusybox_cmd.txt[m      [0;0mline[m                 [0;0msh[m
[0;0mbusybox_testcode.sh[m  [0;0mln[m                   [0;0msleep[m
[0;0mcat[m                  [0;0mls[m                   [0;0msort[m
[0;0mchmod[m                [0;0mmkdir[m                [0;0mtail[m
[0;0mcp[m                   [0;0mmktemp[m               [0;0mtee[m
[0;0mcut[m                  [0;0mmv[m                   [0;0mtimeout[m
[0;0mdate[m                 [0;0mprintf[m               [0;0mtouch[m
[0;0mdirname[m              [0;0mps[m                   [0;0mtr[m
[0;0mecho[m                 [0;0mpwd[m                  [0;0mtrue[m
[0;0mexpr[m                 [0;0mreadlink[m             [0;0muname[m
[0;0mfind[m                 [0;0mrm[m                   [0;0mwc[m
[0;0mgrep[m                 [0;0mrmdir[m                [0;0mxargs[m
[0;0mhead[m                 [0;0msed[m
testcase busybox ls success
testcase busybox sleep 1 success
#### file opration test
testcase busybox echo "#### file opration test" success
testcase busybox touch test.txt success
testcase busybox echo "hello world" > test.txt success
hello world
testcase busybox cat test.txt success
l
testcase busybox cut -c 3 test.txt success
0000000 062550 066154 020157 067567 066162 005144
0000014
testcase busybox od test.txt success
hello world
testcase busybox head test.txt success
hello world
testcase busybox tail test.txt success
00000000  68 65 6c 6c 6f 20 77 6f  72 6c 64 0a              |hello world.|
0000000c
testcase busybox hexdump -C test.txt success
6f5902ac237024bdd0c176cb93063dc4  test.txt
testcase busybox md5sum test.txt success
testcase busybox echo "ccccccc" >> test.txt success
testcase busybox echo "bbbbbbb" >> test.txt success
testcase busybox echo "aaaaaaa" >> test.txt success
testcase busybox echo "2222222" >> test.txt success
testcase busybox echo "1111111" >> test.txt success
testcase busybox echo "bbbbbbb" >> test.txt success
1111111
2222222
aaaaaaa
bbbbbbb
ccccccc
hello world
testcase busybox sort test.txt | /musl/busybox uniq success
  File: test.txt
  Size: 60        	Blocks: 0          IO Block: 512    regular file
Device: 1h/1d	Inode: 14331471978328146352  Links: 1
Access: (0666/-rw-rw-rw-)  Uid: (    0/    root)   Gid: (    0/    root)
Access: 1970-01-01 00:00:00.000000000 +0000
Modify: 1970-01-01 00:00:00.000000000 +0000
Change: 1970-01-01 00:00:00.000000000 +0000
testcase busybox stat test.txt success
hello world
ccccccc
bbbbbbb
aaaaaaa
2222222
1111111
bbbbbbb
testcase busybox strings test.txt success
        7         8        60 test.txt
testcase busybox wc test.txt success
testcase busybox [ -f test.txt ] success
hello world
ccccccc
bbbbbbb
aaaaaaa
2222222
1111111
bbbbbbb
testcase busybox more test.txt success
testcase busybox rm test.txt success
testcase busybox mkdir test_dir success
testcase busybox mv test_dir test success
testcase busybox rmdir test success
echo "hello world" > test.txt
grep hello busybox_cmd.txt
testcase busybox grep hello busybox_cmd.txt success
testcase busybox cp busybox_cmd.txt busybox_cmd.bak success
testcase busybox rm busybox_cmd.bak success
./busybox_cmd.txt
testcase busybox find -name "busybox_cmd.txt" success
#### OS COMP TEST GROUP END busybox-musl ####
#### OS COMP TEST GROUP START busybox-glibc ####
#### independent command test
testcase busybox echo "#### independent command test" success
testcase busybox ash -c exit success
testcase busybox sh -c exit success
bbb
testcase busybox basename /aaa/bbb success
    January 1970
Su Mo Tu We Th Fr Sa
             1  2  3
 4  5  6  7  8  9 10
11 12 13 14 15 16 17
18 19 20 21 22 23 24
25 26 27 28 29 30 31
                     
testcase busybox cal success
[H[Jtestcase busybox clear success
Thu Jan  1 00:07:15 UTC 1970
testcase busybox date success
Filesystem           1K-blocks      Used Available Use% Mounted on
devfs                   784676    155084    629592  20% /dev
tmpfs                   784676    155084    629592  20% /tmp
tmpfs                   784676    155084    629592  20% /var
proc                    784676    155084    629592  20% /proc
sysfs                   784676    155084    629592  20% /sys
testcase busybox df success
/aaa
testcase busybox dirname /aaa/bbb success
testcase busybox dmesg success
0	./lib
0	.
testcase busybox du success
2
testcase busybox expr 1 + 1 success
testcase busybox false success
testcase busybox true success
testcase busybox which ls fail
return: 1, cmd: which ls
Linux
testcase busybox uname success
 00:07:44 up 0 min,  0 users,  load average: 0.00, 0.00, 0.00
testcase busybox uptime success
abc
testcase busybox printf "abc\n" success
PID   USER     TIME  COMMAND
testcase busybox ps success
/tmp/testsuite/glibc/busybox
testcase busybox pwd success
              total        used        free      shared  buff/cache   available
Mem:              0           0           0           0           0      781818
-/+ buffers/cache:            0           0
Swap:             0           0           0
testcase busybox free success
Thu Jan  1 00:07:58 1970  0.000000 seconds
testcase busybox hwclock success
testcase busybox sh -c 'sleep 5' & /glibc/busybox kill $! success
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
lib
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
setsid
sh
sleep
sort
tail
tee
timeout
touch
tr
true
uname
wc
xargs
testcase busybox ls success
testcase busybox sleep 1 success
#### file opration test
testcase busybox echo "#### file opration test" success
testcase busybox touch test.txt success
testcase busybox echo "hello world" > test.txt success
hello world
testcase busybox cat test.txt success
l
testcase busybox cut -c 3 test.txt success
0000000 062550 066154 020157 067567 066162 005144
0000014
testcase busybox od test.txt success
hello world
testcase busybox head test.txt success
hello world
testcase busybox tail test.txt success
00000000  68 65 6c 6c 6f 20 77 6f  72 6c 64 0a              |hello world.|
0000000c
testcase busybox hexdump -C test.txt success
6f5902ac237024bdd0c176cb93063dc4  test.txt
testcase busybox md5sum test.txt success
testcase busybox echo "ccccccc" >> test.txt success
testcase busybox echo "bbbbbbb" >> test.txt success
testcase busybox echo "aaaaaaa" >> test.txt success
testcase busybox echo "2222222" >> test.txt success
testcase busybox echo "1111111" >> test.txt success
testcase busybox echo "bbbbbbb" >> test.txt success
1111111
2222222
aaaaaaa
bbbbbbb
ccccccc
hello world
testcase busybox sort test.txt | /glibc/busybox uniq success
  File: test.txt
  Size: 60        	Blocks: 0          IO Block: 512    regular file
Device: 1h/1d	Inode: 4368043057645409086  Links: 1
Access: (0666/-rw-rw-rw-)  Uid: (    0/    root)   Gid: (    0/    root)
Access: 1970-01-01 00:00:00.000000000 +0000
Modify: 1970-01-01 00:00:00.000000000 +0000
Change: 1970-01-01 00:00:00.000000000 +0000
testcase busybox stat test.txt success
hello world
ccccccc
bbbbbbb
aaaaaaa
2222222
1111111
bbbbbbb
testcase busybox strings test.txt success
        7         8        60 test.txt
testcase busybox wc test.txt success
testcase busybox [ -f test.txt ] success
hello world
ccccccc
bbbbbbb
aaaaaaa
2222222
1111111
bbbbbbb
testcase busybox more test.txt success
testcase busybox rm test.txt success
testcase busybox mkdir test_dir success
testcase busybox mv test_dir test success
testcase busybox rmdir test success
echo "hello world" > test.txt
grep hello busybox_cmd.txt
testcase busybox grep hello busybox_cmd.txt success
testcase busybox cp busybox_cmd.txt busybox_cmd.bak success
testcase busybox rm busybox_cmd.bak success
./busybox_cmd.txt
testcase busybox find -name "busybox_cmd.txt" success
#### OS COMP TEST GROUP END busybox-glibc ####
#### OS COMP TEST GROUP START lua-musl ####
testcase lua date.lua success
testcase lua file_io.lua success
testcase lua max_min.lua success
testcase lua random.lua success
testcase lua remove.lua success
testcase lua round_num.lua success
testcase lua sin30.lua success
testcase lua sort.lua success
testcase lua strings.lua success
#### OS COMP TEST GROUP END lua-musl ####
#### OS COMP TEST GROUP START lua-glibc ####
testcase lua date.lua success
testcase lua file_io.lua success
testcase lua max_min.lua success
testcase lua random.lua success
testcase lua remove.lua success
testcase lua round_num.lua success
testcase lua sin30.lua success
testcase lua sort.lua success
testcase lua strings.lua success
#### OS COMP TEST GROUP END lua-glibc ####
#### OS COMP TEST GROUP START ltp-musl ####
ltp case list: stable (63 cases, timeout 10s)
========== START ltp access01 ==========
RUN LTP CASE access01
LTP MEMORY access01 before: free_frames=157684 allocated_frames=38485
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, F_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, X_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, X_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, W_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, R_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, R_OK|W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, R_OK|W_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, R_OK|X_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, R_OK|X_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, W_OK|X_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, W_OK|X_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, R_OK|W_OK|X_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_rwx, R_OK|W_OK|X_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_x, X_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_x, X_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_w, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_w, W_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_r, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_r, R_OK) as nobody passed
access01.c:242: [1;32mTPASS: [0maccess(accessfile_r, X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_r, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_r, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_w, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_w, X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_w, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_x, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_x, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_r, W_OK|X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_r, W_OK|X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_r, R_OK|X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_r, R_OK|X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_r, R_OK|W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_r, R_OK|W_OK|X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_r, R_OK|W_OK|X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_w, W_OK|X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_w, W_OK|X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_w, R_OK|X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_w, R_OK|X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_w, R_OK|W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_w, R_OK|W_OK|X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_w, R_OK|W_OK|X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_x, W_OK|X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_x, R_OK|X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_x, R_OK|W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessfile_x, R_OK|W_OK|X_OK) as nobody : EACCES (13)
access01.c:245: [1;32mTPASS: [0maccess(accessfile_r, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_r, R_OK|W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_w, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_w, R_OK|W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_x, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_x, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessfile_x, R_OK|W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_r/accessfile_r, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_r/accessfile_r, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_r/accessfile_r, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_r/accessfile_w, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_r/accessfile_w, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_r/accessfile_w, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_r/accessfile_x, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_r/accessfile_x, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_r/accessfile_x, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_r/accessfile_x, X_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_w/accessfile_r, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_w/accessfile_r, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_w/accessfile_r, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_w/accessfile_w, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_w/accessfile_w, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_w/accessfile_w, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_w/accessfile_x, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_w/accessfile_x, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_w/accessfile_x, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_w/accessfile_x, X_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_r, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_r, F_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_r, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_r, R_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_r, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_w, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_w, F_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_w, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_w, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_w, W_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_x, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_x, F_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_x, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_x, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_x, X_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_x/accessfile_x, X_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_r, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_r, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_r, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_w, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_w, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_w, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_x, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_x, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_x, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_x, X_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_r, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_r, F_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_r, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_r, R_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_r, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_w, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_w, F_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_w, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_w, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_w, W_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_x, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_x, F_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_x, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_x, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_x, X_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_x, X_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_r, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_r, F_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_r, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_r, R_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_r, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_w, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_w, F_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_w, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_w, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_w, W_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_x, F_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_x, F_OK) as nobody passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_x, R_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_x, W_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_x, X_OK) as root passed
access01.c:245: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_x, X_OK) as nobody passed
access01.c:242: [1;32mTPASS: [0maccess(accessdir_r/accessfile_r, F_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_r/accessfile_r, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_r/accessfile_r, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_r/accessfile_r, X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_r/accessfile_r, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_r/accessfile_w, F_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_r/accessfile_w, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_r/accessfile_w, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_r/accessfile_w, X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_r/accessfile_w, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_r/accessfile_x, F_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_r/accessfile_x, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_r/accessfile_x, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_r/accessfile_x, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_w/accessfile_r, F_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_w/accessfile_r, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_w/accessfile_r, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_w/accessfile_r, X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_w/accessfile_r, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_w/accessfile_w, F_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_w/accessfile_w, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_w/accessfile_w, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_w/accessfile_w, X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_w/accessfile_w, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_w/accessfile_x, F_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_w/accessfile_x, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_w/accessfile_x, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_w/accessfile_x, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_x/accessfile_r, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_x/accessfile_r, X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_x/accessfile_r, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_x/accessfile_w, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_x/accessfile_w, X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_x/accessfile_w, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_x/accessfile_x, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_x/accessfile_x, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_r, F_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_r, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_r, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_r, X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_r, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_w, F_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_w, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_w, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_w, X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_w, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_x, F_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_x, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_x, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rw/accessfile_x, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_r, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_r, X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_r, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_w, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_w, X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_w, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_x, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_rx/accessfile_x, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_r, W_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_r, X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_r, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_w, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_w, X_OK) as root : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_w, X_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_x, R_OK) as nobody : EACCES (13)
access01.c:242: [1;32mTPASS: [0maccess(accessdir_wx/accessfile_x, W_OK) as nobody : EACCES (13)

Summary:
passed   199
failed   0
broken   0
skipped  0
warnings 0
[37m[618.292372 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.293763 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.294563 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.296152 0:792 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[618.297646 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.300951 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.305094 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.306076 0:792 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[618.308960 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.309956 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.311846 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.314326 0:792 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[618.317339 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.318162 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.318865 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.322801 0:792 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[618.324248 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.325033 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.325825 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.326696 0:792 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[618.328148 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.328986 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.329818 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.331606 0:792 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[618.332637 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.333370 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.334100 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.334806 0:792 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.335937 0:792 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE access01 : 0
Pass!
LTP MEMORY access01 after_run: free_frames=156860 allocated_frames=39309
LTP MEMORY access01 after_cleanup: free_frames=156860 allocated_frames=39309
LTP CASE RUNTIME access01: 4985 ms
========== END ltp access01 ==========
========== START ltp brk01 ==========
RUN LTP CASE brk01
LTP MEMORY brk01 before: free_frames=156860 allocated_frames=39309
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
brk01.c:24: [1;34mTINFO: [0mTesting libc variant
brk01.c:70: [1;32mTPASS: [0mbrk() works fine
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
brk01.c:21: [1;34mTINFO: [0mTesting syscall variant
brk01.c:70: [1;32mTPASS: [0mbrk() works fine

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[619.815412 0:897 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE brk01 : 0
Pass!
LTP MEMORY brk01 after_run: free_frames=156836 allocated_frames=39333
LTP MEMORY brk01 after_cleanup: free_frames=156836 allocated_frames=39333
LTP CASE RUNTIME brk01: 1483 ms
========== END ltp brk01 ==========
========== START ltp chdir01 ==========
RUN LTP CASE chdir01
LTP MEMORY chdir01 before: free_frames=156836 allocated_frames=39333
tst_buffers.c:57: [1;34mTINFO: [0mTest is using guarded buffers
tst_device.c:317: [1;34mTINFO: [0mUsing test device LTP_DEV='/dev/vda'
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_supported_fs_types.c:105: [1;34mTINFO: [0mSkipping bcachefs because of FUSE blacklist
tst_supported_fs_types.c:97: [1;34mTINFO: [0mKernel supports tmpfs
tst_supported_fs_types.c:49: [1;34mTINFO: [0mmkfs is not needed for tmpfs
tst_test.c:1693: [1;34mTINFO: [0m=== Testing on tmpfs ===
tst_test.c:1106: [1;34mTINFO: [0mSkipping mkfs for TMPFS filesystem
tst_test.c:1087: [1;34mTINFO: [0mLimiting tmpfs size to 32MB
tst_test.c:1120: [1;34mTINFO: [0mMounting ltp-tmpfs to /tmp/ltp-work/LTP_chdpCdlbK/mntpoint fstyp=tmpfs flags=0
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
chdir01.c:119: [1;34mTINFO: [0mTesting 'testfile'
chdir01.c:111: [1;32mTPASS: [0mroot: chdir("testfile") returned correct value: ENOTDIR (20)
chdir01.c:111: [1;32mTPASS: [0mnobody: chdir("testfile") returned correct value: ENOTDIR (20)
chdir01.c:119: [1;34mTINFO: [0mTesting 'keep_out'
chdir01.c:111: [1;32mTPASS: [0mroot: chdir("keep_out") returned correct value: SUCCESS (0)
chdir01.c:111: [1;32mTPASS: [0mnobody: chdir("keep_out") returned correct value: EACCES (13)
chdir01.c:119: [1;34mTINFO: [0mTesting 'subdir'
chdir01.c:111: [1;32mTPASS: [0mroot: chdir("subdir") returned correct value: SUCCESS (0)
chdir01.c:111: [1;32mTPASS: [0mnobody: chdir("subdir") returned correct value: SUCCESS (0)
chdir01.c:119: [1;34mTINFO: [0mTesting '.'
chdir01.c:111: [1;32mTPASS: [0mroot: chdir(".") returned correct value: SUCCESS (0)
chdir01.c:111: [1;32mTPASS: [0mnobody: chdir(".") returned correct value: SUCCESS (0)
chdir01.c:119: [1;34mTINFO: [0mTesting '..'
chdir01.c:111: [1;32mTPASS: [0mroot: chdir("..") returned correct value: SUCCESS (0)
chdir01.c:111: [1;32mTPASS: [0mnobody: chdir("..") returned correct value: SUCCESS (0)
chdir01.c:119: [1;34mTINFO: [0mTesting '/'
chdir01.c:111: [1;32mTPASS: [0mroot: chdir("/") returned correct value: SUCCESS (0)
chdir01.c:111: [1;32mTPASS: [0mnobody: chdir("/") returned correct value: SUCCESS (0)
chdir01.c:119: [1;34mTINFO: [0mTesting 'does_not_exist'
chdir01.c:111: [1;32mTPASS: [0mroot: chdir("does_not_exist") returned correct value: ENOENT (2)
chdir01.c:111: [1;32mTPASS: [0mnobody: chdir("does_not_exist") returned correct value: ENOENT (2)
chdir01.c:119: [1;34mTINFO: [0mTesting 'symloop'
chdir01.c:111: [1;32mTPASS: [0mroot: chdir("symloop") returned correct value: ELOOP (40)
chdir01.c:111: [1;32mTPASS: [0mnobody: chdir("symloop") returned correct value: ELOOP (40)

Summary:
passed   16
failed   0
broken   0
skipped  0
warnings 0
[37m[621.546971 0:904 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[621.549403 0:904 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[621.551037 0:904 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[621.552934 0:904 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[621.554705 0:904 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE chdir01 : 0
Pass!
LTP MEMORY chdir01 after_run: free_frames=156820 allocated_frames=39349
LTP MEMORY chdir01 after_cleanup: free_frames=156820 allocated_frames=39349
LTP CASE RUNTIME chdir01: 1733 ms
========== END ltp chdir01 ==========
========== START ltp clone01 ==========
RUN LTP CASE clone01
LTP MEMORY clone01 before: free_frames=156820 allocated_frames=39349
tst_buffers.c:57: [1;34mTINFO: [0mTest is using guarded buffers
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
clone01.c:37: [1;32mTPASS: [0mclone returned 911
clone01.c:43: [1;32mTPASS: [0mChild exited with 0

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[623.007818 0:908 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE clone01 : 0
Pass!
LTP MEMORY clone01 after_run: free_frames=156796 allocated_frames=39373
LTP MEMORY clone01 after_cleanup: free_frames=156796 allocated_frames=39373
LTP CASE RUNTIME clone01: 1448 ms
========== END ltp clone01 ==========
========== START ltp close01 ==========
RUN LTP CASE close01
LTP MEMORY close01 before: free_frames=156796 allocated_frames=39373
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
close01.c:50: [1;32mTPASS: [0mclose a file fd passed
close01.c:50: [1;32mTPASS: [0mclose a pipe fd passed
close01.c:50: [1;32mTPASS: [0mclose a socket fd passed

Summary:
passed   3
failed   0
broken   0
skipped  0
warnings 0
[37m[624.509061 0:913 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[624.511269 0:913 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE close01 : 0
Pass!
LTP MEMORY close01 after_run: free_frames=156780 allocated_frames=39389
LTP MEMORY close01 after_cleanup: free_frames=156780 allocated_frames=39389
LTP CASE RUNTIME close01: 1504 ms
========== END ltp close01 ==========
========== START ltp dup01 ==========
RUN LTP CASE dup01
LTP MEMORY dup01 before: free_frames=156780 allocated_frames=39389
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
dup01.c:24: [1;32mTPASS: [0mdup(fd) returned fd 4
dup01.c:27: [1;32mTPASS: [0mbuf1.st_ino == buf2.st_ino (678513391814680296)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[626.059114 0:917 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[626.062208 0:917 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE dup01 : 0
Pass!
LTP MEMORY dup01 after_run: free_frames=156764 allocated_frames=39405
LTP MEMORY dup01 after_cleanup: free_frames=156764 allocated_frames=39405
LTP CASE RUNTIME dup01: 1550 ms
========== END ltp dup01 ==========
========== START ltp fcntl01 ==========
RUN LTP CASE fcntl01
LTP MEMORY fcntl01 before: free_frames=156764 allocated_frames=39405
[37m[627.513273 0:921 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fcntl01 : 0
Pass!
LTP MEMORY fcntl01 after_run: free_frames=156756 allocated_frames=39413
LTP MEMORY fcntl01 after_cleanup: free_frames=156756 allocated_frames=39413
LTP CASE RUNTIME fcntl01: 1443 ms
========== END ltp fcntl01 ==========
========== START ltp fcntl02 ==========
RUN LTP CASE fcntl02
LTP MEMORY fcntl02 before: free_frames=156756 allocated_frames=39413
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_924, F_DUPFD, 0) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_924, F_DUPFD, 1) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_924, F_DUPFD, 2) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_924, F_DUPFD, 3) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_924, F_DUPFD, 10) returned 10
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_924, F_DUPFD, 100) returned 100

Summary:
passed   6
failed   0
broken   0
skipped  0
warnings 0
[37m[629.015356 0:922 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[629.016629 0:922 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fcntl02 : 0
Pass!
LTP MEMORY fcntl02 after_run: free_frames=156740 allocated_frames=39429
LTP MEMORY fcntl02 after_cleanup: free_frames=156740 allocated_frames=39429
LTP CASE RUNTIME fcntl02: 1504 ms
========== END ltp fcntl02 ==========
========== START ltp fork01 ==========
RUN LTP CASE fork01
LTP MEMORY fork01 before: free_frames=156740 allocated_frames=39429
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
fork01.c:47: [1;32mTPASS: [0mcorrect child status returned 42
fork01.c:50: [1;32mTPASS: [0mchild_pid == pid (929)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[630.524569 0:926 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[630.526945 0:926 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fork01 : 0
Pass!
LTP MEMORY fork01 after_run: free_frames=156716 allocated_frames=39453
LTP MEMORY fork01 after_cleanup: free_frames=156716 allocated_frames=39453
LTP CASE RUNTIME fork01: 1513 ms
========== END ltp fork01 ==========
========== START ltp getpid01 ==========
RUN LTP CASE getpid01
LTP MEMORY getpid01 before: free_frames=156716 allocated_frames=39453
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 934
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 935
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 936
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 937
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 938
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 939
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 940
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 941
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 942
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 943
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 944
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 945
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 946
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 947
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 948
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 949
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 950
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 951
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 952
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 953
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 954
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 955
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 956
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 957
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 958
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 959
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 960
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 961
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 962
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 963
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 964
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 965
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 966
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 967
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 968
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 969
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 970
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 971
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 972
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 973
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 974
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 975
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 976
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 977
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 978
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 979
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 980
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 981
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 982
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 983
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 984
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 985
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 986
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 987
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 988
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 989
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 990
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 991
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 992
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 993
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 994
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 995
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 996
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 997
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 998
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 999
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1000
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1001
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1002
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1003
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1004
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1005
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1006
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1007
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1008
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1009
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1010
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1011
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1012
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1013
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1014
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1015
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1016
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1017
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1018
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1019
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1020
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1021
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1022
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1023
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1024
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1025
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1026
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1027
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1028
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1029
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1030
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1031
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1032
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1033

Summary:
passed   100
failed   0
broken   0
skipped  0
warnings 0
[37m[634.933487 0:931 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpid01 : 0
Pass!
LTP MEMORY getpid01 after_run: free_frames=155900 allocated_frames=40269
LTP MEMORY getpid01 after_cleanup: free_frames=155900 allocated_frames=40269
LTP CASE RUNTIME getpid01: 4401 ms
========== END ltp getpid01 ==========
========== START ltp mmap01 ==========
RUN LTP CASE mmap01
LTP MEMORY mmap01 before: free_frames=155900 allocated_frames=40269
mmap01      1  [1;32mTPASS[0m  :  Functionality of mmap() successful
[37m[636.399059 0:1035 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[636.402216 0:1035 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE mmap01 : 0
Pass!
LTP MEMORY mmap01 after_run: free_frames=155884 allocated_frames=40285
LTP MEMORY mmap01 after_cleanup: free_frames=155884 allocated_frames=40285
LTP CASE RUNTIME mmap01: 1474 ms
========== END ltp mmap01 ==========
========== START ltp open01 ==========
RUN LTP CASE open01
LTP MEMORY open01 before: free_frames=155884 allocated_frames=40285
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
open01.c:59: [1;32mTPASS: [0msticky bit is set as expected
open01.c:59: [1;32mTPASS: [0msirectory bit is set as expected

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[637.971424 0:1037 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[637.972572 0:1037 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE open01 : 0
Pass!
LTP MEMORY open01 after_run: free_frames=155868 allocated_frames=40301
LTP MEMORY open01 after_cleanup: free_frames=155868 allocated_frames=40301
LTP CASE RUNTIME open01: 1556 ms
========== END ltp open01 ==========
========== START ltp pipe01 ==========
RUN LTP CASE pipe01
LTP MEMORY pipe01 before: free_frames=155868 allocated_frames=40301
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
pipe01.c:48: [1;32mTPASS: [0mpipe() functionality is correct

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[639.493510 0:1041 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE pipe01 : 0
Pass!
LTP MEMORY pipe01 after_run: free_frames=155852 allocated_frames=40317
LTP MEMORY pipe01 after_cleanup: free_frames=155852 allocated_frames=40317
LTP CASE RUNTIME pipe01: 1520 ms
========== END ltp pipe01 ==========
========== START ltp read01 ==========
RUN LTP CASE read01
LTP MEMORY read01 before: free_frames=155852 allocated_frames=40317
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
read01.c:24: [1;32mTPASS: [0mread(2) returned 512

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[640.931251 0:1045 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[640.933114 0:1045 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE read01 : 0
Pass!
LTP MEMORY read01 after_run: free_frames=155836 allocated_frames=40333
LTP MEMORY read01 after_cleanup: free_frames=155836 allocated_frames=40333
LTP CASE RUNTIME read01: 1440 ms
========== END ltp read01 ==========
========== START ltp stat01 ==========
RUN LTP CASE stat01
LTP MEMORY stat01 before: free_frames=155836 allocated_frames=40333
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
stat01.c:43: [1;32mTPASS: [0mstat(tc->pathname, &stat_buf) passed
stat01.c:45: [1;32mTPASS: [0mstat_buf.st_uid == user_id (65534)
stat01.c:46: [1;32mTPASS: [0mstat_buf.st_gid == group_id (0)
stat01.c:47: [1;32mTPASS: [0mstat_buf.st_size == FILE_SIZE (1024)
stat01.c:48: [1;32mTPASS: [0mstat_buf.st_mode & MASK == tc->mode (438)
stat01.c:49: [1;32mTPASS: [0mstat_buf.st_nlink == 1 (1)
stat01.c:43: [1;32mTPASS: [0mstat(tc->pathname, &stat_buf) passed
stat01.c:45: [1;32mTPASS: [0mstat_buf.st_uid == user_id (65534)
stat01.c:46: [1;32mTPASS: [0mstat_buf.st_gid == group_id (0)
stat01.c:47: [1;32mTPASS: [0mstat_buf.st_size == FILE_SIZE (1024)
stat01.c:48: [1;32mTPASS: [0mstat_buf.st_mode & MASK == tc->mode (146)
stat01.c:49: [1;32mTPASS: [0mstat_buf.st_nlink == 1 (1)

Summary:
passed   12
failed   0
broken   0
skipped  0
warnings 0
[37m[642.505262 0:1049 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[642.506045 0:1049 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[642.507076 0:1049 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE stat01 : 0
Pass!
LTP MEMORY stat01 after_run: free_frames=155820 allocated_frames=40349
LTP MEMORY stat01 after_cleanup: free_frames=155820 allocated_frames=40349
LTP CASE RUNTIME stat01: 1571 ms
========== END ltp stat01 ==========
========== START ltp wait401 ==========
RUN LTP CASE wait401
LTP MEMORY wait401 before: free_frames=155820 allocated_frames=40349
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
wait401.c:40: [1;32mTPASS: [0mwait4() returned correct pid 1056
wait401.c:49: [1;32mTPASS: [0mWIFEXITED() is set in status
wait401.c:54: [1;32mTPASS: [0mWEXITSTATUS() == 0

Summary:
passed   3
failed   0
broken   0
skipped  0
warnings 0
[37m[644.009243 0:1053 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE wait401 : 0
Pass!
LTP MEMORY wait401 after_run: free_frames=155796 allocated_frames=40373
LTP MEMORY wait401 after_cleanup: free_frames=155796 allocated_frames=40373
LTP CASE RUNTIME wait401: 1502 ms
========== END ltp wait401 ==========
========== START ltp write01 ==========
RUN LTP CASE write01
LTP MEMORY write01 before: free_frames=155796 allocated_frames=40373
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
write01.c:40: [1;32mTPASS: [0mwrite() passed

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[645.487171 0:1058 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[645.489035 0:1058 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE write01 : 0
Pass!
LTP MEMORY write01 after_run: free_frames=155780 allocated_frames=40389
LTP MEMORY write01 after_cleanup: free_frames=155780 allocated_frames=40389
LTP CASE RUNTIME write01: 1478 ms
========== END ltp write01 ==========
========== START ltp access03 ==========
RUN LTP CASE access03
LTP MEMORY access03 before: free_frames=155780 allocated_frames=40389
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
access03.c:37: [1;32mTPASS: [0minvalid address as root : EFAULT (14)
access03.c:46: [1;32mTPASS: [0minvalid address as nobody : EFAULT (14)
access03.c:37: [1;32mTPASS: [0minvalid address as root : EFAULT (14)
access03.c:46: [1;32mTPASS: [0minvalid address as nobody : EFAULT (14)
access03.c:37: [1;32mTPASS: [0minvalid address as root : EFAULT (14)
access03.c:46: [1;32mTPASS: [0minvalid address as nobody : EFAULT (14)
access03.c:37: [1;32mTPASS: [0minvalid address as root : EFAULT (14)
access03.c:46: [1;32mTPASS: [0minvalid address as nobody : EFAULT (14)

Summary:
passed   8
failed   0
broken   0
skipped  0
warnings 0
[37m[647.186123 0:1062 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE access03 : 0
Pass!
LTP MEMORY access03 after_run: free_frames=155732 allocated_frames=40437
LTP MEMORY access03 after_cleanup: free_frames=155732 allocated_frames=40437
LTP CASE RUNTIME access03: 1705 ms
========== END ltp access03 ==========
========== START ltp close02 ==========
RUN LTP CASE close02
LTP MEMORY close02 before: free_frames=155732 allocated_frames=40437
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
close02.c:20: [1;32mTPASS: [0mclose(-1) : EBADF (9)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[648.624206 0:1070 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE close02 : 0
Pass!
LTP MEMORY close02 after_run: free_frames=155716 allocated_frames=40453
LTP MEMORY close02 after_cleanup: free_frames=155716 allocated_frames=40453
LTP CASE RUNTIME close02: 1432 ms
========== END ltp close02 ==========
========== START ltp dup02 ==========
RUN LTP CASE dup02
LTP MEMORY dup02 before: free_frames=155716 allocated_frames=40453
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
dup02.c:29: [1;32mTPASS: [0mdup(-1) : EBADF (9)
dup02.c:29: [1;32mTPASS: [0mdup(1500) : EBADF (9)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[650.067895 0:1074 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE dup02 : 0
Pass!
LTP MEMORY dup02 after_run: free_frames=155700 allocated_frames=40469
LTP MEMORY dup02 after_cleanup: free_frames=155700 allocated_frames=40469
LTP CASE RUNTIME dup02: 1448 ms
========== END ltp dup02 ==========
========== START ltp fcntl03 ==========
RUN LTP CASE fcntl03
LTP MEMORY fcntl03 before: free_frames=155700 allocated_frames=40469
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
fcntl03.c:32: [1;32mTPASS: [0mfcntl(fcntl03_1080, F_GETFD, 0) returned 0

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[651.648368 0:1078 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[651.652852 0:1078 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fcntl03 : 0
Pass!
LTP MEMORY fcntl03 after_run: free_frames=155684 allocated_frames=40485
LTP MEMORY fcntl03 after_cleanup: free_frames=155684 allocated_frames=40485
LTP CASE RUNTIME fcntl03: 1585 ms
========== END ltp fcntl03 ==========
========== START ltp getcwd01 ==========
RUN LTP CASE getcwd01
LTP MEMORY getcwd01 before: free_frames=155684 allocated_frames=40485
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getcwd01.c:48: [1;32mTPASS: [0mtst_syscall(__NR_getcwd, tc->buf, tc->size) : EFAULT (14)
getcwd01.c:48: [1;32mTPASS: [0mtst_syscall(__NR_getcwd, tc->buf, tc->size) : EFAULT (14)
getcwd01.c:48: [1;32mTPASS: [0mtst_syscall(__NR_getcwd, tc->buf, tc->size) : ERANGE (34)
getcwd01.c:48: [1;32mTPASS: [0mtst_syscall(__NR_getcwd, tc->buf, tc->size) : ERANGE (34)
getcwd01.c:48: [1;32mTPASS: [0mtst_syscall(__NR_getcwd, tc->buf, tc->size) : ERANGE (34)

Summary:
passed   5
failed   0
broken   0
skipped  0
warnings 0
[37m[653.207191 0:1082 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getcwd01 : 0
Pass!
LTP MEMORY getcwd01 after_run: free_frames=155668 allocated_frames=40501
LTP MEMORY getcwd01 after_cleanup: free_frames=155668 allocated_frames=40501
LTP CASE RUNTIME getcwd01: 1550 ms
========== END ltp getcwd01 ==========
========== START ltp getpid02 ==========
RUN LTP CASE getpid02
LTP MEMORY getpid02 before: free_frames=155668 allocated_frames=40501
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getpid02.c:37: [1;32mTPASS: [0mchild getppid() == parent getpid() (1088)
getpid02.c:50: [1;32mTPASS: [0mchild getpid() == parent fork() (1089)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[654.734407 0:1086 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpid02 : 0
Pass!
LTP MEMORY getpid02 after_run: free_frames=155644 allocated_frames=40525
LTP MEMORY getpid02 after_cleanup: free_frames=155644 allocated_frames=40525
LTP CASE RUNTIME getpid02: 1517 ms
========== END ltp getpid02 ==========
========== START ltp getppid01 ==========
RUN LTP CASE getppid01
LTP MEMORY getppid01 before: free_frames=155644 allocated_frames=40525
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getppid01.c:31: [1;32mTPASS: [0mgetppid() returned 1091

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[656.352221 0:1091 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getppid01 : 0
Pass!
LTP MEMORY getppid01 after_run: free_frames=155628 allocated_frames=40541
LTP MEMORY getppid01 after_cleanup: free_frames=155628 allocated_frames=40541
LTP CASE RUNTIME getppid01: 1611 ms
========== END ltp getppid01 ==========
========== START ltp getuid01 ==========
RUN LTP CASE getuid01
LTP MEMORY getuid01 before: free_frames=155628 allocated_frames=40541
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getuid01.c:20: [1;32mTPASS: [0mgetuid() returned 0

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[657.909591 0:1095 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getuid01 : 0
Pass!
LTP MEMORY getuid01 after_run: free_frames=155612 allocated_frames=40557
LTP MEMORY getuid01 after_cleanup: free_frames=155612 allocated_frames=40557
LTP CASE RUNTIME getuid01: 1560 ms
========== END ltp getuid01 ==========
========== START ltp geteuid01 ==========
RUN LTP CASE geteuid01
LTP MEMORY geteuid01 before: free_frames=155612 allocated_frames=40557
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
geteuid01.c:21: [1;32mTPASS: [0mgeteuid() returned 0

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[659.381002 0:1099 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE geteuid01 : 0
Pass!
LTP MEMORY geteuid01 after_run: free_frames=155596 allocated_frames=40573
LTP MEMORY geteuid01 after_cleanup: free_frames=155596 allocated_frames=40573
LTP CASE RUNTIME geteuid01: 1466 ms
========== END ltp geteuid01 ==========
========== START ltp getgid01 ==========
RUN LTP CASE getgid01
LTP MEMORY getgid01 before: free_frames=155596 allocated_frames=40573
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getgid01.c:26: [1;32mTPASS: [0mgetgid returned as expectedly

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[660.990801 0:1103 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getgid01 : 0
Pass!
LTP MEMORY getgid01 after_run: free_frames=155580 allocated_frames=40589
LTP MEMORY getgid01 after_cleanup: free_frames=155580 allocated_frames=40589
LTP CASE RUNTIME getgid01: 1590 ms
========== END ltp getgid01 ==========
========== START ltp getegid01 ==========
RUN LTP CASE getegid01
LTP MEMORY getegid01 before: free_frames=155580 allocated_frames=40589
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getegid01.c:25: [1;32mTPASS: [0mgid == st_egid (0)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[662.472591 0:1107 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getegid01 : 0
Pass!
LTP MEMORY getegid01 after_run: free_frames=155564 allocated_frames=40605
LTP MEMORY getegid01 after_cleanup: free_frames=155564 allocated_frames=40605
LTP CASE RUNTIME getegid01: 1480 ms
========== END ltp getegid01 ==========
========== START ltp lseek01 ==========
RUN LTP CASE lseek01
LTP MEMORY lseek01 before: free_frames=155564 allocated_frames=40605
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
lseek01.c:67: [1;32mTPASS: [0mlseek(tfile, 4, SEEK_SET) read correct data
lseek01.c:67: [1;32mTPASS: [0mlseek(tfile, -2, SEEK_CUR) read correct data
lseek01.c:67: [1;32mTPASS: [0mlseek(tfile, -4, SEEK_END) read correct data
lseek01.c:67: [1;32mTPASS: [0mlseek(tfile, 0, SEEK_END) read correct data

Summary:
passed   4
failed   0
broken   0
skipped  0
warnings 0
[37m[663.917841 0:1111 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[663.920605 0:1111 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE lseek01 : 0
Pass!
LTP MEMORY lseek01 after_run: free_frames=155548 allocated_frames=40621
LTP MEMORY lseek01 after_cleanup: free_frames=155548 allocated_frames=40621
LTP CASE RUNTIME lseek01: 1458 ms
========== END ltp lseek01 ==========
========== START ltp read02 ==========
RUN LTP CASE read02
LTP MEMORY read02 before: free_frames=155548 allocated_frames=40621
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
read02.c:84: [1;32mTPASS: [0mread() failed as expected: EBADF (9)
read02.c:84: [1;32mTPASS: [0mread() failed as expected: EISDIR (21)
read02.c:84: [1;32mTPASS: [0mread() failed as expected: EFAULT (14)
read02.c:65: [1;33mTCONF: [0mO_DIRECT not supported on tmpfs filesystem
read02.c:65: [1;33mTCONF: [0mO_DIRECT not supported on tmpfs filesystem

Summary:
passed   3
failed   0
broken   0
skipped  2
warnings 0
[37m[665.441233 0:1115 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[665.444089 0:1115 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE read02 : 0
Pass!
LTP MEMORY read02 after_run: free_frames=155532 allocated_frames=40637
LTP MEMORY read02 after_cleanup: free_frames=155532 allocated_frames=40637
LTP CASE RUNTIME read02: 1527 ms
========== END ltp read02 ==========
========== START ltp write02 ==========
RUN LTP CASE write02
LTP MEMORY write02 before: free_frames=155532 allocated_frames=40637
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
write02.c:20: [1;32mTPASS: [0mwrite(fd, NULL, 0) returned 0
write02.c:22: [1;32mTPASS: [0mExpect: write(fd, NULL, 0) == 0

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[666.983920 0:1119 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.985898 0:1119 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE write02 : 0
Pass!
LTP MEMORY write02 after_run: free_frames=155516 allocated_frames=40653
LTP MEMORY write02 after_cleanup: free_frames=155516 allocated_frames=40653
LTP CASE RUNTIME write02: 1524 ms
========== END ltp write02 ==========
========== START ltp creat01 ==========
RUN LTP CASE creat01
LTP MEMORY creat01 before: free_frames=155516 allocated_frames=40653
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
creat01.c:50: [1;32mTPASS: [0mcreat() truncated file to 0 bytes
creat01.c:55: [1;32mTPASS: [0mfile was created and written to successfully
creat01.c:60: [1;32mTPASS: [0mread failed expectedly: EACCES (13)
creat01.c:50: [1;32mTPASS: [0mcreat() truncated file to 0 bytes
creat01.c:55: [1;32mTPASS: [0mfile was created and written to successfully
creat01.c:60: [1;32mTPASS: [0mread failed expectedly: EACCES (13)

Summary:
passed   6
failed   0
broken   0
skipped  0
warnings 0
[37m[668.485457 0:1123 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[668.486554 0:1123 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE creat01 : 0
Pass!
LTP MEMORY creat01 after_run: free_frames=155500 allocated_frames=40669
LTP MEMORY creat01 after_cleanup: free_frames=155500 allocated_frames=40669
LTP CASE RUNTIME creat01: 1498 ms
========== END ltp creat01 ==========
========== START ltp creat03 ==========
RUN LTP CASE creat03
LTP MEMORY creat03 before: free_frames=155500 allocated_frames=40669
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
creat03.c:36: [1;34mTINFO: [0mCreated file has mode = 0100674
creat03.c:41: [1;32mTPASS: [0msave text bit cleared

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[669.951411 0:1127 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[669.952698 0:1127 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE creat03 : 0
Pass!
LTP MEMORY creat03 after_run: free_frames=155484 allocated_frames=40685
LTP MEMORY creat03 after_cleanup: free_frames=155484 allocated_frames=40685
LTP CASE RUNTIME creat03: 1465 ms
========== END ltp creat03 ==========
========== START ltp open02 ==========
RUN LTP CASE open02
LTP MEMORY open02 before: free_frames=155484 allocated_frames=40685
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
open02.c:49: [1;32mTPASS: [0mopen() new file without O_CREAT : ENOENT (2)
open02.c:49: [1;32mTPASS: [0mopen() unprivileged O_RDONLY | O_NOATIME : EPERM (1)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[671.545070 0:1131 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[671.547375 0:1131 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE open02 : 0
Pass!
LTP MEMORY open02 after_run: free_frames=155468 allocated_frames=40701
LTP MEMORY open02 after_cleanup: free_frames=155468 allocated_frames=40701
LTP CASE RUNTIME open02: 1597 ms
========== END ltp open02 ==========
========== START ltp open03 ==========
RUN LTP CASE open03
LTP MEMORY open03 before: free_frames=155468 allocated_frames=40701
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
open03.c:19: [1;32mTPASS: [0mopen(TEST_FILE, O_RDWR | O_CREAT, 0700) returned fd 3

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[673.042156 0:1135 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE open03 : 0
Pass!
LTP MEMORY open03 after_run: free_frames=155452 allocated_frames=40717
LTP MEMORY open03 after_cleanup: free_frames=155452 allocated_frames=40717
LTP CASE RUNTIME open03: 1493 ms
========== END ltp open03 ==========
========== START ltp stat02 ==========
RUN LTP CASE stat02
LTP MEMORY stat02 before: free_frames=155452 allocated_frames=40717
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
stat02.c:67: [1;32mTPASS: [0mFile size reported as expected
stat02.c:67: [1;32mTPASS: [0mFile size reported as expected

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[674.481539 0:1139 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE stat02 : 0
Pass!
LTP MEMORY stat02 after_run: free_frames=155436 allocated_frames=40733
LTP MEMORY stat02 after_cleanup: free_frames=155436 allocated_frames=40733
LTP CASE RUNTIME stat02: 1434 ms
========== END ltp stat02 ==========
========== START ltp lstat01 ==========
RUN LTP CASE lstat01
LTP MEMORY lstat01 before: free_frames=155436 allocated_frames=40733
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
lstat01.c:46: [1;32mTPASS: [0mlstat() reported correct values for the symlink!

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[675.987685 0:1143 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[675.992678 0:1143 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE lstat01 : 0
Pass!
LTP MEMORY lstat01 after_run: free_frames=155420 allocated_frames=40749
LTP MEMORY lstat01 after_cleanup: free_frames=155420 allocated_frames=40749
LTP CASE RUNTIME lstat01: 1525 ms
========== END ltp lstat01 ==========
========== START ltp chmod01 ==========
RUN LTP CASE chmod01
LTP MEMORY chmod01 before: free_frames=155420 allocated_frames=40749
tst_buffers.c:57: [1;34mTINFO: [0mTest is using guarded buffers
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
chmod01.c:60: [1;34mTINFO: [0mTesting variant: verify permissions of file
chmod01.c:40: [1;32mTPASS: [0mchmod(testfile, 0000) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testfile) mode=0000
chmod01.c:40: [1;32mTPASS: [0mchmod(testfile, 0007) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testfile) mode=0007
chmod01.c:40: [1;32mTPASS: [0mchmod(testfile, 0070) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testfile) mode=0070
chmod01.c:40: [1;32mTPASS: [0mchmod(testfile, 0700) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testfile) mode=0700
chmod01.c:40: [1;32mTPASS: [0mchmod(testfile, 0777) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testfile) mode=0777
chmod01.c:40: [1;32mTPASS: [0mchmod(testfile, 2777) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testfile) mode=2777
chmod01.c:40: [1;32mTPASS: [0mchmod(testfile, 4777) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testfile) mode=4777
chmod01.c:40: [1;32mTPASS: [0mchmod(testfile, 6777) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testfile) mode=6777
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
chmod01.c:60: [1;34mTINFO: [0mTesting variant: verify permissions of directory
chmod01.c:40: [1;32mTPASS: [0mchmod(testdir_1, 0000) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testdir_1) mode=0000
chmod01.c:40: [1;32mTPASS: [0mchmod(testdir_1, 0007) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testdir_1) mode=0007
chmod01.c:40: [1;32mTPASS: [0mchmod(testdir_1, 0070) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testdir_1) mode=0070
chmod01.c:40: [1;32mTPASS: [0mchmod(testdir_1, 0700) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testdir_1) mode=0700
chmod01.c:40: [1;32mTPASS: [0mchmod(testdir_1, 0777) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testdir_1) mode=0777
chmod01.c:40: [1;32mTPASS: [0mchmod(testdir_1, 2777) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testdir_1) mode=2777
chmod01.c:40: [1;32mTPASS: [0mchmod(testdir_1, 4777) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testdir_1) mode=4777
chmod01.c:40: [1;32mTPASS: [0mchmod(testdir_1, 6777) passed
chmod01.c:50: [1;32mTPASS: [0mstat(testdir_1) mode=6777

Summary:
passed   32
failed   0
broken   0
skipped  0
warnings 0
[37m[677.548458 0:1147 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[677.549235 0:1147 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[677.550037 0:1147 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE chmod01 : 0
Pass!
LTP MEMORY chmod01 after_run: free_frames=155396 allocated_frames=40773
LTP MEMORY chmod01 after_cleanup: free_frames=155396 allocated_frames=40773
LTP CASE RUNTIME chmod01: 1537 ms
========== END ltp chmod01 ==========
========== START ltp fchmod01 ==========
RUN LTP CASE fchmod01
LTP MEMORY fchmod01 before: free_frames=155396 allocated_frames=40773
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
fchmod01.c:40: [1;32mTPASS: [0mFunctionality of fchmod(3, 0) successful
fchmod01.c:40: [1;32mTPASS: [0mFunctionality of fchmod(3, 07) successful
fchmod01.c:40: [1;32mTPASS: [0mFunctionality of fchmod(3, 070) successful
fchmod01.c:40: [1;32mTPASS: [0mFunctionality of fchmod(3, 0700) successful
fchmod01.c:40: [1;32mTPASS: [0mFunctionality of fchmod(3, 0777) successful
fchmod01.c:40: [1;32mTPASS: [0mFunctionality of fchmod(3, 02777) successful
fchmod01.c:40: [1;32mTPASS: [0mFunctionality of fchmod(3, 04777) successful
fchmod01.c:40: [1;32mTPASS: [0mFunctionality of fchmod(3, 06777) successful

Summary:
passed   8
failed   0
broken   0
skipped  0
warnings 0
[37m[679.044056 0:1154 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[679.045923 0:1154 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fchmod01 : 0
Pass!
LTP MEMORY fchmod01 after_run: free_frames=155380 allocated_frames=40789
LTP MEMORY fchmod01 after_cleanup: free_frames=155380 allocated_frames=40789
LTP CASE RUNTIME fchmod01: 1496 ms
========== END ltp fchmod01 ==========
========== START ltp rmdir01 ==========
RUN LTP CASE rmdir01
LTP MEMORY rmdir01 before: free_frames=155380 allocated_frames=40789
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
rmdir01.c:33: [1;32mTPASS: [0mrmdir(testdir) success

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[680.541123 0:1158 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE rmdir01 : 0
Pass!
LTP MEMORY rmdir01 after_run: free_frames=155364 allocated_frames=40805
LTP MEMORY rmdir01 after_cleanup: free_frames=155364 allocated_frames=40805
LTP CASE RUNTIME rmdir01: 1490 ms
========== END ltp rmdir01 ==========
========== START ltp symlink01 ==========
RUN LTP CASE symlink01
LTP MEMORY symlink01 before: free_frames=155364 allocated_frames=40805
symlink01    1  [1;32mTPASS[0m  :  Creation of symbolic link file to no object file is ok
symlink01    2  [1;32mTPASS[0m  :  Creation of symbolic link file to no object file is ok
symlink01    3  [1;32mTPASS[0m  :  Creation of symbolic link file and object file via symbolic link is ok
symlink01    4  [1;32mTPASS[0m  :  Creating an existing symbolic link file error is caught
symlink01    5  [1;32mTPASS[0m  :  Creating a symbolic link which exceeds maximum pathname error is caught
[37m[682.070813 0:1162 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE symlink01 : 0
Pass!
LTP MEMORY symlink01 after_run: free_frames=155356 allocated_frames=40813
LTP MEMORY symlink01 after_cleanup: free_frames=155356 allocated_frames=40813
LTP CASE RUNTIME symlink01: 1527 ms
========== END ltp symlink01 ==========
========== START ltp readlink01 ==========
RUN LTP CASE readlink01
LTP MEMORY readlink01 before: free_frames=155356 allocated_frames=40813
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
readlink01.c:64: [1;34mTINFO: [0mRunning test as root
readlink01.c:45: [1;32mTPASS: [0mreadlink() functionality on 'slink_file' was correct
readlink01.c:55: [1;34mTINFO: [0mRunning test as nobody
readlink01.c:45: [1;32mTPASS: [0mreadlink() functionality on 'slink_file' was correct

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[683.538177 0:1163 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[683.540856 0:1163 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE readlink01 : 0
Pass!
LTP MEMORY readlink01 after_run: free_frames=155332 allocated_frames=40837
LTP MEMORY readlink01 after_cleanup: free_frames=155332 allocated_frames=40837
LTP CASE RUNTIME readlink01: 1473 ms
========== END ltp readlink01 ==========
========== START ltp ftruncate01 ==========
RUN LTP CASE ftruncate01
LTP MEMORY ftruncate01 before: free_frames=155332 allocated_frames=40837
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
ftruncate01.c:65: [1;34mTINFO: [0mTruncated length smaller than file size
ftruncate01.c:60: [1;32mTPASS: [0mftruncate() succeeded
ftruncate01.c:74: [1;34mTINFO: [0mTruncated length exceeds file size
ftruncate01.c:60: [1;32mTPASS: [0mftruncate() succeeded

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[684.972506 0:1168 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[684.973916 0:1168 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE ftruncate01 : 0
Pass!
LTP MEMORY ftruncate01 after_run: free_frames=155316 allocated_frames=40853
LTP MEMORY ftruncate01 after_cleanup: free_frames=155316 allocated_frames=40853
LTP CASE RUNTIME ftruncate01: 1427 ms
========== END ltp ftruncate01 ==========
========== START ltp umask01 ==========
RUN LTP CASE umask01
LTP MEMORY umask01 before: free_frames=155316 allocated_frames=40853
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
umask01.c:57: [1;32mTPASS: [0mAll files created with correct mode

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[686.712246 0:1172 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE umask01 : 0
Pass!
LTP MEMORY umask01 after_run: free_frames=155300 allocated_frames=40869
LTP MEMORY umask01 after_cleanup: free_frames=155300 allocated_frames=40869
LTP CASE RUNTIME umask01: 1741 ms
========== END ltp umask01 ==========
========== START ltp alarm02 ==========
RUN LTP CASE alarm02
LTP MEMORY alarm02 before: free_frames=155300 allocated_frames=40869
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
alarm02.c:36: [1;32mTPASS: [0malarm(2147483647) passed
alarm02.c:38: [1;32mTPASS: [0malarm(0) passed
alarm02.c:36: [1;32mTPASS: [0malarm(2147483647) passed
alarm02.c:38: [1;32mTPASS: [0malarm(0) passed
alarm02.c:36: [1;32mTPASS: [0malarm(1073741823) passed
alarm02.c:38: [1;32mTPASS: [0malarm(0) passed

Summary:
passed   6
failed   0
broken   0
skipped  0
warnings 0
[37m[688.189774 0:1177 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE alarm02 : 0
Pass!
LTP MEMORY alarm02 after_run: free_frames=155284 allocated_frames=40885
LTP MEMORY alarm02 after_cleanup: free_frames=155284 allocated_frames=40885
LTP CASE RUNTIME alarm02: 1486 ms
========== END ltp alarm02 ==========
========== START ltp alarm03 ==========
RUN LTP CASE alarm03
LTP MEMORY alarm03 before: free_frames=155284 allocated_frames=40885
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
alarm03.c:30: [1;32mTPASS: [0malarm(0) in parent process passed
alarm03.c:26: [1;32mTPASS: [0malarm(0) in child process passed

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[689.713891 0:1184 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE alarm03 : 0
Pass!
LTP MEMORY alarm03 after_run: free_frames=155260 allocated_frames=40909
LTP MEMORY alarm03 after_cleanup: free_frames=155260 allocated_frames=40909
LTP CASE RUNTIME alarm03: 1505 ms
========== END ltp alarm03 ==========
========== START ltp clock_gettime02 ==========
RUN LTP CASE clock_gettime02
LTP MEMORY clock_gettime02 before: free_frames=155260 allocated_frames=40909
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
clock_gettime02.c:102: [1;34mTINFO: [0mTesting variant: 0: syscall with old kernel spec
clock_gettime02.c:130: [1;32mTPASS: [0mclock_gettime(2): clock INVALID/UNKNOWN CLOCK failed as expected: EINVAL (22)
clock_gettime02.c:130: [1;32mTPASS: [0mclock_gettime(2): clock INVALID/UNKNOWN CLOCK failed as expected: EINVAL (22)
clock_gettime02.c:130: [1;32mTPASS: [0mclock_gettime(2): clock CLOCK_REALTIME failed as expected: EFAULT (14)
clock_gettime02.c:130: [1;32mTPASS: [0mclock_gettime(2): clock CLOCK_MONOTONIC failed as expected: EFAULT (14)
clock_gettime02.c:130: [1;32mTPASS: [0mclock_gettime(2): clock CLOCK_PROCESS_CPUTIME_ID failed as expected: EFAULT (14)
clock_gettime02.c:130: [1;32mTPASS: [0mclock_gettime(2): clock CLOCK_THREAD_CPUTIME_ID failed as expected: EFAULT (14)
clock_gettime02.c:130: [1;32mTPASS: [0mclock_gettime(2): clock CLOCK_REALTIME_COARSE failed as expected: EFAULT (14)
clock_gettime02.c:130: [1;32mTPASS: [0mclock_gettime(2): clock CLOCK_MONOTONIC_COARSE failed as expected: EFAULT (14)
clock_gettime02.c:130: [1;32mTPASS: [0mclock_gettime(2): clock CLOCK_MONOTONIC_RAW failed as expected: EFAULT (14)
clock_gettime02.c:130: [1;32mTPASS: [0mclock_gettime(2): clock CLOCK_BOOTTIME failed as expected: EFAULT (14)

Summary:
passed   10
failed   0
broken   0
skipped  0
warnings 0
[37m[691.328573 0:1191 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE clock_gettime02 : 0
Pass!
LTP MEMORY clock_gettime02 after_run: free_frames=155244 allocated_frames=40925
LTP MEMORY clock_gettime02 after_cleanup: free_frames=155244 allocated_frames=40925
LTP CASE RUNTIME clock_gettime02: 1615 ms
========== END ltp clock_gettime02 ==========
========== START ltp gettimeofday01 ==========
RUN LTP CASE gettimeofday01
LTP MEMORY gettimeofday01 before: free_frames=155244 allocated_frames=40925
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
gettimeofday01.c:36: [1;32mTPASS: [0mtst_syscall(__NR_gettimeofday, tc->tv, tc->tz) : EFAULT (14)
gettimeofday01.c:36: [1;32mTPASS: [0mtst_syscall(__NR_gettimeofday, tc->tv, tc->tz) : EFAULT (14)
gettimeofday01.c:36: [1;32mTPASS: [0mtst_syscall(__NR_gettimeofday, tc->tv, tc->tz) : EFAULT (14)

Summary:
passed   3
failed   0
broken   0
skipped  0
warnings 0
[37m[692.983891 0:1195 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE gettimeofday01 : 0
Pass!
LTP MEMORY gettimeofday01 after_run: free_frames=155228 allocated_frames=40941
LTP MEMORY gettimeofday01 after_cleanup: free_frames=155228 allocated_frames=40941
LTP CASE RUNTIME gettimeofday01: 1641 ms
========== END ltp gettimeofday01 ==========
========== START ltp time01 ==========
RUN LTP CASE time01
LTP MEMORY time01 before: free_frames=155228 allocated_frames=40941
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
time01.c:36: [1;32mTPASS: [0mtime() returned value 694
time01.c:38: [1;32mTPASS: [0mtime() returned value 694, stored value 694 are same

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[694.447913 0:1199 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE time01 : 0
Pass!
LTP MEMORY time01 after_run: free_frames=155212 allocated_frames=40957
LTP MEMORY time01 after_cleanup: free_frames=155212 allocated_frames=40957
LTP CASE RUNTIME time01: 1486 ms
========== END ltp time01 ==========
========== START ltp times01 ==========
RUN LTP CASE times01
LTP MEMORY times01 before: free_frames=155212 allocated_frames=40957
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
times01.c:25: [1;32mTPASS: [0mtimes(&mytimes) returned 695962

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[695.973787 0:1203 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE times01 : 0
Pass!
LTP MEMORY times01 after_run: free_frames=155196 allocated_frames=40973
LTP MEMORY times01 after_cleanup: free_frames=155196 allocated_frames=40973
LTP CASE RUNTIME times01: 1497 ms
========== END ltp times01 ==========
========== START ltp kill03 ==========
RUN LTP CASE kill03
LTP MEMORY kill03 before: free_frames=155196 allocated_frames=40973
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
kill03.c:41: [1;32mTPASS: [0mkill failed as expected: EINVAL (22)
kill03.c:41: [1;32mTPASS: [0mkill failed as expected: ESRCH (3)
kill03.c:41: [1;32mTPASS: [0mkill failed as expected: ESRCH (3)

Summary:
passed   3
failed   0
broken   0
skipped  0
warnings 0
[37m[697.453923 0:1207 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE kill03 : 0
Pass!
LTP MEMORY kill03 after_run: free_frames=155180 allocated_frames=40989
LTP MEMORY kill03 after_cleanup: free_frames=155180 allocated_frames=40989
LTP CASE RUNTIME kill03: 1479 ms
========== END ltp kill03 ==========
========== START ltp rt_sigaction01 ==========
RUN LTP CASE rt_sigaction01
LTP MEMORY rt_sigaction01 before: free_frames=155180 allocated_frames=40989
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 35 
rt_sigaction01    1  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 35
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 35 
rt_sigaction01    2  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 35
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 35 
rt_sigaction01    3  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 35
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 35 
rt_sigaction01    4  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 35
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 35 
rt_sigaction01    5  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 35
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 36 
rt_sigaction01    6  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 36
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 36 
rt_sigaction01    7  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 36
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 36 
rt_sigaction01    8  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 36
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 36 
rt_sigaction01    9  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 36
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 36 
rt_sigaction01   10  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 36
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 37 
rt_sigaction01   11  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 37
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 37 
rt_sigaction01   12  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 37
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 37 
rt_sigaction01   13  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 37
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 37 
rt_sigaction01   14  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 37
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 37 
rt_sigaction01   15  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 37
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 38 
rt_sigaction01   16  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 38
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 38 
rt_sigaction01   17  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 38
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 38 
rt_sigaction01   18  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 38
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 38 
rt_sigaction01   19  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 38
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 38 
rt_sigaction01   20  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 38
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 39 
rt_sigaction01   21  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 39
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 39 
rt_sigaction01   22  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 39
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 39 
rt_sigaction01   23  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 39
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 39 
rt_sigaction01   24  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 39
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 39 
rt_sigaction01   25  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 39
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 40 
rt_sigaction01   26  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 40
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 40 
rt_sigaction01   27  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 40
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 40 
rt_sigaction01   28  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 40
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 40 
rt_sigaction01   29  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 40
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 40 
rt_sigaction01   30  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 40
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 41 
rt_sigaction01   31  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 41
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 41 
rt_sigaction01   32  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 41
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 41 
rt_sigaction01   33  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 41
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 41 
rt_sigaction01   34  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 41
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 41 
rt_sigaction01   35  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 41
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 42 
rt_sigaction01   36  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 42
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 42 
rt_sigaction01   37  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 42
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 42 
rt_sigaction01   38  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 42
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 42 
rt_sigaction01   39  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 42
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 42 
rt_sigaction01   40  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 42
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 43 
rt_sigaction01   41  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 43
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 43 
rt_sigaction01   42  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 43
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 43 
rt_sigaction01   43  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 43
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 43 
rt_sigaction01   44  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 43
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 43 
rt_sigaction01   45  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 43
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 44 
rt_sigaction01   46  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 44
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 44 
rt_sigaction01   47  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 44
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 44 
rt_sigaction01   48  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 44
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 44 
rt_sigaction01   49  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 44
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 44 
rt_sigaction01   50  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 44
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 45 
rt_sigaction01   51  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 45
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 45 
rt_sigaction01   52  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 45
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 45 
rt_sigaction01   53  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 45
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 45 
rt_sigaction01   54  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 45
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 45 
rt_sigaction01   55  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 45
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 46 
rt_sigaction01   56  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 46
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 46 
rt_sigaction01   57  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 46
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 46 
rt_sigaction01   58  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 46
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 46 
rt_sigaction01   59  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 46
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 46 
rt_sigaction01   60  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 46
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 47 
rt_sigaction01   61  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 47
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 47 
rt_sigaction01   62  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 47
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 47 
rt_sigaction01   63  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 47
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 47 
rt_sigaction01   64  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 47
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 47 
rt_sigaction01   65  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 47
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 48 
rt_sigaction01   66  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 48
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 48 
rt_sigaction01   67  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 48
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 48 
rt_sigaction01   68  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 48
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 48 
rt_sigaction01   69  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 48
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 48 
rt_sigaction01   70  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 48
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 49 
rt_sigaction01   71  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 49
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 49 
rt_sigaction01   72  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 49
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 49 
rt_sigaction01   73  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 49
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 49 
rt_sigaction01   74  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 49
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 49 
rt_sigaction01   75  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 49
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 50 
rt_sigaction01   76  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 50
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 50 
rt_sigaction01   77  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 50
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 50 
rt_sigaction01   78  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 50
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 50 
rt_sigaction01   79  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 50
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 50 
rt_sigaction01   80  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 50
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 51 
rt_sigaction01   81  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 51
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 51 
rt_sigaction01   82  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 51
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 51 
rt_sigaction01   83  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 51
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 51 
rt_sigaction01   84  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 51
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 51 
rt_sigaction01   85  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 51
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 52 
rt_sigaction01   86  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 52
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 52 
rt_sigaction01   87  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 52
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 52 
rt_sigaction01   88  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 52
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 52 
rt_sigaction01   89  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 52
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 52 
rt_sigaction01   90  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 52
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 53 
rt_sigaction01   91  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 53
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 53 
rt_sigaction01   92  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 53
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 53 
rt_sigaction01   93  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 53
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 53 
rt_sigaction01   94  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 53
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 53 
rt_sigaction01   95  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 53
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 54 
rt_sigaction01   96  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 54
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 54 
rt_sigaction01   97  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 54
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 54 
rt_sigaction01   98  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 54
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 54 
rt_sigaction01   99  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 54
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 54 
rt_sigaction01  100  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 54
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 55 
rt_sigaction01  101  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 55
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 55 
rt_sigaction01  102  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 55
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 55 
rt_sigaction01  103  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 55
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 55 
rt_sigaction01  104  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 55
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 55 
rt_sigaction01  105  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 55
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 56 
rt_sigaction01  106  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 56
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 56 
rt_sigaction01  107  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 56
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 56 
rt_sigaction01  108  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 56
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 56 
rt_sigaction01  109  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 56
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 56 
rt_sigaction01  110  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 56
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 57 
rt_sigaction01  111  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 57
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 57 
rt_sigaction01  112  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 57
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 57 
rt_sigaction01  113  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 57
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 57 
rt_sigaction01  114  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 57
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 57 
rt_sigaction01  115  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 57
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 58 
rt_sigaction01  116  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 58
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 58 
rt_sigaction01  117  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 58
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 58 
rt_sigaction01  118  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 58
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 58 
rt_sigaction01  119  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 58
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 58 
rt_sigaction01  120  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 58
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 59 
rt_sigaction01  121  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 59
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 59 
rt_sigaction01  122  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 59
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 59 
rt_sigaction01  123  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 59
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 59 
rt_sigaction01  124  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 59
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 59 
rt_sigaction01  125  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 59
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 60 
rt_sigaction01  126  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 60
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 60 
rt_sigaction01  127  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 60
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 60 
rt_sigaction01  128  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 60
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 60 
rt_sigaction01  129  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 60
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 60 
rt_sigaction01  130  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 60
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 61 
rt_sigaction01  131  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 61
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 61 
rt_sigaction01  132  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 61
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 61 
rt_sigaction01  133  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 61
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 61 
rt_sigaction01  134  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 61
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 61 
rt_sigaction01  135  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 61
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 62 
rt_sigaction01  136  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 62
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 62 
rt_sigaction01  137  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 62
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 62 
rt_sigaction01  138  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 62
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 62 
rt_sigaction01  139  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 62
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 62 
rt_sigaction01  140  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 62
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 63 
rt_sigaction01  141  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 63
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 63 
rt_sigaction01  142  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 63
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 63 
rt_sigaction01  143  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 63
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 63 
rt_sigaction01  144  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 63
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 63 
rt_sigaction01  145  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 63
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 64 
rt_sigaction01  146  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 64
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 64 
rt_sigaction01  147  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 64
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 64 
rt_sigaction01  148  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 64
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 64 
rt_sigaction01  149  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 64
rt_sigaction01    0  [1;34mTINFO[0m  :  signal: 64 
rt_sigaction01  150  [1;32mTPASS[0m  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  [1;34mTINFO[0m  :  Signal Handler Called with signal number 64
[37m[699.222028 0:1211 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE rt_sigaction01 : 0
Pass!
LTP MEMORY rt_sigaction01 after_run: free_frames=155172 allocated_frames=40997
LTP MEMORY rt_sigaction01 after_cleanup: free_frames=155172 allocated_frames=40997
LTP CASE RUNTIME rt_sigaction01: 1769 ms
========== END ltp rt_sigaction01 ==========
========== START ltp sigaction01 ==========
RUN LTP CASE sigaction01
LTP MEMORY sigaction01 before: free_frames=155172 allocated_frames=40997
sigaction01    1  [1;32mTPASS[0m  :  SA_RESETHAND did not cause SA_SIGINFO to be cleared
sigaction01    2  [1;32mTPASS[0m  :  SA_RESETHAND was masked when handler executed
sigaction01    3  [1;32mTPASS[0m  :  sig has been masked because sa_mask originally contained sig
sigaction01    4  [1;32mTPASS[0m  :  siginfo pointer non NULL
PASS LTP CASE sigaction01 : 0
Pass!
LTP MEMORY sigaction01 after_run: free_frames=155164 allocated_frames=41005
LTP MEMORY sigaction01 after_cleanup: free_frames=155164 allocated_frames=41005
LTP CASE RUNTIME sigaction01: 1548 ms
========== END ltp sigaction01 ==========
========== START ltp proc01 ==========
RUN LTP CASE proc01
LTP MEMORY proc01 before: free_frames=155164 allocated_frames=41005
proc01      1  [1;32mTPASS[0m  :  readproc() completed successfully, total read: 875 bytes, 20 objs
[37m[702.364319 0:1213 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE proc01 : 0
Pass!
LTP MEMORY proc01 after_run: free_frames=155156 allocated_frames=41013
LTP MEMORY proc01 after_cleanup: free_frames=155156 allocated_frames=41013
LTP CASE RUNTIME proc01: 1608 ms
========== END ltp proc01 ==========
========== START ltp exit01 ==========
RUN LTP CASE exit01
LTP MEMORY exit01 before: free_frames=155156 allocated_frames=41013
exit01      1  [1;32mTPASS[0m  :  exit() test PASSED
PASS LTP CASE exit01 : 0
Pass!
LTP MEMORY exit01 after_run: free_frames=155140 allocated_frames=41029
LTP MEMORY exit01 after_cleanup: free_frames=155140 allocated_frames=41029
LTP CASE RUNTIME exit01: 1579 ms
========== END ltp exit01 ==========
========== START ltp exit02 ==========
RUN LTP CASE exit02
LTP MEMORY exit02 before: free_frames=155140 allocated_frames=41029
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
exit02.c:46: [1;32mTPASS: [0mFile written by child read back correctly

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[705.465130 0:1216 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE exit02 : 0
Pass!
LTP MEMORY exit02 after_run: free_frames=155116 allocated_frames=41053
LTP MEMORY exit02 after_cleanup: free_frames=155116 allocated_frames=41053
LTP CASE RUNTIME exit02: 1489 ms
========== END ltp exit02 ==========
========== START ltp exit_group01 ==========
RUN LTP CASE exit_group01
LTP MEMORY exit_group01 before: free_frames=155116 allocated_frames=41053
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
exit_group01.c:98: [1;32mTPASS: [0mExpect: exit_group() succeeded
exit_group01.c:61: [1;34mTINFO: [0mChecking if threads are still running
exit_group01.c:77: [1;34mTINFO: [0mThreads counters value didn't change

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[707.197605 0:1221 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[707.203465 0:1221 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE exit_group01 : 0
Pass!
LTP MEMORY exit_group01 after_run: free_frames=155092 allocated_frames=41077
LTP MEMORY exit_group01 after_cleanup: free_frames=155092 allocated_frames=41077
LTP CASE RUNTIME exit_group01: 1748 ms
========== END ltp exit_group01 ==========
========== START ltp getpgrp01 ==========
RUN LTP CASE getpgrp01
LTP MEMORY getpgrp01 before: free_frames=155092 allocated_frames=41077
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getpgrp01.c:18: [1;32mTPASS: [0mgetpgrp() returned pid 1230
getpgrp01.c:19: [1;32mTPASS: [0mTST_RET == SAFE_GETPGID(0) (1230)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[708.808247 0:1228 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpgrp01 : 0
Pass!
LTP MEMORY getpgrp01 after_run: free_frames=155076 allocated_frames=41093
LTP MEMORY getpgrp01 after_cleanup: free_frames=155076 allocated_frames=41093
LTP CASE RUNTIME getpgrp01: 1611 ms
========== END ltp getpgrp01 ==========
========== START ltp gettid01 ==========
RUN LTP CASE gettid01
LTP MEMORY gettid01 before: free_frames=155076 allocated_frames=41093
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
gettid01.c:26: [1;32mTPASS: [0mtst_syscall(__NR_gettid) == tst_syscall(__NR_getpid) (1234)
gettid01.c:27: [1;32mTPASS: [0mtst_syscall(__NR_gettid) == pid (1234)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[710.207955 0:1232 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE gettid01 : 0
Pass!
LTP MEMORY gettid01 after_run: free_frames=155060 allocated_frames=41109
LTP MEMORY gettid01 after_cleanup: free_frames=155060 allocated_frames=41109
LTP CASE RUNTIME gettid01: 1386 ms
========== END ltp gettid01 ==========
========== START ltp uname01 ==========
RUN LTP CASE uname01
LTP MEMORY uname01 before: free_frames=155060 allocated_frames=41109
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
uname01.c:21: [1;32mTPASS: [0muname(&un) passed
uname01.c:31: [1;32mTPASS: [0msysname set to Linux

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[711.737456 0:1236 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE uname01 : 0
Pass!
LTP MEMORY uname01 after_run: free_frames=155044 allocated_frames=41125
LTP MEMORY uname01 after_cleanup: free_frames=155044 allocated_frames=41125
LTP CASE RUNTIME uname01: 1523 ms
========== END ltp uname01 ==========
========== START ltp getrlimit01 ==========
RUN LTP CASE getrlimit01
LTP MEMORY getrlimit01 before: free_frames=155044 allocated_frames=41125
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_CPU passed
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_FSIZE passed
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_DATA passed
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_STACK passed
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_CORE passed
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_RSS passed
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_NPROC passed
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_NOFILE passed
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_MEMLOCK passed
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_AS passed
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_LOCKS passed
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_MSGQUEUE passed
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_NICE passed
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_RTPRIO passed
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_SIGPENDING passed
getrlimit01.c:50: [1;32mTPASS: [0mgetrlimit() test for RLIMIT_RTTIME passed

Summary:
passed   16
failed   0
broken   0
skipped  0
warnings 0
[37m[713.255137 0:1240 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getrlimit01 : 0
Pass!
LTP MEMORY getrlimit01 after_run: free_frames=155028 allocated_frames=41141
LTP MEMORY getrlimit01 after_cleanup: free_frames=155028 allocated_frames=41141
LTP CASE RUNTIME getrlimit01: 1501 ms
========== END ltp getrlimit01 ==========
========== START ltp getrusage01 ==========
RUN LTP CASE getrusage01
LTP MEMORY getrusage01 before: free_frames=155028 allocated_frames=41141
tst_buffers.c:57: [1;34mTINFO: [0mTest is using guarded buffers
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getrusage01.c:29: [1;32mTPASS: [0mgetrusage(RUSAGE_SELF, 0x10000beef0) passed
getrusage01.c:29: [1;32mTPASS: [0mgetrusage(RUSAGE_CHILDREN, 0x10000beef0) passed

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[714.752380 0:1244 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getrusage01 : 0
Pass!
LTP MEMORY getrusage01 after_run: free_frames=155012 allocated_frames=41157
LTP MEMORY getrusage01 after_cleanup: free_frames=155012 allocated_frames=41157
LTP CASE RUNTIME getrusage01: 1511 ms
========== END ltp getrusage01 ==========
========== START ltp sched_yield01 ==========
RUN LTP CASE sched_yield01
LTP MEMORY sched_yield01 before: free_frames=155012 allocated_frames=41157
sched_yield01    1  [1;32mTPASS[0m  :  sched_yield() call succeeded
PASS LTP CASE sched_yield01 : 0
Pass!
LTP MEMORY sched_yield01 after_run: free_frames=155004 allocated_frames=41165
LTP MEMORY sched_yield01 after_cleanup: free_frames=155004 allocated_frames=41165
LTP CASE RUNTIME sched_yield01: 1439 ms
========== END ltp sched_yield01 ==========
ltp cases: 63 passed, 0 failed, 0 timed out
#### OS COMP TEST GROUP END ltp-musl ####
#### OS COMP TEST GROUP START ltp-glibc ####
ltp case list: stable (63 cases, timeout 10s)
========== START ltp access01 ==========
RUN LTP CASE access01
LTP MEMORY access01 before: free_frames=155004 allocated_frames=41165
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
access01.c:245: TPASS: access(accessfile_rwx, F_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, F_OK) as nobody passed
access01.c:245: TPASS: access(accessfile_rwx, X_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, X_OK) as nobody passed
access01.c:245: TPASS: access(accessfile_rwx, W_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, W_OK) as nobody passed
access01.c:245: TPASS: access(accessfile_rwx, R_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, R_OK) as nobody passed
access01.c:245: TPASS: access(accessfile_rwx, R_OK|W_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, R_OK|W_OK) as nobody passed
access01.c:245: TPASS: access(accessfile_rwx, R_OK|X_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, R_OK|X_OK) as nobody passed
access01.c:245: TPASS: access(accessfile_rwx, W_OK|X_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, W_OK|X_OK) as nobody passed
access01.c:245: TPASS: access(accessfile_rwx, R_OK|W_OK|X_OK) as root passed
access01.c:245: TPASS: access(accessfile_rwx, R_OK|W_OK|X_OK) as nobody passed
access01.c:245: TPASS: access(accessfile_x, X_OK) as root passed
access01.c:245: TPASS: access(accessfile_x, X_OK) as nobody passed
access01.c:245: TPASS: access(accessfile_w, W_OK) as root passed
access01.c:245: TPASS: access(accessfile_w, W_OK) as nobody passed
access01.c:245: TPASS: access(accessfile_r, R_OK) as root passed
access01.c:245: TPASS: access(accessfile_r, R_OK) as nobody passed
access01.c:242: TPASS: access(accessfile_r, X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessfile_r, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_r, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_w, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_w, X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessfile_w, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_x, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_x, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_r, W_OK|X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessfile_r, W_OK|X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_r, R_OK|X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessfile_r, R_OK|X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_r, R_OK|W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_r, R_OK|W_OK|X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessfile_r, R_OK|W_OK|X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_w, W_OK|X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessfile_w, W_OK|X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_w, R_OK|X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessfile_w, R_OK|X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_w, R_OK|W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_w, R_OK|W_OK|X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessfile_w, R_OK|W_OK|X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_x, W_OK|X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_x, R_OK|X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_x, R_OK|W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessfile_x, R_OK|W_OK|X_OK) as nobody : EACCES (13)
access01.c:245: TPASS: access(accessfile_r, W_OK) as root passed
access01.c:245: TPASS: access(accessfile_r, R_OK|W_OK) as root passed
access01.c:245: TPASS: access(accessfile_w, R_OK) as root passed
access01.c:245: TPASS: access(accessfile_w, R_OK|W_OK) as root passed
access01.c:245: TPASS: access(accessfile_x, R_OK) as root passed
access01.c:245: TPASS: access(accessfile_x, W_OK) as root passed
access01.c:245: TPASS: access(accessfile_x, R_OK|W_OK) as root passed
access01.c:245: TPASS: access(accessdir_r/accessfile_r, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_r/accessfile_r, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_r/accessfile_r, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_r/accessfile_w, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_r/accessfile_w, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_r/accessfile_w, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_r/accessfile_x, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_r/accessfile_x, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_r/accessfile_x, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_r/accessfile_x, X_OK) as root passed
access01.c:245: TPASS: access(accessdir_w/accessfile_r, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_w/accessfile_r, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_w/accessfile_r, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_w/accessfile_w, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_w/accessfile_w, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_w/accessfile_w, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_w/accessfile_x, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_w/accessfile_x, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_w/accessfile_x, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_w/accessfile_x, X_OK) as root passed
access01.c:245: TPASS: access(accessdir_x/accessfile_r, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_x/accessfile_r, F_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_x/accessfile_r, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_x/accessfile_r, R_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_x/accessfile_r, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_x/accessfile_w, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_x/accessfile_w, F_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_x/accessfile_w, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_x/accessfile_w, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_x/accessfile_w, W_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_x/accessfile_x, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_x/accessfile_x, F_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_x/accessfile_x, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_x/accessfile_x, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_x/accessfile_x, X_OK) as root passed
access01.c:245: TPASS: access(accessdir_x/accessfile_x, X_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_rw/accessfile_r, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_rw/accessfile_r, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_rw/accessfile_r, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_rw/accessfile_w, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_rw/accessfile_w, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_rw/accessfile_w, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_rw/accessfile_x, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_rw/accessfile_x, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_rw/accessfile_x, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_rw/accessfile_x, X_OK) as root passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_r, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_r, F_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_r, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_r, R_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_r, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_w, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_w, F_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_w, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_w, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_w, W_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_x, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_x, F_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_x, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_x, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_x, X_OK) as root passed
access01.c:245: TPASS: access(accessdir_rx/accessfile_x, X_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_r, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_r, F_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_r, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_r, R_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_r, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_w, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_w, F_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_w, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_w, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_w, W_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_x, F_OK) as root passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_x, F_OK) as nobody passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_x, R_OK) as root passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_x, W_OK) as root passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_x, X_OK) as root passed
access01.c:245: TPASS: access(accessdir_wx/accessfile_x, X_OK) as nobody passed
access01.c:242: TPASS: access(accessdir_r/accessfile_r, F_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_r/accessfile_r, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_r/accessfile_r, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_r/accessfile_r, X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessdir_r/accessfile_r, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_r/accessfile_w, F_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_r/accessfile_w, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_r/accessfile_w, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_r/accessfile_w, X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessdir_r/accessfile_w, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_r/accessfile_x, F_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_r/accessfile_x, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_r/accessfile_x, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_r/accessfile_x, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_w/accessfile_r, F_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_w/accessfile_r, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_w/accessfile_r, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_w/accessfile_r, X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessdir_w/accessfile_r, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_w/accessfile_w, F_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_w/accessfile_w, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_w/accessfile_w, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_w/accessfile_w, X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessdir_w/accessfile_w, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_w/accessfile_x, F_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_w/accessfile_x, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_w/accessfile_x, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_w/accessfile_x, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_x/accessfile_r, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_x/accessfile_r, X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessdir_x/accessfile_r, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_x/accessfile_w, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_x/accessfile_w, X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessdir_x/accessfile_w, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_x/accessfile_x, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_x/accessfile_x, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rw/accessfile_r, F_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rw/accessfile_r, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rw/accessfile_r, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rw/accessfile_r, X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessdir_rw/accessfile_r, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rw/accessfile_w, F_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rw/accessfile_w, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rw/accessfile_w, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rw/accessfile_w, X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessdir_rw/accessfile_w, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rw/accessfile_x, F_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rw/accessfile_x, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rw/accessfile_x, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rw/accessfile_x, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rx/accessfile_r, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rx/accessfile_r, X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessdir_rx/accessfile_r, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rx/accessfile_w, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rx/accessfile_w, X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessdir_rx/accessfile_w, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rx/accessfile_x, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_rx/accessfile_x, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_wx/accessfile_r, W_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_wx/accessfile_r, X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessdir_wx/accessfile_r, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_wx/accessfile_w, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_wx/accessfile_w, X_OK) as root : EACCES (13)
access01.c:242: TPASS: access(accessdir_wx/accessfile_w, X_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_wx/accessfile_x, R_OK) as nobody : EACCES (13)
access01.c:242: TPASS: access(accessdir_wx/accessfile_x, W_OK) as nobody : EACCES (13)

Summary:
passed   199
failed   0
broken   0
skipped  0
warnings 0
[37m[723.732105 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.733181 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.734020 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.735745 0:1249 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[723.737541 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.742524 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.743359 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.744388 0:1249 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[723.745911 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.746742 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.747597 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.748590 0:1249 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[723.749978 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.750877 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.751831 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.752795 0:1249 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[723.754229 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.755126 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.755963 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.756937 0:1249 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[723.758380 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.759312 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.760159 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.761128 0:1249 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[723.762005 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.762764 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.763528 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.764335 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[723.765461 0:1249 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE access01 : 0
Pass!
LTP MEMORY access01 after_run: free_frames=154074 allocated_frames=42095
LTP MEMORY access01 after_cleanup: free_frames=154074 allocated_frames=42095
LTP CASE RUNTIME access01: 7539 ms
========== END ltp access01 ==========
========== START ltp brk01 ==========
RUN LTP CASE brk01
LTP MEMORY brk01 before: free_frames=154074 allocated_frames=42095
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
brk01.c:24: TINFO: Testing libc variant
brk01.c:70: TPASS: brk() works fine
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
brk01.c:21: TINFO: Testing syscall variant
brk01.c:70: TPASS: brk() works fine

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[726.571016 0:1354 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE brk01 : 0
Pass!
LTP MEMORY brk01 after_run: free_frames=154044 allocated_frames=42125
LTP MEMORY brk01 after_cleanup: free_frames=154044 allocated_frames=42125
LTP CASE RUNTIME brk01: 2754 ms
========== END ltp brk01 ==========
========== START ltp chdir01 ==========
RUN LTP CASE chdir01
LTP MEMORY chdir01 before: free_frames=154044 allocated_frames=42125
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_device.c:317: TINFO: Using test device LTP_DEV='/dev/vda'
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_supported_fs_types.c:105: TINFO: Skipping bcachefs because of FUSE blacklist
tst_supported_fs_types.c:97: TINFO: Kernel supports tmpfs
tst_supported_fs_types.c:49: TINFO: mkfs is not needed for tmpfs
tst_test.c:1693: TINFO: === Testing on tmpfs ===
tst_test.c:1106: TINFO: Skipping mkfs for TMPFS filesystem
tst_test.c:1087: TINFO: Limiting tmpfs size to 32MB
tst_test.c:1120: TINFO: Mounting ltp-tmpfs to /tmp/ltp-work/LTP_chdXfMbYA/mntpoint fstyp=tmpfs flags=0
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
chdir01.c:119: TINFO: Testing 'testfile'
chdir01.c:111: TPASS: root: chdir("testfile") returned correct value: ENOTDIR (20)
chdir01.c:111: TPASS: nobody: chdir("testfile") returned correct value: ENOTDIR (20)
chdir01.c:119: TINFO: Testing 'keep_out'
chdir01.c:111: TPASS: root: chdir("keep_out") returned correct value: SUCCESS (0)
chdir01.c:111: TPASS: nobody: chdir("keep_out") returned correct value: EACCES (13)
chdir01.c:119: TINFO: Testing 'subdir'
chdir01.c:111: TPASS: root: chdir("subdir") returned correct value: SUCCESS (0)
chdir01.c:111: TPASS: nobody: chdir("subdir") returned correct value: SUCCESS (0)
chdir01.c:119: TINFO: Testing '.'
chdir01.c:111: TPASS: root: chdir(".") returned correct value: SUCCESS (0)
chdir01.c:111: TPASS: nobody: chdir(".") returned correct value: SUCCESS (0)
chdir01.c:119: TINFO: Testing '..'
chdir01.c:111: TPASS: root: chdir("..") returned correct value: SUCCESS (0)
chdir01.c:111: TPASS: nobody: chdir("..") returned correct value: SUCCESS (0)
chdir01.c:119: TINFO: Testing '/'
chdir01.c:111: TPASS: root: chdir("/") returned correct value: SUCCESS (0)
chdir01.c:111: TPASS: nobody: chdir("/") returned correct value: SUCCESS (0)
chdir01.c:119: TINFO: Testing 'does_not_exist'
chdir01.c:111: TPASS: root: chdir("does_not_exist") returned correct value: ENOENT (2)
chdir01.c:111: TPASS: nobody: chdir("does_not_exist") returned correct value: ENOENT (2)
chdir01.c:119: TINFO: Testing 'symloop'
chdir01.c:111: TPASS: root: chdir("symloop") returned correct value: ELOOP (40)
chdir01.c:111: TPASS: nobody: chdir("symloop") returned correct value: ELOOP (40)

Summary:
passed   16
failed   0
broken   0
skipped  0
warnings 0
[37m[729.511560 0:1361 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[729.513126 0:1361 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[729.513848 0:1361 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[729.514636 0:1361 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[729.515283 0:1361 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE chdir01 : 0
Pass!
LTP MEMORY chdir01 after_run: free_frames=154023 allocated_frames=42146
LTP MEMORY chdir01 after_cleanup: free_frames=154023 allocated_frames=42146
LTP CASE RUNTIME chdir01: 2939 ms
========== END ltp chdir01 ==========
========== START ltp clone01 ==========
RUN LTP CASE clone01
LTP MEMORY clone01 before: free_frames=154023 allocated_frames=42146
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clone01.c:37: TPASS: clone returned 1368
clone01.c:43: TPASS: Child exited with 0

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[732.445840 0:1365 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE clone01 : 0
Pass!
LTP MEMORY clone01 after_run: free_frames=153993 allocated_frames=42176
LTP MEMORY clone01 after_cleanup: free_frames=153993 allocated_frames=42176
LTP CASE RUNTIME clone01: 2928 ms
========== END ltp clone01 ==========
========== START ltp close01 ==========
RUN LTP CASE close01
LTP MEMORY close01 before: free_frames=153993 allocated_frames=42176
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
close01.c:50: TPASS: close a file fd passed
close01.c:50: TPASS: close a pipe fd passed
close01.c:50: TPASS: close a socket fd passed

Summary:
passed   3
failed   0
broken   0
skipped  0
warnings 0
[37m[735.207084 0:1370 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[735.208885 0:1370 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE close01 : 0
Pass!
LTP MEMORY close01 after_run: free_frames=153972 allocated_frames=42197
LTP MEMORY close01 after_cleanup: free_frames=153972 allocated_frames=42197
LTP CASE RUNTIME close01: 2771 ms
========== END ltp close01 ==========
========== START ltp dup01 ==========
RUN LTP CASE dup01
LTP MEMORY dup01 before: free_frames=153972 allocated_frames=42197
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
dup01.c:24: TPASS: dup(fd) returned fd 4
dup01.c:27: TPASS: buf1.st_ino == buf2.st_ino (3277890279393648972)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[738.055330 0:1374 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[738.057784 0:1374 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE dup01 : 0
Pass!
LTP MEMORY dup01 after_run: free_frames=153951 allocated_frames=42218
LTP MEMORY dup01 after_cleanup: free_frames=153951 allocated_frames=42218
LTP CASE RUNTIME dup01: 2842 ms
========== END ltp dup01 ==========
========== START ltp fcntl01 ==========
RUN LTP CASE fcntl01
LTP MEMORY fcntl01 before: free_frames=153951 allocated_frames=42218
[37m[740.967980 0:1378 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fcntl01 : 0
Pass!
LTP MEMORY fcntl01 after_run: free_frames=153939 allocated_frames=42230
LTP MEMORY fcntl01 after_cleanup: free_frames=153939 allocated_frames=42230
LTP CASE RUNTIME fcntl01: 2907 ms
========== END ltp fcntl01 ==========
========== START ltp fcntl02 ==========
RUN LTP CASE fcntl02
LTP MEMORY fcntl02 before: free_frames=153939 allocated_frames=42230
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
fcntl02.c:41: TPASS: fcntl(fcntl02_1381, F_DUPFD, 0) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1381, F_DUPFD, 1) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1381, F_DUPFD, 2) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1381, F_DUPFD, 3) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1381, F_DUPFD, 10) returned 10
fcntl02.c:41: TPASS: fcntl(fcntl02_1381, F_DUPFD, 100) returned 100

Summary:
passed   6
failed   0
broken   0
skipped  0
warnings 0
[37m[743.553667 0:1379 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[743.554814 0:1379 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fcntl02 : 0
Pass!
LTP MEMORY fcntl02 after_run: free_frames=153918 allocated_frames=42251
LTP MEMORY fcntl02 after_cleanup: free_frames=153918 allocated_frames=42251
LTP CASE RUNTIME fcntl02: 2583 ms
========== END ltp fcntl02 ==========
========== START ltp fork01 ==========
RUN LTP CASE fork01
LTP MEMORY fork01 before: free_frames=153918 allocated_frames=42251
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
fork01.c:47: TPASS: correct child status returned 42
fork01.c:50: TPASS: child_pid == pid (1386)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[746.356824 0:1383 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[746.361800 0:1383 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fork01 : 0
Pass!
LTP MEMORY fork01 after_run: free_frames=153888 allocated_frames=42281
LTP MEMORY fork01 after_cleanup: free_frames=153888 allocated_frames=42281
LTP CASE RUNTIME fork01: 2816 ms
========== END ltp fork01 ==========
========== START ltp getpid01 ==========
RUN LTP CASE getpid01
LTP MEMORY getpid01 before: free_frames=153888 allocated_frames=42281
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getpid01.c:34: TPASS: getpid() returns 1391
getpid01.c:34: TPASS: getpid() returns 1392
getpid01.c:34: TPASS: getpid() returns 1393
getpid01.c:34: TPASS: getpid() returns 1394
getpid01.c:34: TPASS: getpid() returns 1395
getpid01.c:34: TPASS: getpid() returns 1396
getpid01.c:34: TPASS: getpid() returns 1397
getpid01.c:34: TPASS: getpid() returns 1398
getpid01.c:34: TPASS: getpid() returns 1399
getpid01.c:34: TPASS: getpid() returns 1400
getpid01.c:34: TPASS: getpid() returns 1401
getpid01.c:34: TPASS: getpid() returns 1402
getpid01.c:34: TPASS: getpid() returns 1403
getpid01.c:34: TPASS: getpid() returns 1404
getpid01.c:34: TPASS: getpid() returns 1405
getpid01.c:34: TPASS: getpid() returns 1406
getpid01.c:34: TPASS: getpid() returns 1407
getpid01.c:34: TPASS: getpid() returns 1408
getpid01.c:34: TPASS: getpid() returns 1409
getpid01.c:34: TPASS: getpid() returns 1410
getpid01.c:34: TPASS: getpid() returns 1411
getpid01.c:34: TPASS: getpid() returns 1412
getpid01.c:34: TPASS: getpid() returns 1413
getpid01.c:34: TPASS: getpid() returns 1414
getpid01.c:34: TPASS: getpid() returns 1415
getpid01.c:34: TPASS: getpid() returns 1416
getpid01.c:34: TPASS: getpid() returns 1417
getpid01.c:34: TPASS: getpid() returns 1418
getpid01.c:34: TPASS: getpid() returns 1419
getpid01.c:34: TPASS: getpid() returns 1420
getpid01.c:34: TPASS: getpid() returns 1421
getpid01.c:34: TPASS: getpid() returns 1422
getpid01.c:34: TPASS: getpid() returns 1423
getpid01.c:34: TPASS: getpid() returns 1424
getpid01.c:34: TPASS: getpid() returns 1425
getpid01.c:34: TPASS: getpid() returns 1426
getpid01.c:34: TPASS: getpid() returns 1427
getpid01.c:34: TPASS: getpid() returns 1428
getpid01.c:34: TPASS: getpid() returns 1429
getpid01.c:34: TPASS: getpid() returns 1430
getpid01.c:34: TPASS: getpid() returns 1431
getpid01.c:34: TPASS: getpid() returns 1432
getpid01.c:34: TPASS: getpid() returns 1433
getpid01.c:34: TPASS: getpid() returns 1434
getpid01.c:34: TPASS: getpid() returns 1435
getpid01.c:34: TPASS: getpid() returns 1436
getpid01.c:34: TPASS: getpid() returns 1437
getpid01.c:34: TPASS: getpid() returns 1438
getpid01.c:34: TPASS: getpid() returns 1439
getpid01.c:34: TPASS: getpid() returns 1440
getpid01.c:34: TPASS: getpid() returns 1441
getpid01.c:34: TPASS: getpid() returns 1442
getpid01.c:34: TPASS: getpid() returns 1443
getpid01.c:34: TPASS: getpid() returns 1444
getpid01.c:34: TPASS: getpid() returns 1445
getpid01.c:34: TPASS: getpid() returns 1446
getpid01.c:34: TPASS: getpid() returns 1447
getpid01.c:34: TPASS: getpid() returns 1448
getpid01.c:34: TPASS: getpid() returns 1449
getpid01.c:34: TPASS: getpid() returns 1450
getpid01.c:34: TPASS: getpid() returns 1451
getpid01.c:34: TPASS: getpid() returns 1452
getpid01.c:34: TPASS: getpid() returns 1453
getpid01.c:34: TPASS: getpid() returns 1454
getpid01.c:34: TPASS: getpid() returns 1455
getpid01.c:34: TPASS: getpid() returns 1456
getpid01.c:34: TPASS: getpid() returns 1457
getpid01.c:34: TPASS: getpid() returns 1458
getpid01.c:34: TPASS: getpid() returns 1459
getpid01.c:34: TPASS: getpid() returns 1460
getpid01.c:34: TPASS: getpid() returns 1461
getpid01.c:34: TPASS: getpid() returns 1462
getpid01.c:34: TPASS: getpid() returns 1463
getpid01.c:34: TPASS: getpid() returns 1464
getpid01.c:34: TPASS: getpid() returns 1465
getpid01.c:34: TPASS: getpid() returns 1466
getpid01.c:34: TPASS: getpid() returns 1467
getpid01.c:34: TPASS: getpid() returns 1468
getpid01.c:34: TPASS: getpid() returns 1469
getpid01.c:34: TPASS: getpid() returns 1470
getpid01.c:34: TPASS: getpid() returns 1471
getpid01.c:34: TPASS: getpid() returns 1472
getpid01.c:34: TPASS: getpid() returns 1473
getpid01.c:34: TPASS: getpid() returns 1474
getpid01.c:34: TPASS: getpid() returns 1475
getpid01.c:34: TPASS: getpid() returns 1476
getpid01.c:34: TPASS: getpid() returns 1477
getpid01.c:34: TPASS: getpid() returns 1478
getpid01.c:34: TPASS: getpid() returns 1479
getpid01.c:34: TPASS: getpid() returns 1480
getpid01.c:34: TPASS: getpid() returns 1481
getpid01.c:34: TPASS: getpid() returns 1482
getpid01.c:34: TPASS: getpid() returns 1483
getpid01.c:34: TPASS: getpid() returns 1484
getpid01.c:34: TPASS: getpid() returns 1485
getpid01.c:34: TPASS: getpid() returns 1486
getpid01.c:34: TPASS: getpid() returns 1487
getpid01.c:34: TPASS: getpid() returns 1488
getpid01.c:34: TPASS: getpid() returns 1489
getpid01.c:34: TPASS: getpid() returns 1490

Summary:
passed   100
failed   0
broken   0
skipped  0
warnings 0
[37m[752.975656 0:1388 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpid01 : 0
Pass!
LTP MEMORY getpid01 after_run: free_frames=152967 allocated_frames=43202
LTP MEMORY getpid01 after_cleanup: free_frames=152967 allocated_frames=43202
LTP CASE RUNTIME getpid01: 6601 ms
========== END ltp getpid01 ==========
========== START ltp mmap01 ==========
RUN LTP CASE mmap01
LTP MEMORY mmap01 before: free_frames=152967 allocated_frames=43202
[37m[755.772393 0:1492 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[755.775614 0:1492 axfs::root:433] [33m[AxError::IsADirectory][m
[mmmap01      1  TPASS  :  Functionality of mmap() successful
PASS LTP CASE mmap01 : 0
Pass!
LTP MEMORY mmap01 after_run: free_frames=152946 allocated_frames=43223
LTP MEMORY mmap01 after_cleanup: free_frames=152946 allocated_frames=43223
LTP CASE RUNTIME mmap01: 2829 ms
========== END ltp mmap01 ==========
========== START ltp open01 ==========
RUN LTP CASE open01
LTP MEMORY open01 before: free_frames=152946 allocated_frames=43223
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
open01.c:59: TPASS: sticky bit is set as expected
open01.c:59: TPASS: sirectory bit is set as expected

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[758.592849 0:1494 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[758.594827 0:1494 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE open01 : 0
Pass!
LTP MEMORY open01 after_run: free_frames=152925 allocated_frames=43244
LTP MEMORY open01 after_cleanup: free_frames=152925 allocated_frames=43244
LTP CASE RUNTIME open01: 2796 ms
========== END ltp open01 ==========
========== START ltp pipe01 ==========
RUN LTP CASE pipe01
LTP MEMORY pipe01 before: free_frames=152925 allocated_frames=43244
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
pipe01.c:48: TPASS: pipe() functionality is correct

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[761.399950 0:1498 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE pipe01 : 0
Pass!
LTP MEMORY pipe01 after_run: free_frames=152904 allocated_frames=43265
LTP MEMORY pipe01 after_cleanup: free_frames=152904 allocated_frames=43265
LTP CASE RUNTIME pipe01: 2791 ms
========== END ltp pipe01 ==========
========== START ltp read01 ==========
RUN LTP CASE read01
LTP MEMORY read01 before: free_frames=152904 allocated_frames=43265
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
read01.c:24: TPASS: read(2) returned 512

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[764.149070 0:1502 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[764.153756 0:1502 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE read01 : 0
Pass!
LTP MEMORY read01 after_run: free_frames=152883 allocated_frames=43286
LTP MEMORY read01 after_cleanup: free_frames=152883 allocated_frames=43286
LTP CASE RUNTIME read01: 2769 ms
========== END ltp read01 ==========
========== START ltp stat01 ==========
RUN LTP CASE stat01
LTP MEMORY stat01 before: free_frames=152883 allocated_frames=43286
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
stat01.c:43: TPASS: stat(tc->pathname, &stat_buf) passed
stat01.c:45: TPASS: stat_buf.st_uid == user_id (65534)
stat01.c:46: TPASS: stat_buf.st_gid == group_id (0)
stat01.c:47: TPASS: stat_buf.st_size == FILE_SIZE (1024)
stat01.c:48: TPASS: stat_buf.st_mode & MASK == tc->mode (438)
stat01.c:49: TPASS: stat_buf.st_nlink == 1 (1)
stat01.c:43: TPASS: stat(tc->pathname, &stat_buf) passed
stat01.c:45: TPASS: stat_buf.st_uid == user_id (65534)
stat01.c:46: TPASS: stat_buf.st_gid == group_id (0)
stat01.c:47: TPASS: stat_buf.st_size == FILE_SIZE (1024)
stat01.c:48: TPASS: stat_buf.st_mode & MASK == tc->mode (146)
stat01.c:49: TPASS: stat_buf.st_nlink == 1 (1)

Summary:
passed   12
failed   0
broken   0
skipped  0
warnings 0
[37m[767.056581 0:1506 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[767.058819 0:1506 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[767.061168 0:1506 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE stat01 : 0
Pass!
LTP MEMORY stat01 after_run: free_frames=152862 allocated_frames=43307
LTP MEMORY stat01 after_cleanup: free_frames=152862 allocated_frames=43307
LTP CASE RUNTIME stat01: 2885 ms
========== END ltp stat01 ==========
========== START ltp wait401 ==========
RUN LTP CASE wait401
LTP MEMORY wait401 before: free_frames=152862 allocated_frames=43307
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
wait401.c:40: TPASS: wait4() returned correct pid 1513
wait401.c:49: TPASS: WIFEXITED() is set in status
wait401.c:54: TPASS: WEXITSTATUS() == 0

Summary:
passed   3
failed   0
broken   0
skipped  0
warnings 0
[37m[769.962186 0:1510 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE wait401 : 0
Pass!
LTP MEMORY wait401 after_run: free_frames=152832 allocated_frames=43337
LTP MEMORY wait401 after_cleanup: free_frames=152832 allocated_frames=43337
LTP CASE RUNTIME wait401: 2907 ms
========== END ltp wait401 ==========
========== START ltp write01 ==========
RUN LTP CASE write01
LTP MEMORY write01 before: free_frames=152832 allocated_frames=43337
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
write01.c:40: TPASS: write() passed

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[773.693337 0:1515 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[773.701942 0:1515 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE write01 : 0
Pass!
LTP MEMORY write01 after_run: free_frames=120043 allocated_frames=76126
LTP MEMORY write01 after_cleanup: free_frames=120043 allocated_frames=76126
LTP CASE RUNTIME write01: 3751 ms
========== END ltp write01 ==========
========== START ltp access03 ==========
RUN LTP CASE access03
LTP MEMORY access03 before: free_frames=120043 allocated_frames=76126
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
access03.c:37: TPASS: invalid address as root : EFAULT (14)
access03.c:46: TPASS: invalid address as nobody : EFAULT (14)
access03.c:37: TPASS: invalid address as root : EFAULT (14)
access03.c:46: TPASS: invalid address as nobody : EFAULT (14)
access03.c:37: TPASS: invalid address as root : EFAULT (14)
access03.c:46: TPASS: invalid address as nobody : EFAULT (14)
access03.c:37: TPASS: invalid address as root : EFAULT (14)
access03.c:46: TPASS: invalid address as nobody : EFAULT (14)

Summary:
passed   8
failed   0
broken   0
skipped  0
warnings 0
[37m[776.904144 0:1519 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE access03 : 0
Pass!
LTP MEMORY access03 after_run: free_frames=119986 allocated_frames=76183
LTP MEMORY access03 after_cleanup: free_frames=119986 allocated_frames=76183
LTP CASE RUNTIME access03: 3175 ms
========== END ltp access03 ==========
========== START ltp close02 ==========
RUN LTP CASE close02
LTP MEMORY close02 before: free_frames=119986 allocated_frames=76183
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
close02.c:20: TPASS: close(-1) : EBADF (9)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[779.868875 0:1528 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE close02 : 0
Pass!
LTP MEMORY close02 after_run: free_frames=119965 allocated_frames=76204
LTP MEMORY close02 after_cleanup: free_frames=119965 allocated_frames=76204
LTP CASE RUNTIME close02: 2968 ms
========== END ltp close02 ==========
========== START ltp dup02 ==========
RUN LTP CASE dup02
LTP MEMORY dup02 before: free_frames=119965 allocated_frames=76204
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
dup02.c:29: TPASS: dup(-1) : EBADF (9)
dup02.c:29: TPASS: dup(1500) : EBADF (9)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[782.696043 0:1532 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE dup02 : 0
Pass!
LTP MEMORY dup02 after_run: free_frames=119944 allocated_frames=76225
LTP MEMORY dup02 after_cleanup: free_frames=119944 allocated_frames=76225
LTP CASE RUNTIME dup02: 2831 ms
========== END ltp dup02 ==========
========== START ltp fcntl03 ==========
RUN LTP CASE fcntl03
LTP MEMORY fcntl03 before: free_frames=119944 allocated_frames=76225
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
fcntl03.c:32: TPASS: fcntl(fcntl03_1538, F_GETFD, 0) returned 0

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[785.498895 0:1536 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[785.500093 0:1536 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fcntl03 : 0
Pass!
LTP MEMORY fcntl03 after_run: free_frames=119923 allocated_frames=76246
LTP MEMORY fcntl03 after_cleanup: free_frames=119923 allocated_frames=76246
LTP CASE RUNTIME fcntl03: 2781 ms
========== END ltp fcntl03 ==========
========== START ltp getcwd01 ==========
RUN LTP CASE getcwd01
LTP MEMORY getcwd01 before: free_frames=119923 allocated_frames=76246
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getcwd01.c:48: TPASS: tst_syscall(__NR_getcwd, tc->buf, tc->size) : EFAULT (14)
getcwd01.c:48: TPASS: tst_syscall(__NR_getcwd, tc->buf, tc->size) : EFAULT (14)
getcwd01.c:48: TPASS: tst_syscall(__NR_getcwd, tc->buf, tc->size) : ERANGE (34)
getcwd01.c:48: TPASS: tst_syscall(__NR_getcwd, tc->buf, tc->size) : ERANGE (34)
getcwd01.c:48: TPASS: tst_syscall(__NR_getcwd, tc->buf, tc->size) : ERANGE (34)

Summary:
passed   5
failed   0
broken   0
skipped  0
warnings 0
[37m[788.454567 0:1540 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getcwd01 : 0
Pass!
LTP MEMORY getcwd01 after_run: free_frames=119902 allocated_frames=76267
LTP MEMORY getcwd01 after_cleanup: free_frames=119902 allocated_frames=76267
LTP CASE RUNTIME getcwd01: 2963 ms
========== END ltp getcwd01 ==========
========== START ltp getpid02 ==========
RUN LTP CASE getpid02
LTP MEMORY getpid02 before: free_frames=119902 allocated_frames=76267
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getpid02.c:37: TPASS: child getppid() == parent getpid() (1546)
getpid02.c:50: TPASS: child getpid() == parent fork() (1547)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[791.218263 0:1544 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpid02 : 0
Pass!
LTP MEMORY getpid02 after_run: free_frames=119872 allocated_frames=76297
LTP MEMORY getpid02 after_cleanup: free_frames=119872 allocated_frames=76297
LTP CASE RUNTIME getpid02: 2771 ms
========== END ltp getpid02 ==========
========== START ltp getppid01 ==========
RUN LTP CASE getppid01
LTP MEMORY getppid01 before: free_frames=119872 allocated_frames=76297
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getppid01.c:31: TPASS: getppid() returned 1549

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[794.021583 0:1549 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getppid01 : 0
Pass!
LTP MEMORY getppid01 after_run: free_frames=119851 allocated_frames=76318
LTP MEMORY getppid01 after_cleanup: free_frames=119851 allocated_frames=76318
LTP CASE RUNTIME getppid01: 2780 ms
========== END ltp getppid01 ==========
========== START ltp getuid01 ==========
RUN LTP CASE getuid01
LTP MEMORY getuid01 before: free_frames=119851 allocated_frames=76318
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getuid01.c:20: TPASS: getuid() returned 0

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[796.914417 0:1553 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getuid01 : 0
Pass!
LTP MEMORY getuid01 after_run: free_frames=119830 allocated_frames=76339
LTP MEMORY getuid01 after_cleanup: free_frames=119830 allocated_frames=76339
LTP CASE RUNTIME getuid01: 2892 ms
========== END ltp getuid01 ==========
========== START ltp geteuid01 ==========
RUN LTP CASE geteuid01
LTP MEMORY geteuid01 before: free_frames=119830 allocated_frames=76339
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
geteuid01.c:21: TPASS: geteuid() returned 0

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[799.622147 0:1557 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE geteuid01 : 0
Pass!
LTP MEMORY geteuid01 after_run: free_frames=119809 allocated_frames=76360
LTP MEMORY geteuid01 after_cleanup: free_frames=119809 allocated_frames=76360
LTP CASE RUNTIME geteuid01: 2720 ms
========== END ltp geteuid01 ==========
========== START ltp getgid01 ==========
RUN LTP CASE getgid01
LTP MEMORY getgid01 before: free_frames=119809 allocated_frames=76360
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getgid01.c:26: TPASS: getgid returned as expectedly

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[802.513615 0:1561 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getgid01 : 0
Pass!
LTP MEMORY getgid01 after_run: free_frames=119788 allocated_frames=76381
LTP MEMORY getgid01 after_cleanup: free_frames=119788 allocated_frames=76381
LTP CASE RUNTIME getgid01: 2869 ms
========== END ltp getgid01 ==========
========== START ltp getegid01 ==========
RUN LTP CASE getegid01
LTP MEMORY getegid01 before: free_frames=119788 allocated_frames=76381
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getegid01.c:25: TPASS: gid == st_egid (0)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[805.296053 0:1565 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getegid01 : 0
Pass!
LTP MEMORY getegid01 after_run: free_frames=119767 allocated_frames=76402
LTP MEMORY getegid01 after_cleanup: free_frames=119767 allocated_frames=76402
LTP CASE RUNTIME getegid01: 2810 ms
========== END ltp getegid01 ==========
========== START ltp lseek01 ==========
RUN LTP CASE lseek01
LTP MEMORY lseek01 before: free_frames=119767 allocated_frames=76402
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
lseek01.c:67: TPASS: lseek(tfile, 4, SEEK_SET) read correct data
lseek01.c:67: TPASS: lseek(tfile, -2, SEEK_CUR) read correct data
lseek01.c:67: TPASS: lseek(tfile, -4, SEEK_END) read correct data
lseek01.c:67: TPASS: lseek(tfile, 0, SEEK_END) read correct data

Summary:
passed   4
failed   0
broken   0
skipped  0
warnings 0
[37m[808.227941 0:1569 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[808.229768 0:1569 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE lseek01 : 0
Pass!
LTP MEMORY lseek01 after_run: free_frames=119746 allocated_frames=76423
LTP MEMORY lseek01 after_cleanup: free_frames=119746 allocated_frames=76423
LTP CASE RUNTIME lseek01: 2914 ms
========== END ltp lseek01 ==========
========== START ltp read02 ==========
RUN LTP CASE read02
LTP MEMORY read02 before: free_frames=119746 allocated_frames=76423
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
read02.c:84: TPASS: read() failed as expected: EBADF (9)
read02.c:84: TPASS: read() failed as expected: EISDIR (21)
read02.c:84: TPASS: read() failed as expected: EFAULT (14)
read02.c:65: TCONF: O_DIRECT not supported on tmpfs filesystem
read02.c:65: TCONF: O_DIRECT not supported on tmpfs filesystem

Summary:
passed   3
failed   0
broken   0
skipped  2
warnings 0
[37m[810.983867 0:1573 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[810.984861 0:1573 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE read02 : 0
Pass!
LTP MEMORY read02 after_run: free_frames=119725 allocated_frames=76444
LTP MEMORY read02 after_cleanup: free_frames=119725 allocated_frames=76444
LTP CASE RUNTIME read02: 2741 ms
========== END ltp read02 ==========
========== START ltp write02 ==========
RUN LTP CASE write02
LTP MEMORY write02 before: free_frames=119725 allocated_frames=76444
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
write02.c:20: TPASS: write(fd, NULL, 0) returned 0
write02.c:22: TPASS: Expect: write(fd, NULL, 0) == 0

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[813.732722 0:1577 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[813.735391 0:1577 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE write02 : 0
Pass!
LTP MEMORY write02 after_run: free_frames=119704 allocated_frames=76465
LTP MEMORY write02 after_cleanup: free_frames=119704 allocated_frames=76465
LTP CASE RUNTIME write02: 2763 ms
========== END ltp write02 ==========
========== START ltp creat01 ==========
RUN LTP CASE creat01
LTP MEMORY creat01 before: free_frames=119704 allocated_frames=76465
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
creat01.c:50: TPASS: creat() truncated file to 0 bytes
creat01.c:55: TPASS: file was created and written to successfully
creat01.c:60: TPASS: read failed expectedly: EACCES (13)
creat01.c:50: TPASS: creat() truncated file to 0 bytes
creat01.c:55: TPASS: file was created and written to successfully
creat01.c:60: TPASS: read failed expectedly: EACCES (13)

Summary:
passed   6
failed   0
broken   0
skipped  0
warnings 0
[37m[816.585199 0:1581 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[816.586604 0:1581 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE creat01 : 0
Pass!
LTP MEMORY creat01 after_run: free_frames=119683 allocated_frames=76486
LTP MEMORY creat01 after_cleanup: free_frames=119683 allocated_frames=76486
LTP CASE RUNTIME creat01: 2834 ms
========== END ltp creat01 ==========
========== START ltp creat03 ==========
RUN LTP CASE creat03
LTP MEMORY creat03 before: free_frames=119683 allocated_frames=76486
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
creat03.c:36: TINFO: Created file has mode = 0100674
creat03.c:41: TPASS: save text bit cleared

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[819.395188 0:1585 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[819.396833 0:1585 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE creat03 : 0
Pass!
LTP MEMORY creat03 after_run: free_frames=119662 allocated_frames=76507
LTP MEMORY creat03 after_cleanup: free_frames=119662 allocated_frames=76507
LTP CASE RUNTIME creat03: 2828 ms
========== END ltp creat03 ==========
========== START ltp open02 ==========
RUN LTP CASE open02
LTP MEMORY open02 before: free_frames=119662 allocated_frames=76507
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
open02.c:49: TPASS: open() new file without O_CREAT : ENOENT (2)
open02.c:49: TPASS: open() unprivileged O_RDONLY | O_NOATIME : EPERM (1)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[822.300768 0:1589 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[822.326610 0:1589 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE open02 : 0
Pass!
LTP MEMORY open02 after_run: free_frames=119641 allocated_frames=76528
LTP MEMORY open02 after_cleanup: free_frames=119641 allocated_frames=76528
LTP CASE RUNTIME open02: 2977 ms
========== END ltp open02 ==========
========== START ltp open03 ==========
RUN LTP CASE open03
LTP MEMORY open03 before: free_frames=119641 allocated_frames=76528
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
open03.c:19: TPASS: open(TEST_FILE, O_RDWR | O_CREAT, 0700) returned fd 3

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[826.394838 0:1593 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE open03 : 0
Pass!
LTP MEMORY open03 after_run: free_frames=119620 allocated_frames=76549
LTP MEMORY open03 after_cleanup: free_frames=119620 allocated_frames=76549
LTP CASE RUNTIME open03: 4011 ms
========== END ltp open03 ==========
========== START ltp stat02 ==========
RUN LTP CASE stat02
LTP MEMORY stat02 before: free_frames=119620 allocated_frames=76549
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
stat02.c:67: TPASS: File size reported as expected
stat02.c:67: TPASS: File size reported as expected

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[829.296876 0:1597 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE stat02 : 0
Pass!
LTP MEMORY stat02 after_run: free_frames=119599 allocated_frames=76570
LTP MEMORY stat02 after_cleanup: free_frames=119599 allocated_frames=76570
LTP CASE RUNTIME stat02: 2920 ms
========== END ltp stat02 ==========
========== START ltp lstat01 ==========
RUN LTP CASE lstat01
LTP MEMORY lstat01 before: free_frames=119599 allocated_frames=76570
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
lstat01.c:46: TPASS: lstat() reported correct values for the symlink!

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[831.984592 0:1601 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[831.985883 0:1601 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE lstat01 : 0
Pass!
LTP MEMORY lstat01 after_run: free_frames=119578 allocated_frames=76591
LTP MEMORY lstat01 after_cleanup: free_frames=119578 allocated_frames=76591
LTP CASE RUNTIME lstat01: 2655 ms
========== END ltp lstat01 ==========
========== START ltp chmod01 ==========
RUN LTP CASE chmod01
LTP MEMORY chmod01 before: free_frames=119578 allocated_frames=76591
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

Summary:
passed   32
failed   0
broken   0
skipped  0
warnings 0
[37m[834.875942 0:1605 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[834.877072 0:1605 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[834.878171 0:1605 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE chmod01 : 0
Pass!
LTP MEMORY chmod01 after_run: free_frames=119548 allocated_frames=76621
LTP MEMORY chmod01 after_cleanup: free_frames=119548 allocated_frames=76621
LTP CASE RUNTIME chmod01: 2930 ms
========== END ltp chmod01 ==========
========== START ltp fchmod01 ==========
RUN LTP CASE fchmod01
LTP MEMORY fchmod01 before: free_frames=119548 allocated_frames=76621
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
fchmod01.c:40: TPASS: Functionality of fchmod(3, 0) successful
fchmod01.c:40: TPASS: Functionality of fchmod(3, 07) successful
fchmod01.c:40: TPASS: Functionality of fchmod(3, 070) successful
fchmod01.c:40: TPASS: Functionality of fchmod(3, 0700) successful
fchmod01.c:40: TPASS: Functionality of fchmod(3, 0777) successful
fchmod01.c:40: TPASS: Functionality of fchmod(3, 02777) successful
fchmod01.c:40: TPASS: Functionality of fchmod(3, 04777) successful
fchmod01.c:40: TPASS: Functionality of fchmod(3, 06777) successful

Summary:
passed   8
failed   0
broken   0
skipped  0
warnings 0
[37m[837.807781 0:1612 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[837.810224 0:1612 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fchmod01 : 0
Pass!
LTP MEMORY fchmod01 after_run: free_frames=119527 allocated_frames=76642
LTP MEMORY fchmod01 after_cleanup: free_frames=119527 allocated_frames=76642
LTP CASE RUNTIME fchmod01: 2893 ms
========== END ltp fchmod01 ==========
========== START ltp rmdir01 ==========
RUN LTP CASE rmdir01
LTP MEMORY rmdir01 before: free_frames=119527 allocated_frames=76642
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
rmdir01.c:33: TPASS: rmdir(testdir) success

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[840.520149 0:1616 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE rmdir01 : 0
Pass!
LTP MEMORY rmdir01 after_run: free_frames=119506 allocated_frames=76663
LTP MEMORY rmdir01 after_cleanup: free_frames=119506 allocated_frames=76663
LTP CASE RUNTIME rmdir01: 2699 ms
========== END ltp rmdir01 ==========
========== START ltp symlink01 ==========
RUN LTP CASE symlink01
LTP MEMORY symlink01 before: free_frames=119506 allocated_frames=76663
[37m[843.376264 0:1620 axfs::root:433] [33m[AxError::IsADirectory][m
[msymlink01    1  TPASS  :  Creation of symbolic link file to no object file is ok
symlink01    2  TPASS  :  Creation of symbolic link file to no object file is ok
symlink01    3  TPASS  :  Creation of symbolic link file and object file via symbolic link is ok
symlink01    4  TPASS  :  Creating an existing symbolic link file error is caught
symlink01    5  TPASS  :  Creating a symbolic link which exceeds maximum pathname error is caught
PASS LTP CASE symlink01 : 0
Pass!
LTP MEMORY symlink01 after_run: free_frames=119494 allocated_frames=76675
LTP MEMORY symlink01 after_cleanup: free_frames=119494 allocated_frames=76675
LTP CASE RUNTIME symlink01: 2871 ms
========== END ltp symlink01 ==========
========== START ltp readlink01 ==========
RUN LTP CASE readlink01
LTP MEMORY readlink01 before: free_frames=119494 allocated_frames=76675
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
readlink01.c:64: TINFO: Running test as root
readlink01.c:45: TPASS: readlink() functionality on 'slink_file' was correct
readlink01.c:55: TINFO: Running test as nobody
readlink01.c:45: TPASS: readlink() functionality on 'slink_file' was correct

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[846.198716 0:1621 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[846.201796 0:1621 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE readlink01 : 0
Pass!
LTP MEMORY readlink01 after_run: free_frames=119464 allocated_frames=76705
LTP MEMORY readlink01 after_cleanup: free_frames=119464 allocated_frames=76705
LTP CASE RUNTIME readlink01: 2825 ms
========== END ltp readlink01 ==========
========== START ltp ftruncate01 ==========
RUN LTP CASE ftruncate01
LTP MEMORY ftruncate01 before: free_frames=119464 allocated_frames=76705
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
ftruncate01.c:65: TINFO: Truncated length smaller than file size
ftruncate01.c:60: TPASS: ftruncate() succeeded
ftruncate01.c:74: TINFO: Truncated length exceeds file size
ftruncate01.c:60: TPASS: ftruncate() succeeded

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[849.182233 0:1626 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[849.186842 0:1626 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE ftruncate01 : 0
Pass!
LTP MEMORY ftruncate01 after_run: free_frames=119443 allocated_frames=76726
LTP MEMORY ftruncate01 after_cleanup: free_frames=119443 allocated_frames=76726
LTP CASE RUNTIME ftruncate01: 2983 ms
========== END ltp ftruncate01 ==========
========== START ltp umask01 ==========
RUN LTP CASE umask01
LTP MEMORY umask01 before: free_frames=119443 allocated_frames=76726
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
umask01.c:57: TPASS: All files created with correct mode

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[852.175801 0:1630 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE umask01 : 0
Pass!
LTP MEMORY umask01 after_run: free_frames=119422 allocated_frames=76747
LTP MEMORY umask01 after_cleanup: free_frames=119422 allocated_frames=76747
LTP CASE RUNTIME umask01: 2964 ms
========== END ltp umask01 ==========
========== START ltp alarm02 ==========
RUN LTP CASE alarm02
LTP MEMORY alarm02 before: free_frames=119422 allocated_frames=76747
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

Summary:
passed   6
failed   0
broken   0
skipped  0
warnings 0
[37m[855.030833 0:1634 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE alarm02 : 0
Pass!
LTP MEMORY alarm02 after_run: free_frames=119401 allocated_frames=76768
LTP MEMORY alarm02 after_cleanup: free_frames=119401 allocated_frames=76768
LTP CASE RUNTIME alarm02: 2845 ms
========== END ltp alarm02 ==========
========== START ltp alarm03 ==========
RUN LTP CASE alarm03
LTP MEMORY alarm03 before: free_frames=119401 allocated_frames=76768
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
alarm03.c:30: TPASS: alarm(0) in parent process passed
alarm03.c:26: TPASS: alarm(0) in child process passed

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[857.928117 0:1641 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE alarm03 : 0
Pass!
LTP MEMORY alarm03 after_run: free_frames=119371 allocated_frames=76798
LTP MEMORY alarm03 after_cleanup: free_frames=119371 allocated_frames=76798
LTP CASE RUNTIME alarm03: 2904 ms
========== END ltp alarm03 ==========
========== START ltp clock_gettime02 ==========
RUN LTP CASE clock_gettime02
LTP MEMORY clock_gettime02 before: free_frames=119371 allocated_frames=76798
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

Summary:
passed   10
failed   0
broken   0
skipped  0
warnings 0
[37m[860.686548 0:1647 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE clock_gettime02 : 0
Pass!
LTP MEMORY clock_gettime02 after_run: free_frames=119350 allocated_frames=76819
LTP MEMORY clock_gettime02 after_cleanup: free_frames=119350 allocated_frames=76819
LTP CASE RUNTIME clock_gettime02: 2822 ms
========== END ltp clock_gettime02 ==========
========== START ltp gettimeofday01 ==========
RUN LTP CASE gettimeofday01
LTP MEMORY gettimeofday01 before: free_frames=119350 allocated_frames=76819
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
gettimeofday01.c:36: TPASS: tst_syscall(__NR_gettimeofday, tc->tv, tc->tz) : EFAULT (14)
gettimeofday01.c:36: TPASS: tst_syscall(__NR_gettimeofday, tc->tv, tc->tz) : EFAULT (14)
gettimeofday01.c:36: TPASS: tst_syscall(__NR_gettimeofday, tc->tv, tc->tz) : EFAULT (14)

Summary:
passed   3
failed   0
broken   0
skipped  0
warnings 0
[37m[863.513910 0:1651 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE gettimeofday01 : 0
Pass!
LTP MEMORY gettimeofday01 after_run: free_frames=119329 allocated_frames=76840
LTP MEMORY gettimeofday01 after_cleanup: free_frames=119329 allocated_frames=76840
LTP CASE RUNTIME gettimeofday01: 2747 ms
========== END ltp gettimeofday01 ==========
========== START ltp time01 ==========
RUN LTP CASE time01
LTP MEMORY time01 before: free_frames=119329 allocated_frames=76840
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
time01.c:36: TPASS: time() returned value 866
time01.c:38: TPASS: time() returned value 866, stored value 866 are same

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[866.345235 0:1655 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE time01 : 0
Pass!
LTP MEMORY time01 after_run: free_frames=119308 allocated_frames=76861
LTP MEMORY time01 after_cleanup: free_frames=119308 allocated_frames=76861
LTP CASE RUNTIME time01: 2856 ms
========== END ltp time01 ==========
========== START ltp times01 ==========
RUN LTP CASE times01
LTP MEMORY times01 before: free_frames=119308 allocated_frames=76861
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
times01.c:25: TPASS: times(&mytimes) returned 869212

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[869.245633 0:1659 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE times01 : 0
Pass!
LTP MEMORY times01 after_run: free_frames=119287 allocated_frames=76882
LTP MEMORY times01 after_cleanup: free_frames=119287 allocated_frames=76882
LTP CASE RUNTIME times01: 2881 ms
========== END ltp times01 ==========
========== START ltp kill03 ==========
RUN LTP CASE kill03
LTP MEMORY kill03 before: free_frames=119287 allocated_frames=76882
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
kill03.c:41: TPASS: kill failed as expected: EINVAL (22)
kill03.c:41: TPASS: kill failed as expected: ESRCH (3)
kill03.c:41: TPASS: kill failed as expected: ESRCH (3)

Summary:
passed   3
failed   0
broken   0
skipped  0
warnings 0
[37m[872.060844 0:1663 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE kill03 : 0
Pass!
LTP MEMORY kill03 after_run: free_frames=119266 allocated_frames=76903
LTP MEMORY kill03 after_cleanup: free_frames=119266 allocated_frames=76903
LTP CASE RUNTIME kill03: 2802 ms
========== END ltp kill03 ==========
========== START ltp rt_sigaction01 ==========
RUN LTP CASE rt_sigaction01
LTP MEMORY rt_sigaction01 before: free_frames=119266 allocated_frames=76903
rt_sigaction01    0  TINFO  :  signal: 34 
rt_sigaction01    1  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 34
rt_sigaction01    0  TINFO  :  signal: 34 
rt_sigaction01    2  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 34
rt_sigaction01    0  TINFO  :  signal: 34 
rt_sigaction01    3  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 34
rt_sigaction01    0  TINFO  :  signal: 34 
rt_sigaction01    4  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 34
rt_sigaction01    0  TINFO  :  signal: 34 
rt_sigaction01    5  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 34
rt_sigaction01    0  TINFO  :  signal: 35 
rt_sigaction01    6  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 35
rt_sigaction01    0  TINFO  :  signal: 35 
rt_sigaction01    7  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 35
rt_sigaction01    0  TINFO  :  signal: 35 
rt_sigaction01    8  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 35
rt_sigaction01    0  TINFO  :  signal: 35 
rt_sigaction01    9  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 35
rt_sigaction01    0  TINFO  :  signal: 35 
rt_sigaction01   10  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 35
rt_sigaction01    0  TINFO  :  signal: 36 
rt_sigaction01   11  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 36
rt_sigaction01    0  TINFO  :  signal: 36 
rt_sigaction01   12  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 36
rt_sigaction01    0  TINFO  :  signal: 36 
rt_sigaction01   13  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 36
rt_sigaction01    0  TINFO  :  signal: 36 
rt_sigaction01   14  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 36
rt_sigaction01    0  TINFO  :  signal: 36 
rt_sigaction01   15  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 36
rt_sigaction01    0  TINFO  :  signal: 37 
rt_sigaction01   16  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 37
rt_sigaction01    0  TINFO  :  signal: 37 
rt_sigaction01   17  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 37
rt_sigaction01    0  TINFO  :  signal: 37 
rt_sigaction01   18  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 37
rt_sigaction01    0  TINFO  :  signal: 37 
rt_sigaction01   19  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 37
rt_sigaction01    0  TINFO  :  signal: 37 
rt_sigaction01   20  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 37
rt_sigaction01    0  TINFO  :  signal: 38 
rt_sigaction01   21  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 38
rt_sigaction01    0  TINFO  :  signal: 38 
rt_sigaction01   22  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 38
rt_sigaction01    0  TINFO  :  signal: 38 
rt_sigaction01   23  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 38
rt_sigaction01    0  TINFO  :  signal: 38 
rt_sigaction01   24  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 38
rt_sigaction01    0  TINFO  :  signal: 38 
rt_sigaction01   25  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 38
rt_sigaction01    0  TINFO  :  signal: 39 
rt_sigaction01   26  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 39
rt_sigaction01    0  TINFO  :  signal: 39 
rt_sigaction01   27  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 39
rt_sigaction01    0  TINFO  :  signal: 39 
rt_sigaction01   28  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 39
rt_sigaction01    0  TINFO  :  signal: 39 
rt_sigaction01   29  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 39
rt_sigaction01    0  TINFO  :  signal: 39 
rt_sigaction01   30  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 39
rt_sigaction01    0  TINFO  :  signal: 40 
rt_sigaction01   31  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 40
rt_sigaction01    0  TINFO  :  signal: 40 
rt_sigaction01   32  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 40
rt_sigaction01    0  TINFO  :  signal: 40 
rt_sigaction01   33  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 40
rt_sigaction01    0  TINFO  :  signal: 40 
rt_sigaction01   34  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 40
rt_sigaction01    0  TINFO  :  signal: 40 
rt_sigaction01   35  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 40
rt_sigaction01    0  TINFO  :  signal: 41 
rt_sigaction01   36  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 41
rt_sigaction01    0  TINFO  :  signal: 41 
rt_sigaction01   37  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 41
rt_sigaction01    0  TINFO  :  signal: 41 
rt_sigaction01   38  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 41
rt_sigaction01    0  TINFO  :  signal: 41 
rt_sigaction01   39  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 41
rt_sigaction01    0  TINFO  :  signal: 41 
rt_sigaction01   40  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 41
rt_sigaction01    0  TINFO  :  signal: 42 
rt_sigaction01   41  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 42
rt_sigaction01    0  TINFO  :  signal: 42 
rt_sigaction01   42  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 42
rt_sigaction01    0  TINFO  :  signal: 42 
rt_sigaction01   43  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 42
rt_sigaction01    0  TINFO  :  signal: 42 
rt_sigaction01   44  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 42
rt_sigaction01    0  TINFO  :  signal: 42 
rt_sigaction01   45  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 42
rt_sigaction01    0  TINFO  :  signal: 43 
rt_sigaction01   46  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 43
rt_sigaction01    0  TINFO  :  signal: 43 
rt_sigaction01   47  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 43
rt_sigaction01    0  TINFO  :  signal: 43 
rt_sigaction01   48  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 43
rt_sigaction01    0  TINFO  :  signal: 43 
rt_sigaction01   49  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 43
rt_sigaction01    0  TINFO  :  signal: 43 
rt_sigaction01   50  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 43
rt_sigaction01    0  TINFO  :  signal: 44 
rt_sigaction01   51  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 44
rt_sigaction01    0  TINFO  :  signal: 44 
rt_sigaction01   52  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 44
rt_sigaction01    0  TINFO  :  signal: 44 
rt_sigaction01   53  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 44
rt_sigaction01    0  TINFO  :  signal: 44 
rt_sigaction01   54  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 44
rt_sigaction01    0  TINFO  :  signal: 44 
rt_sigaction01   55  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 44
rt_sigaction01    0  TINFO  :  signal: 45 
rt_sigaction01   56  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 45
rt_sigaction01    0  TINFO  :  signal: 45 
rt_sigaction01   57  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 45
rt_sigaction01    0  TINFO  :  signal: 45 
rt_sigaction01   58  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 45
rt_sigaction01    0  TINFO  :  signal: 45 
rt_sigaction01   59  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 45
rt_sigaction01    0  TINFO  :  signal: 45 
rt_sigaction01   60  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 45
rt_sigaction01    0  TINFO  :  signal: 46 
rt_sigaction01   61  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 46
rt_sigaction01    0  TINFO  :  signal: 46 
rt_sigaction01   62  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 46
rt_sigaction01    0  TINFO  :  signal: 46 
rt_sigaction01   63  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 46
rt_sigaction01    0  TINFO  :  signal: 46 
rt_sigaction01   64  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 46
rt_sigaction01    0  TINFO  :  signal: 46 
rt_sigaction01   65  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 46
rt_sigaction01    0  TINFO  :  signal: 47 
rt_sigaction01   66  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 47
rt_sigaction01    0  TINFO  :  signal: 47 
rt_sigaction01   67  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 47
rt_sigaction01    0  TINFO  :  signal: 47 
rt_sigaction01   68  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 47
rt_sigaction01    0  TINFO  :  signal: 47 
rt_sigaction01   69  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 47
rt_sigaction01    0  TINFO  :  signal: 47 
rt_sigaction01   70  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 47
rt_sigaction01    0  TINFO  :  signal: 48 
rt_sigaction01   71  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 48
rt_sigaction01    0  TINFO  :  signal: 48 
rt_sigaction01   72  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 48
rt_sigaction01    0  TINFO  :  signal: 48 
rt_sigaction01   73  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 48
rt_sigaction01    0  TINFO  :  signal: 48 
rt_sigaction01   74  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 48
rt_sigaction01    0  TINFO  :  signal: 48 
rt_sigaction01   75  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 48
rt_sigaction01    0  TINFO  :  signal: 49 
rt_sigaction01   76  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 49
rt_sigaction01    0  TINFO  :  signal: 49 
rt_sigaction01   77  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 49
rt_sigaction01    0  TINFO  :  signal: 49 
rt_sigaction01   78  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 49
rt_sigaction01    0  TINFO  :  signal: 49 
rt_sigaction01   79  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 49
rt_sigaction01    0  TINFO  :  signal: 49 
rt_sigaction01   80  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 49
rt_sigaction01    0  TINFO  :  signal: 50 
rt_sigaction01   81  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 50
rt_sigaction01    0  TINFO  :  signal: 50 
rt_sigaction01   82  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 50
rt_sigaction01    0  TINFO  :  signal: 50 
rt_sigaction01   83  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 50
rt_sigaction01    0  TINFO  :  signal: 50 
rt_sigaction01   84  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 50
rt_sigaction01    0  TINFO  :  signal: 50 
rt_sigaction01   85  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 50
rt_sigaction01    0  TINFO  :  signal: 51 
rt_sigaction01   86  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 51
rt_sigaction01    0  TINFO  :  signal: 51 
rt_sigaction01   87  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 51
rt_sigaction01    0  TINFO  :  signal: 51 
rt_sigaction01   88  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 51
rt_sigaction01    0  TINFO  :  signal: 51 
rt_sigaction01   89  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 51
rt_sigaction01    0  TINFO  :  signal: 51 
rt_sigaction01   90  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 51
rt_sigaction01    0  TINFO  :  signal: 52 
rt_sigaction01   91  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 52
rt_sigaction01    0  TINFO  :  signal: 52 
rt_sigaction01   92  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 52
rt_sigaction01    0  TINFO  :  signal: 52 
rt_sigaction01   93  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 52
rt_sigaction01    0  TINFO  :  signal: 52 
rt_sigaction01   94  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 52
rt_sigaction01    0  TINFO  :  signal: 52 
rt_sigaction01   95  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 52
rt_sigaction01    0  TINFO  :  signal: 53 
rt_sigaction01   96  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 53
rt_sigaction01    0  TINFO  :  signal: 53 
rt_sigaction01   97  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 53
rt_sigaction01    0  TINFO  :  signal: 53 
rt_sigaction01   98  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 53
rt_sigaction01    0  TINFO  :  signal: 53 
rt_sigaction01   99  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 53
rt_sigaction01    0  TINFO  :  signal: 53 
rt_sigaction01  100  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 53
rt_sigaction01    0  TINFO  :  signal: 54 
rt_sigaction01  101  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 54
rt_sigaction01    0  TINFO  :  signal: 54 
rt_sigaction01  102  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 54
rt_sigaction01    0  TINFO  :  signal: 54 
rt_sigaction01  103  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 54
rt_sigaction01    0  TINFO  :  signal: 54 
rt_sigaction01  104  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 54
rt_sigaction01    0  TINFO  :  signal: 54 
rt_sigaction01  105  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 54
rt_sigaction01    0  TINFO  :  signal: 55 
rt_sigaction01  106  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 55
rt_sigaction01    0  TINFO  :  signal: 55 
rt_sigaction01  107  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 55
rt_sigaction01    0  TINFO  :  signal: 55 
rt_sigaction01  108  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 55
rt_sigaction01    0  TINFO  :  signal: 55 
rt_sigaction01  109  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 55
rt_sigaction01    0  TINFO  :  signal: 55 
rt_sigaction01  110  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 55
rt_sigaction01    0  TINFO  :  signal: 56 
rt_sigaction01  111  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 56
rt_sigaction01    0  TINFO  :  signal: 56 
rt_sigaction01  112  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 56
rt_sigaction01    0  TINFO  :  signal: 56 
rt_sigaction01  113  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 56
rt_sigaction01    0  TINFO  :  signal: 56 
rt_sigaction01  114  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 56
rt_sigaction01    0  TINFO  :  signal: 56 
rt_sigaction01  115  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 56
rt_sigaction01    0  TINFO  :  signal: 57 
rt_sigaction01  116  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 57
rt_sigaction01    0  TINFO  :  signal: 57 
rt_sigaction01  117  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 57
rt_sigaction01    0  TINFO  :  signal: 57 
rt_sigaction01  118  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 57
rt_sigaction01    0  TINFO  :  signal: 57 
rt_sigaction01  119  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 57
rt_sigaction01    0  TINFO  :  signal: 57 
rt_sigaction01  120  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 57
rt_sigaction01    0  TINFO  :  signal: 58 
rt_sigaction01  121  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 58
rt_sigaction01    0  TINFO  :  signal: 58 
rt_sigaction01  122  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 58
rt_sigaction01    0  TINFO  :  signal: 58 
rt_sigaction01  123  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 58
rt_sigaction01    0  TINFO  :  signal: 58 
rt_sigaction01  124  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 58
rt_sigaction01    0  TINFO  :  signal: 58 
rt_sigaction01  125  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 58
rt_sigaction01    0  TINFO  :  signal: 59 
rt_sigaction01  126  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 59
rt_sigaction01    0  TINFO  :  signal: 59 
rt_sigaction01  127  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 59
rt_sigaction01    0  TINFO  :  signal: 59 
rt_sigaction01  128  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 59
rt_sigaction01    0  TINFO  :  signal: 59 
rt_sigaction01  129  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 59
rt_sigaction01    0  TINFO  :  signal: 59 
rt_sigaction01  130  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 59
rt_sigaction01    0  TINFO  :  signal: 60 
rt_sigaction01  131  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 60
rt_sigaction01    0  TINFO  :  signal: 60 
rt_sigaction01  132  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 60
rt_sigaction01    0  TINFO  :  signal: 60 
rt_sigaction01  133  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 60
rt_sigaction01    0  TINFO  :  signal: 60 
rt_sigaction01  134  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 60
rt_sigaction01    0  TINFO  :  signal: 60 
rt_sigaction01  135  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 60
rt_sigaction01    0  TINFO  :  signal: 61 
rt_sigaction01  136  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 61
rt_sigaction01    0  TINFO  :  signal: 61 
rt_sigaction01  137  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 61
rt_sigaction01    0  TINFO  :  signal: 61 
rt_sigaction01  138  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 61
rt_sigaction01    0  TINFO  :  signal: 61 
rt_sigaction01  139  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 61
rt_sigaction01    0  TINFO  :  signal: 61 
rt_sigaction01  140  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 61
rt_sigaction01    0  TINFO  :  signal: 62 
rt_sigaction01  141  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 62
rt_sigaction01    0  TINFO  :  signal: 62 
rt_sigaction01  142  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 62
rt_sigaction01    0  TINFO  :  signal: 62 
rt_sigaction01  143  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 62
rt_sigaction01    0  TINFO  :  signal: 62 
rt_sigaction01  144  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 62
rt_sigaction01    0  TINFO  :  signal: 62 
rt_sigaction01  145  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 62
rt_sigaction01    0  TINFO  :  signal: 63 
rt_sigaction01  146  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 63
rt_sigaction01    0  TINFO  :  signal: 63 
rt_sigaction01  147  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 63
rt_sigaction01    0  TINFO  :  signal: 63 
rt_sigaction01  148  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 63
rt_sigaction01    0  TINFO  :  signal: 63 
rt_sigaction01  149  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 63
rt_sigaction01    0  TINFO  :  signal: 63 
rt_sigaction01  150  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 63
rt_sigaction01    0  TINFO  :  signal: 64 
rt_sigaction01  151  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 64
rt_sigaction01    0  TINFO  :  signal: 64 
rt_sigaction01  152  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 64
rt_sigaction01    0  TINFO  :  signal: 64 
rt_sigaction01  153  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 64
rt_sigaction01    0  TINFO  :  signal: 64 
rt_sigaction01  154  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 64
rt_sigaction01    0  TINFO  :  signal: 64 
rt_sigaction01  155  TPASS  :  rt_sigaction call succeeded: result = 0 
rt_sigaction01    0  TINFO  : [37m[875.169169 0:1667 axfs::root:433] [33m[AxError::IsADirectory][m
[m sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 64
PASS LTP CASE rt_sigaction01 : 0
Pass!
LTP MEMORY rt_sigaction01 after_run: free_frames=119254 allocated_frames=76915
LTP MEMORY rt_sigaction01 after_cleanup: free_frames=119254 allocated_frames=76915
LTP CASE RUNTIME rt_sigaction01: 3114 ms
========== END ltp rt_sigaction01 ==========
========== START ltp sigaction01 ==========
RUN LTP CASE sigaction01
LTP MEMORY sigaction01 before: free_frames=119254 allocated_frames=76915
sigaction01    1  TPASS  :  SA_RESETHAND did not cause SA_SIGINFO to be cleared
sigaction01    2  TPASS  :  SA_RESETHAND was masked when handler executed
sigaction01    3  TPASS  :  sig has been masked because sa_mask originally contained sig
sigaction01    4  TPASS  :  siginfo pointer non NULL
PASS LTP CASE sigaction01 : 0
Pass!
LTP MEMORY sigaction01 after_run: free_frames=119242 allocated_frames=76927
LTP MEMORY sigaction01 after_cleanup: free_frames=119242 allocated_frames=76927
LTP CASE RUNTIME sigaction01: 2838 ms
========== END ltp sigaction01 ==========
========== START ltp proc01 ==========
RUN LTP CASE proc01
LTP MEMORY proc01 before: free_frames=119242 allocated_frames=76927
[37m[880.991371 0:1669 axfs::root:433] [33m[AxError::IsADirectory][m
[mproc01      1  TPASS  :  readproc() completed successfully, total read: 875 bytes, 20 objs
PASS LTP CASE proc01 : 0
Pass!
LTP MEMORY proc01 after_run: free_frames=119230 allocated_frames=76939
LTP MEMORY proc01 after_cleanup: free_frames=119230 allocated_frames=76939
LTP CASE RUNTIME proc01: 2980 ms
========== END ltp proc01 ==========
========== START ltp exit01 ==========
RUN LTP CASE exit01
LTP MEMORY exit01 before: free_frames=119230 allocated_frames=76939
exit01      1  TPASS  :  exit() test PASSED
PASS LTP CASE exit01 : 0
Pass!
LTP MEMORY exit01 after_run: free_frames=119209 allocated_frames=76960
LTP MEMORY exit01 after_cleanup: free_frames=119209 allocated_frames=76960
LTP CASE RUNTIME exit01: 2848 ms
========== END ltp exit01 ==========
========== START ltp exit02 ==========
RUN LTP CASE exit02
LTP MEMORY exit02 before: free_frames=119209 allocated_frames=76960
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
exit02.c:46: TPASS: File written by child read back correctly

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[886.616522 0:1672 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE exit02 : 0
Pass!
LTP MEMORY exit02 after_run: free_frames=119179 allocated_frames=76990
LTP MEMORY exit02 after_cleanup: free_frames=119179 allocated_frames=76990
LTP CASE RUNTIME exit02: 2772 ms
========== END ltp exit02 ==========
========== START ltp exit_group01 ==========
RUN LTP CASE exit_group01
LTP MEMORY exit_group01 before: free_frames=119179 allocated_frames=76990
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
exit_group01.c:98: TPASS: Expect: exit_group() succeeded
exit_group01.c:61: TINFO: Checking if threads are still running
exit_group01.c:77: TINFO: Threads counters value didn't change

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[889.638664 0:1677 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[889.640480 0:1677 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE exit_group01 : 0
Pass!
LTP MEMORY exit_group01 after_run: free_frames=119147 allocated_frames=77022
LTP MEMORY exit_group01 after_cleanup: free_frames=119147 allocated_frames=77022
LTP CASE RUNTIME exit_group01: 3021 ms
========== END ltp exit_group01 ==========
========== START ltp getpgrp01 ==========
RUN LTP CASE getpgrp01
LTP MEMORY getpgrp01 before: free_frames=119147 allocated_frames=77022
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getpgrp01.c:18: TPASS: getpgrp() returned pid 1686
getpgrp01.c:19: TPASS: TST_RET == SAFE_GETPGID(0) (1686)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[892.585311 0:1684 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpgrp01 : 0
Pass!
LTP MEMORY getpgrp01 after_run: free_frames=119126 allocated_frames=77043
LTP MEMORY getpgrp01 after_cleanup: free_frames=119126 allocated_frames=77043
LTP CASE RUNTIME getpgrp01: 2944 ms
========== END ltp getpgrp01 ==========
========== START ltp gettid01 ==========
RUN LTP CASE gettid01
LTP MEMORY gettid01 before: free_frames=119126 allocated_frames=77043
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
gettid01.c:26: TPASS: tst_syscall(__NR_gettid) == tst_syscall(__NR_getpid) (1690)
gettid01.c:27: TPASS: tst_syscall(__NR_gettid) == pid (1690)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[895.292017 0:1688 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE gettid01 : 0
Pass!
LTP MEMORY gettid01 after_run: free_frames=119105 allocated_frames=77064
LTP MEMORY gettid01 after_cleanup: free_frames=119105 allocated_frames=77064
LTP CASE RUNTIME gettid01: 2727 ms
========== END ltp gettid01 ==========
========== START ltp uname01 ==========
RUN LTP CASE uname01
LTP MEMORY uname01 before: free_frames=119105 allocated_frames=77064
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
uname01.c:21: TPASS: uname(&un) passed
uname01.c:31: TPASS: sysname set to Linux

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[898.164886 0:1692 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE uname01 : 0
Pass!
LTP MEMORY uname01 after_run: free_frames=119084 allocated_frames=77085
LTP MEMORY uname01 after_cleanup: free_frames=119084 allocated_frames=77085
LTP CASE RUNTIME uname01: 2917 ms
========== END ltp uname01 ==========
========== START ltp getrlimit01 ==========
RUN LTP CASE getrlimit01
LTP MEMORY getrlimit01 before: free_frames=119084 allocated_frames=77085
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_CPU passed
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_FSIZE passed
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_DATA passed
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_STACK passed
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_CORE passed
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_RSS passed
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_NPROC passed
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_NOFILE passed
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_MEMLOCK passed
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_AS passed
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_LOCKS passed
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_MSGQUEUE passed
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_NICE passed
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_RTPRIO passed
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_SIGPENDING passed
getrlimit01.c:50: TPASS: getrlimit() test for RLIMIT_RTTIME passed

Summary:
passed   16
failed   0
broken   0
skipped  0
warnings 0
[37m[901.055725 0:1696 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getrlimit01 : 0
Pass!
LTP MEMORY getrlimit01 after_run: free_frames=119063 allocated_frames=77106
LTP MEMORY getrlimit01 after_cleanup: free_frames=119063 allocated_frames=77106
LTP CASE RUNTIME getrlimit01: 2800 ms
========== END ltp getrlimit01 ==========
========== START ltp getrusage01 ==========
RUN LTP CASE getrusage01
LTP MEMORY getrusage01 before: free_frames=119063 allocated_frames=77106
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getrusage01.c:29: TPASS: getrusage(RUSAGE_SELF, 0x1000223f70) passed
getrusage01.c:29: TPASS: getrusage(RUSAGE_CHILDREN, 0x1000223f70) passed

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[903.907798 0:1700 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getrusage01 : 0
Pass!
LTP MEMORY getrusage01 after_run: free_frames=119042 allocated_frames=77127
LTP MEMORY getrusage01 after_cleanup: free_frames=119042 allocated_frames=77127
LTP CASE RUNTIME getrusage01: 2870 ms
========== END ltp getrusage01 ==========
========== START ltp sched_yield01 ==========
RUN LTP CASE sched_yield01
LTP MEMORY sched_yield01 before: free_frames=119042 allocated_frames=77127
sched_yield01    1  TPASS  :  sched_yield() call succeeded
PASS LTP CASE sched_yield01 : 0
Pass!
LTP MEMORY sched_yield01 after_run: free_frames=119030 allocated_frames=77139
LTP MEMORY sched_yield01 after_cleanup: free_frames=119030 allocated_frames=77139
LTP CASE RUNTIME sched_yield01: 2806 ms
========== END ltp sched_yield01 ==========
ltp cases: 63 passed, 0 failed, 0 timed out
#### OS COMP TEST GROUP END ltp-glibc ####
#### OS COMP TEST GROUP START libcbench-musl ####
b_malloc_sparse (0)
  time: 1.858284810, virt: 0, res: 0, dirty: 0

b_malloc_bubble (0)
  time: 1.783211600, virt: 0, res: 0, dirty: 0

b_malloc_tiny1 (0)
  time: 0.074924000, virt: 0, res: 0, dirty: 0

b_malloc_tiny2 (0)
  time: 0.020889930, virt: 0, res: 0, dirty: 0

b_malloc_big1 (0)
  time: 0.612436440, virt: 0, res: 0, dirty: 0

b_malloc_big2 (0)
  time: 0.650003760, virt: 0, res: 0, dirty: 0

b_malloc_thread_stress (0)
  time: 0.153398770, virt: 0, res: 0, dirty: 0

b_malloc_thread_local (0)
  time: 0.172562450, virt: 0, res: 0, dirty: 0

b_string_strstr ("abcdefghijklmnopqrstuvwxyz")
  time: 0.052325870, virt: 0, res: 0, dirty: 0

b_string_strstr ("azbycxdwevfugthsirjqkplomn")
  time: 0.076411550, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaacccccccccccc")
  time: 0.052205340, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.027848230, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.033708760, virt: 0, res: 0, dirty: 0

b_string_memset (0)
  time: 0.020912260, virt: 0, res: 0, dirty: 0

b_string_strchr (0)
  time: 0.036684270, virt: 0, res: 0, dirty: 0

b_string_strlen (0)
  time: 0.047132340, virt: 0, res: 0, dirty: 0

b_pthread_createjoin_serial1 (0)
  time: 0.009926740, virt: 0, res: 0, dirty: 0

b_pthread_createjoin_serial2 (0)
  time: 1.282777360, virt: 0, res: 0, dirty: 0

b_pthread_create_serial1 (0)
  time: 0.001691650, virt: 0, res: 0, dirty: 0

b_pthread_uselesslock (0)
  time: 0.108820130, virt: 0, res: 0, dirty: 0

b_utf8_bigbuf (0)
  time: 0.137168140, virt: 0, res: 0, dirty: 0

b_utf8_onebyone (0)
  time: 0.322090030, virt: 0, res: 0, dirty: 0

b_stdio_putcgetc (0)
  time: 0.738418990, virt: 0, res: 0, dirty: 0

b_stdio_putcgetc_unlocked (0)
  time: 0.602696210, virt: 0, res: 0, dirty: 0

b_regex_compile ("(a|b|c)*d*b")
  time: 0.865281430, virt: 0, res: 0, dirty: 0

b_regex_search ("(a|b|c)*d*b")
  time: 0.265730130, virt: 0, res: 0, dirty: 0

b_regex_search ("a{25}b")
  time: 0.632143110, virt: 0, res: 0, dirty: 0

#### OS COMP TEST GROUP END libcbench-musl ####
#### OS COMP TEST GROUP START libcbench-glibc ####
b_malloc_sparse (0)
  time: 1.610559160, virt: 0, res: 0, dirty: 0

b_malloc_bubble (0)
  time: 1.522607560, virt: 0, res: 0, dirty: 0

b_malloc_tiny1 (0)
  time: 0.033432380, virt: 0, res: 0, dirty: 0

b_malloc_tiny2 (0)
  time: 0.036804860, virt: 0, res: 0, dirty: 0

b_malloc_big1 (0)
  time: 0.323382630, virt: 0, res: 0, dirty: 0

b_malloc_big2 (0)
  time: 0.280331920, virt: 0, res: 0, dirty: 0

The futex facility returned an unexpected error code.
The futex facility returned an unexpected error code.
b_string_strstr ("abcdefghijklmnopqrstuvwxyz")
  time: 0.037714080, virt: 0, res: 0, dirty: 0

b_string_strstr ("azbycxdwevfugthsirjqkplomn")
  time: 0.063435200, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaacccccccccccc")
  time: 0.059036290, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.024186670, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.026733340, virt: 0, res: 0, dirty: 0

b_string_memset (0)
  time: 0.038862160, virt: 0, res: 0, dirty: 0

b_string_strchr (0)
  time: 0.062097090, virt: 0, res: 0, dirty: 0

b_string_strlen (0)
  time: 0.050093730, virt: 0, res: 0, dirty: 0

The futex facility returned an unexpected error code.
b_pthread_create_serial1 (0)
  time: 0.005090060, virt: 0, res: 0, dirty: 0

The futex facility returned an unexpected error code.
b_utf8_bigbuf (0)
  time: 0.111604810, virt: 0, res: 0, dirty: 0

b_utf8_onebyone (0)
  time: 0.112415510, virt: 0, res: 0, dirty: 0

b_regex_compile ("(a|b|c)*d*b")
  time: 0.069208390, virt: 0, res: 0, dirty: 0

b_regex_search ("(a|b|c)*d*b")
  time: 0.025412960, virt: 0, res: 0, dirty: 0

b_regex_search ("a{25}b")
  time: 0.300342190, virt: 0, res: 0, dirty: 0

#### OS COMP TEST GROUP END libcbench-glibc ####
#### OS COMP TEST GROUP START iperf-musl ####
====== iperf BASIC_UDP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 0.0.0.0 port 49152 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  5]   0.00-2.00   sec  2.34 MBytes  9.83 Mbits/sec  1684  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.00   sec  2.34 MBytes  9.83 Mbits/sec  0.000 ms  0/1684 (0%)  sender
[  5]   0.00-2.00   sec  2.34 MBytes  9.81 Mbits/sec  0.084 ms  0/1684 (0%)  receiver

iperf Done.
====== iperf BASIC_UDP end: success ======

====== iperf BASIC_TCP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 127.0.0.1 port 49154 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.00   sec  51.2 MBytes   215 Mbits/sec    0   0.00 Bytes       
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.00   sec  51.2 MBytes   215 Mbits/sec    0             sender
[  5]   0.00-2.01   sec  50.4 MBytes   210 Mbits/sec                  receiver

iperf Done.
====== iperf BASIC_TCP end: success ======

====== iperf PARALLEL_UDP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 0.0.0.0 port 49153 connected to 127.0.0.1 port 5001
[  7] local 0.0.0.0 port 49154 connected to 127.0.0.1 port 5001
[  9] local 0.0.0.0 port 49155 connected to 127.0.0.1 port 5001
[ 11] local 0.0.0.0 port 49156 connected to 127.0.0.1 port 5001
[ 13] local 0.0.0.0 port 49157 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  5]   0.00-2.00   sec   654 KBytes  2.68 Mbits/sec  459  
[  7]   0.00-2.00   sec   654 KBytes  2.68 Mbits/sec  459  
[  9]   0.00-2.00   sec   654 KBytes  2.68 Mbits/sec  459  
[ 11]   0.00-2.00   sec   654 KBytes  2.68 Mbits/sec  459  
[ 13]   0.00-2.00   sec   654 KBytes  2.68 Mbits/sec  459  
[SUM]   0.00-2.00   sec  3.20 MBytes  13.4 Mbits/sec  2295  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.00   sec   654 KBytes  2.68 Mbits/sec  0.000 ms  0/459 (0%)  sender
[  5]   0.00-2.01   sec   654 KBytes  2.67 Mbits/sec  0.924 ms  0/459 (0%)  receiver
[  7]   0.00-2.00   sec   654 KBytes  2.68 Mbits/sec  0.000 ms  0/459 (0%)  sender
[  7]   0.00-2.01   sec   654 KBytes  2.67 Mbits/sec  0.635 ms  0/459 (0%)  receiver
[  9]   0.00-2.00   sec   654 KBytes  2.68 Mbits/sec  0.000 ms  0/459 (0%)  sender
[  9]   0.00-2.01   sec   654 KBytes  2.67 Mbits/sec  0.447 ms  0/459 (0%)  receiver
[ 11]   0.00-2.00   sec   654 KBytes  2.68 Mbits/sec  0.000 ms  0/459 (0%)  sender
[ 11]   0.00-2.01   sec   654 KBytes  2.67 Mbits/sec  1.500 ms  0/459 (0%)  receiver
[ 13]   0.00-2.00   sec   654 KBytes  2.68 Mbits/sec  0.000 ms  0/459 (0%)  sender
[ 13]   0.00-2.01   sec   654 KBytes  2.67 Mbits/sec  1.195 ms  0/459 (0%)  receiver
[SUM]   0.00-2.00   sec  3.20 MBytes  13.4 Mbits/sec  0.000 ms  0/2295 (0%)  sender
[SUM]   0.00-2.01   sec  3.20 MBytes  13.3 Mbits/sec  0.940 ms  0/2295 (0%)  receiver

iperf Done.
====== iperf PARALLEL_UDP end: success ======

====== iperf PARALLEL_TCP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 127.0.0.1 port 49157 connected to 127.0.0.1 port 5001
[  7] local 127.0.0.1 port 49158 connected to 127.0.0.1 port 5001
[  9] local 127.0.0.1 port 49159 connected to 127.0.0.1 port 5001
[ 11] local 127.0.0.1 port 49160 connected to 127.0.0.1 port 5001
[ 13] local 127.0.0.1 port 49161 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.00   sec  12.5 MBytes  52.3 Mbits/sec    0   0.00 Bytes       
[  7]   0.00-2.01   sec  12.5 MBytes  52.3 Mbits/sec    0   0.00 Bytes       
[  9]   0.00-2.01   sec  12.5 MBytes  52.3 Mbits/sec    0   0.00 Bytes       
[ 11]   0.00-2.01   sec  12.5 MBytes  52.3 Mbits/sec    0   0.00 Bytes       
[ 13]   0.00-2.01   sec  12.5 MBytes  52.3 Mbits/sec    0   0.00 Bytes       
[SUM]   0.00-2.00   sec  62.5 MBytes   262 Mbits/sec    0             
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.00   sec  12.5 MBytes  52.3 Mbits/sec    0             sender
[  5]   0.00-2.01   sec  11.6 MBytes  48.4 Mbits/sec                  receiver
[  7]   0.00-2.00   sec  12.5 MBytes  52.3 Mbits/sec    0             sender
[  7]   0.00-2.01   sec  11.6 MBytes  48.4 Mbits/sec                  receiver
[  9]   0.00-2.00   sec  12.5 MBytes  52.3 Mbits/sec    0             sender
[  9]   0.00-2.01   sec  11.6 MBytes  48.4 Mbits/sec                  receiver
[ 11]   0.00-2.00   sec  12.5 MBytes  52.3 Mbits/sec    0             sender
[ 11]   0.00-2.01   sec  11.6 MBytes  48.4 Mbits/sec                  receiver
[ 13]   0.00-2.00   sec  12.5 MBytes  52.3 Mbits/sec    0             sender
[ 13]   0.00-2.01   sec  11.6 MBytes  48.4 Mbits/sec                  receiver
[SUM]   0.00-2.00   sec  62.5 MBytes   262 Mbits/sec    0             sender
[SUM]   0.00-2.01   sec  58.1 MBytes   242 Mbits/sec                  receiver

iperf Done.
====== iperf PARALLEL_TCP end: success ======

====== iperf REVERSE_UDP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 0.0.0.0 port 49158 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.00   sec  2.36 MBytes  9.90 Mbits/sec  0.558 ms  0/1696 (0%)  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.01   sec  2.36 MBytes  9.88 Mbits/sec  0.000 ms  0/1697 (0%)  sender
[  5]   0.00-2.00   sec  2.36 MBytes  9.90 Mbits/sec  0.558 ms  0/1696 (0%)  receiver

iperf Done.
====== iperf REVERSE_UDP end: success ======

====== iperf REVERSE_TCP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 127.0.0.1 port 49164 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.00   sec  53.8 MBytes   225 Mbits/sec                  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.02   sec  54.8 MBytes   227 Mbits/sec    0             sender
[  5]   0.00-2.00   sec  53.8 MBytes   225 Mbits/sec                  receiver

iperf Done.
====== iperf REVERSE_TCP end: success ======

#### OS COMP TEST GROUP END iperf-musl ####
#### OS COMP TEST GROUP START iperf-glibc ####
====== iperf BASIC_UDP begin ======
iperf3: error - control socket has closed unexpectedly
====== iperf BASIC_UDP end: fail ======

====== iperf BASIC_TCP begin ======
[37m[968.018521 0:4365 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[968.020592 0:4365 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf BASIC_TCP end: fail ======

====== iperf PARALLEL_UDP begin ======
[37m[968.110190 0:4366 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[968.110845 0:4366 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf PARALLEL_UDP end: fail ======

====== iperf PARALLEL_TCP begin ======
[37m[968.228766 0:4367 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[968.229427 0:4367 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf PARALLEL_TCP end: fail ======

====== iperf REVERSE_UDP begin ======
[37m[968.380439 0:4368 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[968.386899 0:4368 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf REVERSE_UDP end: fail ======

====== iperf REVERSE_TCP begin ======
[37m[968.469856 0:4369 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[968.470499 0:4369 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf REVERSE_TCP end: fail ======

#### OS COMP TEST GROUP END iperf-glibc ####
#### OS COMP TEST GROUP START lmbench-musl ####
latency measurements
Simple syscall: 2.5250 microseconds
Simple read: 11.2090 microseconds
Simple write: 9.7721 microseconds
[37m[987.598478 0:4383 axfs::root:423] [33m[AxError::AlreadyExists][m
[m[37m[987.600545 0:4383 axfs::root:423] [33m[AxError::AlreadyExists][m
[mSimple stat: 179.1754 microseconds
Simple fstat: 16.3730 microseconds
Simple open/close: 192.7571 microseconds
Select on 100 fd's: 273.7218 microseconds
autorun: /tmp/testsuite/musl/lmbench/lmbench_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START lmbench-glibc ####
latency measurements
Simple syscall: 1.7287 microseconds
Simple read: 12.0918 microseconds
Simple write: 8.6212 microseconds
[37m[1081.002471 0:4410 axfs::root:423] [33m[AxError::AlreadyExists][m
[m[37m[1081.003517 0:4410 axfs::root:423] [33m[AxError::AlreadyExists][m
[m[37m[1081.004144 0:4410 axfs::root:423] [33m[AxError::AlreadyExists][m
[mautorun: /tmp/testsuite/glibc/lmbench/lmbench_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START netperf-musl ####
====== netperf UDP_STREAM begin ======
Starting netserver with host '127.0.0.1' port '12865' and family AF_UNSPEC
MIGRATED UDP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Socket  Message  Elapsed      Messages                
Size    Size     Time         Okay Errors   Throughput
bytes   bytes    secs            #      #   10^6bits/sec

 65536    1000   1.01         1352      0      10.76
 65536           1.01         1352             10.76

====== netperf UDP_STREAM end: success ======
====== netperf TCP_STREAM begin ======
MIGRATED TCP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Recv   Send    Send                          
Socket Socket  Message  Elapsed              
Size   Size    Size     Time     Throughput  
bytes  bytes   bytes    secs.    10^6bits/sec  

 65536  65536   1000    0.10       83.69   
====== netperf TCP_STREAM end: success ======
====== netperf UDP_RR begin ======
MIGRATED UDP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.01     1143.49   
65536  65536 
====== netperf UDP_RR end: success ======
====== netperf TCP_RR begin ======
MIGRATED TCP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00     1234.40   
65536  65536 
====== netperf TCP_RR end: success ======
====== netperf TCP_CRR begin ======
MIGRATED TCP Connect/Request/Response TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00      808.19   
65536  65536 
====== netperf TCP_CRR end: success ======
#### OS COMP TEST GROUP END netperf-musl ####
#### OS COMP TEST GROUP START netperf-glibc ####
====== netperf UDP_STREAM begin ======
[37m[1133.810644 0:4445 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[1133.817291 0:4445 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[mStarting netserver with host '127.0.0.1' port '12865' and family AF_UNSPEC
establish control: are you sure there is a netserver listening on 127.0.0.1 at port 12865?
establish_control could not establish the control connection from 0.0.0.0 port 0 address family AF_UNSPEC to 127.0.0.1 port 12865 address family AF_INET
====== netperf UDP_STREAM end: fail ======
====== netperf TCP_STREAM begin ======
MIGRATED TCP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET
Recv   Send    Send                          
Socket Socket  Message  Elapsed              
Size   Size    Size     Time     Throughput  
bytes  bytes   bytes    secs.    10^6bits/sec  

 65536  65536   1000    0.08       98.79   
====== netperf TCP_STREAM end: success ======
====== netperf UDP_RR begin ======
MIGRATED UDP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00     1113.52   
65536  65536 
====== netperf UDP_RR end: success ======
====== netperf TCP_RR begin ======
MIGRATED TCP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00     1165.72   
65536  65536 
====== netperf TCP_RR end: success ======
====== netperf TCP_CRR begin ======
MIGRATED TCP Connect/Request/Response TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00      819.59   
65536  65536 
====== netperf TCP_CRR end: success ======
#### OS COMP TEST GROUP END netperf-glibc ####
#### OS COMP TEST GROUP START cyclictest-musl ####
====== cyclictest NO_STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 4469) P:99 I:1000 C:    957 Min:      2 Act: 4438 Avg:   78 Max:    7289
====== cyclictest NO_STRESS_P1 end: success ======
====== cyclictest NO_STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 4471) P:99 I:1000 C:    945 Min:      2 Act:   19 Avg:   84 Max:   12792
T: 1 ( 4472) P:99 I:1500 C:    637 Min:      2 Act:  120 Avg:  110 Max:   12839
T: 2 ( 4473) P:99 I:2000 C:    483 Min:      2 Act:  635 Avg:  135 Max:   11323
T: 3 ( 4474) P:99 I:2500 C:    389 Min:      2 Act:  631 Avg:  158 Max:   12274
T: 4 ( 4475) P:99 I:3000 C:    325 Min:      2 Act:   81 Avg:  176 Max:   11224
T: 5 ( 4476) P:99 I:3500 C:    278 Min:      2 Act:  134 Avg:  178 Max:   11674
T: 6 ( 4477) P:99 I:4000 C:    248 Min:      2 Act:  490 Avg:  168 Max:   11125
T: 7 ( 4478) P:99 I:4500 C:    219 Min:      2 Act:   36 Avg:  169 Max:   11074
====== cyclictest NO_STRESS_P8 end: success ======
====== start hackbench ======
Running in process mode with 10 groups using 40 file descriptors each (== 400 tasks)
Each sender will pass 100000000 messages of 100 bytes
====== cyclictest STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 4882) P:99 I:1000 C:      4 Min: 233081 Act:323198 Avg:290846 Max:  323198
====== cyclictest STRESS_P1 end: success ======
====== cyclictest STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 4884) P:99 I:1000 C:      4 Min: 269273 Act:330983 Avg:299553 Max:  330983
T: 1 ( 4885) P:99 I:1500 C:      4 Min: 268920 Act:331682 Avg:299296 Max:  331682
T: 2 ( 4886) P:99 I:2000 C:      4 Min: 268372 Act:331703 Avg:299134 Max:  331703
T: 3 ( 4887) P:99 I:2500 C:      4 Min: 267809 Act:331687 Avg:298578 Max:  331687
T: 4 ( 4888) P:99 I:3000 C:      4 Min: 267246 Act:331688 Avg:298027 Max:  331688
T: 5 ( 4889) P:99 I:3500 C:      4 Min: 266679 Act:330182 Avg:297971 Max:  330182
T: 6 ( 4890) P:99 I:4000 C:      4 Min: 266143 Act:329681 Avg:297427 Max:  329681
T: 7 ( 4891) P:99 I:4500 C:      4 Min: 265585 Act:328680 Avg:296380 Max:  328680
autorun: /tmp/testsuite/musl/cyclictest/cyclictest_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START cyclictest-glibc ####
====== cyclictest NO_STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 4897) P:99 I:1000 C:    887 Min:      2 Act:   13 Avg:  160 Max:   23991
====== cyclictest NO_STRESS_P1 end: success ======
====== cyclictest NO_STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 4899) P:99 I:1000 C:    959 Min:      2 Act:   13 Avg:   69 Max:    6053
T: 1 ( 4900) P:99 I:1500 C:    647 Min:      2 Act:   50 Avg:   86 Max:    5660
T: 2 ( 4901) P:99 I:2000 C:    487 Min:      2 Act:  525 Avg:   99 Max:    6563
T: 3 ( 4902) P:99 I:2500 C:    392 Min:      2 Act:  428 Avg:  121 Max:    5705
T: 4 ( 4903) P:99 I:3000 C:    332 Min:      2 Act:   72 Avg:  100 Max:    5134
T: 5 ( 4904) P:99 I:3500 C:    283 Min:      2 Act:  121 Avg:  133 Max:    5045
T: 6 ( 4905) P:99 I:4000 C:    249 Min:      2 Act:  155 Avg:  146 Max:    5961
T: 7 ( 4906) P:99 I:4500 C:    221 Min:      2 Act: 1112 Avg:  107 Max:    4876
====== cyclictest NO_STRESS_P8 end: success ======
====== start hackbench ======
Running in process mode with 10 groups using 40 file descriptors each (== 400 tasks)
Each sender will pass 100000000 messages of 100 bytes
====== cyclictest STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 5310) P:99 I:1000 C:      4 Min: 246304 Act:273476 Avg:304873 Max:  356659
====== cyclictest STRESS_P1 end: success ======
====== cyclictest STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 5312) P:99 I:1000 C:      5 Min:    404 Act:384532 Avg:239170 Max:  384532
T: 1 ( 5313) P:99 I:1500 C:      4 Min: 248764 Act:385899 Avg:299181 Max:  385899
T: 2 ( 5314) P:99 I:2000 C:      4 Min: 248241 Act:384461 Avg:298555 Max:  384461
T: 3 ( 5315) P:99 I:2500 C:      4 Min: 247726 Act:385507 Avg:298305 Max:  385507
T: 4 ( 5316) P:99 I:3000 C:      4 Min: 247211 Act:384552 Avg:298054 Max:  384552
T: 5 ( 5317) P:99 I:3500 C:      4 Min: 246694 Act:385084 Avg:297672 Max:  385084
T: 6 ( 5318) P:99 I:4000 C:      4 Min: 246178 Act:384612 Avg:297539 Max:  384612
T: 7 ( 5319) P:99 I:4500 C:      4 Min: 245634 Act:386118 Avg:298381 Max:  386118
autorun: /tmp/testsuite/glibc/cyclictest/cyclictest_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START iozone-musl ####
iozone automatic measurements
	Iozone: Performance Test of File I/O
	        Version $Revision: 3.506 $
		Compiled for 64 bit mode.
		Build: linux 

	Contributors:William Norcott, Don Capps, Isom Crawford, Kirby Collins
	             Al Slater, Scott Rhine, Mike Wisner, Ken Goss
	             Steve Landherr, Brad Smith, Mark Kelly, Dr. Alain CYR,
	             Randy Dunlap, Mark Montague, Dan Million, Gavin Brebner,
	             Jean-Marc Zucconi, Jeff Blomberg, Benny Halevy, Dave Boone,
	             Erik Habbinga, Kris Strecker, Walter Wong, Joshua Root,
	             Fabrice Bacchella, Zhenghua Xue, Qin Li, Darren Sawyer,
	             Vangel Bojaxhi, Ben England, Vikentsi Lapa,
	             Alexey Skidanov, Sudhir Kumar.

	Run began: Thu Jan  1 00:21:30 1970

	Auto Mode
	Record Size 1 kB
	File size set to 4096 kB
	Command line used: ./iozone -a -r 1k -s 4m
	Output is in kBytes/sec
	Time Resolution = 0.000004 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
                                                                    random    random      bkwd     record     stride                                        
              kB  reclen    write    rewrite      read    reread      read     write      read    rewrite       read    fwrite  frewrite     fread   freread
            4096       1     44559     46155     45727     47515     48148     86938[37m[1292.093595 0:5325 axfs::fops:269] [33m[AxError::InvalidInput][m
[m[37m[1292.164100 0:5325 axfs::fops:269] [33m[AxError::InvalidInput][m
[m     80561      59764      37189     47552     81083     49376     43262

iozone test complete.
iozone throughput write/read measurements
	Iozone: Performance Test of File I/O
	        Version $Revision: 3.506 $
		Compiled for 64 bit mode.
		Build: linux 

	Contributors:William Norcott, Don Capps, Isom Crawford, Kirby Collins
	             Al Slater, Scott Rhine, Mike Wisner, Ken Goss
	             Steve Landherr, Brad Smith, Mark Kelly, Dr. Alain CYR,
	             Randy Dunlap, Mark Montague, Dan Million, Gavin Brebner,
	             Jean-Marc Zucconi, Jeff Blomberg, Benny Halevy, Dave Boone,
	             Erik Habbinga, Kris Strecker, Walter Wong, Joshua Root,
	             Fabrice Bacchella, Zhenghua Xue, Qin Li, Darren Sawyer,
	             Vangel Bojaxhi, Ben England, Vikentsi Lapa,
	             Alexey Skidanov, Sudhir Kumar.

	Run began: Thu Jan  1 00:21:33 1970

	Record Size 1 kB
	File size set to 1024 kB
	Command line used: ./iozone -t 4 -i 0 -i 1 -r 1k -s 1m
	Output is in kBytes/sec
	Time Resolution = 0.000008 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
	Throughput test with 4 processes
	Each process writes a 1024 kByte file in 1 kByte records

	Children see throughput for  4 initial writers 	=   60228.21 kB/sec
	Parent sees throughput for  4 initial writers 	=    2057.07 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   60228.21 kB/sec
	Avg throughput per process 			=   15057.05 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=  122400.20 kB/sec
	Parent sees throughput for  4 rewriters 	=    2104.93 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  122400.20 kB/sec
	Avg throughput per process 			=   30600.05 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 readers 		=  104012.20 kB/sec
	Parent sees throughput for  4 readers 		=    2278.59 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  104012.20 kB/sec
	Avg throughput per process 			=   26003.05 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 re-readers 	=   61813.35 kB/sec
	Parent sees throughput for 4 re-readers 	=    2319.76 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   61813.35 kB/sec
	Avg throughput per process 			=   15453.34 kB/sec
	Min xfer 					=       0.00 kB



iozone test complete.
iozone throughput random-read measurements
	Iozone: Performance Test of File I/O
	        Version $Revision: 3.506 $
		Compiled for 64 bit mode.
		Build: linux 

	Contributors:William Norcott, Don Capps, Isom Crawford, Kirby Collins
	             Al Slater, Scott Rhine, Mike Wisner, Ken Goss
	             Steve Landherr, Brad Smith, Mark Kelly, Dr. Alain CYR,
	             Randy Dunlap, Mark Montague, Dan Million, Gavin Brebner,
	             Jean-Marc Zucconi, Jeff Blomberg, Benny Halevy, Dave Boone,
	             Erik Habbinga, Kris Strecker, Walter Wong, Joshua Root,
	             Fabrice Bacchella, Zhenghua Xue, Qin Li, Darren Sawyer,
	             Vangel Bojaxhi, Ben England, Vikentsi Lapa,
	             Alexey Skidanov, Sudhir Kumar.

	Run began: Thu Jan  1 00:21:52 1970

	Record Size 1 kB
	File size set to 1024 kB
	Command line used: ./iozone -t 4 -i 0 -i 2 -r 1k -s 1m
	Output is in kBytes/sec
	Time Resolution = 0.000007 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
	Throughput test with 4 processes
	Each process writes a 1024 kByte file in 1 kByte records

	Children see throughput for  4 initial writers 	=   62049.32 kB/sec
	Parent sees throughput for  4 initial writers 	=    1966.01 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   62049.32 kB/sec
	Avg throughput per process 			=   15512.33 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=  121759.80 kB/sec
	Parent sees throughput for  4 rewriters 	=    1882.31 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  121759.80 kB/sec
	Avg throughput per process 			=   30439.95 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 random readers 	=   51292.32 kB/sec
	Parent sees throughput for 4 random readers 	=    2216.96 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   51292.32 kB/sec
	Avg throughput per process 			=   12823.08 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 random writers 	=   90933.30 kB/sec
	Parent sees throughput for 4 random writers 	=    2107.59 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   90933.30 kB/sec
	Avg throughput per process 			=   22733.33 kB/sec
	Min xfer 					=       0.00 kB



iozone test complete.
iozone throughput read-backwards measurements
	Iozone: Performance Test of File I/O
	        Version $Revision: 3.506 $
		Compiled for 64 bit mode.
		Build: linux 

	Contributors:William Norcott, Don Capps, Isom Crawford, Kirby Collins
	             Al Slater, Scott Rhine, Mike Wisner, Ken Goss
	             Steve Landherr, Brad Smith, Mark Kelly, Dr. Alain CYR,
	             Randy Dunlap, Mark Montague, Dan Million, Gavin Brebner,
	             Jean-Marc Zucconi, Jeff Blomberg, Benny Halevy, Dave Boone,
	             Erik Habbinga, Kris Strecker, Walter Wong, Joshua Root,
	             Fabrice Bacchella, Zhenghua Xue, Qin Li, Darren Sawyer,
	             Vangel Bojaxhi, Ben England, Vikentsi Lapa,
	             Alexey Skidanov, Sudhir Kumar.

	Run began: Thu Jan  1 00:22:15 1970

	Record Size 1 kB
	File size set to 1024 kB
	Command line used: ./iozone -t 4 -i 0 -i 3 -r 1k -s 1m
	Output is in kBytes/sec
	Time Resolution = 0.000008 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
	Throughput test with 4 processes
	Each process writes a 1024 kByte file in 1 kByte records

	Children see throughput for  4 initial writers 	=  107371.29 kB/sec
	Parent sees throughput for  4 initial writers 	=    1891.18 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  107371.29 kB/sec
	Avg throughput per process 			=   26842.82 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=   99630.28 kB/sec
	Parent sees throughput for  4 rewriters 	=    2631.25 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   99630.28 kB/sec
	Avg throughput per process 			=   24907.57 kB/sec
	Min xfer 					=       0.00 kB
[37m[1344.192523 0:5380 axfs::fops:269] [33m[AxError::InvalidInput][m
[m
	Children see throughput for 4 reverse readers 	=   43822.48 kB/sec
	Parent sees throughput for 4 reverse readers 	=    1935.58 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   43822.48 kB/sec
	Avg throughput per process 			=   10955.62 kB/sec
	Min xfer 					=       0.00 kB
autorun: /tmp/testsuite/musl/iozone/iozone_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START iozone-glibc ####
iozone automatic measurements
	Iozone: Performance Test of File I/O
	        Version $Revision: 3.506 $
		Compiled for 64 bit mode.
		Build: linux 

	Contributors:William Norcott, Don Capps, Isom Crawford, Kirby Collins
	             Al Slater, Scott Rhine, Mike Wisner, Ken Goss
	             Steve Landherr, Brad Smith, Mark Kelly, Dr. Alain CYR,
	             Randy Dunlap, Mark Montague, Dan Million, Gavin Brebner,
	             Jean-Marc Zucconi, Jeff Blomberg, Benny Halevy, Dave Boone,
	             Erik Habbinga, Kris Strecker, Walter Wong, Joshua Root,
	             Fabrice Bacchella, Zhenghua Xue, Qin Li, Darren Sawyer,
	             Vangel Bojaxhi, Ben England, Vikentsi Lapa,
	             Alexey Skidanov, Sudhir Kumar.

	Run began: Thu Jan  1 00:22:41 1970

	Auto Mode
	Record Size 1 kB
	File size set to 4096 kB
	Command line used: ./iozone -a -r 1k -s 4m
	Output is in kBytes/sec
	Time Resolution = 0.000004 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
                                                                    random    random      bkwd     record     stride                                        
              kB  reclen    write    rewrite      read    reread      read     write      read    rewrite       read    fwrite  frewrite     fread   freread
            4096       1     63395    120406     57892     56638     37632     63523[37m[1364.018168 0:5389 axfs::fops:269] [33m[AxError::InvalidInput][m
[m[37m[1364.070370 0:5389 axfs::fops:269] [33m[AxError::InvalidInput][m
[m     77370     101789      46492     69396     47205     50562     98338

iozone test complete.
iozone throughput write/read measurements
	Iozone: Performance Test of File I/O
	        Version $Revision: 3.506 $
		Compiled for 64 bit mode.
		Build: linux 

	Contributors:William Norcott, Don Capps, Isom Crawford, Kirby Collins
	             Al Slater, Scott Rhine, Mike Wisner, Ken Goss
	             Steve Landherr, Brad Smith, Mark Kelly, Dr. Alain CYR,
	             Randy Dunlap, Mark Montague, Dan Million, Gavin Brebner,
	             Jean-Marc Zucconi, Jeff Blomberg, Benny Halevy, Dave Boone,
	             Erik Habbinga, Kris Strecker, Walter Wong, Joshua Root,
	             Fabrice Bacchella, Zhenghua Xue, Qin Li, Darren Sawyer,
	             Vangel Bojaxhi, Ben England, Vikentsi Lapa,
	             Alexey Skidanov, Sudhir Kumar.

	Run began: Thu Jan  1 00:22:47 1970

	Record Size 1 kB
	File size set to 1024 kB
	Command line used: ./iozone -t 4 -i 0 -i 1 -r 1k -s 1m
	Output is in kBytes/sec
	Time Resolution = 0.000004 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
	Throughput test with 4 processes
	Each process writes a 1024 kByte file in 1 kByte records

	Children see throughput for  4 initial writers 	=   62351.58 kB/sec
	Parent sees throughput for  4 initial writers 	=    1764.28 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   62351.58 kB/sec
	Avg throughput per process 			=   15587.89 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=  103612.27 kB/sec
	Parent sees throughput for  4 rewriters 	=    2006.33 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  103612.27 kB/sec
	Avg throughput per process 			=   25903.07 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 readers 		=   89604.48 kB/sec
	Parent sees throughput for  4 readers 		=    2102.79 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   89604.48 kB/sec
	Avg throughput per process 			=   22401.12 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 re-readers 	=  102955.96 kB/sec
	Parent sees throughput for 4 re-readers 	=    2053.43 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  102955.96 kB/sec
	Avg throughput per process 			=   25738.99 kB/sec
	Min xfer 					=       0.00 kB



iozone test complete.
iozone throughput random-read measurements
	Iozone: Performance Test of File I/O
	        Version $Revision: 3.506 $
		Compiled for 64 bit mode.
		Build: linux 

	Contributors:William Norcott, Don Capps, Isom Crawford, Kirby Collins
	             Al Slater, Scott Rhine, Mike Wisner, Ken Goss
	             Steve Landherr, Brad Smith, Mark Kelly, Dr. Alain CYR,
	             Randy Dunlap, Mark Montague, Dan Million, Gavin Brebner,
	             Jean-Marc Zucconi, Jeff Blomberg, Benny Halevy, Dave Boone,
	             Erik Habbinga, Kris Strecker, Walter Wong, Joshua Root,
	             Fabrice Bacchella, Zhenghua Xue, Qin Li, Darren Sawyer,
	             Vangel Bojaxhi, Ben England, Vikentsi Lapa,
	             Alexey Skidanov, Sudhir Kumar.

	Run began: Thu Jan  1 00:23:07 1970

	Record Size 1 kB
	File size set to 1024 kB
	Command line used: ./iozone -t 4 -i 0 -i 2 -r 1k -s 1m
	Output is in kBytes/sec
	Time Resolution = 0.000004 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
	Throughput test with 4 processes
	Each process writes a 1024 kByte file in 1 kByte records

	Children see throughput for  4 initial writers 	=   94169.58 kB/sec
	Parent sees throughput for  4 initial writers 	=    1960.37 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   94169.58 kB/sec
	Avg throughput per process 			=   23542.39 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=  100127.11 kB/sec
	Parent sees throughput for  4 rewriters 	=    1927.49 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  100127.11 kB/sec
	Avg throughput per process 			=   25031.78 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 random readers 	=   56999.72 kB/sec
	Parent sees throughput for 4 random readers 	=    2068.21 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   56999.72 kB/sec
	Avg throughput per process 			=   14249.93 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 random writers 	=   55270.69 kB/sec
	Parent sees throughput for 4 random writers 	=    1974.49 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   55270.69 kB/sec
	Avg throughput per process 			=   13817.67 kB/sec
	Min xfer 					=       0.00 kB



iozone test complete.
iozone throughput read-backwards measurements
	Iozone: Performance Test of File I/O
	        Version $Revision: 3.506 $
		Compiled for 64 bit mode.
		Build: linux 

	Contributors:William Norcott, Don Capps, Isom Crawford, Kirby Collins
	             Al Slater, Scott Rhine, Mike Wisner, Ken Goss
	             Steve Landherr, Brad Smith, Mark Kelly, Dr. Alain CYR,
	             Randy Dunlap, Mark Montague, Dan Million, Gavin Brebner,
	             Jean-Marc Zucconi, Jeff Blomberg, Benny Halevy, Dave Boone,
	             Erik Habbinga, Kris Strecker, Walter Wong, Joshua Root,
	             Fabrice Bacchella, Zhenghua Xue, Qin Li, Darren Sawyer,
	             Vangel Bojaxhi, Ben England, Vikentsi Lapa,
	             Alexey Skidanov, Sudhir Kumar.

	Run began: Thu Jan  1 00:23:32 1970

	Record Size 1 kB
	File size set to 1024 kB
	Command line used: ./iozone -t 4 -i 0 -i 3 -r 1k -s 1m
	Output is in kBytes/sec
	Time Resolution = 0.000007 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
	Throughput test with 4 processes
	Each process writes a 1024 kByte file in 1 kByte records

	Children see throughput for  4 initial writers 	=   60495.07 kB/sec
	Parent sees throughput for  4 initial writers 	=    1680.00 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   60495.07 kB/sec
	Avg throughput per process 			=   15123.77 kB/sec
	Min xfer 					=       0.00 kB
autorun: /tmp/testsuite/glibc/iozone/iozone_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START unixbench-musl ####
Unixbench DHRY2 test(lps): 13753374
Unixbench WHETSTONE test(MFLOPS): 142.045
Unixbench SYSCALL test(lps): 465180
Unixbench CONTEXT test(lps): 10614
autorun: /tmp/testsuite/musl/unixbench/unixbench_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START unixbench-glibc ####
Unixbench DHRY2 test(lps): 21810716
Unixbench WHETSTONE test(MFLOPS): 163.917
Unixbench SYSCALL test(lps): 532683
autorun: /tmp/testsuite/glibc/unixbench/unixbench_testcode.sh timed out after 60s
[37m[1547.070013 0:2 axplat_loongarch64_qemu_virt::power:23] [32mShutting down...[m
[m