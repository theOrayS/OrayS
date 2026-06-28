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
    Finished `release` profile [optimized] target(s) in 24.06s
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

[37m[  0.004624 0 axruntime:135] [32mLogging is enabled.[m
[m[37m[  0.006216 0 axruntime:136] [32mPrimary CPU 0 started, arg = 0x0.[m
[m[37m[  0.007500 0 axruntime:139] [32mFound physcial memory regions:[m
[m[37m[  0.007988 0 axruntime:141] [32m  [PA:0x100d0000, PA:0x100d1000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.008818 0 axruntime:141] [32m  [PA:0x100e0000, PA:0x100e1000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.009330 0 axruntime:141] [32m  [PA:0x1fe00000, PA:0x1fe01000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.009793 0 axruntime:141] [32m  [PA:0x20000000, PA:0x30000000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.010254 0 axruntime:141] [32m  [PA:0x40000000, PA:0x40020000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.010719 0 axruntime:141] [32m  [PA:0x80000000, PA:0x80120000) .text (READ | EXECUTE | RESERVED)[m
[m[37m[  0.011112 0 axruntime:141] [32m  [PA:0x80120000, PA:0x8014b000) .rodata (READ | RESERVED)[m
[m[37m[  0.011535 0 axruntime:141] [32m  [PA:0x8014b000, PA:0x80150000) .data .tdata .tbss .percpu (READ | WRITE | RESERVED)[m
[m[37m[  0.012017 0 axruntime:141] [32m  [PA:0x80150000, PA:0x80190000) boot stack (READ | WRITE | RESERVED)[m
[m[37m[  0.012429 0 axruntime:141] [32m  [PA:0x80190000, PA:0x801b7000) .bss (READ | WRITE | RESERVED)[m
[m[37m[  0.012850 0 axruntime:141] [32m  [PA:0x801b7000, PA:0xb0000000) free memory (READ | WRITE | FREE)[m
[m[37m[  0.013376 0 axruntime:216] [32mInitialize global memory allocator...[m
[m[37m[  0.013834 0 axruntime:217] [32m  use TLSF allocator.[m
[m[37m[  0.016589 0 axmm:103] [32mInitialize virtual memory management...[m
[m[37m[  0.050350 0 axruntime:156] [32mInitialize platform devices...[m
[msmp = 1
[37m[  0.050916 0 axtask::api:73] [32mInitialize scheduling...[m
[m[37m[  0.053430 0 axtask::api:83] [32m  use Round-robin scheduler.[m
[m[37m[  0.054006 0 axdriver:152] [32mInitialize device drivers...[m
[m[37m[  0.054426 0 axdriver:153] [32m  device model: static[m
[m[37m[  0.061429 0 virtio_drivers::device::blk:63] [32mfound a block device of size 4194304KB[m
[m[37m[  0.063099 0 axdriver::bus::pci:107] [32mregistered a new Block device at 00:01.0: "virtio-blk"[m
[m[37m[  0.072113 0 virtio_drivers::device::net::dev_raw:33] [32mnegotiated_features Features(MAC | STATUS | RING_INDIRECT_DESC | RING_EVENT_IDX | VERSION_1)[m
[m[37m[  0.091475 0 axdriver::bus::pci:107] [32mregistered a new Net device at 00:02.0: "virtio-net"[m
[m[37m[  0.137390 0 axfs:44] [32mInitialize filesystems...[m
[m[37m[  0.137869 0 axfs:47] [32m  use block device 0: "virtio-blk"[m
[m[37m[  0.139457 0 axfs::root:336] [32m  detected root filesystem: Ext4[m
[m[37m[  0.161985 0 axnet:42] [32mInitialize network subsystem...[m
[m[37m[  0.162474 0 axnet:45] [32m  use NIC 0: "virtio-net"[m
[m[37m[  0.165900 0 axnet::smoltcp_impl:335] [32mcreated net interface "eth0":[m
[m[37m[  0.166505 0 axnet::smoltcp_impl:336] [32m  ether:    52-54-00-12-34-56[m
[m[37m[  0.167136 0 axnet::smoltcp_impl:337] [32m  ip:       10.0.2.15/24[m
[m[37m[  0.167763 0 axnet::smoltcp_impl:338] [32m  gateway:  10.0.2.2[m
[m[37m[  0.168241 0 axruntime:182] [32mInitialize interrupt handlers...[m
[m[37m[  0.169653 0 axruntime:194] [32mPrimary CPU 0 init OK.[m
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
Pass!
========== END entry-static.exe qsort ==========
========== START entry-static.exe random ==========
Pass!
========== END entry-static.exe random ==========
========== START entry-static.exe search_hsearch ==========
Pass!
========== END entry-static.exe search_hsearch ==========
========== START entry-static.exe search_insque ==========
Pass!
========== END entry-static.exe search_insque ==========
========== START entry-static.exe search_lsearch ==========
Pass!
========== END entry-static.exe search_lsearch ==========
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
Pass!
========== END entry-static.exe search_hsearch ==========
========== START entry-static.exe search_insque ==========
Pass!
========== END entry-static.exe search_insque ==========
========== START entry-static.exe search_lsearch ==========
Pass!
========== END entry-static.exe search_lsearch ==========
========== START entry-static.exe search_tsearch ==========
Pass!
========== END entry-static.exe search_tsearch ==========
========== START entry-static.exe setjmp ==========
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
pid:382
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
pid = 394
========== END test_getpid ==========
Testing getppid :
========== START test_getppid ==========
  getppid success. ppid : 378
========== END test_getppid ==========
Testing gettimeofday :
========== START test_gettimeofday ==========
gettimeofday success.
start:258632, end:258796
interval: 164
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
cpid: 404
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
  I am child process: 417. iteration 0.
  I am child process: 418. iteration 1.
  I am child process: 417. iteration 0.
  I am child process: 418. iteration 1.
  I am child process: 419. iteration 2.
  I am child process: 417. iteration 0.
  I am child process: 418. iteration 1.
  I am child process: 419. iteration 2.
  I am child process: 417. iteration 0.
  I am child process: 418. iteration 1.
  I am child process: 419. iteration 2.
  I am child process: 417. iteration 0.
  I am child process: 418. iteration 1.
  I am child process: 419. iteration 2.
  I am child process: 419. iteration 2.
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
pid:429
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
pid = 441
========== END test_getpid ==========
Testing getppid :
========== START test_getppid ==========
  getppid success. ppid : 425
========== END test_getppid ==========
Testing gettimeofday :
========== START test_gettimeofday ==========
gettimeofday success.
start:283329, end:283414
interval: 85
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
cpid: 451
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
  I am child process: 464. iteration 0.
  I am child process: 465. iteration 1.
  I am child process: 466. iteration 2.
  I am child process: 464. iteration 0.
  I am child process: 465. iteration 1.
  I am child process: 466. iteration 2.
  I am child process: 464. iteration 0.
  I am child process: 465. iteration 1.
  I am child process: 466. iteration 2.
  I am child process: 464. iteration 0.
  I am child process: 465. iteration 1.
  I am child process: 466. iteration 2.
  I am child process: 464. iteration 0.
  I am child process: 465. iteration 1.
  I am child process: 466. iteration 2.
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
Thu Jan  1 00:05:03 UTC 1970
testcase busybox date success
Filesystem           1K-blocks      Used Available Use% Mounted on
devfs                   784676     83548    701128  11% /dev
tmpfs                   784676     83548    701128  11% /tmp
tmpfs                   784676     83548    701128  11% /var
proc                    784676     83548    701128  11% /proc
sysfs                   784676     83548    701128  11% /sys
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
 00:05:20 up 0 min,  0 users,  load average: 0.00, 0.00, 0.00
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
Thu Jan  1 00:05:29 1970  0.000000 seconds
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
Thu Jan  1 00:06:46 UTC 1970
testcase busybox date success
Filesystem           1K-blocks      Used Available Use% Mounted on
devfs                   784676    156828    627848  20% /dev
tmpfs                   784676    156828    627848  20% /tmp
tmpfs                   784676    156828    627848  20% /var
proc                    784676    156828    627848  20% /proc
sysfs                   784676    156828    627848  20% /sys
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
 00:07:11 up 0 min,  0 users,  load average: 0.00, 0.00, 0.00
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
Thu Jan  1 00:07:24 1970  0.000000 seconds
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
ltp case list: stable (75 cases, timeout 15s)
========== START ltp access01 ==========
RUN LTP CASE access01
LTP MEMORY access01 before: free_frames=157255 allocated_frames=38914
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
[37m[563.884339 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.885953 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.886945 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.888201 0:849 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[563.889618 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.890313 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.890936 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.891675 0:849 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[563.892575 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.893199 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.893806 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.894519 0:849 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[563.895384 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.896043 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.897991 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.898685 0:849 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[563.899570 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.900174 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.900741 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.901391 0:849 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[563.902291 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.902871 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.903442 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.904092 0:849 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[563.904639 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.905204 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.905754 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.906333 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[563.907154 0:849 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE access01 : 0
Pass!
LTP MEMORY access01 after_run: free_frames=156431 allocated_frames=39738
LTP MEMORY access01 after_cleanup: free_frames=156431 allocated_frames=39738
LTP CASE RUNTIME access01: 4862 ms
========== END ltp access01 ==========
========== START ltp brk01 ==========
RUN LTP CASE brk01
LTP MEMORY brk01 before: free_frames=156431 allocated_frames=39738
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
[37m[565.090631 0:954 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE brk01 : 0
Pass!
LTP MEMORY brk01 after_run: free_frames=156407 allocated_frames=39762
LTP MEMORY brk01 after_cleanup: free_frames=156407 allocated_frames=39762
LTP CASE RUNTIME brk01: 1181 ms
========== END ltp brk01 ==========
========== START ltp chdir01 ==========
RUN LTP CASE chdir01
LTP MEMORY chdir01 before: free_frames=156407 allocated_frames=39762
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
tst_test.c:1120: [1;34mTINFO: [0mMounting ltp-tmpfs to /tmp/ltp-work/LTP_chdpMLFLE/mntpoint fstyp=tmpfs flags=0
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
[37m[566.186224 0:961 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[566.188304 0:961 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[566.189242 0:961 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[566.190392 0:961 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[566.191666 0:961 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE chdir01 : 0
Pass!
LTP MEMORY chdir01 after_run: free_frames=156391 allocated_frames=39778
LTP MEMORY chdir01 after_cleanup: free_frames=156391 allocated_frames=39778
LTP CASE RUNTIME chdir01: 1107 ms
========== END ltp chdir01 ==========
========== START ltp clone01 ==========
RUN LTP CASE clone01
LTP MEMORY clone01 before: free_frames=156391 allocated_frames=39778
tst_buffers.c:57: [1;34mTINFO: [0mTest is using guarded buffers
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
clone01.c:37: [1;32mTPASS: [0mclone returned 968
clone01.c:43: [1;32mTPASS: [0mChild exited with 0

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[567.431222 0:965 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE clone01 : 0
Pass!
LTP MEMORY clone01 after_run: free_frames=156367 allocated_frames=39802
LTP MEMORY clone01 after_cleanup: free_frames=156367 allocated_frames=39802
LTP CASE RUNTIME clone01: 1260 ms
========== END ltp clone01 ==========
========== START ltp close01 ==========
RUN LTP CASE close01
LTP MEMORY close01 before: free_frames=156367 allocated_frames=39802
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
[37m[568.777863 0:970 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[568.793097 0:970 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE close01 : 0
Pass!
LTP MEMORY close01 after_run: free_frames=156351 allocated_frames=39818
LTP MEMORY close01 after_cleanup: free_frames=156351 allocated_frames=39818
LTP CASE RUNTIME close01: 1348 ms
========== END ltp close01 ==========
========== START ltp dup01 ==========
RUN LTP CASE dup01
LTP MEMORY dup01 before: free_frames=156351 allocated_frames=39818
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
dup01.c:24: [1;32mTPASS: [0mdup(fd) returned fd 4
dup01.c:27: [1;32mTPASS: [0mbuf1.st_ino == buf2.st_ino (10459834440142341613)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[569.961290 0:974 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[569.962439 0:974 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE dup01 : 0
Pass!
LTP MEMORY dup01 after_run: free_frames=156335 allocated_frames=39834
LTP MEMORY dup01 after_cleanup: free_frames=156335 allocated_frames=39834
LTP CASE RUNTIME dup01: 1149 ms
========== END ltp dup01 ==========
========== START ltp fcntl01 ==========
RUN LTP CASE fcntl01
LTP MEMORY fcntl01 before: free_frames=156335 allocated_frames=39834
[37m[571.200196 0:978 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fcntl01 : 0
Pass!
LTP MEMORY fcntl01 after_run: free_frames=156327 allocated_frames=39842
LTP MEMORY fcntl01 after_cleanup: free_frames=156327 allocated_frames=39842
LTP CASE RUNTIME fcntl01: 1240 ms
========== END ltp fcntl01 ==========
========== START ltp fcntl02 ==========
RUN LTP CASE fcntl02
LTP MEMORY fcntl02 before: free_frames=156327 allocated_frames=39842
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_981, F_DUPFD, 0) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_981, F_DUPFD, 1) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_981, F_DUPFD, 2) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_981, F_DUPFD, 3) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_981, F_DUPFD, 10) returned 10
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_981, F_DUPFD, 100) returned 100

Summary:
passed   6
failed   0
broken   0
skipped  0
warnings 0
[37m[572.506894 0:979 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[572.525692 0:979 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fcntl02 : 0
Pass!
LTP MEMORY fcntl02 after_run: free_frames=156311 allocated_frames=39858
LTP MEMORY fcntl02 after_cleanup: free_frames=156311 allocated_frames=39858
LTP CASE RUNTIME fcntl02: 1363 ms
========== END ltp fcntl02 ==========
========== START ltp fork01 ==========
RUN LTP CASE fork01
LTP MEMORY fork01 before: free_frames=156311 allocated_frames=39858
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
fork01.c:47: [1;32mTPASS: [0mcorrect child status returned 42
fork01.c:50: [1;32mTPASS: [0mchild_pid == pid (986)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[573.737610 0:983 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[573.741284 0:983 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fork01 : 0
Pass!
LTP MEMORY fork01 after_run: free_frames=156287 allocated_frames=39882
LTP MEMORY fork01 after_cleanup: free_frames=156287 allocated_frames=39882
LTP CASE RUNTIME fork01: 1182 ms
========== END ltp fork01 ==========
========== START ltp getpid01 ==========
RUN LTP CASE getpid01
LTP MEMORY getpid01 before: free_frames=156287 allocated_frames=39882
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
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
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1034
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1035
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1036
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1037
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1038
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1039
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1040
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1041
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1042
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1043
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1044
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1045
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1046
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1047
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1048
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1049
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1050
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1051
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1052
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1053
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1054
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1055
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1056
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1057
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1058
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1059
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1060
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1061
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1062
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1063
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1064
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1065
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1066
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1067
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1068
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1069
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1070
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1071
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1072
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1073
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1074
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1075
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1076
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1077
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1078
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1079
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1080
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1081
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1082
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1083
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1084
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1085
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1086
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1087
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1088
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1089
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1090

Summary:
passed   100
failed   0
broken   0
skipped  0
warnings 0
[37m[577.562686 0:988 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpid01 : 0
Pass!
LTP MEMORY getpid01 after_run: free_frames=155471 allocated_frames=40698
LTP MEMORY getpid01 after_cleanup: free_frames=155471 allocated_frames=40698
LTP CASE RUNTIME getpid01: 3824 ms
========== END ltp getpid01 ==========
========== START ltp mmap01 ==========
RUN LTP CASE mmap01
LTP MEMORY mmap01 before: free_frames=155471 allocated_frames=40698
mmap01      1  [1;32mTPASS[0m  :  Functionality of mmap() successful
[37m[578.718252 0:1092 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[578.727237 0:1092 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE mmap01 : 0
Pass!
LTP MEMORY mmap01 after_run: free_frames=155455 allocated_frames=40714
LTP MEMORY mmap01 after_cleanup: free_frames=155455 allocated_frames=40714
LTP CASE RUNTIME mmap01: 1184 ms
========== END ltp mmap01 ==========
========== START ltp open01 ==========
RUN LTP CASE open01
LTP MEMORY open01 before: free_frames=155455 allocated_frames=40714
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
[37m[579.996266 0:1094 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[579.998240 0:1094 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE open01 : 0
Pass!
LTP MEMORY open01 after_run: free_frames=155439 allocated_frames=40730
LTP MEMORY open01 after_cleanup: free_frames=155439 allocated_frames=40730
LTP CASE RUNTIME open01: 1226 ms
========== END ltp open01 ==========
========== START ltp pipe01 ==========
RUN LTP CASE pipe01
LTP MEMORY pipe01 before: free_frames=155439 allocated_frames=40730
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
[37m[581.193932 0:1098 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE pipe01 : 0
Pass!
LTP MEMORY pipe01 after_run: free_frames=155423 allocated_frames=40746
LTP MEMORY pipe01 after_cleanup: free_frames=155423 allocated_frames=40746
LTP CASE RUNTIME pipe01: 1200 ms
========== END ltp pipe01 ==========
========== START ltp read01 ==========
RUN LTP CASE read01
LTP MEMORY read01 before: free_frames=155423 allocated_frames=40746
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
[37m[582.321101 0:1102 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[582.323355 0:1102 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE read01 : 0
Pass!
LTP MEMORY read01 after_run: free_frames=155407 allocated_frames=40762
LTP MEMORY read01 after_cleanup: free_frames=155407 allocated_frames=40762
LTP CASE RUNTIME read01: 1125 ms
========== END ltp read01 ==========
========== START ltp stat01 ==========
RUN LTP CASE stat01
LTP MEMORY stat01 before: free_frames=155407 allocated_frames=40762
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
[37m[583.879809 0:1106 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[583.880567 0:1106 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[583.881563 0:1106 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE stat01 : 0
Pass!
LTP MEMORY stat01 after_run: free_frames=155391 allocated_frames=40778
LTP MEMORY stat01 after_cleanup: free_frames=155391 allocated_frames=40778
LTP CASE RUNTIME stat01: 1548 ms
========== END ltp stat01 ==========
========== START ltp wait401 ==========
RUN LTP CASE wait401
LTP MEMORY wait401 before: free_frames=155391 allocated_frames=40778
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
wait401.c:40: [1;32mTPASS: [0mwait4() returned correct pid 1114
wait401.c:49: [1;32mTPASS: [0mWIFEXITED() is set in status
wait401.c:54: [1;32mTPASS: [0mWEXITSTATUS() == 0

Summary:
passed   3
failed   0
broken   0
skipped  0
warnings 0
[37m[585.070733 0:1111 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE wait401 : 0
Pass!
LTP MEMORY wait401 after_run: free_frames=155367 allocated_frames=40802
LTP MEMORY wait401 after_cleanup: free_frames=155367 allocated_frames=40802
LTP CASE RUNTIME wait401: 1189 ms
========== END ltp wait401 ==========
========== START ltp write01 ==========
RUN LTP CASE write01
LTP MEMORY write01 before: free_frames=155367 allocated_frames=40802
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
[37m[586.188015 0:1116 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[586.189491 0:1116 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE write01 : 0
Pass!
LTP MEMORY write01 after_run: free_frames=155351 allocated_frames=40818
LTP MEMORY write01 after_cleanup: free_frames=155351 allocated_frames=40818
LTP CASE RUNTIME write01: 1117 ms
========== END ltp write01 ==========
========== START ltp access03 ==========
RUN LTP CASE access03
LTP MEMORY access03 before: free_frames=155351 allocated_frames=40818
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
[37m[587.766502 0:1120 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE access03 : 0
Pass!
LTP MEMORY access03 after_run: free_frames=155303 allocated_frames=40866
LTP MEMORY access03 after_cleanup: free_frames=155303 allocated_frames=40866
LTP CASE RUNTIME access03: 1594 ms
========== END ltp access03 ==========
========== START ltp close02 ==========
RUN LTP CASE close02
LTP MEMORY close02 before: free_frames=155303 allocated_frames=40866
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
[37m[589.096583 0:1128 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE close02 : 0
Pass!
LTP MEMORY close02 after_run: free_frames=155287 allocated_frames=40882
LTP MEMORY close02 after_cleanup: free_frames=155287 allocated_frames=40882
LTP CASE RUNTIME close02: 1319 ms
========== END ltp close02 ==========
========== START ltp dup02 ==========
RUN LTP CASE dup02
LTP MEMORY dup02 before: free_frames=155287 allocated_frames=40882
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
[37m[590.223768 0:1132 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE dup02 : 0
Pass!
LTP MEMORY dup02 after_run: free_frames=155271 allocated_frames=40898
LTP MEMORY dup02 after_cleanup: free_frames=155271 allocated_frames=40898
LTP CASE RUNTIME dup02: 1109 ms
========== END ltp dup02 ==========
========== START ltp fcntl03 ==========
RUN LTP CASE fcntl03
LTP MEMORY fcntl03 before: free_frames=155271 allocated_frames=40898
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
fcntl03.c:32: [1;32mTPASS: [0mfcntl(fcntl03_1138, F_GETFD, 0) returned 0

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[591.818756 0:1136 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[591.820066 0:1136 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fcntl03 : 0
Pass!
LTP MEMORY fcntl03 after_run: free_frames=155255 allocated_frames=40914
LTP MEMORY fcntl03 after_cleanup: free_frames=155255 allocated_frames=40914
LTP CASE RUNTIME fcntl03: 1595 ms
========== END ltp fcntl03 ==========
========== START ltp getcwd01 ==========
RUN LTP CASE getcwd01
LTP MEMORY getcwd01 before: free_frames=155255 allocated_frames=40914
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
[37m[592.955089 0:1140 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getcwd01 : 0
Pass!
LTP MEMORY getcwd01 after_run: free_frames=155239 allocated_frames=40930
LTP MEMORY getcwd01 after_cleanup: free_frames=155239 allocated_frames=40930
LTP CASE RUNTIME getcwd01: 1132 ms
========== END ltp getcwd01 ==========
========== START ltp getpid02 ==========
RUN LTP CASE getpid02
LTP MEMORY getpid02 before: free_frames=155239 allocated_frames=40930
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getpid02.c:37: [1;32mTPASS: [0mchild getppid() == parent getpid() (1146)
getpid02.c:50: [1;32mTPASS: [0mchild getpid() == parent fork() (1147)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[594.211358 0:1144 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpid02 : 0
Pass!
LTP MEMORY getpid02 after_run: free_frames=155215 allocated_frames=40954
LTP MEMORY getpid02 after_cleanup: free_frames=155215 allocated_frames=40954
LTP CASE RUNTIME getpid02: 1260 ms
========== END ltp getpid02 ==========
========== START ltp getppid01 ==========
RUN LTP CASE getppid01
LTP MEMORY getppid01 before: free_frames=155215 allocated_frames=40954
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getppid01.c:31: [1;32mTPASS: [0mgetppid() returned 1149

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[595.430457 0:1149 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getppid01 : 0
Pass!
LTP MEMORY getppid01 after_run: free_frames=155199 allocated_frames=40970
LTP MEMORY getppid01 after_cleanup: free_frames=155199 allocated_frames=40970
LTP CASE RUNTIME getppid01: 1232 ms
========== END ltp getppid01 ==========
========== START ltp getuid01 ==========
RUN LTP CASE getuid01
LTP MEMORY getuid01 before: free_frames=155199 allocated_frames=40970
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
[37m[596.650600 0:1153 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getuid01 : 0
Pass!
LTP MEMORY getuid01 after_run: free_frames=155183 allocated_frames=40986
LTP MEMORY getuid01 after_cleanup: free_frames=155183 allocated_frames=40986
LTP CASE RUNTIME getuid01: 1218 ms
========== END ltp getuid01 ==========
========== START ltp geteuid01 ==========
RUN LTP CASE geteuid01
LTP MEMORY geteuid01 before: free_frames=155183 allocated_frames=40986
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
[37m[598.026934 0:1157 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE geteuid01 : 0
Pass!
LTP MEMORY geteuid01 after_run: free_frames=155167 allocated_frames=41002
LTP MEMORY geteuid01 after_cleanup: free_frames=155167 allocated_frames=41002
LTP CASE RUNTIME geteuid01: 1348 ms
========== END ltp geteuid01 ==========
========== START ltp getgid01 ==========
RUN LTP CASE getgid01
LTP MEMORY getgid01 before: free_frames=155167 allocated_frames=41002
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
[37m[599.183310 0:1161 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getgid01 : 0
Pass!
LTP MEMORY getgid01 after_run: free_frames=155151 allocated_frames=41018
LTP MEMORY getgid01 after_cleanup: free_frames=155151 allocated_frames=41018
LTP CASE RUNTIME getgid01: 1172 ms
========== END ltp getgid01 ==========
========== START ltp getegid01 ==========
RUN LTP CASE getegid01
LTP MEMORY getegid01 before: free_frames=155151 allocated_frames=41018
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
[37m[600.588298 0:1165 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getegid01 : 0
Pass!
LTP MEMORY getegid01 after_run: free_frames=155135 allocated_frames=41034
LTP MEMORY getegid01 after_cleanup: free_frames=155135 allocated_frames=41034
LTP CASE RUNTIME getegid01: 1442 ms
========== END ltp getegid01 ==========
========== START ltp lseek01 ==========
RUN LTP CASE lseek01
LTP MEMORY lseek01 before: free_frames=155135 allocated_frames=41034
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
[37m[601.845641 0:1169 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[601.854498 0:1169 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE lseek01 : 0
Pass!
LTP MEMORY lseek01 after_run: free_frames=155119 allocated_frames=41050
LTP MEMORY lseek01 after_cleanup: free_frames=155119 allocated_frames=41050
LTP CASE RUNTIME lseek01: 1241 ms
========== END ltp lseek01 ==========
========== START ltp read02 ==========
RUN LTP CASE read02
LTP MEMORY read02 before: free_frames=155119 allocated_frames=41050
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
[37m[603.101208 0:1173 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[603.103613 0:1173 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE read02 : 0
Pass!
LTP MEMORY read02 after_run: free_frames=155103 allocated_frames=41066
LTP MEMORY read02 after_cleanup: free_frames=155103 allocated_frames=41066
LTP CASE RUNTIME read02: 1213 ms
========== END ltp read02 ==========
========== START ltp write02 ==========
RUN LTP CASE write02
LTP MEMORY write02 before: free_frames=155103 allocated_frames=41066
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
[37m[604.303168 0:1177 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[604.305315 0:1177 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE write02 : 0
Pass!
LTP MEMORY write02 after_run: free_frames=155087 allocated_frames=41082
LTP MEMORY write02 after_cleanup: free_frames=155087 allocated_frames=41082
LTP CASE RUNTIME write02: 1199 ms
========== END ltp write02 ==========
========== START ltp creat01 ==========
RUN LTP CASE creat01
LTP MEMORY creat01 before: free_frames=155087 allocated_frames=41082
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
[37m[605.738895 0:1181 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[605.742741 0:1181 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE creat01 : 0
Pass!
LTP MEMORY creat01 after_run: free_frames=155071 allocated_frames=41098
LTP MEMORY creat01 after_cleanup: free_frames=155071 allocated_frames=41098
LTP CASE RUNTIME creat01: 1437 ms
========== END ltp creat01 ==========
========== START ltp creat03 ==========
RUN LTP CASE creat03
LTP MEMORY creat03 before: free_frames=155071 allocated_frames=41098
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
[37m[606.955876 0:1185 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[606.958217 0:1185 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE creat03 : 0
Pass!
LTP MEMORY creat03 after_run: free_frames=155055 allocated_frames=41114
LTP MEMORY creat03 after_cleanup: free_frames=155055 allocated_frames=41114
LTP CASE RUNTIME creat03: 1216 ms
========== END ltp creat03 ==========
========== START ltp open02 ==========
RUN LTP CASE open02
LTP MEMORY open02 before: free_frames=155055 allocated_frames=41114
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
[37m[608.238288 0:1189 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[608.240637 0:1189 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE open02 : 0
Pass!
LTP MEMORY open02 after_run: free_frames=155039 allocated_frames=41130
LTP MEMORY open02 after_cleanup: free_frames=155039 allocated_frames=41130
LTP CASE RUNTIME open02: 1271 ms
========== END ltp open02 ==========
========== START ltp open03 ==========
RUN LTP CASE open03
LTP MEMORY open03 before: free_frames=155039 allocated_frames=41130
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
[37m[609.589567 0:1193 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE open03 : 0
Pass!
LTP MEMORY open03 after_run: free_frames=155023 allocated_frames=41146
LTP MEMORY open03 after_cleanup: free_frames=155023 allocated_frames=41146
LTP CASE RUNTIME open03: 1371 ms
========== END ltp open03 ==========
========== START ltp stat02 ==========
RUN LTP CASE stat02
LTP MEMORY stat02 before: free_frames=155023 allocated_frames=41146
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
[37m[610.951044 0:1197 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE stat02 : 0
Pass!
LTP MEMORY stat02 after_run: free_frames=155007 allocated_frames=41162
LTP MEMORY stat02 after_cleanup: free_frames=155007 allocated_frames=41162
LTP CASE RUNTIME stat02: 1328 ms
========== END ltp stat02 ==========
========== START ltp lstat01 ==========
RUN LTP CASE lstat01
LTP MEMORY lstat01 before: free_frames=155007 allocated_frames=41162
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
[37m[612.082056 0:1201 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[612.084358 0:1201 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE lstat01 : 0
Pass!
LTP MEMORY lstat01 after_run: free_frames=154991 allocated_frames=41178
LTP MEMORY lstat01 after_cleanup: free_frames=154991 allocated_frames=41178
LTP CASE RUNTIME lstat01: 1136 ms
========== END ltp lstat01 ==========
========== START ltp chmod01 ==========
RUN LTP CASE chmod01
LTP MEMORY chmod01 before: free_frames=154991 allocated_frames=41178
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
[37m[613.302737 0:1205 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[613.305872 0:1205 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[613.309062 0:1205 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE chmod01 : 0
Pass!
LTP MEMORY chmod01 after_run: free_frames=154967 allocated_frames=41202
LTP MEMORY chmod01 after_cleanup: free_frames=154967 allocated_frames=41202
LTP CASE RUNTIME chmod01: 1237 ms
========== END ltp chmod01 ==========
========== START ltp fchmod01 ==========
RUN LTP CASE fchmod01
LTP MEMORY fchmod01 before: free_frames=154967 allocated_frames=41202
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
[37m[614.450041 0:1212 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[614.455019 0:1212 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fchmod01 : 0
Pass!
LTP MEMORY fchmod01 after_run: free_frames=154951 allocated_frames=41218
LTP MEMORY fchmod01 after_cleanup: free_frames=154951 allocated_frames=41218
LTP CASE RUNTIME fchmod01: 1154 ms
========== END ltp fchmod01 ==========
========== START ltp rmdir01 ==========
RUN LTP CASE rmdir01
LTP MEMORY rmdir01 before: free_frames=154951 allocated_frames=41218
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
[37m[615.891877 0:1216 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE rmdir01 : 0
Pass!
LTP MEMORY rmdir01 after_run: free_frames=154935 allocated_frames=41234
LTP MEMORY rmdir01 after_cleanup: free_frames=154935 allocated_frames=41234
LTP CASE RUNTIME rmdir01: 1393 ms
========== END ltp rmdir01 ==========
========== START ltp symlink01 ==========
RUN LTP CASE symlink01
LTP MEMORY symlink01 before: free_frames=154935 allocated_frames=41234
symlink01    1  [1;32mTPASS[0m  :  Creation of symbolic link file to no object file is ok
symlink01    2  [1;32mTPASS[0m  :  Creation of symbolic link file to no object file is ok
symlink01    3  [1;32mTPASS[0m  :  Creation of symbolic link file and object file via symbolic link is ok
symlink01    4  [1;32mTPASS[0m  :  Creating an existing symbolic link file error is caught
symlink01    5  [1;32mTPASS[0m  :  Creating a symbolic link which exceeds maximum pathname error is caught
[37m[617.130619 0:1220 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE symlink01 : 0
Pass!
LTP MEMORY symlink01 after_run: free_frames=154927 allocated_frames=41242
LTP MEMORY symlink01 after_cleanup: free_frames=154927 allocated_frames=41242
LTP CASE RUNTIME symlink01: 1238 ms
========== END ltp symlink01 ==========
========== START ltp readlink01 ==========
RUN LTP CASE readlink01
LTP MEMORY readlink01 before: free_frames=154927 allocated_frames=41242
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
[37m[618.508424 0:1221 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[618.513013 0:1221 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE readlink01 : 0
Pass!
LTP MEMORY readlink01 after_run: free_frames=154903 allocated_frames=41266
LTP MEMORY readlink01 after_cleanup: free_frames=154903 allocated_frames=41266
LTP CASE RUNTIME readlink01: 1416 ms
========== END ltp readlink01 ==========
========== START ltp ftruncate01 ==========
RUN LTP CASE ftruncate01
LTP MEMORY ftruncate01 before: free_frames=154903 allocated_frames=41266
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
[37m[619.753398 0:1226 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[619.754999 0:1226 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE ftruncate01 : 0
Pass!
LTP MEMORY ftruncate01 after_run: free_frames=154887 allocated_frames=41282
LTP MEMORY ftruncate01 after_cleanup: free_frames=154887 allocated_frames=41282
LTP CASE RUNTIME ftruncate01: 1217 ms
========== END ltp ftruncate01 ==========
========== START ltp umask01 ==========
RUN LTP CASE umask01
LTP MEMORY umask01 before: free_frames=154887 allocated_frames=41282
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
[37m[621.160980 0:1230 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE umask01 : 0
Pass!
LTP MEMORY umask01 after_run: free_frames=154871 allocated_frames=41298
LTP MEMORY umask01 after_cleanup: free_frames=154871 allocated_frames=41298
LTP CASE RUNTIME umask01: 1387 ms
========== END ltp umask01 ==========
========== START ltp alarm02 ==========
RUN LTP CASE alarm02
LTP MEMORY alarm02 before: free_frames=154871 allocated_frames=41298
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
[37m[622.537335 0:1234 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE alarm02 : 0
Pass!
LTP MEMORY alarm02 after_run: free_frames=154855 allocated_frames=41314
LTP MEMORY alarm02 after_cleanup: free_frames=154855 allocated_frames=41314
LTP CASE RUNTIME alarm02: 1445 ms
========== END ltp alarm02 ==========
========== START ltp alarm03 ==========
RUN LTP CASE alarm03
LTP MEMORY alarm03 before: free_frames=154855 allocated_frames=41314
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
[37m[623.939191 0:1241 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE alarm03 : 0
Pass!
LTP MEMORY alarm03 after_run: free_frames=154831 allocated_frames=41338
LTP MEMORY alarm03 after_cleanup: free_frames=154831 allocated_frames=41338
LTP CASE RUNTIME alarm03: 1320 ms
========== END ltp alarm03 ==========
========== START ltp clock_gettime02 ==========
RUN LTP CASE clock_gettime02
LTP MEMORY clock_gettime02 before: free_frames=154831 allocated_frames=41338
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
[37m[625.243085 0:1247 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE clock_gettime02 : 0
Pass!
LTP MEMORY clock_gettime02 after_run: free_frames=154815 allocated_frames=41354
LTP MEMORY clock_gettime02 after_cleanup: free_frames=154815 allocated_frames=41354
LTP CASE RUNTIME clock_gettime02: 1307 ms
========== END ltp clock_gettime02 ==========
========== START ltp gettimeofday01 ==========
RUN LTP CASE gettimeofday01
LTP MEMORY gettimeofday01 before: free_frames=154815 allocated_frames=41354
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
[37m[626.510828 0:1251 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE gettimeofday01 : 0
Pass!
LTP MEMORY gettimeofday01 after_run: free_frames=154799 allocated_frames=41370
LTP MEMORY gettimeofday01 after_cleanup: free_frames=154799 allocated_frames=41370
LTP CASE RUNTIME gettimeofday01: 1270 ms
========== END ltp gettimeofday01 ==========
========== START ltp time01 ==========
RUN LTP CASE time01
LTP MEMORY time01 before: free_frames=154799 allocated_frames=41370
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
time01.c:36: [1;32mTPASS: [0mtime() returned value 627
time01.c:38: [1;32mTPASS: [0mtime() returned value 627, stored value 627 are same

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[627.681878 0:1255 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE time01 : 0
Pass!
LTP MEMORY time01 after_run: free_frames=154783 allocated_frames=41386
LTP MEMORY time01 after_cleanup: free_frames=154783 allocated_frames=41386
LTP CASE RUNTIME time01: 1171 ms
========== END ltp time01 ==========
========== START ltp times01 ==========
RUN LTP CASE times01
LTP MEMORY times01 before: free_frames=154783 allocated_frames=41386
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
times01.c:25: [1;32mTPASS: [0mtimes(&mytimes) returned 628872

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[628.893880 0:1259 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE times01 : 0
Pass!
LTP MEMORY times01 after_run: free_frames=154767 allocated_frames=41402
LTP MEMORY times01 after_cleanup: free_frames=154767 allocated_frames=41402
LTP CASE RUNTIME times01: 1207 ms
========== END ltp times01 ==========
========== START ltp kill03 ==========
RUN LTP CASE kill03
LTP MEMORY kill03 before: free_frames=154767 allocated_frames=41402
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
[37m[630.118886 0:1263 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE kill03 : 0
Pass!
LTP MEMORY kill03 after_run: free_frames=154751 allocated_frames=41418
LTP MEMORY kill03 after_cleanup: free_frames=154751 allocated_frames=41418
LTP CASE RUNTIME kill03: 1217 ms
========== END ltp kill03 ==========
========== START ltp rt_sigaction01 ==========
RUN LTP CASE rt_sigaction01
LTP MEMORY rt_sigaction01 before: free_frames=154751 allocated_frames=41418
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
[37m[632.081886 0:1267 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE rt_sigaction01 : 0
Pass!
LTP MEMORY rt_sigaction01 after_run: free_frames=154743 allocated_frames=41426
LTP MEMORY rt_sigaction01 after_cleanup: free_frames=154743 allocated_frames=41426
LTP CASE RUNTIME rt_sigaction01: 1957 ms
========== END ltp rt_sigaction01 ==========
========== START ltp sigaction01 ==========
RUN LTP CASE sigaction01
LTP MEMORY sigaction01 before: free_frames=154743 allocated_frames=41426
sigaction01    1  [1;32mTPASS[0m  :  SA_RESETHAND did not cause SA_SIGINFO to be cleared
sigaction01    2  [1;32mTPASS[0m  :  SA_RESETHAND was masked when handler executed
sigaction01    3  [1;32mTPASS[0m  :  sig has been masked because sa_mask originally contained sig
sigaction01    4  [1;32mTPASS[0m  :  siginfo pointer non NULL
PASS LTP CASE sigaction01 : 0
Pass!
LTP MEMORY sigaction01 after_run: free_frames=154735 allocated_frames=41434
LTP MEMORY sigaction01 after_cleanup: free_frames=154735 allocated_frames=41434
LTP CASE RUNTIME sigaction01: 1156 ms
========== END ltp sigaction01 ==========
========== START ltp proc01 ==========
RUN LTP CASE proc01
LTP MEMORY proc01 before: free_frames=154735 allocated_frames=41434
proc01      1  [1;32mTPASS[0m  :  readproc() completed successfully, total read: 875 bytes, 20 objs
[37m[634.678681 0:1269 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE proc01 : 0
Pass!
LTP MEMORY proc01 after_run: free_frames=154727 allocated_frames=41442
LTP MEMORY proc01 after_cleanup: free_frames=154727 allocated_frames=41442
LTP CASE RUNTIME proc01: 1454 ms
========== END ltp proc01 ==========
========== START ltp exit01 ==========
RUN LTP CASE exit01
LTP MEMORY exit01 before: free_frames=154727 allocated_frames=41442
exit01      1  [1;32mTPASS[0m  :  exit() test PASSED
PASS LTP CASE exit01 : 0
Pass!
LTP MEMORY exit01 after_run: free_frames=154711 allocated_frames=41458
LTP MEMORY exit01 after_cleanup: free_frames=154711 allocated_frames=41458
LTP CASE RUNTIME exit01: 1177 ms
========== END ltp exit01 ==========
========== START ltp exit02 ==========
RUN LTP CASE exit02
LTP MEMORY exit02 before: free_frames=154711 allocated_frames=41458
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
[37m[637.048098 0:1272 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE exit02 : 0
Pass!
LTP MEMORY exit02 after_run: free_frames=154687 allocated_frames=41482
LTP MEMORY exit02 after_cleanup: free_frames=154687 allocated_frames=41482
LTP CASE RUNTIME exit02: 1175 ms
========== END ltp exit02 ==========
========== START ltp exit_group01 ==========
RUN LTP CASE exit_group01
LTP MEMORY exit_group01 before: free_frames=154687 allocated_frames=41482
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
[37m[638.193165 0:1277 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[638.199935 0:1277 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE exit_group01 : 0
Pass!
LTP MEMORY exit_group01 after_run: free_frames=154663 allocated_frames=41506
LTP MEMORY exit_group01 after_cleanup: free_frames=154663 allocated_frames=41506
LTP CASE RUNTIME exit_group01: 1161 ms
========== END ltp exit_group01 ==========
========== START ltp getpgrp01 ==========
RUN LTP CASE getpgrp01
LTP MEMORY getpgrp01 before: free_frames=154663 allocated_frames=41506
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getpgrp01.c:18: [1;32mTPASS: [0mgetpgrp() returned pid 1286
getpgrp01.c:19: [1;32mTPASS: [0mTST_RET == SAFE_GETPGID(0) (1286)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[639.320569 0:1284 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpgrp01 : 0
Pass!
LTP MEMORY getpgrp01 after_run: free_frames=154647 allocated_frames=41522
LTP MEMORY getpgrp01 after_cleanup: free_frames=154647 allocated_frames=41522
LTP CASE RUNTIME getpgrp01: 1108 ms
========== END ltp getpgrp01 ==========
========== START ltp gettid01 ==========
RUN LTP CASE gettid01
LTP MEMORY gettid01 before: free_frames=154647 allocated_frames=41522
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
gettid01.c:26: [1;32mTPASS: [0mtst_syscall(__NR_gettid) == tst_syscall(__NR_getpid) (1290)
gettid01.c:27: [1;32mTPASS: [0mtst_syscall(__NR_gettid) == pid (1290)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[640.724971 0:1288 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE gettid01 : 0
Pass!
LTP MEMORY gettid01 after_run: free_frames=154631 allocated_frames=41538
LTP MEMORY gettid01 after_cleanup: free_frames=154631 allocated_frames=41538
LTP CASE RUNTIME gettid01: 1421 ms
========== END ltp gettid01 ==========
========== START ltp uname01 ==========
RUN LTP CASE uname01
LTP MEMORY uname01 before: free_frames=154631 allocated_frames=41538
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
[37m[641.861135 0:1292 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE uname01 : 0
Pass!
LTP MEMORY uname01 after_run: free_frames=154615 allocated_frames=41554
LTP MEMORY uname01 after_cleanup: free_frames=154615 allocated_frames=41554
LTP CASE RUNTIME uname01: 1113 ms
========== END ltp uname01 ==========
========== START ltp getrlimit01 ==========
RUN LTP CASE getrlimit01
LTP MEMORY getrlimit01 before: free_frames=154615 allocated_frames=41554
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
[37m[642.959640 0:1296 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getrlimit01 : 0
Pass!
LTP MEMORY getrlimit01 after_run: free_frames=154599 allocated_frames=41570
LTP MEMORY getrlimit01 after_cleanup: free_frames=154599 allocated_frames=41570
LTP CASE RUNTIME getrlimit01: 1085 ms
========== END ltp getrlimit01 ==========
========== START ltp getrusage01 ==========
RUN LTP CASE getrusage01
LTP MEMORY getrusage01 before: free_frames=154599 allocated_frames=41570
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
[37m[644.195783 0:1300 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getrusage01 : 0
Pass!
LTP MEMORY getrusage01 after_run: free_frames=154583 allocated_frames=41586
LTP MEMORY getrusage01 after_cleanup: free_frames=154583 allocated_frames=41586
LTP CASE RUNTIME getrusage01: 1236 ms
========== END ltp getrusage01 ==========
========== START ltp sched_yield01 ==========
RUN LTP CASE sched_yield01
LTP MEMORY sched_yield01 before: free_frames=154583 allocated_frames=41586
sched_yield01    1  [1;32mTPASS[0m  :  sched_yield() call succeeded
PASS LTP CASE sched_yield01 : 0
Pass!
LTP MEMORY sched_yield01 after_run: free_frames=154575 allocated_frames=41594
LTP MEMORY sched_yield01 after_cleanup: free_frames=154575 allocated_frames=41594
LTP CASE RUNTIME sched_yield01: 1084 ms
========== END ltp sched_yield01 ==========
========== START ltp getpgid02 ==========
RUN LTP CASE getpgid02
LTP MEMORY getpgid02 before: free_frames=154575 allocated_frames=41594
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getpgid02.c:27: [1;32mTPASS: [0mgetpgid(-99) : ESRCH (3)
getpgid02.c:28: [1;32mTPASS: [0mgetpgid(4194304) : ESRCH (3)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[646.490536 0:1305 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpgid02 : 0
Pass!
LTP MEMORY getpgid02 after_run: free_frames=154559 allocated_frames=41610
LTP MEMORY getpgid02 after_cleanup: free_frames=154559 allocated_frames=41610
LTP CASE RUNTIME getpgid02: 1215 ms
========== END ltp getpgid02 ==========
========== START ltp getsid02 ==========
RUN LTP CASE getsid02
LTP MEMORY getsid02 before: free_frames=154559 allocated_frames=41610
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getsid02.c:22: [1;32mTPASS: [0mgetsid(unused_pid) : ESRCH (3)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[648.002079 0:1309 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getsid02 : 0
Pass!
LTP MEMORY getsid02 after_run: free_frames=154543 allocated_frames=41626
LTP MEMORY getsid02 after_cleanup: free_frames=154543 allocated_frames=41626
LTP CASE RUNTIME getsid02: 1505 ms
========== END ltp getsid02 ==========
========== START ltp getppid02 ==========
RUN LTP CASE getppid02
LTP MEMORY getppid02 before: free_frames=154543 allocated_frames=41626
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getppid02.c:31: [1;32mTPASS: [0mgetppid() returned parent pid (1315)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[649.193174 0:1313 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getppid02 : 0
Pass!
LTP MEMORY getppid02 after_run: free_frames=154519 allocated_frames=41650
LTP MEMORY getppid02 after_cleanup: free_frames=154519 allocated_frames=41650
LTP CASE RUNTIME getppid02: 1195 ms
========== END ltp getppid02 ==========
========== START ltp getuid03 ==========
RUN LTP CASE getuid03
LTP MEMORY getuid03 before: free_frames=154519 allocated_frames=41650
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getuid03.c:20: [1;32mTPASS: [0mgetuid() returned 0
getuid03.c:32: [1;32mTPASS: [0mgetuid() ret == /proc/self/status Uid: 0

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[650.367562 0:1318 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getuid03 : 0
Pass!
LTP MEMORY getuid03 after_run: free_frames=154503 allocated_frames=41666
LTP MEMORY getuid03 after_cleanup: free_frames=154503 allocated_frames=41666
LTP CASE RUNTIME getuid03: 1189 ms
========== END ltp getuid03 ==========
========== START ltp geteuid02 ==========
RUN LTP CASE geteuid02
LTP MEMORY geteuid02 before: free_frames=154503 allocated_frames=41666
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
geteuid02.c:21: [1;32mTPASS: [0mgeteuid() returned 0
geteuid02.c:29: [1;32mTPASS: [0mExpect: geteuid() ret 0 == /proc/self/status EUID: 0

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[651.700996 0:1322 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE geteuid02 : 0
Pass!
LTP MEMORY geteuid02 after_run: free_frames=154487 allocated_frames=41682
LTP MEMORY geteuid02 after_cleanup: free_frames=154487 allocated_frames=41682
LTP CASE RUNTIME geteuid02: 1359 ms
========== END ltp geteuid02 ==========
========== START ltp getgid03 ==========
RUN LTP CASE getgid03
LTP MEMORY getgid03 before: free_frames=154487 allocated_frames=41682
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getgid03.c:43: [1;32mTPASS: [0mvalues from getgid() and getpwuid() match

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[652.896374 0:1326 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getgid03 : 0
Pass!
LTP MEMORY getgid03 after_run: free_frames=154471 allocated_frames=41698
LTP MEMORY getgid03 after_cleanup: free_frames=154471 allocated_frames=41698
LTP CASE RUNTIME getgid03: 1155 ms
========== END ltp getgid03 ==========
========== START ltp getegid02 ==========
RUN LTP CASE getegid02
LTP MEMORY getegid02 before: free_frames=154471 allocated_frames=41698
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getegid02.c:34: [1;32mTPASS: [0mpwent->pw_gid == egid (0)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[654.051228 0:1330 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getegid02 : 0
Pass!
LTP MEMORY getegid02 after_run: free_frames=154455 allocated_frames=41714
LTP MEMORY getegid02 after_cleanup: free_frames=154455 allocated_frames=41714
LTP CASE RUNTIME getegid02: 1137 ms
========== END ltp getegid02 ==========
========== START ltp getgroups03 ==========
RUN LTP CASE getgroups03
LTP MEMORY getgroups03 before: free_frames=154455 allocated_frames=41714
getgroups03    1  [1;32mTPASS[0m  :  getgroups functionality correct
PASS LTP CASE getgroups03 : 0
Pass!
LTP MEMORY getgroups03 after_run: free_frames=154447 allocated_frames=41722
LTP MEMORY getgroups03 after_cleanup: free_frames=154447 allocated_frames=41722
LTP CASE RUNTIME getgroups03: 1200 ms
========== END ltp getgroups03 ==========
========== START ltp uname02 ==========
RUN LTP CASE uname02
LTP MEMORY uname02 before: free_frames=154447 allocated_frames=41722
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
uname02.c:19: [1;32mTPASS: [0muname(bad_addr) : EFAULT (14)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[656.409575 0:1335 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE uname02 : 0
Pass!
LTP MEMORY uname02 after_run: free_frames=154431 allocated_frames=41738
LTP MEMORY uname02 after_cleanup: free_frames=154431 allocated_frames=41738
LTP CASE RUNTIME uname02: 1164 ms
========== END ltp uname02 ==========
========== START ltp wait01 ==========
RUN LTP CASE wait01
LTP MEMORY wait01 before: free_frames=154431 allocated_frames=41738
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
wait01.c:19: [1;32mTPASS: [0mwait(NULL) : ECHILD (10)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[657.851118 0:1339 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE wait01 : 0
Pass!
LTP MEMORY wait01 after_run: free_frames=154415 allocated_frames=41754
LTP MEMORY wait01 after_cleanup: free_frames=154415 allocated_frames=41754
LTP CASE RUNTIME wait01: 1441 ms
========== END ltp wait01 ==========
========== START ltp wait02 ==========
RUN LTP CASE wait02
LTP MEMORY wait02 before: free_frames=154415 allocated_frames=41754
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
wait02.c:41: [1;32mTPASS: [0mwait() succeeded

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[659.144271 0:1343 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE wait02 : 0
Pass!
LTP MEMORY wait02 after_run: free_frames=154391 allocated_frames=41778
LTP MEMORY wait02 after_cleanup: free_frames=154391 allocated_frames=41778
LTP CASE RUNTIME wait02: 1282 ms
========== END ltp wait02 ==========
========== START ltp getrlimit02 ==========
RUN LTP CASE getrlimit02
LTP MEMORY getrlimit02 before: free_frames=154391 allocated_frames=41778
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getrlimit02.c:40: [1;32mTPASS: [0mgetrlimit() with invalid address : EFAULT (14)
getrlimit02.c:40: [1;32mTPASS: [0mgetrlimit() with invalid resource type : EINVAL (22)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[660.258223 0:1348 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getrlimit02 : 0
Pass!
LTP MEMORY getrlimit02 after_run: free_frames=154375 allocated_frames=41794
LTP MEMORY getrlimit02 after_cleanup: free_frames=154375 allocated_frames=41794
LTP CASE RUNTIME getrlimit02: 1125 ms
========== END ltp getrlimit02 ==========
ltp cases: 75 passed, 0 failed, 0 timed out
#### OS COMP TEST GROUP END ltp-musl ####
#### OS COMP TEST GROUP START ltp-glibc ####
ltp case list: stable (75 cases, timeout 15s)
========== START ltp access01 ==========
RUN LTP CASE access01
LTP MEMORY access01 before: free_frames=154375 allocated_frames=41794
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
[37m[666.910599 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.911411 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.912050 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.913128 0:1352 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[666.914302 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.915032 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.915777 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.916493 0:1352 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[666.917437 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.918076 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.918705 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.919388 0:1352 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[666.920317 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.920919 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.921511 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.922175 0:1352 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[666.923107 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.923732 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.924306 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.925286 0:1352 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[666.926534 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.927125 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.927703 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.928366 0:1352 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[666.929052 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.929710 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.930412 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.931093 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[666.932059 0:1352 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE access01 : 0
Pass!
LTP MEMORY access01 after_run: free_frames=153445 allocated_frames=42724
LTP MEMORY access01 after_cleanup: free_frames=153445 allocated_frames=42724
LTP CASE RUNTIME access01: 6625 ms
========== END ltp access01 ==========
========== START ltp brk01 ==========
RUN LTP CASE brk01
LTP MEMORY brk01 before: free_frames=153445 allocated_frames=42724
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
[37m[669.276562 0:1458 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE brk01 : 0
Pass!
LTP MEMORY brk01 after_run: free_frames=153415 allocated_frames=42754
LTP MEMORY brk01 after_cleanup: free_frames=153415 allocated_frames=42754
LTP CASE RUNTIME brk01: 2358 ms
========== END ltp brk01 ==========
========== START ltp chdir01 ==========
RUN LTP CASE chdir01
LTP MEMORY chdir01 before: free_frames=153415 allocated_frames=42754
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
tst_test.c:1120: TINFO: Mounting ltp-tmpfs to /tmp/ltp-work/LTP_chdbZFNGR/mntpoint fstyp=tmpfs flags=0
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
[37m[671.958560 0:1465 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[671.960861 0:1465 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[671.962104 0:1465 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[671.963385 0:1465 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[671.964799 0:1465 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE chdir01 : 0
Pass!
LTP MEMORY chdir01 after_run: free_frames=153394 allocated_frames=42775
LTP MEMORY chdir01 after_cleanup: free_frames=153394 allocated_frames=42775
LTP CASE RUNTIME chdir01: 2665 ms
========== END ltp chdir01 ==========
========== START ltp clone01 ==========
RUN LTP CASE clone01
LTP MEMORY clone01 before: free_frames=153394 allocated_frames=42775
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clone01.c:37: TPASS: clone returned 1472
clone01.c:43: TPASS: Child exited with 0

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[674.289202 0:1469 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE clone01 : 0
Pass!
LTP MEMORY clone01 after_run: free_frames=153364 allocated_frames=42805
LTP MEMORY clone01 after_cleanup: free_frames=153364 allocated_frames=42805
LTP CASE RUNTIME clone01: 2336 ms
========== END ltp clone01 ==========
========== START ltp close01 ==========
RUN LTP CASE close01
LTP MEMORY close01 before: free_frames=153364 allocated_frames=42805
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
[37m[676.816690 0:1474 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[676.821748 0:1474 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE close01 : 0
Pass!
LTP MEMORY close01 after_run: free_frames=153343 allocated_frames=42826
LTP MEMORY close01 after_cleanup: free_frames=153343 allocated_frames=42826
LTP CASE RUNTIME close01: 2545 ms
========== END ltp close01 ==========
========== START ltp dup01 ==========
RUN LTP CASE dup01
LTP MEMORY dup01 before: free_frames=153343 allocated_frames=42826
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
dup01.c:24: TPASS: dup(fd) returned fd 4
dup01.c:27: TPASS: buf1.st_ino == buf2.st_ino (13850509113488738198)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[679.152871 0:1478 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[679.156667 0:1478 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE dup01 : 0
Pass!
LTP MEMORY dup01 after_run: free_frames=153322 allocated_frames=42847
LTP MEMORY dup01 after_cleanup: free_frames=153322 allocated_frames=42847
LTP CASE RUNTIME dup01: 2307 ms
========== END ltp dup01 ==========
========== START ltp fcntl01 ==========
RUN LTP CASE fcntl01
LTP MEMORY fcntl01 before: free_frames=153322 allocated_frames=42847
[37m[681.309666 0:1482 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fcntl01 : 0
Pass!
LTP MEMORY fcntl01 after_run: free_frames=153310 allocated_frames=42859
LTP MEMORY fcntl01 after_cleanup: free_frames=153310 allocated_frames=42859
LTP CASE RUNTIME fcntl01: 2160 ms
========== END ltp fcntl01 ==========
========== START ltp fcntl02 ==========
RUN LTP CASE fcntl02
LTP MEMORY fcntl02 before: free_frames=153310 allocated_frames=42859
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
fcntl02.c:41: TPASS: fcntl(fcntl02_1485, F_DUPFD, 0) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1485, F_DUPFD, 1) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1485, F_DUPFD, 2) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1485, F_DUPFD, 3) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1485, F_DUPFD, 10) returned 10
fcntl02.c:41: TPASS: fcntl(fcntl02_1485, F_DUPFD, 100) returned 100

Summary:
passed   6
failed   0
broken   0
skipped  0
warnings 0
[37m[683.680424 0:1483 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[683.684388 0:1483 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fcntl02 : 0
Pass!
LTP MEMORY fcntl02 after_run: free_frames=153289 allocated_frames=42880
LTP MEMORY fcntl02 after_cleanup: free_frames=153289 allocated_frames=42880
LTP CASE RUNTIME fcntl02: 2406 ms
========== END ltp fcntl02 ==========
========== START ltp fork01 ==========
RUN LTP CASE fork01
LTP MEMORY fork01 before: free_frames=153289 allocated_frames=42880
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
fork01.c:47: TPASS: correct child status returned 42
fork01.c:50: TPASS: child_pid == pid (1490)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[686.075149 0:1487 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[686.076340 0:1487 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fork01 : 0
Pass!
LTP MEMORY fork01 after_run: free_frames=153259 allocated_frames=42910
LTP MEMORY fork01 after_cleanup: free_frames=153259 allocated_frames=42910
LTP CASE RUNTIME fork01: 2346 ms
========== END ltp fork01 ==========
========== START ltp getpid01 ==========
RUN LTP CASE getpid01
LTP MEMORY getpid01 before: free_frames=153259 allocated_frames=42910
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getpid01.c:34: TPASS: getpid() returns 1495
getpid01.c:34: TPASS: getpid() returns 1497
getpid01.c:34: TPASS: getpid() returns 1498
getpid01.c:34: TPASS: getpid() returns 1499
getpid01.c:34: TPASS: getpid() returns 1500
getpid01.c:34: TPASS: getpid() returns 1501
getpid01.c:34: TPASS: getpid() returns 1502
getpid01.c:34: TPASS: getpid() returns 1503
getpid01.c:34: TPASS: getpid() returns 1504
getpid01.c:34: TPASS: getpid() returns 1505
getpid01.c:34: TPASS: getpid() returns 1506
getpid01.c:34: TPASS: getpid() returns 1507
getpid01.c:34: TPASS: getpid() returns 1508
getpid01.c:34: TPASS: getpid() returns 1509
getpid01.c:34: TPASS: getpid() returns 1510
getpid01.c:34: TPASS: getpid() returns 1511
getpid01.c:34: TPASS: getpid() returns 1512
getpid01.c:34: TPASS: getpid() returns 1513
getpid01.c:34: TPASS: getpid() returns 1514
getpid01.c:34: TPASS: getpid() returns 1515
getpid01.c:34: TPASS: getpid() returns 1516
getpid01.c:34: TPASS: getpid() returns 1517
getpid01.c:34: TPASS: getpid() returns 1518
getpid01.c:34: TPASS: getpid() returns 1519
getpid01.c:34: TPASS: getpid() returns 1520
getpid01.c:34: TPASS: getpid() returns 1521
getpid01.c:34: TPASS: getpid() returns 1522
getpid01.c:34: TPASS: getpid() returns 1523
getpid01.c:34: TPASS: getpid() returns 1524
getpid01.c:34: TPASS: getpid() returns 1525
getpid01.c:34: TPASS: getpid() returns 1526
getpid01.c:34: TPASS: getpid() returns 1527
getpid01.c:34: TPASS: getpid() returns 1528
getpid01.c:34: TPASS: getpid() returns 1529
getpid01.c:34: TPASS: getpid() returns 1530
getpid01.c:34: TPASS: getpid() returns 1531
getpid01.c:34: TPASS: getpid() returns 1532
getpid01.c:34: TPASS: getpid() returns 1533
getpid01.c:34: TPASS: getpid() returns 1534
getpid01.c:34: TPASS: getpid() returns 1535
getpid01.c:34: TPASS: getpid() returns 1536
getpid01.c:34: TPASS: getpid() returns 1537
getpid01.c:34: TPASS: getpid() returns 1538
getpid01.c:34: TPASS: getpid() returns 1539
getpid01.c:34: TPASS: getpid() returns 1540
getpid01.c:34: TPASS: getpid() returns 1541
getpid01.c:34: TPASS: getpid() returns 1542
getpid01.c:34: TPASS: getpid() returns 1543
getpid01.c:34: TPASS: getpid() returns 1544
getpid01.c:34: TPASS: getpid() returns 1545
getpid01.c:34: TPASS: getpid() returns 1546
getpid01.c:34: TPASS: getpid() returns 1547
getpid01.c:34: TPASS: getpid() returns 1548
getpid01.c:34: TPASS: getpid() returns 1549
getpid01.c:34: TPASS: getpid() returns 1550
getpid01.c:34: TPASS: getpid() returns 1551
getpid01.c:34: TPASS: getpid() returns 1552
getpid01.c:34: TPASS: getpid() returns 1553
getpid01.c:34: TPASS: getpid() returns 1554
getpid01.c:34: TPASS: getpid() returns 1555
getpid01.c:34: TPASS: getpid() returns 1556
getpid01.c:34: TPASS: getpid() returns 1557
getpid01.c:34: TPASS: getpid() returns 1558
getpid01.c:34: TPASS: getpid() returns 1559
getpid01.c:34: TPASS: getpid() returns 1560
getpid01.c:34: TPASS: getpid() returns 1561
getpid01.c:34: TPASS: getpid() returns 1562
getpid01.c:34: TPASS: getpid() returns 1563
getpid01.c:34: TPASS: getpid() returns 1564
getpid01.c:34: TPASS: getpid() returns 1565
getpid01.c:34: TPASS: getpid() returns 1566
getpid01.c:34: TPASS: getpid() returns 1567
getpid01.c:34: TPASS: getpid() returns 1568
getpid01.c:34: TPASS: getpid() returns 1569
getpid01.c:34: TPASS: getpid() returns 1570
getpid01.c:34: TPASS: getpid() returns 1571
getpid01.c:34: TPASS: getpid() returns 1572
getpid01.c:34: TPASS: getpid() returns 1573
getpid01.c:34: TPASS: getpid() returns 1574
getpid01.c:34: TPASS: getpid() returns 1575
getpid01.c:34: TPASS: getpid() returns 1576
getpid01.c:34: TPASS: getpid() returns 1577
getpid01.c:34: TPASS: getpid() returns 1578
getpid01.c:34: TPASS: getpid() returns 1579
getpid01.c:34: TPASS: getpid() returns 1580
getpid01.c:34: TPASS: getpid() returns 1581
getpid01.c:34: TPASS: getpid() returns 1582
getpid01.c:34: TPASS: getpid() returns 1583
getpid01.c:34: TPASS: getpid() returns 1584
getpid01.c:34: TPASS: getpid() returns 1585
getpid01.c:34: TPASS: getpid() returns 1586
getpid01.c:34: TPASS: getpid() returns 1587
getpid01.c:34: TPASS: getpid() returns 1588
getpid01.c:34: TPASS: getpid() returns 1589
getpid01.c:34: TPASS: getpid() returns 1590
getpid01.c:34: TPASS: getpid() returns 1591
getpid01.c:34: TPASS: getpid() returns 1592
getpid01.c:34: TPASS: getpid() returns 1593
getpid01.c:34: TPASS: getpid() returns 1594
getpid01.c:34: TPASS: getpid() returns 1595

Summary:
passed   100
failed   0
broken   0
skipped  0
warnings 0
[37m[691.942833 0:1492 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpid01 : 0
Pass!
LTP MEMORY getpid01 after_run: free_frames=152338 allocated_frames=43831
LTP MEMORY getpid01 after_cleanup: free_frames=152338 allocated_frames=43831
LTP CASE RUNTIME getpid01: 5864 ms
========== END ltp getpid01 ==========
========== START ltp mmap01 ==========
RUN LTP CASE mmap01
LTP MEMORY mmap01 before: free_frames=152338 allocated_frames=43831
[37m[694.295238 0:1597 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[694.296338 0:1597 axfs::root:433] [33m[AxError::IsADirectory][m
[mmmap01      1  TPASS  :  Functionality of mmap() successful
PASS LTP CASE mmap01 : 0
Pass!
LTP MEMORY mmap01 after_run: free_frames=152317 allocated_frames=43852
LTP MEMORY mmap01 after_cleanup: free_frames=152317 allocated_frames=43852
LTP CASE RUNTIME mmap01: 2355 ms
========== END ltp mmap01 ==========
========== START ltp open01 ==========
RUN LTP CASE open01
LTP MEMORY open01 before: free_frames=152317 allocated_frames=43852
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
[37m[696.315092 0:1599 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[696.316340 0:1599 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE open01 : 0
Pass!
LTP MEMORY open01 after_run: free_frames=152296 allocated_frames=43873
LTP MEMORY open01 after_cleanup: free_frames=152296 allocated_frames=43873
LTP CASE RUNTIME open01: 2026 ms
========== END ltp open01 ==========
========== START ltp pipe01 ==========
RUN LTP CASE pipe01
LTP MEMORY pipe01 before: free_frames=152296 allocated_frames=43873
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
[37m[698.919454 0:1603 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE pipe01 : 0
Pass!
LTP MEMORY pipe01 after_run: free_frames=152275 allocated_frames=43894
LTP MEMORY pipe01 after_cleanup: free_frames=152275 allocated_frames=43894
LTP CASE RUNTIME pipe01: 2595 ms
========== END ltp pipe01 ==========
========== START ltp read01 ==========
RUN LTP CASE read01
LTP MEMORY read01 before: free_frames=152275 allocated_frames=43894
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
[37m[701.169505 0:1607 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[701.171790 0:1607 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE read01 : 0
Pass!
LTP MEMORY read01 after_run: free_frames=152254 allocated_frames=43915
LTP MEMORY read01 after_cleanup: free_frames=152254 allocated_frames=43915
LTP CASE RUNTIME read01: 2251 ms
========== END ltp read01 ==========
========== START ltp stat01 ==========
RUN LTP CASE stat01
LTP MEMORY stat01 before: free_frames=152254 allocated_frames=43915
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
[37m[703.338503 0:1611 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[703.339517 0:1611 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[703.342505 0:1611 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE stat01 : 0
Pass!
LTP MEMORY stat01 after_run: free_frames=152233 allocated_frames=43936
LTP MEMORY stat01 after_cleanup: free_frames=152233 allocated_frames=43936
LTP CASE RUNTIME stat01: 2184 ms
========== END ltp stat01 ==========
========== START ltp wait401 ==========
RUN LTP CASE wait401
LTP MEMORY wait401 before: free_frames=152233 allocated_frames=43936
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
wait401.c:40: TPASS: wait4() returned correct pid 1618
wait401.c:49: TPASS: WIFEXITED() is set in status
wait401.c:54: TPASS: WEXITSTATUS() == 0

Summary:
passed   3
failed   0
broken   0
skipped  0
warnings 0
[37m[705.758959 0:1615 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE wait401 : 0
Pass!
LTP MEMORY wait401 after_run: free_frames=152203 allocated_frames=43966
LTP MEMORY wait401 after_cleanup: free_frames=152203 allocated_frames=43966
LTP CASE RUNTIME wait401: 2408 ms
========== END ltp wait401 ==========
========== START ltp write01 ==========
RUN LTP CASE write01
LTP MEMORY write01 before: free_frames=152203 allocated_frames=43966
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
[37m[708.783716 0:1620 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[708.793429 0:1620 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE write01 : 0
Pass!
LTP MEMORY write01 after_run: free_frames=119414 allocated_frames=76755
LTP MEMORY write01 after_cleanup: free_frames=119414 allocated_frames=76755
LTP CASE RUNTIME write01: 3057 ms
========== END ltp write01 ==========
========== START ltp access03 ==========
RUN LTP CASE access03
LTP MEMORY access03 before: free_frames=119414 allocated_frames=76755
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
[37m[711.079924 0:1624 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE access03 : 0
Pass!
LTP MEMORY access03 after_run: free_frames=119357 allocated_frames=76812
LTP MEMORY access03 after_cleanup: free_frames=119357 allocated_frames=76812
LTP CASE RUNTIME access03: 2245 ms
========== END ltp access03 ==========
========== START ltp close02 ==========
RUN LTP CASE close02
LTP MEMORY close02 before: free_frames=119357 allocated_frames=76812
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
[37m[713.269579 0:1633 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE close02 : 0
Pass!
LTP MEMORY close02 after_run: free_frames=119336 allocated_frames=76833
LTP MEMORY close02 after_cleanup: free_frames=119336 allocated_frames=76833
LTP CASE RUNTIME close02: 2190 ms
========== END ltp close02 ==========
========== START ltp dup02 ==========
RUN LTP CASE dup02
LTP MEMORY dup02 before: free_frames=119336 allocated_frames=76833
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
[37m[715.436678 0:1637 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE dup02 : 0
Pass!
LTP MEMORY dup02 after_run: free_frames=119315 allocated_frames=76854
LTP MEMORY dup02 after_cleanup: free_frames=119315 allocated_frames=76854
LTP CASE RUNTIME dup02: 2197 ms
========== END ltp dup02 ==========
========== START ltp fcntl03 ==========
RUN LTP CASE fcntl03
LTP MEMORY fcntl03 before: free_frames=119315 allocated_frames=76854
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
fcntl03.c:32: TPASS: fcntl(fcntl03_1643, F_GETFD, 0) returned 0

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[717.907530 0:1641 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[717.909536 0:1641 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fcntl03 : 0
Pass!
LTP MEMORY fcntl03 after_run: free_frames=119294 allocated_frames=76875
LTP MEMORY fcntl03 after_cleanup: free_frames=119294 allocated_frames=76875
LTP CASE RUNTIME fcntl03: 2440 ms
========== END ltp fcntl03 ==========
========== START ltp getcwd01 ==========
RUN LTP CASE getcwd01
LTP MEMORY getcwd01 before: free_frames=119294 allocated_frames=76875
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
[37m[720.105467 0:1645 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getcwd01 : 0
Pass!
LTP MEMORY getcwd01 after_run: free_frames=119273 allocated_frames=76896
LTP MEMORY getcwd01 after_cleanup: free_frames=119273 allocated_frames=76896
LTP CASE RUNTIME getcwd01: 2217 ms
========== END ltp getcwd01 ==========
========== START ltp getpid02 ==========
RUN LTP CASE getpid02
LTP MEMORY getpid02 before: free_frames=119273 allocated_frames=76896
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getpid02.c:37: TPASS: child getppid() == parent getpid() (1651)
getpid02.c:50: TPASS: child getpid() == parent fork() (1652)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[722.292169 0:1649 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpid02 : 0
Pass!
LTP MEMORY getpid02 after_run: free_frames=119243 allocated_frames=76926
LTP MEMORY getpid02 after_cleanup: free_frames=119243 allocated_frames=76926
LTP CASE RUNTIME getpid02: 2160 ms
========== END ltp getpid02 ==========
========== START ltp getppid01 ==========
RUN LTP CASE getppid01
LTP MEMORY getppid01 before: free_frames=119243 allocated_frames=76926
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getppid01.c:31: TPASS: getppid() returned 1654

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[724.623295 0:1654 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getppid01 : 0
Pass!
LTP MEMORY getppid01 after_run: free_frames=119222 allocated_frames=76947
LTP MEMORY getppid01 after_cleanup: free_frames=119222 allocated_frames=76947
LTP CASE RUNTIME getppid01: 2371 ms
========== END ltp getppid01 ==========
========== START ltp getuid01 ==========
RUN LTP CASE getuid01
LTP MEMORY getuid01 before: free_frames=119222 allocated_frames=76947
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
[37m[727.090935 0:1658 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getuid01 : 0
Pass!
LTP MEMORY getuid01 after_run: free_frames=119201 allocated_frames=76968
LTP MEMORY getuid01 after_cleanup: free_frames=119201 allocated_frames=76968
LTP CASE RUNTIME getuid01: 2449 ms
========== END ltp getuid01 ==========
========== START ltp geteuid01 ==========
RUN LTP CASE geteuid01
LTP MEMORY geteuid01 before: free_frames=119201 allocated_frames=76968
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
[37m[729.395641 0:1662 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE geteuid01 : 0
Pass!
LTP MEMORY geteuid01 after_run: free_frames=119180 allocated_frames=76989
LTP MEMORY geteuid01 after_cleanup: free_frames=119180 allocated_frames=76989
LTP CASE RUNTIME geteuid01: 2295 ms
========== END ltp geteuid01 ==========
========== START ltp getgid01 ==========
RUN LTP CASE getgid01
LTP MEMORY getgid01 before: free_frames=119180 allocated_frames=76989
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
[37m[731.944682 0:1666 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getgid01 : 0
Pass!
LTP MEMORY getgid01 after_run: free_frames=119159 allocated_frames=77010
LTP MEMORY getgid01 after_cleanup: free_frames=119159 allocated_frames=77010
LTP CASE RUNTIME getgid01: 2532 ms
========== END ltp getgid01 ==========
========== START ltp getegid01 ==========
RUN LTP CASE getegid01
LTP MEMORY getegid01 before: free_frames=119159 allocated_frames=77010
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
[37m[734.253620 0:1670 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getegid01 : 0
Pass!
LTP MEMORY getegid01 after_run: free_frames=119138 allocated_frames=77031
LTP MEMORY getegid01 after_cleanup: free_frames=119138 allocated_frames=77031
LTP CASE RUNTIME getegid01: 2308 ms
========== END ltp getegid01 ==========
========== START ltp lseek01 ==========
RUN LTP CASE lseek01
LTP MEMORY lseek01 before: free_frames=119138 allocated_frames=77031
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
[37m[736.661761 0:1674 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[736.672010 0:1674 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE lseek01 : 0
Pass!
LTP MEMORY lseek01 after_run: free_frames=119117 allocated_frames=77052
LTP MEMORY lseek01 after_cleanup: free_frames=119117 allocated_frames=77052
LTP CASE RUNTIME lseek01: 2483 ms
========== END ltp lseek01 ==========
========== START ltp read02 ==========
RUN LTP CASE read02
LTP MEMORY read02 before: free_frames=119117 allocated_frames=77052
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
[37m[739.078612 0:1678 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[739.082338 0:1678 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE read02 : 0
Pass!
LTP MEMORY read02 after_run: free_frames=119096 allocated_frames=77073
LTP MEMORY read02 after_cleanup: free_frames=119096 allocated_frames=77073
LTP CASE RUNTIME read02: 2338 ms
========== END ltp read02 ==========
========== START ltp write02 ==========
RUN LTP CASE write02
LTP MEMORY write02 before: free_frames=119096 allocated_frames=77073
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
[37m[741.409175 0:1682 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[741.414473 0:1682 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE write02 : 0
Pass!
LTP MEMORY write02 after_run: free_frames=119075 allocated_frames=77094
LTP MEMORY write02 after_cleanup: free_frames=119075 allocated_frames=77094
LTP CASE RUNTIME write02: 2366 ms
========== END ltp write02 ==========
========== START ltp creat01 ==========
RUN LTP CASE creat01
LTP MEMORY creat01 before: free_frames=119075 allocated_frames=77094
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
[37m[743.582237 0:1686 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[743.584024 0:1686 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE creat01 : 0
Pass!
LTP MEMORY creat01 after_run: free_frames=119054 allocated_frames=77115
LTP MEMORY creat01 after_cleanup: free_frames=119054 allocated_frames=77115
LTP CASE RUNTIME creat01: 2134 ms
========== END ltp creat01 ==========
========== START ltp creat03 ==========
RUN LTP CASE creat03
LTP MEMORY creat03 before: free_frames=119054 allocated_frames=77115
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
[37m[745.950948 0:1690 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[745.952165 0:1690 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE creat03 : 0
Pass!
LTP MEMORY creat03 after_run: free_frames=119033 allocated_frames=77136
LTP MEMORY creat03 after_cleanup: free_frames=119033 allocated_frames=77136
LTP CASE RUNTIME creat03: 2358 ms
========== END ltp creat03 ==========
========== START ltp open02 ==========
RUN LTP CASE open02
LTP MEMORY open02 before: free_frames=119033 allocated_frames=77136
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
[37m[748.202382 0:1694 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[748.203528 0:1694 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE open02 : 0
Pass!
LTP MEMORY open02 after_run: free_frames=119012 allocated_frames=77157
LTP MEMORY open02 after_cleanup: free_frames=119012 allocated_frames=77157
LTP CASE RUNTIME open02: 2252 ms
========== END ltp open02 ==========
========== START ltp open03 ==========
RUN LTP CASE open03
LTP MEMORY open03 before: free_frames=119012 allocated_frames=77157
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
[37m[750.379779 0:1698 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE open03 : 0
Pass!
LTP MEMORY open03 after_run: free_frames=118991 allocated_frames=77178
LTP MEMORY open03 after_cleanup: free_frames=118991 allocated_frames=77178
LTP CASE RUNTIME open03: 2185 ms
========== END ltp open03 ==========
========== START ltp stat02 ==========
RUN LTP CASE stat02
LTP MEMORY stat02 before: free_frames=118991 allocated_frames=77178
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
[37m[752.856256 0:1702 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE stat02 : 0
Pass!
LTP MEMORY stat02 after_run: free_frames=118970 allocated_frames=77199
LTP MEMORY stat02 after_cleanup: free_frames=118970 allocated_frames=77199
LTP CASE RUNTIME stat02: 2466 ms
========== END ltp stat02 ==========
========== START ltp lstat01 ==========
RUN LTP CASE lstat01
LTP MEMORY lstat01 before: free_frames=118970 allocated_frames=77199
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
[37m[755.220787 0:1706 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[755.222397 0:1706 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE lstat01 : 0
Pass!
LTP MEMORY lstat01 after_run: free_frames=118949 allocated_frames=77220
LTP MEMORY lstat01 after_cleanup: free_frames=118949 allocated_frames=77220
LTP CASE RUNTIME lstat01: 2364 ms
========== END ltp lstat01 ==========
========== START ltp chmod01 ==========
RUN LTP CASE chmod01
LTP MEMORY chmod01 before: free_frames=118949 allocated_frames=77220
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
[37m[757.316122 0:1710 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[757.317955 0:1710 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[757.319708 0:1710 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE chmod01 : 0
Pass!
LTP MEMORY chmod01 after_run: free_frames=118919 allocated_frames=77250
LTP MEMORY chmod01 after_cleanup: free_frames=118919 allocated_frames=77250
LTP CASE RUNTIME chmod01: 2098 ms
========== END ltp chmod01 ==========
========== START ltp fchmod01 ==========
RUN LTP CASE fchmod01
LTP MEMORY fchmod01 before: free_frames=118919 allocated_frames=77250
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
[37m[759.475400 0:1717 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[759.480576 0:1717 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fchmod01 : 0
Pass!
LTP MEMORY fchmod01 after_run: free_frames=118898 allocated_frames=77271
LTP MEMORY fchmod01 after_cleanup: free_frames=118898 allocated_frames=77271
LTP CASE RUNTIME fchmod01: 2174 ms
========== END ltp fchmod01 ==========
========== START ltp rmdir01 ==========
RUN LTP CASE rmdir01
LTP MEMORY rmdir01 before: free_frames=118898 allocated_frames=77271
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
[37m[761.872104 0:1721 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE rmdir01 : 0
Pass!
LTP MEMORY rmdir01 after_run: free_frames=118877 allocated_frames=77292
LTP MEMORY rmdir01 after_cleanup: free_frames=118877 allocated_frames=77292
LTP CASE RUNTIME rmdir01: 2379 ms
========== END ltp rmdir01 ==========
========== START ltp symlink01 ==========
RUN LTP CASE symlink01
LTP MEMORY symlink01 before: free_frames=118877 allocated_frames=77292
[37m[764.130080 0:1725 axfs::root:433] [33m[AxError::IsADirectory][m
[msymlink01    1  TPASS  :  Creation of symbolic link file to no object file is ok
symlink01    2  TPASS  :  Creation of symbolic link file to no object file is ok
symlink01    3  TPASS  :  Creation of symbolic link file and object file via symbolic link is ok
symlink01    4  TPASS  :  Creating an existing symbolic link file error is caught
symlink01    5  TPASS  :  Creating a symbolic link which exceeds maximum pathname error is caught
PASS LTP CASE symlink01 : 0
Pass!
LTP MEMORY symlink01 after_run: free_frames=118865 allocated_frames=77304
LTP MEMORY symlink01 after_cleanup: free_frames=118865 allocated_frames=77304
LTP CASE RUNTIME symlink01: 2246 ms
========== END ltp symlink01 ==========
========== START ltp readlink01 ==========
RUN LTP CASE readlink01
LTP MEMORY readlink01 before: free_frames=118865 allocated_frames=77304
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
[37m[766.525050 0:1726 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[766.533313 0:1726 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE readlink01 : 0
Pass!
LTP MEMORY readlink01 after_run: free_frames=118835 allocated_frames=77334
LTP MEMORY readlink01 after_cleanup: free_frames=118835 allocated_frames=77334
LTP CASE RUNTIME readlink01: 2466 ms
========== END ltp readlink01 ==========
========== START ltp ftruncate01 ==========
RUN LTP CASE ftruncate01
LTP MEMORY ftruncate01 before: free_frames=118835 allocated_frames=77334
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
[37m[768.999262 0:1731 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[769.009030 0:1731 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE ftruncate01 : 0
Pass!
LTP MEMORY ftruncate01 after_run: free_frames=118814 allocated_frames=77355
LTP MEMORY ftruncate01 after_cleanup: free_frames=118814 allocated_frames=77355
LTP CASE RUNTIME ftruncate01: 2413 ms
========== END ltp ftruncate01 ==========
========== START ltp umask01 ==========
RUN LTP CASE umask01
LTP MEMORY umask01 before: free_frames=118814 allocated_frames=77355
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
[37m[771.318493 0:1735 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE umask01 : 0
Pass!
LTP MEMORY umask01 after_run: free_frames=118793 allocated_frames=77376
LTP MEMORY umask01 after_cleanup: free_frames=118793 allocated_frames=77376
LTP CASE RUNTIME umask01: 2299 ms
========== END ltp umask01 ==========
========== START ltp alarm02 ==========
RUN LTP CASE alarm02
LTP MEMORY alarm02 before: free_frames=118793 allocated_frames=77376
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
[37m[773.873619 0:1739 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE alarm02 : 0
Pass!
LTP MEMORY alarm02 after_run: free_frames=118772 allocated_frames=77397
LTP MEMORY alarm02 after_cleanup: free_frames=118772 allocated_frames=77397
LTP CASE RUNTIME alarm02: 2555 ms
========== END ltp alarm02 ==========
========== START ltp alarm03 ==========
RUN LTP CASE alarm03
LTP MEMORY alarm03 before: free_frames=118772 allocated_frames=77397
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
[37m[776.192261 0:1746 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE alarm03 : 0
Pass!
LTP MEMORY alarm03 after_run: free_frames=118742 allocated_frames=77427
LTP MEMORY alarm03 after_cleanup: free_frames=118742 allocated_frames=77427
LTP CASE RUNTIME alarm03: 2317 ms
========== END ltp alarm03 ==========
========== START ltp clock_gettime02 ==========
RUN LTP CASE clock_gettime02
LTP MEMORY clock_gettime02 before: free_frames=118742 allocated_frames=77427
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
[37m[778.507611 0:1752 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE clock_gettime02 : 0
Pass!
LTP MEMORY clock_gettime02 after_run: free_frames=118721 allocated_frames=77448
LTP MEMORY clock_gettime02 after_cleanup: free_frames=118721 allocated_frames=77448
LTP CASE RUNTIME clock_gettime02: 2374 ms
========== END ltp clock_gettime02 ==========
========== START ltp gettimeofday01 ==========
RUN LTP CASE gettimeofday01
LTP MEMORY gettimeofday01 before: free_frames=118721 allocated_frames=77448
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
[37m[780.900251 0:1756 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE gettimeofday01 : 0
Pass!
LTP MEMORY gettimeofday01 after_run: free_frames=118700 allocated_frames=77469
LTP MEMORY gettimeofday01 after_cleanup: free_frames=118700 allocated_frames=77469
LTP CASE RUNTIME gettimeofday01: 2321 ms
========== END ltp gettimeofday01 ==========
========== START ltp time01 ==========
RUN LTP CASE time01
LTP MEMORY time01 before: free_frames=118700 allocated_frames=77469
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
time01.c:36: TPASS: time() returned value 783
time01.c:38: TPASS: time() returned value 783, stored value 783 are same

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[783.139702 0:1760 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE time01 : 0
Pass!
LTP MEMORY time01 after_run: free_frames=118679 allocated_frames=77490
LTP MEMORY time01 after_cleanup: free_frames=118679 allocated_frames=77490
LTP CASE RUNTIME time01: 2240 ms
========== END ltp time01 ==========
========== START ltp times01 ==========
RUN LTP CASE times01
LTP MEMORY times01 before: free_frames=118679 allocated_frames=77490
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
times01.c:25: TPASS: times(&mytimes) returned 785405

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[785.437538 0:1764 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE times01 : 0
Pass!
LTP MEMORY times01 after_run: free_frames=118658 allocated_frames=77511
LTP MEMORY times01 after_cleanup: free_frames=118658 allocated_frames=77511
LTP CASE RUNTIME times01: 2346 ms
========== END ltp times01 ==========
========== START ltp kill03 ==========
RUN LTP CASE kill03
LTP MEMORY kill03 before: free_frames=118658 allocated_frames=77511
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
[37m[787.945390 0:1768 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE kill03 : 0
Pass!
LTP MEMORY kill03 after_run: free_frames=118637 allocated_frames=77532
LTP MEMORY kill03 after_cleanup: free_frames=118637 allocated_frames=77532
LTP CASE RUNTIME kill03: 2467 ms
========== END ltp kill03 ==========
========== START ltp rt_sigaction01 ==========
RUN LTP CASE rt_sigaction01
LTP MEMORY rt_sigaction01 before: free_frames=118637 allocated_frames=77532
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
rt_sigaction01    0  TINFO  : [37m[790.276327 0:1772 axfs::root:433] [33m[AxError::IsADirectory][m
[m sa.sa_flags = SA_NOMASK 
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 64
PASS LTP CASE rt_sigaction01 : 0
Pass!
LTP MEMORY rt_sigaction01 after_run: free_frames=118625 allocated_frames=77544
LTP MEMORY rt_sigaction01 after_cleanup: free_frames=118625 allocated_frames=77544
LTP CASE RUNTIME rt_sigaction01: 2316 ms
========== END ltp rt_sigaction01 ==========
========== START ltp sigaction01 ==========
RUN LTP CASE sigaction01
LTP MEMORY sigaction01 before: free_frames=118625 allocated_frames=77544
sigaction01    1  TPASS  :  SA_RESETHAND did not cause SA_SIGINFO to be cleared
sigaction01    2  TPASS  :  SA_RESETHAND was masked when handler executed
sigaction01    3  TPASS  :  sig has been masked because sa_mask originally contained sig
sigaction01    4  TPASS  :  siginfo pointer non NULL
PASS LTP CASE sigaction01 : 0
Pass!
LTP MEMORY sigaction01 after_run: free_frames=118613 allocated_frames=77556
LTP MEMORY sigaction01 after_cleanup: free_frames=118613 allocated_frames=77556
LTP CASE RUNTIME sigaction01: 2484 ms
========== END ltp sigaction01 ==========
========== START ltp proc01 ==========
RUN LTP CASE proc01
LTP MEMORY proc01 before: free_frames=118613 allocated_frames=77556
[37m[795.033361 0:1774 axfs::root:433] [33m[AxError::IsADirectory][m
[mproc01      1  TPASS  :  readproc() completed successfully, total read: 875 bytes, 20 objs
PASS LTP CASE proc01 : 0
Pass!
LTP MEMORY proc01 after_run: free_frames=118601 allocated_frames=77568
LTP MEMORY proc01 after_cleanup: free_frames=118601 allocated_frames=77568
LTP CASE RUNTIME proc01: 2290 ms
========== END ltp proc01 ==========
========== START ltp exit01 ==========
RUN LTP CASE exit01
LTP MEMORY exit01 before: free_frames=118601 allocated_frames=77568
exit01      1  TPASS  :  exit() test PASSED
PASS LTP CASE exit01 : 0
Pass!
LTP MEMORY exit01 after_run: free_frames=118580 allocated_frames=77589
LTP MEMORY exit01 after_cleanup: free_frames=118580 allocated_frames=77589
LTP CASE RUNTIME exit01: 2110 ms
========== END ltp exit01 ==========
========== START ltp exit02 ==========
RUN LTP CASE exit02
LTP MEMORY exit02 before: free_frames=118580 allocated_frames=77589
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
[37m[799.354245 0:1777 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE exit02 : 0
Pass!
LTP MEMORY exit02 after_run: free_frames=118550 allocated_frames=77619
LTP MEMORY exit02 after_cleanup: free_frames=118550 allocated_frames=77619
LTP CASE RUNTIME exit02: 2202 ms
========== END ltp exit02 ==========
========== START ltp exit_group01 ==========
RUN LTP CASE exit_group01
LTP MEMORY exit_group01 before: free_frames=118550 allocated_frames=77619
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
[37m[802.058648 0:1783 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[802.060932 0:1783 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE exit_group01 : 0
Pass!
LTP MEMORY exit_group01 after_run: free_frames=118518 allocated_frames=77651
LTP MEMORY exit_group01 after_cleanup: free_frames=118518 allocated_frames=77651
LTP CASE RUNTIME exit_group01: 2695 ms
========== END ltp exit_group01 ==========
========== START ltp getpgrp01 ==========
RUN LTP CASE getpgrp01
LTP MEMORY getpgrp01 before: free_frames=118518 allocated_frames=77651
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getpgrp01.c:18: TPASS: getpgrp() returned pid 1792
getpgrp01.c:19: TPASS: TST_RET == SAFE_GETPGID(0) (1792)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[804.138028 0:1790 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpgrp01 : 0
Pass!
LTP MEMORY getpgrp01 after_run: free_frames=118497 allocated_frames=77672
LTP MEMORY getpgrp01 after_cleanup: free_frames=118497 allocated_frames=77672
LTP CASE RUNTIME getpgrp01: 2074 ms
========== END ltp getpgrp01 ==========
========== START ltp gettid01 ==========
RUN LTP CASE gettid01
LTP MEMORY gettid01 before: free_frames=118497 allocated_frames=77672
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
gettid01.c:26: TPASS: tst_syscall(__NR_gettid) == tst_syscall(__NR_getpid) (1796)
gettid01.c:27: TPASS: tst_syscall(__NR_gettid) == pid (1796)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[806.317400 0:1794 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE gettid01 : 0
Pass!
LTP MEMORY gettid01 after_run: free_frames=118476 allocated_frames=77693
LTP MEMORY gettid01 after_cleanup: free_frames=118476 allocated_frames=77693
LTP CASE RUNTIME gettid01: 2177 ms
========== END ltp gettid01 ==========
========== START ltp uname01 ==========
RUN LTP CASE uname01
LTP MEMORY uname01 before: free_frames=118476 allocated_frames=77693
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
[37m[808.523678 0:1798 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE uname01 : 0
Pass!
LTP MEMORY uname01 after_run: free_frames=118455 allocated_frames=77714
LTP MEMORY uname01 after_cleanup: free_frames=118455 allocated_frames=77714
LTP CASE RUNTIME uname01: 2272 ms
========== END ltp uname01 ==========
========== START ltp getrlimit01 ==========
RUN LTP CASE getrlimit01
LTP MEMORY getrlimit01 before: free_frames=118455 allocated_frames=77714
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
[37m[811.016706 0:1802 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getrlimit01 : 0
Pass!
LTP MEMORY getrlimit01 after_run: free_frames=118434 allocated_frames=77735
LTP MEMORY getrlimit01 after_cleanup: free_frames=118434 allocated_frames=77735
LTP CASE RUNTIME getrlimit01: 2422 ms
========== END ltp getrlimit01 ==========
========== START ltp getrusage01 ==========
RUN LTP CASE getrusage01
LTP MEMORY getrusage01 before: free_frames=118434 allocated_frames=77735
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
[37m[813.224597 0:1806 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getrusage01 : 0
Pass!
LTP MEMORY getrusage01 after_run: free_frames=118413 allocated_frames=77756
LTP MEMORY getrusage01 after_cleanup: free_frames=118413 allocated_frames=77756
LTP CASE RUNTIME getrusage01: 2202 ms
========== END ltp getrusage01 ==========
========== START ltp sched_yield01 ==========
RUN LTP CASE sched_yield01
LTP MEMORY sched_yield01 before: free_frames=118413 allocated_frames=77756
sched_yield01    1  TPASS  :  sched_yield() call succeeded
PASS LTP CASE sched_yield01 : 0
Pass!
LTP MEMORY sched_yield01 after_run: free_frames=118401 allocated_frames=77768
LTP MEMORY sched_yield01 after_cleanup: free_frames=118401 allocated_frames=77768
LTP CASE RUNTIME sched_yield01: 2080 ms
========== END ltp sched_yield01 ==========
========== START ltp getpgid02 ==========
RUN LTP CASE getpgid02
LTP MEMORY getpgid02 before: free_frames=118401 allocated_frames=77768
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getpgid02.c:27: TPASS: getpgid(-99) : ESRCH (3)
getpgid02.c:28: TPASS: getpgid(4194304) : ESRCH (3)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[817.680438 0:1811 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpgid02 : 0
Pass!
LTP MEMORY getpgid02 after_run: free_frames=118380 allocated_frames=77789
LTP MEMORY getpgid02 after_cleanup: free_frames=118380 allocated_frames=77789
LTP CASE RUNTIME getpgid02: 2409 ms
========== END ltp getpgid02 ==========
========== START ltp getsid02 ==========
RUN LTP CASE getsid02
LTP MEMORY getsid02 before: free_frames=118380 allocated_frames=77789
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getsid02.c:22: TPASS: getsid(unused_pid) : ESRCH (3)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[820.049932 0:1815 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getsid02 : 0
Pass!
LTP MEMORY getsid02 after_run: free_frames=118359 allocated_frames=77810
LTP MEMORY getsid02 after_cleanup: free_frames=118359 allocated_frames=77810
LTP CASE RUNTIME getsid02: 2334 ms
========== END ltp getsid02 ==========
========== START ltp getppid02 ==========
RUN LTP CASE getppid02
LTP MEMORY getppid02 before: free_frames=118359 allocated_frames=77810
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getppid02.c:31: TPASS: getppid() returned parent pid (1821)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[822.199026 0:1819 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getppid02 : 0
Pass!
LTP MEMORY getppid02 after_run: free_frames=118329 allocated_frames=77840
LTP MEMORY getppid02 after_cleanup: free_frames=118329 allocated_frames=77840
LTP CASE RUNTIME getppid02: 2144 ms
========== END ltp getppid02 ==========
========== START ltp getuid03 ==========
RUN LTP CASE getuid03
LTP MEMORY getuid03 before: free_frames=118329 allocated_frames=77840
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getuid03.c:20: TPASS: getuid() returned 0
getuid03.c:32: TPASS: getuid() ret == /proc/self/status Uid: 0

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[824.553520 0:1824 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getuid03 : 0
Pass!
LTP MEMORY getuid03 after_run: free_frames=118308 allocated_frames=77861
LTP MEMORY getuid03 after_cleanup: free_frames=118308 allocated_frames=77861
LTP CASE RUNTIME getuid03: 2388 ms
========== END ltp getuid03 ==========
========== START ltp geteuid02 ==========
RUN LTP CASE geteuid02
LTP MEMORY geteuid02 before: free_frames=118308 allocated_frames=77861
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
geteuid02.c:21: TPASS: geteuid() returned 0
geteuid02.c:29: TPASS: Expect: geteuid() ret 0 == /proc/self/status EUID: 0

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[827.004782 0:1828 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE geteuid02 : 0
Pass!
LTP MEMORY geteuid02 after_run: free_frames=118287 allocated_frames=77882
LTP MEMORY geteuid02 after_cleanup: free_frames=118287 allocated_frames=77882
LTP CASE RUNTIME geteuid02: 2418 ms
========== END ltp geteuid02 ==========
========== START ltp getgid03 ==========
RUN LTP CASE getgid03
LTP MEMORY getgid03 before: free_frames=118287 allocated_frames=77882
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getgid03.c:43: TPASS: values from getgid() and getpwuid() match

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[829.258046 0:1832 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getgid03 : 0
Pass!
LTP MEMORY getgid03 after_run: free_frames=118266 allocated_frames=77903
LTP MEMORY getgid03 after_cleanup: free_frames=118266 allocated_frames=77903
LTP CASE RUNTIME getgid03: 2252 ms
========== END ltp getgid03 ==========
========== START ltp getegid02 ==========
RUN LTP CASE getegid02
LTP MEMORY getegid02 before: free_frames=118266 allocated_frames=77903
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getegid02.c:34: TPASS: pwent->pw_gid == egid (0)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[831.622556 0:1836 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getegid02 : 0
Pass!
LTP MEMORY getegid02 after_run: free_frames=118245 allocated_frames=77924
LTP MEMORY getegid02 after_cleanup: free_frames=118245 allocated_frames=77924
LTP CASE RUNTIME getegid02: 2416 ms
========== END ltp getegid02 ==========
========== START ltp getgroups03 ==========
RUN LTP CASE getgroups03
LTP MEMORY getgroups03 before: free_frames=118245 allocated_frames=77924
getgroups03    1  TPASS  :  getgroups functionality correct
PASS LTP CASE getgroups03 : 0
Pass!
LTP MEMORY getgroups03 after_run: free_frames=118233 allocated_frames=77936
LTP MEMORY getgroups03 after_cleanup: free_frames=118233 allocated_frames=77936
LTP CASE RUNTIME getgroups03: 2219 ms
========== END ltp getgroups03 ==========
========== START ltp uname02 ==========
RUN LTP CASE uname02
LTP MEMORY uname02 before: free_frames=118233 allocated_frames=77936
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
uname02.c:19: TPASS: uname(bad_addr) : EFAULT (14)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[836.005499 0:1841 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE uname02 : 0
Pass!
LTP MEMORY uname02 after_run: free_frames=118212 allocated_frames=77957
LTP MEMORY uname02 after_cleanup: free_frames=118212 allocated_frames=77957
LTP CASE RUNTIME uname02: 2098 ms
========== END ltp uname02 ==========
========== START ltp wait01 ==========
RUN LTP CASE wait01
LTP MEMORY wait01 before: free_frames=118212 allocated_frames=77957
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
wait01.c:19: TPASS: wait(NULL) : ECHILD (10)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[838.339181 0:1845 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE wait01 : 0
Pass!
LTP MEMORY wait01 after_run: free_frames=118191 allocated_frames=77978
LTP MEMORY wait01 after_cleanup: free_frames=118191 allocated_frames=77978
LTP CASE RUNTIME wait01: 2342 ms
========== END ltp wait01 ==========
========== START ltp wait02 ==========
RUN LTP CASE wait02
LTP MEMORY wait02 before: free_frames=118191 allocated_frames=77978
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
wait02.c:41: TPASS: wait() succeeded

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[840.734976 0:1849 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE wait02 : 0
Pass!
LTP MEMORY wait02 after_run: free_frames=118161 allocated_frames=78008
LTP MEMORY wait02 after_cleanup: free_frames=118161 allocated_frames=78008
LTP CASE RUNTIME wait02: 2420 ms
========== END ltp wait02 ==========
========== START ltp getrlimit02 ==========
RUN LTP CASE getrlimit02
LTP MEMORY getrlimit02 before: free_frames=118161 allocated_frames=78008
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getrlimit02.c:40: TPASS: getrlimit() with invalid address : EFAULT (14)
getrlimit02.c:40: TPASS: getrlimit() with invalid resource type : EINVAL (22)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[842.952161 0:1854 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getrlimit02 : 0
Pass!
LTP MEMORY getrlimit02 after_run: free_frames=118140 allocated_frames=78029
LTP MEMORY getrlimit02 after_cleanup: free_frames=118140 allocated_frames=78029
LTP CASE RUNTIME getrlimit02: 2175 ms
========== END ltp getrlimit02 ==========
ltp cases: 75 passed, 0 failed, 0 timed out
#### OS COMP TEST GROUP END ltp-glibc ####
#### OS COMP TEST GROUP START libcbench-musl ####
b_malloc_sparse (0)
  time: 1.773123110, virt: 0, res: 0, dirty: 0

b_malloc_bubble (0)
  time: 1.988687280, virt: 0, res: 0, dirty: 0

b_malloc_tiny1 (0)
  time: 0.041027210, virt: 0, res: 0, dirty: 0

b_malloc_tiny2 (0)
  time: 0.035993520, virt: 0, res: 0, dirty: 0

b_malloc_big1 (0)
  time: 0.782109570, virt: 0, res: 0, dirty: 0

b_malloc_big2 (0)
  time: 0.730093560, virt: 0, res: 0, dirty: 0

b_malloc_thread_stress (0)
  time: 0.169253900, virt: 0, res: 0, dirty: 0

b_malloc_thread_local (0)
  time: 0.117950420, virt: 0, res: 0, dirty: 0

b_string_strstr ("abcdefghijklmnopqrstuvwxyz")
  time: 0.050058750, virt: 0, res: 0, dirty: 0

b_string_strstr ("azbycxdwevfugthsirjqkplomn")
  time: 0.107754130, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaacccccccccccc")
  time: 0.176478450, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.113832640, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.058611550, virt: 0, res: 0, dirty: 0

b_string_memset (0)
  time: 0.019096460, virt: 0, res: 0, dirty: 0

b_string_strchr (0)
  time: 0.036541630, virt: 0, res: 0, dirty: 0

b_string_strlen (0)
  time: 0.030491190, virt: 0, res: 0, dirty: 0

b_pthread_createjoin_serial1 (0)
  time: 0.005716690, virt: 0, res: 0, dirty: 0

b_pthread_createjoin_serial2 (0)
  time: 1.162172230, virt: 0, res: 0, dirty: 0

b_pthread_create_serial1 (0)
  time: 0.001669690, virt: 0, res: 0, dirty: 0

b_pthread_uselesslock (0)
  time: 0.123423370, virt: 0, res: 0, dirty: 0

b_utf8_bigbuf (0)
  time: 0.161779780, virt: 0, res: 0, dirty: 0

b_utf8_onebyone (0)
  time: 0.389575270, virt: 0, res: 0, dirty: 0

b_stdio_putcgetc (0)
  time: 0.912205500, virt: 0, res: 0, dirty: 0

b_stdio_putcgetc_unlocked (0)
  time: 0.415810130, virt: 0, res: 0, dirty: 0

b_regex_compile ("(a|b|c)*d*b")
  time: 1.030967160, virt: 0, res: 0, dirty: 0

b_regex_search ("(a|b|c)*d*b")
  time: 0.662555720, virt: 0, res: 0, dirty: 0

b_regex_search ("a{25}b")
  time: 0.652871690, virt: 0, res: 0, dirty: 0

#### OS COMP TEST GROUP END libcbench-musl ####
#### OS COMP TEST GROUP START libcbench-glibc ####
b_malloc_sparse (0)
  time: 1.726727060, virt: 0, res: 0, dirty: 0

b_malloc_bubble (0)
  time: 1.499599610, virt: 0, res: 0, dirty: 0

b_malloc_tiny1 (0)
  time: 0.036458570, virt: 0, res: 0, dirty: 0

b_malloc_tiny2 (0)
  time: 0.023041550, virt: 0, res: 0, dirty: 0

b_malloc_big1 (0)
  time: 0.233598210, virt: 0, res: 0, dirty: 0

b_malloc_big2 (0)
  time: 0.441886450, virt: 0, res: 0, dirty: 0

The futex facility returned an unexpected error code.
The futex facility returned an unexpected error code.
b_string_strstr ("abcdefghijklmnopqrstuvwxyz")
  time: 0.024429140, virt: 0, res: 0, dirty: 0

b_string_strstr ("azbycxdwevfugthsirjqkplomn")
  time: 0.022738600, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaacccccccccccc")
  time: 0.053462620, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.037110450, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.042697130, virt: 0, res: 0, dirty: 0

b_string_memset (0)
  time: 0.042589330, virt: 0, res: 0, dirty: 0

b_string_strchr (0)
  time: 0.033009210, virt: 0, res: 0, dirty: 0

b_string_strlen (0)
  time: 0.028761060, virt: 0, res: 0, dirty: 0

The futex facility returned an unexpected error code.
b_pthread_create_serial1 (0)
  time: 0.004972120, virt: 0, res: 0, dirty: 0

The futex facility returned an unexpected error code.
b_utf8_bigbuf (0)
  time: 0.183502850, virt: 0, res: 0, dirty: 0

b_utf8_onebyone (0)
  time: 0.164477240, virt: 0, res: 0, dirty: 0

b_regex_compile ("(a|b|c)*d*b")
  time: 0.046258800, virt: 0, res: 0, dirty: 0

b_regex_search ("(a|b|c)*d*b")
  time: 0.015374020, virt: 0, res: 0, dirty: 0

b_regex_search ("a{25}b")
  time: 0.251655000, virt: 0, res: 0, dirty: 0

#### OS COMP TEST GROUP END libcbench-glibc ####
#### OS COMP TEST GROUP START iperf-musl ####
====== iperf BASIC_UDP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 0.0.0.0 port 49152 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  5]   0.00-2.00   sec  2.08 MBytes  8.70 Mbits/sec  1492  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.00   sec  2.08 MBytes  8.70 Mbits/sec  0.000 ms  0/1492 (0%)  sender
[  5]   0.00-2.01   sec  2.08 MBytes  8.67 Mbits/sec  0.954 ms  0/1492 (0%)  receiver

iperf Done.
====== iperf BASIC_UDP end: success ======

====== iperf BASIC_TCP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 127.0.0.1 port 49154 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.01   sec  52.8 MBytes   221 Mbits/sec    0   0.00 Bytes       
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.01   sec  52.8 MBytes   221 Mbits/sec    0             sender
[  5]   0.00-2.01   sec  51.9 MBytes   217 Mbits/sec                  receiver

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
[  5]   0.00-2.00   sec   573 KBytes  2.34 Mbits/sec  402  
[  7]   0.00-2.00   sec   573 KBytes  2.34 Mbits/sec  402  
[  9]   0.00-2.00   sec   573 KBytes  2.34 Mbits/sec  402  
[ 11]   0.00-2.00   sec   573 KBytes  2.34 Mbits/sec  402  
[ 13]   0.00-2.00   sec   573 KBytes  2.34 Mbits/sec  402  
[SUM]   0.00-2.00   sec  2.80 MBytes  11.7 Mbits/sec  2010  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.00   sec   573 KBytes  2.34 Mbits/sec  0.000 ms  0/402 (0%)  sender
[  5]   0.00-2.00   sec   573 KBytes  2.34 Mbits/sec  0.093 ms  0/402 (0%)  receiver
[  7]   0.00-2.00   sec   573 KBytes  2.34 Mbits/sec  0.000 ms  0/402 (0%)  sender
[  7]   0.00-2.00   sec   573 KBytes  2.34 Mbits/sec  0.102 ms  0/402 (0%)  receiver
[  9]   0.00-2.00   sec   573 KBytes  2.34 Mbits/sec  0.000 ms  0/402 (0%)  sender
[  9]   0.00-2.00   sec   573 KBytes  2.34 Mbits/sec  0.142 ms  0/402 (0%)  receiver
[ 11]   0.00-2.00   sec   573 KBytes  2.34 Mbits/sec  0.000 ms  0/402 (0%)  sender
[ 11]   0.00-2.00   sec   573 KBytes  2.34 Mbits/sec  0.125 ms  0/402 (0%)  receiver
[ 13]   0.00-2.00   sec   573 KBytes  2.34 Mbits/sec  0.000 ms  0/402 (0%)  sender
[ 13]   0.00-2.00   sec   573 KBytes  2.34 Mbits/sec  0.192 ms  0/402 (0%)  receiver
[SUM]   0.00-2.00   sec  2.80 MBytes  11.7 Mbits/sec  0.000 ms  0/2010 (0%)  sender
[SUM]   0.00-2.00   sec  2.80 MBytes  11.7 Mbits/sec  0.131 ms  0/2010 (0%)  receiver

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
[  5]   0.00-2.01   sec  12.5 MBytes  52.2 Mbits/sec    0   0.00 Bytes       
[  7]   0.00-2.01   sec  12.5 MBytes  52.2 Mbits/sec    0   0.00 Bytes       
[  9]   0.00-2.01   sec  12.5 MBytes  52.1 Mbits/sec    0   0.00 Bytes       
[ 11]   0.00-2.01   sec  12.5 MBytes  52.1 Mbits/sec    0   0.00 Bytes       
[ 13]   0.00-2.01   sec  12.5 MBytes  52.1 Mbits/sec    0   0.00 Bytes       
[SUM]   0.00-2.01   sec  62.5 MBytes   261 Mbits/sec    0             
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.01   sec  12.5 MBytes  52.2 Mbits/sec    0             sender
[  5]   0.00-2.05   sec  11.6 MBytes  47.7 Mbits/sec                  receiver
[  7]   0.00-2.01   sec  12.5 MBytes  52.2 Mbits/sec    0             sender
[  7]   0.00-2.05   sec  11.6 MBytes  47.7 Mbits/sec                  receiver
[  9]   0.00-2.01   sec  12.5 MBytes  52.2 Mbits/sec    0             sender
[  9]   0.00-2.05   sec  11.6 MBytes  47.7 Mbits/sec                  receiver
[ 11]   0.00-2.01   sec  12.5 MBytes  52.2 Mbits/sec    0             sender
[ 11]   0.00-2.05   sec  11.6 MBytes  47.7 Mbits/sec                  receiver
[ 13]   0.00-2.01   sec  12.5 MBytes  52.2 Mbits/sec    0             sender
[ 13]   0.00-2.05   sec  11.6 MBytes  47.7 Mbits/sec                  receiver
[SUM]   0.00-2.01   sec  62.5 MBytes   261 Mbits/sec    0             sender
[SUM]   0.00-2.05   sec  58.1 MBytes   238 Mbits/sec                  receiver

iperf Done.
====== iperf PARALLEL_TCP end: success ======

====== iperf REVERSE_UDP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 0.0.0.0 port 49158 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.00   sec  2.13 MBytes  8.91 Mbits/sec  0.236 ms  0/1527 (0%)  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.01   sec  2.13 MBytes  8.90 Mbits/sec  0.000 ms  0/1528 (0%)  sender
[  5]   0.00-2.00   sec  2.13 MBytes  8.91 Mbits/sec  0.236 ms  0/1527 (0%)  receiver

iperf Done.
====== iperf REVERSE_UDP end: success ======

====== iperf REVERSE_TCP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 127.0.0.1 port 49164 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.00   sec  46.6 MBytes   195 Mbits/sec                  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.01   sec  47.5 MBytes   198 Mbits/sec    0             sender
[  5]   0.00-2.00   sec  46.6 MBytes   195 Mbits/sec                  receiver

iperf Done.
====== iperf REVERSE_TCP end: success ======

#### OS COMP TEST GROUP END iperf-musl ####
#### OS COMP TEST GROUP START iperf-glibc ####
====== iperf BASIC_UDP begin ======
iperf3: error - control socket has closed unexpectedly
====== iperf BASIC_UDP end: fail ======

====== iperf BASIC_TCP begin ======
[37m[901.085292 0:4518 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[901.086104 0:4518 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf BASIC_TCP end: fail ======

====== iperf PARALLEL_UDP begin ======
[37m[901.223009 0:4519 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[901.223731 0:4519 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf PARALLEL_UDP end: fail ======

====== iperf PARALLEL_TCP begin ======
[37m[901.375051 0:4520 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[901.375775 0:4520 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf PARALLEL_TCP end: fail ======

====== iperf REVERSE_UDP begin ======
[37m[901.523956 0:4521 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[901.524542 0:4521 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf REVERSE_UDP end: fail ======

====== iperf REVERSE_TCP begin ======
[37m[901.850610 0:4522 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[901.851192 0:4522 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf REVERSE_TCP end: fail ======

#### OS COMP TEST GROUP END iperf-glibc ####
#### OS COMP TEST GROUP START lmbench-musl ####
latency measurements
Simple syscall: 1.7297 microseconds
Simple read: 11.9798 microseconds
Simple write: 9.2958 microseconds
[37m[957.652057 0:4536 axfs::root:423] [33m[AxError::AlreadyExists][m
[m[37m[957.656776 0:4536 axfs::root:423] [33m[AxError::AlreadyExists][m
[mautorun: /tmp/testsuite/musl/lmbench/lmbench_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START lmbench-glibc ####
latency measurements
Simple syscall: 3.1764 microseconds
Simple read: 14.2071 microseconds
Simple write: 10.1503 microseconds
[37m[1023.769620 0:4551 axfs::root:423] [33m[AxError::AlreadyExists][m
[m[37m[1023.774433 0:4551 axfs::root:423] [33m[AxError::AlreadyExists][m
[m[37m[1023.778605 0:4551 axfs::root:423] [33m[AxError::AlreadyExists][m
[mSimple stat: 203.8462 microseconds
Simple fstat: 16.4281 microseconds
Simple open/close: 349.1429 microseconds
Select on 100 fd's: 257.4286 microseconds
autorun: /tmp/testsuite/glibc/lmbench/lmbench_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START netperf-musl ####
====== netperf UDP_STREAM begin ======
MIGRATED UDP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Starting netserver with host '127.0.0.1' port '12865' and family AF_UNSPEC
Socket  Message  Elapsed      Messages                
Size    Size     Time         Okay Errors   Throughput
bytes   bytes    secs            #      #   10^6bits/sec

 65536    1000   1.00         1201      0       9.58
 65536           1.00         1201              9.58

====== netperf UDP_STREAM end: success ======
====== netperf TCP_STREAM begin ======
MIGRATED TCP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Recv   Send    Send                          
Socket Socket  Message  Elapsed              
Size   Size    Size     Time     Throughput  
bytes  bytes   bytes    secs.    10^6bits/sec  

 65536  65536   1000    0.10       80.92   
====== netperf TCP_STREAM end: success ======
====== netperf UDP_RR begin ======
MIGRATED UDP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00      991.93   
65536  65536 
====== netperf UDP_RR end: success ======
====== netperf TCP_RR begin ======
MIGRATED TCP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00     1174.06   
65536  65536 
====== netperf TCP_RR end: success ======
====== netperf TCP_CRR begin ======
MIGRATED TCP Connect/Request/Response TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00      732.54   
65536  65536 
====== netperf TCP_CRR end: success ======
#### OS COMP TEST GROUP END netperf-musl ####
#### OS COMP TEST GROUP START netperf-glibc ####
====== netperf UDP_STREAM begin ======
Starting netserver with host '127.0.0.1' port '12865' and family AF_UNSPEC
MIGRATED UDP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET
enable_enobufs failed: getprotobyname
Socket  Message  Elapsed      Messages                
Size    Size     Time         Okay Errors   Throughput
bytes   bytes    secs            #      #   10^6bits/sec

 65536    1000   1.00         1323      0      10.54
 65536           1.00         1323             10.54

====== netperf UDP_STREAM end: success ======
====== netperf TCP_STREAM begin ======
MIGRATED TCP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET
Recv   Send    Send                          
Socket Socket  Message  Elapsed              
Size   Size    Size     Time     Throughput  
bytes  bytes   bytes    secs.    10^6bits/sec  

 65536  65536   1000    0.05      159.32   
====== netperf TCP_STREAM end: success ======
====== netperf UDP_RR begin ======
MIGRATED UDP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.01      947.91   
65536  65536 
====== netperf UDP_RR end: success ======
====== netperf TCP_RR begin ======
MIGRATED TCP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00     1151.32   
65536  65536 
====== netperf TCP_RR end: success ======
====== netperf TCP_CRR begin ======
MIGRATED TCP Connect/Request/Response TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00      774.05   
65536  65536 
====== netperf TCP_CRR end: success ======
#### OS COMP TEST GROUP END netperf-glibc ####
#### OS COMP TEST GROUP START cyclictest-musl ####
====== cyclictest NO_STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 4625) P:99 I:1000 C:    877 Min:      2 Act:   25 Avg:  177 Max:    9588
====== cyclictest NO_STRESS_P1 end: success ======
====== cyclictest NO_STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 4627) P:99 I:1000 C:    890 Min:      2 Act:   18 Avg:  160 Max:   16566
T: 1 ( 4628) P:99 I:1500 C:    602 Min:      2 Act: 1553 Avg:  218 Max:   16814
T: 2 ( 4629) P:99 I:2000 C:    458 Min:      2 Act: 1372 Avg:  262 Max:   16273
T: 3 ( 4630) P:99 I:2500 C:    371 Min:      2 Act: 1857 Avg:  287 Max:   14789
T: 4 ( 4631) P:99 I:3000 C:    311 Min:      2 Act:  347 Avg:  324 Max:   15246
T: 5 ( 4632) P:99 I:3500 C:    267 Min:      2 Act: 1834 Avg:  368 Max:   16706
T: 6 ( 4633) P:99 I:4000 C:    236 Min:      2 Act:  206 Avg:  410 Max:   16154
T: 7 ( 4634) P:99 I:4500 C:    210 Min:      2 Act:  249 Avg:  375 Max:   15112
====== cyclictest NO_STRESS_P8 end: success ======
====== start hackbench ======
Running in process mode with 10 groups using 40 file descriptors each (== 400 tasks)
Each sender will pass 100000000 messages of 100 bytes
====== cyclictest STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 5038) P:99 I:1000 C:      4 Min:     43 Act:317129 Avg:253078 Max:  437952
====== cyclictest STRESS_P1 end: success ======
====== cyclictest STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 5040) P:99 I:1000 C:      5 Min:    274 Act:411646 Avg:261582 Max:  411646
T: 1 ( 5041) P:99 I:1500 C:      4 Min: 205196 Act:413020 Avg:327286 Max:  413020
T: 2 ( 5042) P:99 I:2000 C:      4 Min: 204172 Act:411590 Avg:326164 Max:  411590
T: 3 ( 5043) P:99 I:2500 C:      4 Min: 203149 Act:411636 Avg:325913 Max:  411636
T: 4 ( 5044) P:99 I:3000 C:      4 Min: 205124 Act:411681 Avg:326159 Max:  411681
T: 5 ( 5045) P:99 I:3500 C:      4 Min: 205101 Act:412226 Avg:325782 Max:  412226
T: 6 ( 5046) P:99 I:4000 C:      4 Min: 202069 Act:411757 Avg:325144 Max:  411757
T: 7 ( 5047) P:99 I:4500 C:      4 Min: 205042 Act:411799 Avg:325388 Max:  411799
autorun: /tmp/testsuite/musl/cyclictest/cyclictest_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START cyclictest-glibc ####
====== cyclictest NO_STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 5053) P:99 I:1000 C:    826 Min:      2 Act:    4 Avg:  268 Max:   13273
====== cyclictest NO_STRESS_P1 end: success ======
====== cyclictest NO_STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 5055) P:99 I:1000 C:    822 Min:      2 Act:   19 Avg:  262 Max:   14836
T: 1 ( 5056) P:99 I:1500 C:    572 Min:      2 Act:  135 Avg:  326 Max:   15011
T: 2 ( 5057) P:99 I:2000 C:    432 Min:      2 Act:  101 Avg:  399 Max:   13911
T: 3 ( 5058) P:99 I:2500 C:    352 Min:      2 Act:   62 Avg:  511 Max:   14325
T: 4 ( 5059) P:99 I:3000 C:    303 Min:      2 Act:   16 Avg:  501 Max:   14734
T: 5 ( 5060) P:99 I:3500 C:    259 Min:      2 Act:  283 Avg:  537 Max:   14651
T: 6 ( 5061) P:99 I:4000 C:    233 Min:      2 Act:  204 Avg:  587 Max:   11683
T: 7 ( 5062) P:99 I:4500 C:    212 Min:      2 Act:  283 Avg:  525 Max:   12820
====== cyclictest NO_STRESS_P8 end: success ======
====== start hackbench ======
Running in process mode with 10 groups using 40 file descriptors each (== 400 tasks)
Each sender will pass 100000000 messages of 100 bytes
====== cyclictest STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 5466) P:99 I:1000 C:      4 Min: 207175 Act:279280 Avg:312811 Max:  557126
====== cyclictest STRESS_P1 end: success ======
====== cyclictest STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 5468) P:99 I:1000 C:      3 Min: 209079 Act:230087 Avg:341123 Max:  584204
T: 1 ( 5469) P:99 I:1500 C:      3 Min: 208632 Act:230046 Avg:340777 Max:  583655
T: 2 ( 5470) P:99 I:2000 C:      3 Min: 208103 Act:230557 Avg:340601 Max:  583144
T: 3 ( 5471) P:99 I:2500 C:      3 Min: 207569 Act:229054 Avg:339749 Max:  582626
T: 4 ( 5472) P:99 I:3000 C:      3 Min: 207037 Act:228555 Avg:339234 Max:  582112
T: 5 ( 5473) P:99 I:3500 C:      3 Min: 206511 Act:228551 Avg:338886 Max:  581598
T: 6 ( 5474) P:99 I:4000 C:      3 Min: 205979 Act:230544 Avg:339867 Max:  583078
T: 7 ( 5475) P:99 I:4500 C:      3 Min: 205451 Act:230042 Avg:339687 Max:  583568
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

	Run began: Thu Jan  1 00:20:11 1970

	Auto Mode
	Record Size 1 kB
	File size set to 4096 kB
	Command line used: ./iozone -a -r 1k -s 4m
	Output is in kBytes/sec
	Time Resolution = 0.000005 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
                                                                    random    random      bkwd     record     stride                                        
              kB  reclen    write    rewrite      read    reread      read     write      read    rewrite       read    fwrite  frewrite     fread   freread
            4096       1     63281     93315     61563     42032     44363     45323[37m[1212.644683 0:5481 axfs::fops:269] [33m[AxError::InvalidInput][m
[m[37m[1212.753951 0:5481 axfs::fops:269] [33m[AxError::InvalidInput][m
[m     24742      55633      56750     74649     86411     51999     53672

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

	Run began: Thu Jan  1 00:20:14 1970

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

	Children see throughput for  4 initial writers 	=   52266.23 kB/sec
	Parent sees throughput for  4 initial writers 	=    2572.57 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   52266.23 kB/sec
	Avg throughput per process 			=   13066.56 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=   65331.12 kB/sec
	Parent sees throughput for  4 rewriters 	=    1651.18 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   65331.12 kB/sec
	Avg throughput per process 			=   16332.78 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 readers 		=   55101.16 kB/sec
	Parent sees throughput for  4 readers 		=    1452.45 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   55101.16 kB/sec
	Avg throughput per process 			=   13775.29 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 re-readers 	=   61918.01 kB/sec
	Parent sees throughput for 4 re-readers 	=    1972.06 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   61918.01 kB/sec
	Avg throughput per process 			=   15479.50 kB/sec
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

	Run began: Thu Jan  1 00:20:32 1970

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

	Children see throughput for  4 initial writers 	=  105025.64 kB/sec
	Parent sees throughput for  4 initial writers 	=    1616.97 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  105025.64 kB/sec
	Avg throughput per process 			=   26256.41 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=  110463.86 kB/sec
	Parent sees throughput for  4 rewriters 	=    1468.48 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  110463.86 kB/sec
	Avg throughput per process 			=   27615.96 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 random readers 	=   81308.55 kB/sec
	Parent sees throughput for 4 random readers 	=    2249.08 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   81308.55 kB/sec
	Avg throughput per process 			=   20327.14 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 random writers 	=   86530.34 kB/sec
	Parent sees throughput for 4 random writers 	=    1772.27 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   86530.34 kB/sec
	Avg throughput per process 			=   21632.58 kB/sec
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

	Run began: Thu Jan  1 00:20:55 1970

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

	Children see throughput for  4 initial writers 	=   98319.73 kB/sec
	Parent sees throughput for  4 initial writers 	=    1544.86 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   98319.73 kB/sec
	Avg throughput per process 			=   24579.93 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=   92585.89 kB/sec
	Parent sees throughput for  4 rewriters 	=    2001.40 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   92585.89 kB/sec
	Avg throughput per process 			=   23146.47 kB/sec
	Min xfer 					=       0.00 kB
[37m[1265.092498 0:5536 axfs::fops:269] [33m[AxError::InvalidInput][m
[m
	Children see throughput for 4 reverse readers 	=   73315.67 kB/sec
	Parent sees throughput for 4 reverse readers 	=    1649.67 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   73315.67 kB/sec
	Avg throughput per process 			=   18328.92 kB/sec
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

	Run began: Thu Jan  1 00:21:20 1970

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
            4096       1     52576     84829     69733     70173     46497     41308[37m[1282.668481 0:5545 axfs::fops:269] [33m[AxError::InvalidInput][m
[m[37m[1282.773589 0:5545 axfs::fops:269] [33m[AxError::InvalidInput][m
[m     29995      45661      54116     59751     56561     45580     87191

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

	Run began: Thu Jan  1 00:21:25 1970

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

	Children see throughput for  4 initial writers 	=   54329.37 kB/sec
	Parent sees throughput for  4 initial writers 	=    1676.71 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   54329.37 kB/sec
	Avg throughput per process 			=   13582.34 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=  111778.19 kB/sec
	Parent sees throughput for  4 rewriters 	=    1800.29 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  111778.19 kB/sec
	Avg throughput per process 			=   27944.55 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 readers 		=  100718.01 kB/sec
	Parent sees throughput for  4 readers 		=    2327.53 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  100718.01 kB/sec
	Avg throughput per process 			=   25179.50 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 re-readers 	=  100956.33 kB/sec
	Parent sees throughput for 4 re-readers 	=    1966.59 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  100956.33 kB/sec
	Avg throughput per process 			=   25239.08 kB/sec
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

	Run began: Thu Jan  1 00:21:45 1970

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

	Children see throughput for  4 initial writers 	=   39037.78 kB/sec
	Parent sees throughput for  4 initial writers 	=    1653.36 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   39037.78 kB/sec
	Avg throughput per process 			=    9759.45 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=  118313.12 kB/sec
	Parent sees throughput for  4 rewriters 	=    1601.84 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  118313.12 kB/sec
	Avg throughput per process 			=   29578.28 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 random readers 	=   79639.14 kB/sec
	Parent sees throughput for 4 random readers 	=    1815.39 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   79639.14 kB/sec
	Avg throughput per process 			=   19909.79 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 random writers 	=   90571.38 kB/sec
	Parent sees throughput for 4 random writers 	=    1658.74 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   90571.38 kB/sec
	Avg throughput per process 			=   22642.84 kB/sec
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

	Run began: Thu Jan  1 00:22:09 1970

	Record Size 1 kB
	File size set to 1024 kB
	Command line used: ./iozone -t 4 -i 0 -i 3 -r 1k -s 1m
	Output is in kBytes/sec
	Time Resolution = 0.000004 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
	Throughput test with 4 processes
	Each process writes a 1024 kByte file in 1 kByte records

	Children see throughput for  4 initial writers 	=   62972.76 kB/sec
	Parent sees throughput for  4 initial writers 	=    2234.91 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   62972.76 kB/sec
	Avg throughput per process 			=   15743.19 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=  116429.80 kB/sec
	Parent sees throughput for  4 rewriters 	=    3134.43 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  116429.80 kB/sec
	Avg throughput per process 			=   29107.45 kB/sec
	Min xfer 					=       0.00 kB
autorun: /tmp/testsuite/glibc/iozone/iozone_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START unixbench-musl ####
Unixbench DHRY2 test(lps): 13704053
Unixbench WHETSTONE test(MFLOPS): 156.481
Unixbench SYSCALL test(lps): 460875
Unixbench CONTEXT test(lps): 12193
autorun: /tmp/testsuite/musl/unixbench/unixbench_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START unixbench-glibc ####
Unixbench DHRY2 test(lps): 20264994
Unixbench WHETSTONE test(MFLOPS): 156.042
Unixbench SYSCALL test(lps): 450905
autorun: /tmp/testsuite/glibc/unixbench/unixbench_testcode.sh timed out after 60s
[37m[1463.985838 0:2 axplat_loongarch64_qemu_virt::power:23] [32mShutting down...[m
[m