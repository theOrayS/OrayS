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
    Finished `release` profile [optimized] target(s) in 17.82s
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

[37m[  0.005265 0 axruntime:135] [32mLogging is enabled.[m
[m[37m[  0.006293 0 axruntime:136] [32mPrimary CPU 0 started, arg = 0x0.[m
[m[37m[  0.007417 0 axruntime:139] [32mFound physcial memory regions:[m
[m[37m[  0.013661 0 axruntime:141] [32m  [PA:0x100d0000, PA:0x100d1000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.015904 0 axruntime:141] [32m  [PA:0x100e0000, PA:0x100e1000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.020745 0 axruntime:141] [32m  [PA:0x1fe00000, PA:0x1fe01000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.029221 0 axruntime:141] [32m  [PA:0x20000000, PA:0x30000000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.036077 0 axruntime:141] [32m  [PA:0x40000000, PA:0x40020000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.042630 0 axruntime:141] [32m  [PA:0x80000000, PA:0x80118000) .text (READ | EXECUTE | RESERVED)[m
[m[37m[  0.047240 0 axruntime:141] [32m  [PA:0x80118000, PA:0x80142000) .rodata (READ | RESERVED)[m
[m[37m[  0.050798 0 axruntime:141] [32m  [PA:0x80142000, PA:0x80147000) .data .tdata .tbss .percpu (READ | WRITE | RESERVED)[m
[m[37m[  0.053032 0 axruntime:141] [32m  [PA:0x80147000, PA:0x80187000) boot stack (READ | WRITE | RESERVED)[m
[m[37m[  0.057769 0 axruntime:141] [32m  [PA:0x80187000, PA:0x801ae000) .bss (READ | WRITE | RESERVED)[m
[m[37m[  0.060575 0 axruntime:141] [32m  [PA:0x801ae000, PA:0xb0000000) free memory (READ | WRITE | FREE)[m
[m[37m[  0.066287 0 axruntime:216] [32mInitialize global memory allocator...[m
[m[37m[  0.067280 0 axruntime:217] [32m  use TLSF allocator.[m
[m[37m[  0.074307 0 axmm:103] [32mInitialize virtual memory management...[m
[m[37m[  0.135883 0 axruntime:156] [32mInitialize platform devices...[m
[msmp = 1
[37m[  0.139630 0 axtask::api:73] [32mInitialize scheduling...[m
[m[37m[  0.144029 0 axtask::api:83] [32m  use Round-robin scheduler.[m
[m[37m[  0.147370 0 axdriver:152] [32mInitialize device drivers...[m
[m[37m[  0.150561 0 axdriver:153] [32m  device model: static[m
[m[37m[  0.162711 0 virtio_drivers::device::blk:63] [32mfound a block device of size 4194304KB[m
[m[37m[  0.172480 0 axdriver::bus::pci:107] [32mregistered a new Block device at 00:01.0: "virtio-blk"[m
[m[37m[  0.189829 0 virtio_drivers::device::net::dev_raw:33] [32mnegotiated_features Features(MAC | STATUS | RING_INDIRECT_DESC | RING_EVENT_IDX | VERSION_1)[m
[m[37m[  0.206142 0 axdriver::bus::pci:107] [32mregistered a new Net device at 00:02.0: "virtio-net"[m
[m[37m[  0.288719 0 axfs:44] [32mInitialize filesystems...[m
[m[37m[  0.294091 0 axfs:47] [32m  use block device 0: "virtio-blk"[m
[m[37m[  0.300147 0 axfs::root:336] [32m  detected root filesystem: Ext4[m
[m[37m[  0.337040 0 axnet:42] [32mInitialize network subsystem...[m
[m[37m[  0.342888 0 axnet:45] [32m  use NIC 0: "virtio-net"[m
[m[37m[  0.352650 0 axnet::smoltcp_impl:335] [32mcreated net interface "eth0":[m
[m[37m[  0.360107 0 axnet::smoltcp_impl:336] [32m  ether:    52-54-00-12-34-56[m
[m[37m[  0.366965 0 axnet::smoltcp_impl:337] [32m  ip:       10.0.2.15/24[m
[m[37m[  0.372853 0 axnet::smoltcp_impl:338] [32m  gateway:  10.0.2.2[m
[m[37m[  0.373320 0 axruntime:182] [32mInitialize interrupt handlers...[m
[m[37m[  0.380730 0 axruntime:194] [32mPrimary CPU 0 init OK.[m
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
========== START entry-static.exe search_tsearch ==========
Pass!
========== END entry-static.exe search_tsearch ==========
========== START entry-static.exe setjmp ==========
Pass!
========== END entry-static.exe setjmp ==========
========== START entry-static.exe snprintf ==========
Pass!
========== END entry-static.exe snprintf ==========
========== START entry-static.exe socket ==========
[37m[113.475329 0:211 axnet::smoltcp_impl::udp:312] [33m[AxError::NotConnected] socket send() failed[m
[m[37m[113.476318 0:211 arceos_posix_api::imp::net:450] [32msys_sendto => Err(ENOTCONN)[m
[msrc/functional/socket.c:29: sendto(c, "x", 1, 0, (void *)&sa, sizeof sa)==1 failed: errno = Socket not connected
src/functional/socket.c:30: recvfrom(s, buf, sizeof buf, 0, (void *)&sa, (socklen_t[]){sizeof sa})==1 failed: errno = Resource temporarily unavailable
src/functional/socket.c:31: buf[0]=='x' failed: ' '
FAIL libctest entry-static.exe socket: 1
========== END entry-static.exe socket ==========
========== START entry-static.exe sscanf ==========
Pass!
========== END entry-static.exe sscanf ==========
========== START entry-static.exe sscanf_long ==========
Pass!
========== END entry-static.exe sscanf_long ==========
========== START entry-static.exe stat ==========
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
Pass!
========== END entry-static.exe setjmp ==========
========== START entry-static.exe snprintf ==========
Pass!
========== END entry-static.exe snprintf ==========
========== START entry-static.exe socket ==========
[37m[223.200811 0:428 axnet::smoltcp_impl::udp:312] [33m[AxError::NotConnected] socket send() failed[m
[m[37m[223.202177 0:428 arceos_posix_api::imp::net:450] [32msys_sendto => Err(ENOTCONN)[m
[msrc/functional/socket.c:29: sendto(c, "x", 1, 0, (void *)&sa, sizeof sa)==1 failed: errno = Transport endpoint is not connected
src/functional/socket.c:30: recvfrom(s, buf, sizeof buf, 0, (void *)&sa, (socklen_t[]){sizeof sa})==1 failed: errno = Resource temporarily unavailable
src/functional/socket.c:31: buf[0]=='x' failed: ''
FAIL libctest entry-static.exe socket: 1
========== END entry-static.exe socket ==========
========== START entry-static.exe sscanf ==========
src/functional/sscanf.c:39: sscanf("hello, world\n", "%8c%8c", a, b) failed (2 fields, expected 1)
src/functional/sscanf.c:66: sscanf(" 0x12 0x34", "%5i%2i", &x, &y) failed (got 2 fields, expected 1)
src/functional/sscanf.c:83: sscanf("10e", "%lf", &d) failed (got 1 fields, expected no match (0))
FAIL libctest entry-static.exe sscanf: 1
========== END entry-static.exe sscanf ==========
========== START entry-static.exe sscanf_long ==========
Pass!
========== END entry-static.exe sscanf_long ==========
========== START entry-static.exe stat ==========
Pass!
========== END entry-static.exe stat ==========
========== START entry-static.exe strftime ==========
src/functional/strftime.c:14: "%c": expected "Mon Jan  5 05:17:53 +10009", got "Mon Jan  5 05:17:53 10009"
src/functional/strftime.c:14: "%c": expected "Wed Feb 23 12:00:00 0000", got "Wed Feb 23 12:00:00 0"
src/functional/strftime.c:14: "%+3C": expected "+20", got "%+3C"
src/functional/strftime.c:14: "%C": expected "00", got "0"
src/functional/strftime.c:14: "%+10F": expected "2016-01-03", got "%+10F"
src/functional/strftime.c:14: "%+11F": expected "+2016-01-03", got "%+11F"
src/functional/strftime.c:14: "%F": expected "+10009-01-05", got "10009-01-05"
src/functional/strftime.c:14: "%F": expected "0000-02-23", got "0-02-23"
src/functional/strftime.c:14: "%011F": expected "-0123-01-01", got "0-123-01-01"
src/functional/strftime.c:14: "%+5G": expected "+2015", got "%+5G"
src/functional/strftime.c:14: "%+4Y": expected "2016", got "%+4Y"
src/functional/strftime.c:14: "%+5Y": expected "+2016", got "%+5Y"
src/functional/strftime.c:14: "%Y": expected "+10009", got "10009"
src/functional/strftime.c:14: "%Y": expected "0000", got "0"
src/functional/strftime.c:14: "%+5Y": expected "+0000", got "%+5Y"
src/functional/strftime.c:14: "%+4Y": expected "-123", got "%+4Y"
src/functional/strftime.c:14: "%+5Y": expected "-0123", got "%+5Y"
src/functional/strftime.c:14: "%Y": expected "+2147485547", got "-2147481749"
src/functional/strftime.c:14: "%011Y": expected "02147485547", got "-2147481749"
FAIL libctest entry-static.exe strftime: 1
========== END entry-static.exe strftime ==========
========== START entry-static.exe string ==========
Pass!
========== END entry-static.exe string ==========
========== START entry-static.exe string_memcpy ==========
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
pid:481
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
pid = 493
========== END test_getpid ==========
Testing getppid :
========== START test_getppid ==========
  getppid success. ppid : 477
========== END test_getppid ==========
Testing gettimeofday :
========== START test_gettimeofday ==========
gettimeofday success.
start:255037, end:255084
interval: 47
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
cpid: 503
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
  I am child process: 516. iteration 0.
  I am child process: 517. iteration 1.
  I am child process: 518. iteration 2.
  I am child process: 516. iteration 0.
  I am child process: 517. iteration 1.
  I am child process: 518. iteration 2.
  I am child process: 516. iteration 0.
  I am child process: 517. iteration 1.
  I am child process: 518. iteration 2.
  I am child process: 516. iteration 0.
  I am child process: 517. iteration 1.
  I am child process: 518. iteration 2.
  I am child process: 516. iteration 0.
  I am child process: 517. iteration 1.
  I am child process: 518. iteration 2.
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
pid:528
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
pid = 540
========== END test_getpid ==========
Testing getppid :
========== START test_getppid ==========
  getppid success. ppid : 524
========== END test_getppid ==========
Testing gettimeofday :
========== START test_gettimeofday ==========
gettimeofday success.
start:275185, end:275232
interval: 47
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
cpid: 550
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
  I am child process: 563. iteration 0.
  I am child process: 564. iteration 1.
  I am child process: 565. iteration 2.
  I am child process: 563. iteration 0.
  I am child process: 564. iteration 1.
  I am child process: 565. iteration 2.
  I am child process: 563. iteration 0.
  I am child process: 564. iteration 1.
  I am child process: 565. iteration 2.
  I am child process: 563. iteration 0.
  I am child process: 564. iteration 1.
  I am child process: 565. iteration 2.
  I am child process: 563. iteration 0.
  I am child process: 564. iteration 1.
  I am child process: 565. iteration 2.
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
Thu Jan  1 00:04:51 UTC 1970
testcase busybox date success
Filesystem           1K-blocks      Used Available Use% Mounted on
devfs                   784712     86572    698140  11% /dev
tmpfs                   784712     86572    698140  11% /tmp
tmpfs                   784712     86572    698140  11% /var
proc                    784712     86572    698140  11% /proc
sysfs                   784712     86572    698140  11% /sys
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
 00:05:03 up 0 min,  0 users,  load average: 0.00, 0.00, 0.00
testcase busybox uptime success
abc
testcase busybox printf "abc\n" success
PID   USER     TIME  COMMAND
testcase busybox ps success
/tmp/testsuite/musl/busybox
testcase busybox pwd success
              total        used        free      shared  buff/cache   available
Mem:              0           0           0           0           0      781854
-/+ buffers/cache:            0           0
Swap:             0           0           0
testcase busybox free success
Thu Jan  1 00:05:09 1970  0.000000 seconds
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
Thu Jan  1 00:06:07 UTC 1970
testcase busybox date success
Filesystem           1K-blocks      Used Available Use% Mounted on
devfs                   784712    159852    624860  20% /dev
tmpfs                   784712    159852    624860  20% /tmp
tmpfs                   784712    159852    624860  20% /var
proc                    784712    159852    624860  20% /proc
sysfs                   784712    159852    624860  20% /sys
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
 00:06:26 up 0 min,  0 users,  load average: 0.00, 0.00, 0.00
testcase busybox uptime success
abc
testcase busybox printf "abc\n" success
PID   USER     TIME  COMMAND
testcase busybox ps success
/tmp/testsuite/glibc/busybox
testcase busybox pwd success
              total        used        free      shared  buff/cache   available
Mem:              0           0           0           0           0      781854
-/+ buffers/cache:            0           0
Swap:             0           0           0
testcase busybox free success
Thu Jan  1 00:06:36 1970  0.000000 seconds
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
========== START ltp access01 ==========
RUN LTP CASE access01
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
[37m[488.389906 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.391019 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.391764 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.392849 0:949 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[488.393918 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.394533 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.395125 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.396001 0:949 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[488.396906 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.397512 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.398088 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.398752 0:949 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[488.399653 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.400234 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.400813 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.401455 0:949 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[488.402350 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.402939 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.403550 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.404193 0:949 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[488.405072 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.405771 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.406357 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.406992 0:949 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[488.407569 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.408125 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.408681 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.409230 0:949 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[488.410015 0:949 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE access01 : 0
Pass!
========== END ltp access01 ==========
========== START ltp brk01 ==========
RUN LTP CASE brk01
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
[37m[489.365694 0:1054 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE brk01 : 0
Pass!
========== END ltp brk01 ==========
========== START ltp chdir01 ==========
RUN LTP CASE chdir01
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
tst_test.c:1120: [1;34mTINFO: [0mMounting ltp-tmpfs to /tmp/ltp-work/LTP_chdfAAiOK/mntpoint fstyp=tmpfs flags=0
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
chdir01.c:122: [1;33mTCONF: [0mSkipping symlink loop test, not supported

Summary:
passed   14
failed   0
broken   0
skipped  1
warnings 0
[37m[490.354505 0:1061 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[490.360524 0:1061 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[490.367044 0:1061 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[490.372358 0:1061 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[490.377217 0:1061 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE chdir01 : 0
Pass!
========== END ltp chdir01 ==========
========== START ltp clone01 ==========
RUN LTP CASE clone01
tst_buffers.c:57: [1;34mTINFO: [0mTest is using guarded buffers
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
clone01.c:37: [1;32mTPASS: [0mclone returned 1068
clone01.c:43: [1;32mTPASS: [0mChild exited with 0

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[491.436824 0:1065 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE clone01 : 0
Pass!
========== END ltp clone01 ==========
========== START ltp close01 ==========
RUN LTP CASE close01
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
[37m[492.422371 0:1070 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[492.425075 0:1070 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE close01 : 0
Pass!
========== END ltp close01 ==========
========== START ltp dup01 ==========
RUN LTP CASE dup01
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
dup01.c:24: [1;32mTPASS: [0mdup(fd) returned fd 4
dup01.c:27: [1;32mTPASS: [0mbuf1.st_ino == buf2.st_ino (7034136155013304451)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[493.461674 0:1074 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[493.463873 0:1074 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE dup01 : 0
Pass!
========== END ltp dup01 ==========
========== START ltp fcntl02 ==========
RUN LTP CASE fcntl02
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_1080, F_DUPFD, 0) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_1080, F_DUPFD, 1) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_1080, F_DUPFD, 2) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_1080, F_DUPFD, 3) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_1080, F_DUPFD, 10) returned 10
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_1080, F_DUPFD, 100) returned 100

Summary:
passed   6
failed   0
broken   0
skipped  0
warnings 0
[37m[494.417888 0:1078 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[494.419683 0:1078 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fcntl02 : 0
Pass!
========== END ltp fcntl02 ==========
========== START ltp fork01 ==========
RUN LTP CASE fork01
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
fork01.c:47: [1;32mTPASS: [0mcorrect child status returned 42
fork01.c:50: [1;32mTPASS: [0mchild_pid == pid (1085)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[495.362425 0:1082 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[495.363801 0:1082 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fork01 : 0
Pass!
========== END ltp fork01 ==========
========== START ltp getpid01 ==========
RUN LTP CASE getpid01
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1090
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1091
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1092
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1093
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1094
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1095
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1096
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1097
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1098
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1099
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1100
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1101
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1102
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1103
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1104
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1105
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1106
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1107
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1108
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1109
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1110
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1111
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1112
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1113
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1114
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1115
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1116
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1117
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1118
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1119
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1120
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1121
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1122
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1123
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1124
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1125
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1126
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1127
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1128
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1129
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1130
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1131
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1132
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1133
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1134
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1135
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1136
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1137
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1138
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1139
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1140
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1141
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1142
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1143
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1144
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1145
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1146
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1147
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1148
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1149
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1150
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1151
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1152
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1153
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1154
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1155
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1156
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1157
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1158
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1159
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1160
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1161
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1162
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1163
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1164
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1165
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1166
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1167
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1168
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1169
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1170
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1171
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1172
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1173
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1174
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1175
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1176
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1177
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1178
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1179
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1180
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1181
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1182
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1183
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1184
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1185
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1186
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1187
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1188
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 1189

Summary:
passed   100
failed   0
broken   0
skipped  0
warnings 0
[37m[498.322747 0:1087 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpid01 : 0
Pass!
========== END ltp getpid01 ==========
========== START ltp mmap01 ==========
RUN LTP CASE mmap01
mmap01      1  [1;32mTPASS[0m  :  Functionality of mmap() successful
[37m[499.277357 0:1191 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[499.278468 0:1191 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE mmap01 : 0
Pass!
========== END ltp mmap01 ==========
========== START ltp open01 ==========
RUN LTP CASE open01
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
[37m[500.229566 0:1193 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[500.231376 0:1193 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE open01 : 0
Pass!
========== END ltp open01 ==========
========== START ltp pipe01 ==========
RUN LTP CASE pipe01
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
[37m[501.176925 0:1197 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE pipe01 : 0
Pass!
========== END ltp pipe01 ==========
========== START ltp read01 ==========
RUN LTP CASE read01
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
[37m[502.100206 0:1201 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[502.101509 0:1201 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE read01 : 0
Pass!
========== END ltp read01 ==========
========== START ltp stat01 ==========
RUN LTP CASE stat01
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
[37m[503.049049 0:1205 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[503.049805 0:1205 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[503.050758 0:1205 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE stat01 : 0
Pass!
========== END ltp stat01 ==========
========== START ltp wait401 ==========
RUN LTP CASE wait401
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
wait401.c:40: [1;32mTPASS: [0mwait4() returned correct pid 1212
wait401.c:49: [1;32mTPASS: [0mWIFEXITED() is set in status
wait401.c:54: [1;32mTPASS: [0mWEXITSTATUS() == 0

Summary:
passed   3
failed   0
broken   0
skipped  0
warnings 0
[37m[504.097211 0:1209 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE wait401 : 0
Pass!
========== END ltp wait401 ==========
========== START ltp write01 ==========
RUN LTP CASE write01
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
[37m[505.077496 0:1214 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[505.078734 0:1214 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE write01 : 0
Pass!
========== END ltp write01 ==========
ltp cases: 16 passed, 0 failed
#### OS COMP TEST GROUP END ltp-musl ####
#### OS COMP TEST GROUP START ltp-glibc ####
========== START ltp access01 ==========
RUN LTP CASE access01
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
[37m[510.084333 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.085179 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.085811 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.086882 0:1218 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[510.088081 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.088748 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.089380 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.090071 0:1218 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[510.091018 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.091680 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.092435 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.093113 0:1218 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[510.094051 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.094686 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.095287 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.095971 0:1218 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[510.096899 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.097535 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.098141 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.098819 0:1218 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[510.099773 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.100403 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.101023 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.101690 0:1218 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[510.102379 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.103063 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.103748 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.104327 0:1218 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[510.105087 0:1218 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE access01 : 0
Pass!
========== END ltp access01 ==========
========== START ltp brk01 ==========
RUN LTP CASE brk01
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
[37m[511.912726 0:1323 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE brk01 : 0
Pass!
========== END ltp brk01 ==========
========== START ltp chdir01 ==========
RUN LTP CASE chdir01
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
tst_test.c:1120: TINFO: Mounting ltp-tmpfs to /tmp/ltp-work/LTP_chdPxYxuS/mntpoint fstyp=tmpfs flags=0
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
chdir01.c:122: TCONF: Skipping symlink loop test, not supported

Summary:
passed   14
failed   0
broken   0
skipped  1
warnings 0
[37m[513.738822 0:1330 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[513.741502 0:1330 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[513.744084 0:1330 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[513.746921 0:1330 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[513.748165 0:1330 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE chdir01 : 0
Pass!
========== END ltp chdir01 ==========
========== START ltp clone01 ==========
RUN LTP CASE clone01
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clone01.c:37: TPASS: clone returned 1337
clone01.c:43: TPASS: Child exited with 0

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[515.582572 0:1334 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE clone01 : 0
Pass!
========== END ltp clone01 ==========
========== START ltp close01 ==========
RUN LTP CASE close01
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
[37m[517.449735 0:1339 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[517.451639 0:1339 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE close01 : 0
Pass!
========== END ltp close01 ==========
========== START ltp dup01 ==========
RUN LTP CASE dup01
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
dup01.c:24: TPASS: dup(fd) returned fd 4
dup01.c:27: TPASS: buf1.st_ino == buf2.st_ino (13867373987566138326)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[519.321889 0:1343 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[519.324118 0:1343 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE dup01 : 0
Pass!
========== END ltp dup01 ==========
========== START ltp fcntl02 ==========
RUN LTP CASE fcntl02
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
fcntl02.c:41: TPASS: fcntl(fcntl02_1349, F_DUPFD, 0) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1349, F_DUPFD, 1) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1349, F_DUPFD, 2) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1349, F_DUPFD, 3) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1349, F_DUPFD, 10) returned 10
fcntl02.c:41: TPASS: fcntl(fcntl02_1349, F_DUPFD, 100) returned 100

Summary:
passed   6
failed   0
broken   0
skipped  0
warnings 0
[37m[521.076751 0:1347 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[521.078219 0:1347 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fcntl02 : 0
Pass!
========== END ltp fcntl02 ==========
========== START ltp fork01 ==========
RUN LTP CASE fork01
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
fork01.c:47: TPASS: correct child status returned 42
fork01.c:50: TPASS: child_pid == pid (1354)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[522.878348 0:1351 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[522.879550 0:1351 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE fork01 : 0
Pass!
========== END ltp fork01 ==========
========== START ltp getpid01 ==========
RUN LTP CASE getpid01
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getpid01.c:34: TPASS: getpid() returns 1359
getpid01.c:34: TPASS: getpid() returns 1360
getpid01.c:34: TPASS: getpid() returns 1361
getpid01.c:34: TPASS: getpid() returns 1362
getpid01.c:34: TPASS: getpid() returns 1363
getpid01.c:34: TPASS: getpid() returns 1364
getpid01.c:34: TPASS: getpid() returns 1365
getpid01.c:34: TPASS: getpid() returns 1366
getpid01.c:34: TPASS: getpid() returns 1367
getpid01.c:34: TPASS: getpid() returns 1368
getpid01.c:34: TPASS: getpid() returns 1369
getpid01.c:34: TPASS: getpid() returns 1370
getpid01.c:34: TPASS: getpid() returns 1371
getpid01.c:34: TPASS: getpid() returns 1372
getpid01.c:34: TPASS: getpid() returns 1373
getpid01.c:34: TPASS: getpid() returns 1374
getpid01.c:34: TPASS: getpid() returns 1375
getpid01.c:34: TPASS: getpid() returns 1376
getpid01.c:34: TPASS: getpid() returns 1377
getpid01.c:34: TPASS: getpid() returns 1378
getpid01.c:34: TPASS: getpid() returns 1379
getpid01.c:34: TPASS: getpid() returns 1380
getpid01.c:34: TPASS: getpid() returns 1381
getpid01.c:34: TPASS: getpid() returns 1382
getpid01.c:34: TPASS: getpid() returns 1383
getpid01.c:34: TPASS: getpid() returns 1384
getpid01.c:34: TPASS: getpid() returns 1385
getpid01.c:34: TPASS: getpid() returns 1386
getpid01.c:34: TPASS: getpid() returns 1387
getpid01.c:34: TPASS: getpid() returns 1388
getpid01.c:34: TPASS: getpid() returns 1389
getpid01.c:34: TPASS: getpid() returns 1390
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

Summary:
passed   100
failed   0
broken   0
skipped  0
warnings 0
[37m[527.206982 0:1356 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE getpid01 : 0
Pass!
========== END ltp getpid01 ==========
========== START ltp mmap01 ==========
RUN LTP CASE mmap01
[37m[528.973221 0:1460 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[528.974288 0:1460 axfs::root:433] [33m[AxError::IsADirectory][m
[mmmap01      1  TPASS  :  Functionality of mmap() successful
PASS LTP CASE mmap01 : 0
Pass!
========== END ltp mmap01 ==========
========== START ltp open01 ==========
RUN LTP CASE open01
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
[37m[530.845763 0:1462 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[530.847033 0:1462 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE open01 : 0
Pass!
========== END ltp open01 ==========
========== START ltp pipe01 ==========
RUN LTP CASE pipe01
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
[37m[532.663116 0:1466 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE pipe01 : 0
Pass!
========== END ltp pipe01 ==========
========== START ltp read01 ==========
RUN LTP CASE read01
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
[37m[534.385904 0:1470 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[534.387822 0:1470 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE read01 : 0
Pass!
========== END ltp read01 ==========
========== START ltp stat01 ==========
RUN LTP CASE stat01
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
[37m[536.132620 0:1474 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[536.133355 0:1474 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[536.134644 0:1474 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE stat01 : 0
Pass!
========== END ltp stat01 ==========
========== START ltp wait401 ==========
RUN LTP CASE wait401
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
wait401.c:40: TPASS: wait4() returned correct pid 1481
wait401.c:49: TPASS: WIFEXITED() is set in status
wait401.c:54: TPASS: WEXITSTATUS() == 0

Summary:
passed   3
failed   0
broken   0
skipped  0
warnings 0
[37m[538.001664 0:1478 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE wait401 : 0
Pass!
========== END ltp wait401 ==========
========== START ltp write01 ==========
RUN LTP CASE write01
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
[37m[540.237456 0:1484 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[540.238616 0:1484 axfs::root:433] [33m[AxError::IsADirectory][m
[mPASS LTP CASE write01 : 0
Pass!
========== END ltp write01 ==========
ltp cases: 16 passed, 0 failed
#### OS COMP TEST GROUP END ltp-glibc ####
#### OS COMP TEST GROUP START libcbench-musl ####
b_malloc_sparse (0)
  time: 1.565413460, virt: 0, res: 0, dirty: 0

b_malloc_bubble (0)
  time: 1.513273810, virt: 0, res: 0, dirty: 0

b_malloc_tiny1 (0)
  time: 0.026236940, virt: 0, res: 0, dirty: 0

b_malloc_tiny2 (0)
  time: 0.022904120, virt: 0, res: 0, dirty: 0

b_malloc_big1 (0)
  time: 0.642027120, virt: 0, res: 0, dirty: 0

b_malloc_big2 (0)
  time: 0.474488440, virt: 0, res: 0, dirty: 0

b_malloc_thread_stress (0)
  time: 0.205791980, virt: 0, res: 0, dirty: 0

b_malloc_thread_local (0)
  time: 0.123546180, virt: 0, res: 0, dirty: 0

b_string_strstr ("abcdefghijklmnopqrstuvwxyz")
  time: 0.027988210, virt: 0, res: 0, dirty: 0

b_string_strstr ("azbycxdwevfugthsirjqkplomn")
  time: 0.040853740, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaacccccccccccc")
  time: 0.026759510, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.026630040, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.033213680, virt: 0, res: 0, dirty: 0

b_string_memset (0)
  time: 0.020294500, virt: 0, res: 0, dirty: 0

b_string_strchr (0)
  time: 0.038434460, virt: 0, res: 0, dirty: 0

b_string_strlen (0)
  time: 0.030291060, virt: 0, res: 0, dirty: 0

b_pthread_createjoin_serial1 (0)
  time: 0.006080510, virt: 0, res: 0, dirty: 0

b_pthread_createjoin_serial2 (0)
  time: 0.835112690, virt: 0, res: 0, dirty: 0

b_pthread_create_serial1 (0)
  time: 0.001466520, virt: 0, res: 0, dirty: 0

b_pthread_uselesslock (0)
  time: 0.106207400, virt: 0, res: 0, dirty: 0

b_utf8_bigbuf (0)
  time: 0.091633540, virt: 0, res: 0, dirty: 0

b_utf8_onebyone (0)
  time: 0.289838210, virt: 0, res: 0, dirty: 0

b_stdio_putcgetc (0)
  time: 0.497655840, virt: 0, res: 0, dirty: 0

b_stdio_putcgetc_unlocked (0)
  time: 0.469348250, virt: 0, res: 0, dirty: 0

b_regex_compile ("(a|b|c)*d*b")
  time: 0.656213350, virt: 0, res: 0, dirty: 0

b_regex_search ("(a|b|c)*d*b")
  time: 0.270164200, virt: 0, res: 0, dirty: 0

b_regex_search ("a{25}b")
  time: 0.445176950, virt: 0, res: 0, dirty: 0

#### OS COMP TEST GROUP END libcbench-musl ####
#### OS COMP TEST GROUP START libcbench-glibc ####
b_malloc_sparse (0)
  time: 1.299624250, virt: 0, res: 0, dirty: 0

b_malloc_bubble (0)
  time: 1.385794070, virt: 0, res: 0, dirty: 0

b_malloc_tiny1 (0)
  time: 0.021145430, virt: 0, res: 0, dirty: 0

b_malloc_tiny2 (0)
  time: 0.021278390, virt: 0, res: 0, dirty: 0

b_malloc_big1 (0)
  time: 0.287105870, virt: 0, res: 0, dirty: 0

b_malloc_big2 (0)
  time: 0.383525170, virt: 0, res: 0, dirty: 0

The futex facility returned an unexpected error code.
The futex facility returned an unexpected error code.
b_string_strstr ("abcdefghijklmnopqrstuvwxyz")
  time: 0.023924190, virt: 0, res: 0, dirty: 0

b_string_strstr ("azbycxdwevfugthsirjqkplomn")
  time: 0.022933280, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaacccccccccccc")
  time: 0.042136730, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.023092950, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.026534880, virt: 0, res: 0, dirty: 0

b_string_memset (0)
  time: 0.020036640, virt: 0, res: 0, dirty: 0

b_string_strchr (0)
  time: 0.033087290, virt: 0, res: 0, dirty: 0

b_string_strlen (0)
  time: 0.028455000, virt: 0, res: 0, dirty: 0

The futex facility returned an unexpected error code.
b_pthread_create_serial1 (0)
  time: 0.004961460, virt: 0, res: 0, dirty: 0

The futex facility returned an unexpected error code.
b_utf8_bigbuf (0)
  time: 0.077150280, virt: 0, res: 0, dirty: 0

b_utf8_onebyone (0)
  time: 0.063720800, virt: 0, res: 0, dirty: 0

b_regex_compile ("(a|b|c)*d*b")
  time: 0.065157500, virt: 0, res: 0, dirty: 0

b_regex_search ("(a|b|c)*d*b")
  time: 0.026797260, virt: 0, res: 0, dirty: 0

b_regex_search ("a{25}b")
  time: 0.318940030, virt: 0, res: 0, dirty: 0

#### OS COMP TEST GROUP END libcbench-glibc ####
#### OS COMP TEST GROUP START iperf-musl ####
====== iperf BASIC_UDP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 0.0.0.0 port 49154 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  5]   0.00-2.00   sec  3.09 MBytes  12.9 Mbits/sec  2216  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.00   sec  3.09 MBytes  12.9 Mbits/sec  0.000 ms  0/2216 (0%)  sender
[  5]   0.00-2.00   sec  3.09 MBytes  12.9 Mbits/sec  0.090 ms  0/2216 (0%)  receiver

iperf Done.
====== iperf BASIC_UDP end: success ======

====== iperf BASIC_TCP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 127.0.0.1 port 49158 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.00   sec  69.2 MBytes   290 Mbits/sec    0   0.00 Bytes       
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.00   sec  69.2 MBytes   290 Mbits/sec    0             sender
[  5]   0.00-2.01   sec  68.4 MBytes   286 Mbits/sec                  receiver

iperf Done.
====== iperf BASIC_TCP end: success ======

====== iperf PARALLEL_UDP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 0.0.0.0 port 49155 connected to 127.0.0.1 port 5001
[  7] local 0.0.0.0 port 49156 connected to 127.0.0.1 port 5001
[  9] local 0.0.0.0 port 49157 connected to 127.0.0.1 port 5001
[ 11] local 0.0.0.0 port 49158 connected to 127.0.0.1 port 5001
[ 13] local 0.0.0.0 port 49159 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  5]   0.00-2.00   sec   823 KBytes  3.37 Mbits/sec  577  
[  7]   0.00-2.00   sec   823 KBytes  3.37 Mbits/sec  577  
[  9]   0.00-2.00   sec   823 KBytes  3.37 Mbits/sec  577  
[ 11]   0.00-2.00   sec   823 KBytes  3.37 Mbits/sec  577  
[ 13]   0.00-2.00   sec   823 KBytes  3.37 Mbits/sec  577  
[SUM]   0.00-2.00   sec  4.02 MBytes  16.8 Mbits/sec  2885  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.00   sec   823 KBytes  3.37 Mbits/sec  0.000 ms  0/577 (0%)  sender
[  5]   0.00-2.00   sec   823 KBytes  3.36 Mbits/sec  0.061 ms  0/577 (0%)  receiver
[  7]   0.00-2.00   sec   823 KBytes  3.37 Mbits/sec  0.000 ms  0/577 (0%)  sender
[  7]   0.00-2.00   sec   823 KBytes  3.36 Mbits/sec  0.049 ms  0/577 (0%)  receiver
[  9]   0.00-2.00   sec   823 KBytes  3.37 Mbits/sec  0.000 ms  0/577 (0%)  sender
[  9]   0.00-2.00   sec   823 KBytes  3.36 Mbits/sec  0.100 ms  0/577 (0%)  receiver
[ 11]   0.00-2.00   sec   823 KBytes  3.37 Mbits/sec  0.000 ms  0/577 (0%)  sender
[ 11]   0.00-2.00   sec   823 KBytes  3.36 Mbits/sec  0.072 ms  0/577 (0%)  receiver
[ 13]   0.00-2.00   sec   823 KBytes  3.37 Mbits/sec  0.000 ms  0/577 (0%)  sender
[ 13]   0.00-2.00   sec   823 KBytes  3.36 Mbits/sec  0.136 ms  0/577 (0%)  receiver
[SUM]   0.00-2.00   sec  4.02 MBytes  16.8 Mbits/sec  0.000 ms  0/2885 (0%)  sender
[SUM]   0.00-2.00   sec  4.02 MBytes  16.8 Mbits/sec  0.083 ms  0/2885 (0%)  receiver

iperf Done.
====== iperf PARALLEL_UDP end: success ======

====== iperf PARALLEL_TCP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 127.0.0.1 port 49161 connected to 127.0.0.1 port 5001
[  7] local 127.0.0.1 port 49162 connected to 127.0.0.1 port 5001
[  9] local 127.0.0.1 port 49163 connected to 127.0.0.1 port 5001
[ 11] local 127.0.0.1 port 49164 connected to 127.0.0.1 port 5001
[ 13] local 127.0.0.1 port 49165 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.02   sec  17.0 MBytes  70.5 Mbits/sec    0   0.00 Bytes       
[  7]   0.00-2.02   sec  17.0 MBytes  70.5 Mbits/sec    0   0.00 Bytes       
[  9]   0.00-2.02   sec  17.0 MBytes  70.5 Mbits/sec    0   0.00 Bytes       
[ 11]   0.00-2.02   sec  17.0 MBytes  70.5 Mbits/sec    0   0.00 Bytes       
[ 13]   0.00-2.02   sec  17.0 MBytes  70.5 Mbits/sec    0   0.00 Bytes       
[SUM]   0.00-2.02   sec  85.0 MBytes   353 Mbits/sec    0             
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.02   sec  17.0 MBytes  70.5 Mbits/sec    0             sender
[  5]   0.00-2.03   sec  16.1 MBytes  66.6 Mbits/sec                  receiver
[  7]   0.00-2.02   sec  17.0 MBytes  70.5 Mbits/sec    0             sender
[  7]   0.00-2.03   sec  16.1 MBytes  66.6 Mbits/sec                  receiver
[  9]   0.00-2.02   sec  17.0 MBytes  70.5 Mbits/sec    0             sender
[  9]   0.00-2.03   sec  16.1 MBytes  66.6 Mbits/sec                  receiver
[ 11]   0.00-2.02   sec  17.0 MBytes  70.5 Mbits/sec    0             sender
[ 11]   0.00-2.03   sec  16.1 MBytes  66.6 Mbits/sec                  receiver
[ 13]   0.00-2.02   sec  17.0 MBytes  70.5 Mbits/sec    0             sender
[ 13]   0.00-2.03   sec  16.1 MBytes  66.6 Mbits/sec                  receiver
[SUM]   0.00-2.02   sec  85.0 MBytes   353 Mbits/sec    0             sender
[SUM]   0.00-2.03   sec  80.6 MBytes   333 Mbits/sec                  receiver

iperf Done.
====== iperf PARALLEL_TCP end: success ======

====== iperf REVERSE_UDP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 0.0.0.0 port 49160 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.00   sec  3.15 MBytes  13.2 Mbits/sec  0.120 ms  0/2265 (0%)  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.01   sec  3.16 MBytes  13.2 Mbits/sec  0.000 ms  0/2266 (0%)  sender
[  5]   0.00-2.00   sec  3.15 MBytes  13.2 Mbits/sec  0.120 ms  0/2265 (0%)  receiver

iperf Done.
====== iperf REVERSE_UDP end: success ======

====== iperf REVERSE_TCP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 127.0.0.1 port 49168 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.01   sec  69.4 MBytes   290 Mbits/sec                  
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.04   sec  70.2 MBytes   289 Mbits/sec    0             sender
[  5]   0.00-2.01   sec  69.4 MBytes   290 Mbits/sec                  receiver

iperf Done.
====== iperf REVERSE_TCP end: success ======

#### OS COMP TEST GROUP END iperf-musl ####
#### OS COMP TEST GROUP START iperf-glibc ####
====== iperf BASIC_UDP begin ======
iperf3: error - control socket has closed unexpectedly
====== iperf BASIC_UDP end: fail ======

====== iperf BASIC_TCP begin ======
[37m[587.406970 0:4149 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[587.407815 0:4149 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf BASIC_TCP end: fail ======

====== iperf PARALLEL_UDP begin ======
[37m[587.501703 0:4150 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[587.502365 0:4150 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf PARALLEL_UDP end: fail ======

====== iperf PARALLEL_TCP begin ======
[37m[587.640983 0:4151 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[587.641599 0:4151 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf PARALLEL_TCP end: fail ======

====== iperf REVERSE_UDP begin ======
[37m[587.783206 0:4152 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[587.783906 0:4152 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf REVERSE_UDP end: fail ======

====== iperf REVERSE_TCP begin ======
[37m[587.871919 0:4153 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[587.872585 0:4153 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf REVERSE_TCP end: fail ======

#### OS COMP TEST GROUP END iperf-glibc ####
#### OS COMP TEST GROUP START lmbench-musl ####
latency measurements
Simple syscall: 1.4084 microseconds
Simple read: 7.0013 microseconds
Simple write: 5.0426 microseconds
[37m[594.404322 0:4167 axfs::root:423] [33m[AxError::AlreadyExists][m
[m[37m[594.406739 0:4167 axfs::root:423] [33m[AxError::AlreadyExists][m
[mSimple stat: 101.7084 microseconds
Simple fstat: 13.9136 microseconds
Simple open/close: 87.1094 microseconds
Select on 100 fd's: 144.6842 microseconds
Signal handler installation: 5.8174 microseconds
Signal handler overhead: 44.2625 microseconds
autorun: /tmp/testsuite/musl/lmbench/lmbench_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START lmbench-glibc ####
latency measurements
Simple syscall: 1.3872 microseconds
Simple read: 6.8135 microseconds
Simple write: 4.6709 microseconds
[37m[659.900546 0:4207 axfs::root:423] [33m[AxError::AlreadyExists][m
[m[37m[659.901545 0:4207 axfs::root:423] [33m[AxError::AlreadyExists][m
[m[37m[659.902103 0:4207 axfs::root:423] [33m[AxError::AlreadyExists][m
[mSimple stat: 114.4153 microseconds
Simple fstat: 8.0684 microseconds
Simple open/close: 84.8500 microseconds
Select on 100 fd's: 143.9726 microseconds
Signal handler installation: 6.2803 microseconds
Signal handler overhead: 60.8675 microseconds
autorun: /tmp/testsuite/glibc/lmbench/lmbench_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START netperf-musl ####
====== netperf UDP_STREAM begin ======
Starting netserver with host '127.0.0.1' port '12865' and family AF_UNSPEC
MIGRATED UDP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Socket  Message  Elapsed      Messages                
Size    Size     Time         Okay Errors   Throughput
bytes   bytes    secs            #      #   10^6bits/sec

 65536    1000   1.00         1828      0      14.56
 65536           1.00         1828             14.56

====== netperf UDP_STREAM end: success ======
====== netperf TCP_STREAM begin ======
MIGRATED TCP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Recv   Send    Send                          
Socket Socket  Message  Elapsed              
Size   Size    Size     Time     Throughput  
bytes  bytes   bytes    secs.    10^6bits/sec  

 65536  65536   1000    0.05      169.21   
====== netperf TCP_STREAM end: success ======
====== netperf UDP_RR begin ======
MIGRATED UDP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00     1518.08   
65536  65536 
====== netperf UDP_RR end: success ======
====== netperf TCP_RR begin ======
MIGRATED TCP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00     1470.59   
65536  65536 
====== netperf TCP_RR end: success ======
====== netperf TCP_CRR begin ======
MIGRATED TCP Connect/Request/Response TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00     1054.37   
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

 65536    1000   1.00         1797      0      14.33
 65536           1.00         1797             14.33

====== netperf UDP_STREAM end: success ======
====== netperf TCP_STREAM begin ======
MIGRATED TCP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET
Recv   Send    Send                          
Socket Socket  Message  Elapsed              
Size   Size    Size     Time     Throughput  
bytes  bytes   bytes    secs.    10^6bits/sec  

 65536  65536   1000    0.05      155.60   
====== netperf TCP_STREAM end: success ======
====== netperf UDP_RR begin ======
MIGRATED UDP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00     1561.12   
65536  65536 
====== netperf UDP_RR end: success ======
====== netperf TCP_RR begin ======
MIGRATED TCP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00     1781.20   
65536  65536 
====== netperf TCP_RR end: success ======
====== netperf TCP_CRR begin ======
MIGRATED TCP Connect/Request/Response TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate         
bytes  Bytes  bytes    bytes   secs.    per sec   

65536  65536  64       64      1.00     1106.82   
65536  65536 
====== netperf TCP_CRR end: success ======
#### OS COMP TEST GROUP END netperf-glibc ####
#### OS COMP TEST GROUP START cyclictest-musl ####
====== cyclictest NO_STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 4294) P:99 I:1000 C:    968 Min:      2 Act:    2 Avg:   48 Max:   12923
====== cyclictest NO_STRESS_P1 end: success ======
====== cyclictest NO_STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 4296) P:99 I:1000 C:    987 Min:      2 Act:    9 Avg:   23 Max:    7402
T: 1 ( 4297) P:99 I:1500 C:    660 Min:      2 Act:  349 Avg:   28 Max:    7592
T: 2 ( 4298) P:99 I:2000 C:    496 Min:      2 Act:  402 Avg:   32 Max:    8099
T: 3 ( 4299) P:99 I:2500 C:    398 Min:      2 Act:  425 Avg:   34 Max:    6583
T: 4 ( 4300) P:99 I:3000 C:    332 Min:      2 Act:   52 Avg:   30 Max:    6066
T: 5 ( 4301) P:99 I:3500 C:    284 Min:      2 Act:  110 Avg:   40 Max:    8050
T: 6 ( 4302) P:99 I:4000 C:    249 Min:      2 Act:  357 Avg:   32 Max:    6035
T: 7 ( 4303) P:99 I:4500 C:    222 Min:      2 Act:   46 Avg:   47 Max:    6020
====== cyclictest NO_STRESS_P8 end: success ======
====== start hackbench ======
Running in process mode with 10 groups using 40 file descriptors each (== 400 tasks)
Each sender will pass 100000000 messages of 100 bytes
====== cyclictest STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 4707) P:99 I:1000 C:      5 Min: 191528 Act:317301 Avg:234453 Max:  317301
====== cyclictest STRESS_P1 end: success ======
====== cyclictest STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 4709) P:99 I:1000 C:      7 Min:     81 Act:190520 Avg:157628 Max:  315989
T: 1 ( 4710) P:99 I:1500 C:      5 Min: 190512 Act:190512 Avg:220606 Max:  314953
T: 2 ( 4711) P:99 I:2000 C:      5 Min: 191004 Act:191030 Avg:220292 Max:  314941
T: 3 ( 4712) P:99 I:2500 C:      5 Min: 189535 Act:189535 Avg:219776 Max:  315929
T: 4 ( 4713) P:99 I:3000 C:      5 Min: 189043 Act:189043 Avg:219662 Max:  314918
T: 5 ( 4714) P:99 I:3500 C:      5 Min: 189433 Act:190044 Avg:219047 Max:  315407
T: 6 ( 4715) P:99 I:4000 C:      5 Min: 188907 Act:191043 Avg:219429 Max:  314896
T: 7 ( 4716) P:99 I:4500 C:      5 Min: 188379 Act:189039 Avg:219608 Max:  314881
====== cyclictest STRESS_P8 end: success ======
autorun: /tmp/testsuite/musl/cyclictest/cyclictest_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START cyclictest-glibc ####
====== cyclictest NO_STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 4723) P:99 I:1000 C:    988 Min:      2 Act:    2 Avg:   24 Max:    7931
====== cyclictest NO_STRESS_P1 end: success ======
====== cyclictest NO_STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 4725) P:99 I:1000 C:    996 Min:      2 Act:    3 Avg:   22 Max:    3077
T: 1 ( 4726) P:99 I:1500 C:    666 Min:      2 Act:  318 Avg:   26 Max:    2956
T: 2 ( 4727) P:99 I:2000 C:    499 Min:      2 Act:  346 Avg:   23 Max:    2913
T: 3 ( 4728) P:99 I:2500 C:    399 Min:      2 Act:  345 Avg:   26 Max:    2867
T: 4 ( 4729) P:99 I:3000 C:    334 Min:      2 Act:   47 Avg:   28 Max:    2785
T: 5 ( 4730) P:99 I:3500 C:    286 Min:      2 Act:   49 Avg:   36 Max:    2239
T: 6 ( 4731) P:99 I:4000 C:    250 Min:      2 Act:  180 Avg:   23 Max:    2695
T: 7 ( 4732) P:99 I:4500 C:    223 Min:      2 Act:   57 Avg:   30 Max:    2654
====== cyclictest NO_STRESS_P8 end: success ======
====== start hackbench ======
Running in process mode with 10 groups using 40 file descriptors each (== 400 tasks)
Each sender will pass 100000000 messages of 100 bytes
====== cyclictest STRESS_P1 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 5136) P:99 I:1000 C:      5 Min: 197842 Act:200844 Avg:228302 Max:  334549
====== cyclictest STRESS_P1 end: success ======
====== cyclictest STRESS_P8 begin ======
WARN: stat /dev/cpu_dma_latency failed: No such file or directory
T: 0 ( 5138) P:99 I:1000 C:      5 Min: 197379 Act:204016 Avg:228299 Max:  274132
T: 1 ( 5139) P:99 I:1500 C:      5 Min: 197023 Act:204916 Avg:228292 Max:  273521
T: 2 ( 5140) P:99 I:2000 C:      5 Min: 196248 Act:203428 Avg:227474 Max:  273005
T: 3 ( 5141) P:99 I:2500 C:      5 Min: 195959 Act:202924 Avg:227353 Max:  272989
T: 4 ( 5142) P:99 I:3000 C:      5 Min: 195424 Act:203423 Avg:227032 Max:  271974
T: 5 ( 5143) P:99 I:3500 C:      5 Min: 194688 Act:203919 Avg:226612 Max:  273460
T: 6 ( 5144) P:99 I:4000 C:      5 Min: 194355 Act:201412 Avg:226186 Max:  272939
T: 7 ( 5145) P:99 I:4500 C:      5 Min: 193825 Act:204910 Avg:226368 Max:  270424
====== cyclictest STRESS_P8 end: success ======
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

	Run began: Thu Jan  1 00:14:43 1970

	Auto Mode
	Record Size 1 kB
	File size set to 4096 kB
	Command line used: ./iozone -a -r 1k -s 4m
	Output is in kBytes/sec
	Time Resolution = 0.000003 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
                                                                    random    random      bkwd     record     stride                                        
              kB  reclen    write    rewrite      read    reread      read     write      read    rewrite       read    fwrite  frewrite     fread   freread
            4096       1     69745    126697    103544    104794     45647     53754[37m[884.780949 0:5152 axfs::fops:269] [33m[AxError::InvalidInput][m
[m[37m[884.841239 0:5152 axfs::fops:269] [33m[AxError::InvalidInput][m
[m     48797     109756      83590     90762     93288     55640     57553

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

	Run began: Thu Jan  1 00:14:45 1970

	Record Size 1 kB
	File size set to 1024 kB
	Command line used: ./iozone -t 4 -i 0 -i 1 -r 1k -s 1m
	Output is in kBytes/sec
	Time Resolution = 0.000003 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
	Throughput test with 4 processes
	Each process writes a 1024 kByte file in 1 kByte records

	Children see throughput for  4 initial writers 	=  104907.28 kB/sec
	Parent sees throughput for  4 initial writers 	=    2916.25 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  104907.28 kB/sec
	Avg throughput per process 			=   26226.82 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=   75100.84 kB/sec
	Parent sees throughput for  4 rewriters 	=    2524.30 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   75100.84 kB/sec
	Avg throughput per process 			=   18775.21 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 readers 		=  106400.67 kB/sec
	Parent sees throughput for  4 readers 		=    2546.88 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  106400.67 kB/sec
	Avg throughput per process 			=   26600.17 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 re-readers 	=  106059.04 kB/sec
	Parent sees throughput for 4 re-readers 	=    3120.41 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  106059.04 kB/sec
	Avg throughput per process 			=   26514.76 kB/sec
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

	Run began: Thu Jan  1 00:15:02 1970

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

	Children see throughput for  4 initial writers 	=  114631.14 kB/sec
	Parent sees throughput for  4 initial writers 	=    2974.46 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  114631.14 kB/sec
	Avg throughput per process 			=   28657.79 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=   75627.77 kB/sec
	Parent sees throughput for  4 rewriters 	=    2444.83 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   75627.77 kB/sec
	Avg throughput per process 			=   18906.94 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 random readers 	=   81360.24 kB/sec
	Parent sees throughput for 4 random readers 	=    2755.76 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   81360.24 kB/sec
	Avg throughput per process 			=   20340.06 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 random writers 	=   97840.62 kB/sec
	Parent sees throughput for 4 random writers 	=    2888.84 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   97840.62 kB/sec
	Avg throughput per process 			=   24460.16 kB/sec
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

	Run began: Thu Jan  1 00:15:23 1970

	Record Size 1 kB
	File size set to 1024 kB
	Command line used: ./iozone -t 4 -i 0 -i 3 -r 1k -s 1m
	Output is in kBytes/sec
	Time Resolution = 0.000003 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
	Throughput test with 4 processes
	Each process writes a 1024 kByte file in 1 kByte records

	Children see throughput for  4 initial writers 	=  112986.88 kB/sec
	Parent sees throughput for  4 initial writers 	=    2993.25 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  112986.88 kB/sec
	Avg throughput per process 			=   28246.72 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=  129048.52 kB/sec
	Parent sees throughput for  4 rewriters 	=    2174.52 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  129048.52 kB/sec
	Avg throughput per process 			=   32262.13 kB/sec
	Min xfer 					=       0.00 kB
[37m[932.278688 0:5207 axfs::fops:269] [33m[AxError::InvalidInput][m
[m
	Children see throughput for 4 reverse readers 	=   70562.30 kB/sec
	Parent sees throughput for 4 reverse readers 	=    2734.79 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   70562.30 kB/sec
	Avg throughput per process 			=   17640.57 kB/sec
	Min xfer 					=       0.00 kB



iozone test complete.
iozone throughput stride-read measurements
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

	Run began: Thu Jan  1 00:15:40 1970

	Record Size 1 kB
	File size set to 1024 kB
	Command line used: ./iozone -t 4 -i 0 -i 5 -r 1k -s 1m
	Output is in kBytes/sec
	Time Resolution = 0.000005 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
	Throughput test with 4 processes
	Each process writes a 1024 kByte file in 1 kByte records

	Children see throughput for  4 initial writers 	=  112676.05 kB/sec
	Parent sees throughput for  4 initial writers 	=    2942.69 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  112676.05 kB/sec
	Avg throughput per process 			=   28169.01 kB/sec
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

	Run began: Thu Jan  1 00:15:51 1970

	Auto Mode
	Record Size 1 kB
	File size set to 4096 kB
	Command line used: ./iozone -a -r 1k -s 4m
	Output is in kBytes/sec
	Time Resolution = 0.000003 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
                                                                    random    random      bkwd     record     stride                                        
              kB  reclen    write    rewrite      read    reread      read     write      read    rewrite       read    fwrite  frewrite     fread   freread
            4096       1     77127    140615    102320    108491     79884     92615[37m[952.622427 0:5226 axfs::fops:269] [33m[AxError::InvalidInput][m
[m[37m[952.710832 0:5226 axfs::fops:269] [33m[AxError::InvalidInput][m
[m     49294      58734      70537    135070    135899    106020    108093

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

	Run began: Thu Jan  1 00:15:54 1970

	Record Size 1 kB
	File size set to 1024 kB
	Command line used: ./iozone -t 4 -i 0 -i 1 -r 1k -s 1m
	Output is in kBytes/sec
	Time Resolution = 0.000003 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
	Throughput test with 4 processes
	Each process writes a 1024 kByte file in 1 kByte records

	Children see throughput for  4 initial writers 	=  103080.34 kB/sec
	Parent sees throughput for  4 initial writers 	=    2727.44 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  103080.34 kB/sec
	Avg throughput per process 			=   25770.08 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=  124392.61 kB/sec
	Parent sees throughput for  4 rewriters 	=    3065.96 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  124392.61 kB/sec
	Avg throughput per process 			=   31098.15 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 readers 		=   53338.89 kB/sec
	Parent sees throughput for  4 readers 		=    2272.84 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   53338.89 kB/sec
	Avg throughput per process 			=   13334.72 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 re-readers 	=   99572.15 kB/sec
	Parent sees throughput for 4 re-readers 	=    2126.98 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   99572.15 kB/sec
	Avg throughput per process 			=   24893.04 kB/sec
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

	Run began: Thu Jan  1 00:16:13 1970

	Record Size 1 kB
	File size set to 1024 kB
	Command line used: ./iozone -t 4 -i 0 -i 2 -r 1k -s 1m
	Output is in kBytes/sec
	Time Resolution = 0.000003 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
	Throughput test with 4 processes
	Each process writes a 1024 kByte file in 1 kByte records

	Children see throughput for  4 initial writers 	=   97775.23 kB/sec
	Parent sees throughput for  4 initial writers 	=    2106.15 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   97775.23 kB/sec
	Avg throughput per process 			=   24443.81 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=  117579.51 kB/sec
	Parent sees throughput for  4 rewriters 	=    2645.03 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  117579.51 kB/sec
	Avg throughput per process 			=   29394.88 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 random readers 	=   80477.84 kB/sec
	Parent sees throughput for 4 random readers 	=    2880.39 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   80477.84 kB/sec
	Avg throughput per process 			=   20119.46 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for 4 random writers 	=   41654.80 kB/sec
	Parent sees throughput for 4 random writers 	=    2591.62 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   41654.80 kB/sec
	Avg throughput per process 			=   10413.70 kB/sec
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

	Run began: Thu Jan  1 00:16:35 1970

	Record Size 1 kB
	File size set to 1024 kB
	Command line used: ./iozone -t 4 -i 0 -i 3 -r 1k -s 1m
	Output is in kBytes/sec
	Time Resolution = 0.000003 seconds.
	Processor cache size set to 1024 kBytes.
	Processor cache line size set to 32 bytes.
	File stride size set to 17 * record size.
	Throughput test with 4 processes
	Each process writes a 1024 kByte file in 1 kByte records

	Children see throughput for  4 initial writers 	=  107834.88 kB/sec
	Parent sees throughput for  4 initial writers 	=    2789.76 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=  107834.88 kB/sec
	Avg throughput per process 			=   26958.72 kB/sec
	Min xfer 					=       0.00 kB

	Children see throughput for  4 rewriters 	=   69983.60 kB/sec
	Parent sees throughput for  4 rewriters 	=    2479.35 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   69983.60 kB/sec
	Avg throughput per process 			=   17495.90 kB/sec
	Min xfer 					=       0.00 kB
[37m[1004.312708 0:5281 axfs::fops:269] [33m[AxError::InvalidInput][m
[m
	Children see throughput for 4 reverse readers 	=   73069.79 kB/sec
	Parent sees throughput for 4 reverse readers 	=    2394.02 kB/sec
	Min throughput per process 			=       0.00 kB/sec 
	Max throughput per process 			=   73069.79 kB/sec
	Avg throughput per process 			=   18267.45 kB/sec
	Min xfer 					=       0.00 kB
autorun: /tmp/testsuite/glibc/iozone/iozone_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START unixbench-musl ####
Unixbench DHRY2 test(lps): 20254454
Unixbench WHETSTONE test(MFLOPS): 202.039
Unixbench SYSCALL test(lps): 729072
Unixbench CONTEXT test(lps): 16694
autorun: /tmp/testsuite/musl/unixbench/unixbench_testcode.sh timed out after 60s
#### OS COMP TEST GROUP START unixbench-glibc ####
Unixbench DHRY2 test(lps): 28855035
Unixbench WHETSTONE test(MFLOPS): 218.986
Unixbench SYSCALL test(lps): 763638
Unixbench CONTEXT test(lps): 14190
autorun: /tmp/testsuite/glibc/unixbench/unixbench_testcode.sh timed out after 60s
[37m[1134.127725 0:2 axplat_loongarch64_qemu_virt::power:23] [32mShutting down...[m
[m