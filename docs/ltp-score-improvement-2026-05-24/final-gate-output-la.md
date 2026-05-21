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
    Finished `release` profile [optimized] target(s) in 21.34s
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

[37m[  0.026613 0 axruntime:135] [32mLogging is enabled.[m
[m[37m[  0.031109 0 axruntime:136] [32mPrimary CPU 0 started, arg = 0x0.[m
[m[37m[  0.034765 0 axruntime:139] [32mFound physcial memory regions:[m
[m[37m[  0.040634 0 axruntime:141] [32m  [PA:0x100d0000, PA:0x100d1000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.044948 0 axruntime:141] [32m  [PA:0x100e0000, PA:0x100e1000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.048079 0 axruntime:141] [32m  [PA:0x1fe00000, PA:0x1fe01000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.051864 0 axruntime:141] [32m  [PA:0x20000000, PA:0x30000000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.053611 0 axruntime:141] [32m  [PA:0x40000000, PA:0x40020000) mmio (READ | WRITE | DEVICE | RESERVED)[m
[m[37m[  0.055434 0 axruntime:141] [32m  [PA:0x80000000, PA:0x80122000) .text (READ | EXECUTE | RESERVED)[m
[m[37m[  0.057036 0 axruntime:141] [32m  [PA:0x80122000, PA:0x8014e000) .rodata (READ | RESERVED)[m
[m[37m[  0.058424 0 axruntime:141] [32m  [PA:0x8014e000, PA:0x80153000) .data .tdata .tbss .percpu (READ | WRITE | RESERVED)[m
[m[37m[  0.061465 0 axruntime:141] [32m  [PA:0x80153000, PA:0x80193000) boot stack (READ | WRITE | RESERVED)[m
[m[37m[  0.062966 0 axruntime:141] [32m  [PA:0x80193000, PA:0x801ba000) .bss (READ | WRITE | RESERVED)[m
[m[37m[  0.064405 0 axruntime:141] [32m  [PA:0x801ba000, PA:0xb0000000) free memory (READ | WRITE | FREE)[m
[m[37m[  0.065982 0 axruntime:216] [32mInitialize global memory allocator...[m
[m[37m[  0.068795 0 axruntime:217] [32m  use TLSF allocator.[m
[m[37m[  0.080979 0 axmm:103] [32mInitialize virtual memory management...[m
[m[37m[  0.143899 0 axruntime:156] [32mInitialize platform devices...[m
[msmp = 1
[37m[  0.152474 0 axtask::api:73] [32mInitialize scheduling...[m
[m[37m[  0.170359 0 axtask::api:83] [32m  use Round-robin scheduler.[m
[m[37m[  0.182714 0 axdriver:152] [32mInitialize device drivers...[m
[m[37m[  0.189234 0 axdriver:153] [32m  device model: static[m
[m[37m[  0.215904 0 virtio_drivers::device::blk:63] [32mfound a block device of size 4194304KB[m
[m[37m[  0.230494 0 axdriver::bus::pci:107] [32mregistered a new Block device at 00:01.0: "virtio-blk"[m
[m[37m[  0.247257 0 virtio_drivers::device::net::dev_raw:33] [32mnegotiated_features Features(MAC | STATUS | RING_INDIRECT_DESC | RING_EVENT_IDX | VERSION_1)[m
[m[37m[  0.280071 0 axdriver::bus::pci:107] [32mregistered a new Net device at 00:02.0: "virtio-net"[m
[m[37m[  0.362334 0 axfs:44] [32mInitialize filesystems...[m
[m[37m[  0.379862 0 axfs:47] [32m  use block device 0: "virtio-blk"[m
[m[37m[  0.386965 0 axfs::root:336] [32m  detected root filesystem: Ext4[m
[m[37m[  0.428424 0 axnet:42] [32mInitialize network subsystem...[m
[m[37m[  0.446317 0 axnet:45] [32m  use NIC 0: "virtio-net"[m
[m[37m[  0.459302 0 axnet::smoltcp_impl:335] [32mcreated net interface "eth0":[m
[m[37m[  0.461187 0 axnet::smoltcp_impl:336] [32m  ether:    52-54-00-12-34-56[m
[m[37m[  0.469299 0 axnet::smoltcp_impl:337] [32m  ip:       10.0.2.15/24[m
[m[37m[  0.476198 0 axnet::smoltcp_impl:338] [32m  gateway:  10.0.2.2[m
[m[37m[  0.484362 0 axruntime:182] [32mInitialize interrupt handlers...[m
[m[37m[  0.493758 0 axruntime:194] [32mPrimary CPU 0 init OK.[m
[mautorun: skip disabled test group /musl/libctest_testcode.sh
autorun: skip disabled test group /glibc/libctest_testcode.sh
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
pid:12
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
pid = 24
========== END test_getpid ==========
Testing getppid :
========== START test_getppid ==========
  getppid success. ppid : 8
========== END test_getppid ==========
Testing gettimeofday :
========== START test_gettimeofday ==========
gettimeofday success.
start:9079, end:9140
interval: 61
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
cpid: 34
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
  I am child process: 48. iteration 1.
  I am child process: 49. iteration 2.
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
pid:59
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
pid = 71
========== END test_getpid ==========
Testing getppid :
========== START test_getppid ==========
  getppid success. ppid : 55
========== END test_getppid ==========
Testing gettimeofday :
========== START test_gettimeofday ==========
gettimeofday success.
start:30997, end:31044
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
cpid: 81
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
  I am child process: 94. iteration 0.
  I am child process: 95. iteration 1.
  I am child process: 96. iteration 2.
  I am child process: 94. iteration 0.
  I am child process: 95. iteration 1.
  I am child process: 96. iteration 2.
  I am child process: 94. iteration 0.
  I am child process: 95. iteration 1.
  I am child process: 96. iteration 2.
  I am child process: 94. iteration 0.
  I am child process: 95. iteration 1.
  I am child process: 96. iteration 2.
  I am child process: 94. iteration 0.
  I am child process: 95. iteration 1.
  I am child process: 96. iteration 2.
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
Thu Jan  1 00:00:49 UTC 1970
testcase busybox date success
Filesystem           1K-blocks      Used Available Use% Mounted on
devfs                   784664     72828    711836   9% /dev
tmpfs                   784664     72828    711836   9% /tmp
tmpfs                   784664     72828    711836   9% /var
proc                    784664     72828    711836   9% /proc
sysfs                   784664     72828    711836   9% /sys
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
 00:01:02 up 1 min,  0 users,  load average: 0.00, 0.00, 0.00
testcase busybox uptime success
abc
testcase busybox printf "abc\n" success
PID   USER     TIME  COMMAND
testcase busybox ps success
/tmp/testsuite/musl/busybox
testcase busybox pwd success
              total        used        free      shared  buff/cache   available
Mem:         784664       74064      710600           0           0      781804
-/+ buffers/cache:        74064      710600
Swap:             0           0           0
testcase busybox free success
Thu Jan  1 00:01:09 1970  0.000000 seconds
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
Thu Jan  1 00:02:17 UTC 1970
testcase busybox date success
Filesystem           1K-blocks      Used Available Use% Mounted on
devfs                   784664    146108    638556  19% /dev
tmpfs                   784664    146108    638556  19% /tmp
tmpfs                   784664    146108    638556  19% /var
proc                    784664    146108    638556  19% /proc
sysfs                   784664    146108    638556  19% /sys
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
 00:02:39 up 2 min,  0 users,  load average: 0.00, 0.00, 0.00
testcase busybox uptime success
abc
testcase busybox printf "abc\n" success
PID   USER     TIME  COMMAND
testcase busybox ps success
/tmp/testsuite/glibc/busybox
testcase busybox pwd success
              total        used        free      shared  buff/cache   available
Mem:         784664      147196      637468           0           0      781804
-/+ buffers/cache:       147196      637468
Swap:             0           0           0
testcase busybox free success
Thu Jan  1 00:02:50 1970  0.000000 seconds
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
ltp case list: stable (85 cases, timeout 15s)
========== START ltp access01 ==========
RUN LTP CASE access01
LTP MEMORY access01 before: free_frames=159925 allocated_frames=36241
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
[37m[277.286485 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.288817 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.289550 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.291040 0:480 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[277.292728 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.293593 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.294406 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.295457 0:480 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[277.296869 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.297682 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.298546 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.299520 0:480 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[277.300883 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.301689 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.302594 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.303675 0:480 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[277.305069 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.305902 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.306711 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.307696 0:480 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[277.309156 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.309998 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.310792 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.311802 0:480 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[277.313202 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.314048 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.314764 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.315542 0:480 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[277.316843 0:480 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE access01 : 0
Pass!
LTP MEMORY access01 after_run: free_frames=159101 allocated_frames=37065
LTP MEMORY access01 after_cleanup: free_frames=159101 allocated_frames=37065
LTP CASE RUNTIME access01: 4089 ms
========== END ltp access01 ==========
========== START ltp brk01 ==========
RUN LTP CASE brk01
LTP MEMORY brk01 before: free_frames=159101 allocated_frames=37065
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
[37m[278.452441 0:585 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE brk01 : 0
Pass!
LTP MEMORY brk01 after_run: free_frames=159077 allocated_frames=37089
LTP MEMORY brk01 after_cleanup: free_frames=159077 allocated_frames=37089
LTP CASE RUNTIME brk01: 1133 ms
========== END ltp brk01 ==========
========== START ltp chdir01 ==========
RUN LTP CASE chdir01
LTP MEMORY chdir01 before: free_frames=159077 allocated_frames=37089
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
tst_test.c:1120: [1;34mTINFO: [0mMounting ltp-tmpfs to /tmp/ltp-work/LTP_chdDCLFfc/mntpoint fstyp=tmpfs flags=0
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
[37m[279.850640 0:592 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[279.854263 0:592 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[279.857059 0:592 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[279.860715 0:592 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[279.861641 0:592 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE chdir01 : 0
Pass!
LTP MEMORY chdir01 after_run: free_frames=159061 allocated_frames=37105
LTP MEMORY chdir01 after_cleanup: free_frames=159061 allocated_frames=37105
LTP CASE RUNTIME chdir01: 1403 ms
========== END ltp chdir01 ==========
========== START ltp clone01 ==========
RUN LTP CASE clone01
LTP MEMORY clone01 before: free_frames=159061 allocated_frames=37105
tst_buffers.c:57: [1;34mTINFO: [0mTest is using guarded buffers
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
clone01.c:37: [1;32mTPASS: [0mclone returned 599
clone01.c:43: [1;32mTPASS: [0mChild exited with 0

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[280.977915 0:596 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE clone01 : 0
Pass!
LTP MEMORY clone01 after_run: free_frames=159037 allocated_frames=37129
LTP MEMORY clone01 after_cleanup: free_frames=159037 allocated_frames=37129
LTP CASE RUNTIME clone01: 1114 ms
========== END ltp clone01 ==========
========== START ltp close01 ==========
RUN LTP CASE close01
LTP MEMORY close01 before: free_frames=159037 allocated_frames=37129
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
[37m[282.080537 0:601 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[282.081785 0:601 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE close01 : 0
Pass!
LTP MEMORY close01 after_run: free_frames=159021 allocated_frames=37145
LTP MEMORY close01 after_cleanup: free_frames=159021 allocated_frames=37145
LTP CASE RUNTIME close01: 1103 ms
========== END ltp close01 ==========
========== START ltp dup01 ==========
RUN LTP CASE dup01
LTP MEMORY dup01 before: free_frames=159021 allocated_frames=37145
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
dup01.c:24: [1;32mTPASS: [0mdup(fd) returned fd 4
dup01.c:27: [1;32mTPASS: [0mbuf1.st_ino == buf2.st_ino (13040937935916186281)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[283.171022 0:605 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[283.175515 0:605 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE dup01 : 0
Pass!
LTP MEMORY dup01 after_run: free_frames=159005 allocated_frames=37161
LTP MEMORY dup01 after_cleanup: free_frames=159005 allocated_frames=37161
LTP CASE RUNTIME dup01: 1108 ms
========== END ltp dup01 ==========
========== START ltp fcntl01 ==========
RUN LTP CASE fcntl01
LTP MEMORY fcntl01 before: free_frames=159005 allocated_frames=37161
[37m[284.272851 0:609 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE fcntl01 : 0
Pass!
LTP MEMORY fcntl01 after_run: free_frames=158997 allocated_frames=37169
LTP MEMORY fcntl01 after_cleanup: free_frames=158997 allocated_frames=37169
LTP CASE RUNTIME fcntl01: 1091 ms
========== END ltp fcntl01 ==========
========== START ltp fcntl02 ==========
RUN LTP CASE fcntl02
LTP MEMORY fcntl02 before: free_frames=158997 allocated_frames=37169
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_612, F_DUPFD, 0) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_612, F_DUPFD, 1) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_612, F_DUPFD, 2) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_612, F_DUPFD, 3) returned 4
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_612, F_DUPFD, 10) returned 10
fcntl02.c:41: [1;32mTPASS: [0mfcntl(fcntl02_612, F_DUPFD, 100) returned 100

Summary:
passed   6
failed   0
broken   0
skipped  0
warnings 0
[37m[285.423883 0:610 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[285.426042 0:610 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE fcntl02 : 0
Pass!
LTP MEMORY fcntl02 after_run: free_frames=158981 allocated_frames=37185
LTP MEMORY fcntl02 after_cleanup: free_frames=158981 allocated_frames=37185
LTP CASE RUNTIME fcntl02: 1137 ms
========== END ltp fcntl02 ==========
========== START ltp fork01 ==========
RUN LTP CASE fork01
LTP MEMORY fork01 before: free_frames=158981 allocated_frames=37185
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
fork01.c:47: [1;32mTPASS: [0mcorrect child status returned 42
fork01.c:50: [1;32mTPASS: [0mchild_pid == pid (617)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[286.568156 0:614 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[286.570433 0:614 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE fork01 : 0
Pass!
LTP MEMORY fork01 after_run: free_frames=158957 allocated_frames=37209
LTP MEMORY fork01 after_cleanup: free_frames=158957 allocated_frames=37209
LTP CASE RUNTIME fork01: 1142 ms
========== END ltp fork01 ==========
========== START ltp getpid01 ==========
RUN LTP CASE getpid01
LTP MEMORY getpid01 before: free_frames=158957 allocated_frames=37209
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 622
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 623
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 624
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 625
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 626
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 627
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 628
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 629
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 630
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 631
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 632
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 633
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 634
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 635
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 636
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 637
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 638
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 639
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 640
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 641
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 642
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 643
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 644
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 645
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 646
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 647
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 648
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 649
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 650
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 651
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 652
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 653
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 654
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 655
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 656
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 657
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 658
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 659
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 660
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 661
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 662
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 663
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 664
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 665
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 666
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 667
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 668
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 669
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 670
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 671
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 672
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 673
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 674
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 675
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 676
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 677
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 678
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 679
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 680
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 681
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 682
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 683
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 684
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 685
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 686
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 687
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 688
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 689
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 690
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 691
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 692
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 693
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 694
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 695
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 696
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 697
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 698
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 699
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 700
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 701
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 702
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 703
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 704
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 705
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 706
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 707
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 708
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 709
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 710
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 711
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 712
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 713
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 714
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 715
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 716
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 717
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 718
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 719
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 720
getpid01.c:34: [1;32mTPASS: [0mgetpid() returns 721

Summary:
passed   100
failed   0
broken   0
skipped  0
warnings 0
[37m[290.053027 0:619 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getpid01 : 0
Pass!
LTP MEMORY getpid01 after_run: free_frames=158141 allocated_frames=38025
LTP MEMORY getpid01 after_cleanup: free_frames=158141 allocated_frames=38025
LTP CASE RUNTIME getpid01: 3475 ms
========== END ltp getpid01 ==========
========== START ltp mmap01 ==========
RUN LTP CASE mmap01
LTP MEMORY mmap01 before: free_frames=158141 allocated_frames=38025
mmap01      1  [1;32mTPASS[0m  :  Functionality of mmap() successful
[37m[291.087127 0:723 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[291.088323 0:723 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE mmap01 : 0
Pass!
LTP MEMORY mmap01 after_run: free_frames=158125 allocated_frames=38041
LTP MEMORY mmap01 after_cleanup: free_frames=158125 allocated_frames=38041
LTP CASE RUNTIME mmap01: 1033 ms
========== END ltp mmap01 ==========
========== START ltp open01 ==========
RUN LTP CASE open01
LTP MEMORY open01 before: free_frames=158125 allocated_frames=38041
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
[37m[292.195840 0:725 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[292.197127 0:725 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE open01 : 0
Pass!
LTP MEMORY open01 after_run: free_frames=158109 allocated_frames=38057
LTP MEMORY open01 after_cleanup: free_frames=158109 allocated_frames=38057
LTP CASE RUNTIME open01: 1113 ms
========== END ltp open01 ==========
========== START ltp pipe01 ==========
RUN LTP CASE pipe01
LTP MEMORY pipe01 before: free_frames=158109 allocated_frames=38057
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
[37m[293.361030 0:729 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE pipe01 : 0
Pass!
LTP MEMORY pipe01 after_run: free_frames=158093 allocated_frames=38073
LTP MEMORY pipe01 after_cleanup: free_frames=158093 allocated_frames=38073
LTP CASE RUNTIME pipe01: 1170 ms
========== END ltp pipe01 ==========
========== START ltp read01 ==========
RUN LTP CASE read01
LTP MEMORY read01 before: free_frames=158093 allocated_frames=38073
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
[37m[294.385313 0:733 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[294.387043 0:733 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE read01 : 0
Pass!
LTP MEMORY read01 after_run: free_frames=158077 allocated_frames=38089
LTP MEMORY read01 after_cleanup: free_frames=158077 allocated_frames=38089
LTP CASE RUNTIME read01: 1017 ms
========== END ltp read01 ==========
========== START ltp stat01 ==========
RUN LTP CASE stat01
LTP MEMORY stat01 before: free_frames=158077 allocated_frames=38089
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
[37m[295.474455 0:737 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[295.476199 0:737 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[295.478044 0:737 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE stat01 : 0
Pass!
LTP MEMORY stat01 after_run: free_frames=158061 allocated_frames=38105
LTP MEMORY stat01 after_cleanup: free_frames=158061 allocated_frames=38105
LTP CASE RUNTIME stat01: 1102 ms
========== END ltp stat01 ==========
========== START ltp wait401 ==========
RUN LTP CASE wait401
LTP MEMORY wait401 before: free_frames=158061 allocated_frames=38105
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
wait401.c:40: [1;32mTPASS: [0mwait4() returned correct pid 744
wait401.c:49: [1;32mTPASS: [0mWIFEXITED() is set in status
wait401.c:54: [1;32mTPASS: [0mWEXITSTATUS() == 0

Summary:
passed   3
failed   0
broken   0
skipped  0
warnings 0
[37m[296.669940 0:741 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE wait401 : 0
Pass!
LTP MEMORY wait401 after_run: free_frames=158037 allocated_frames=38129
LTP MEMORY wait401 after_cleanup: free_frames=158037 allocated_frames=38129
LTP CASE RUNTIME wait401: 1205 ms
========== END ltp wait401 ==========
========== START ltp write01 ==========
RUN LTP CASE write01
LTP MEMORY write01 before: free_frames=158037 allocated_frames=38129
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
[37m[297.784529 0:746 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[297.790387 0:746 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE write01 : 0
Pass!
LTP MEMORY write01 after_run: free_frames=158021 allocated_frames=38145
LTP MEMORY write01 after_cleanup: free_frames=158021 allocated_frames=38145
LTP CASE RUNTIME write01: 1089 ms
========== END ltp write01 ==========
========== START ltp access03 ==========
RUN LTP CASE access03
LTP MEMORY access03 before: free_frames=158021 allocated_frames=38145
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
[37m[298.893146 0:750 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE access03 : 0
Pass!
LTP MEMORY access03 after_run: free_frames=157973 allocated_frames=38193
LTP MEMORY access03 after_cleanup: free_frames=157973 allocated_frames=38193
LTP CASE RUNTIME access03: 1093 ms
========== END ltp access03 ==========
========== START ltp close02 ==========
RUN LTP CASE close02
LTP MEMORY close02 before: free_frames=157973 allocated_frames=38193
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
[37m[299.869689 0:759 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE close02 : 0
Pass!
LTP MEMORY close02 after_run: free_frames=157957 allocated_frames=38209
LTP MEMORY close02 after_cleanup: free_frames=157957 allocated_frames=38209
LTP CASE RUNTIME close02: 976 ms
========== END ltp close02 ==========
========== START ltp dup02 ==========
RUN LTP CASE dup02
LTP MEMORY dup02 before: free_frames=157957 allocated_frames=38209
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
[37m[300.802704 0:763 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE dup02 : 0
Pass!
LTP MEMORY dup02 after_run: free_frames=157941 allocated_frames=38225
LTP MEMORY dup02 after_cleanup: free_frames=157941 allocated_frames=38225
LTP CASE RUNTIME dup02: 953 ms
========== END ltp dup02 ==========
========== START ltp fcntl03 ==========
RUN LTP CASE fcntl03
LTP MEMORY fcntl03 before: free_frames=157941 allocated_frames=38225
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
fcntl03.c:32: [1;32mTPASS: [0mfcntl(fcntl03_769, F_GETFD, 0) returned 0

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[301.803056 0:767 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[301.804795 0:767 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE fcntl03 : 0
Pass!
LTP MEMORY fcntl03 after_run: free_frames=157925 allocated_frames=38241
LTP MEMORY fcntl03 after_cleanup: free_frames=157925 allocated_frames=38241
LTP CASE RUNTIME fcntl03: 977 ms
========== END ltp fcntl03 ==========
========== START ltp getcwd01 ==========
RUN LTP CASE getcwd01
LTP MEMORY getcwd01 before: free_frames=157925 allocated_frames=38241
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
[37m[302.905979 0:771 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getcwd01 : 0
Pass!
LTP MEMORY getcwd01 after_run: free_frames=157909 allocated_frames=38257
LTP MEMORY getcwd01 after_cleanup: free_frames=157909 allocated_frames=38257
LTP CASE RUNTIME getcwd01: 1099 ms
========== END ltp getcwd01 ==========
========== START ltp getpid02 ==========
RUN LTP CASE getpid02
LTP MEMORY getpid02 before: free_frames=157909 allocated_frames=38257
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getpid02.c:37: [1;32mTPASS: [0mchild getppid() == parent getpid() (777)
getpid02.c:50: [1;32mTPASS: [0mchild getpid() == parent fork() (778)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[303.990682 0:775 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getpid02 : 0
Pass!
LTP MEMORY getpid02 after_run: free_frames=157885 allocated_frames=38281
LTP MEMORY getpid02 after_cleanup: free_frames=157885 allocated_frames=38281
LTP CASE RUNTIME getpid02: 1080 ms
========== END ltp getpid02 ==========
========== START ltp getppid01 ==========
RUN LTP CASE getppid01
LTP MEMORY getppid01 before: free_frames=157885 allocated_frames=38281
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getppid01.c:31: [1;32mTPASS: [0mgetppid() returned 780

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[305.032284 0:780 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getppid01 : 0
Pass!
LTP MEMORY getppid01 after_run: free_frames=157869 allocated_frames=38297
LTP MEMORY getppid01 after_cleanup: free_frames=157869 allocated_frames=38297
LTP CASE RUNTIME getppid01: 1039 ms
========== END ltp getppid01 ==========
========== START ltp getuid01 ==========
RUN LTP CASE getuid01
LTP MEMORY getuid01 before: free_frames=157869 allocated_frames=38297
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
[37m[306.067833 0:784 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getuid01 : 0
Pass!
LTP MEMORY getuid01 after_run: free_frames=157853 allocated_frames=38313
LTP MEMORY getuid01 after_cleanup: free_frames=157853 allocated_frames=38313
LTP CASE RUNTIME getuid01: 1035 ms
========== END ltp getuid01 ==========
========== START ltp geteuid01 ==========
RUN LTP CASE geteuid01
LTP MEMORY geteuid01 before: free_frames=157853 allocated_frames=38313
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
[37m[307.153340 0:788 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE geteuid01 : 0
Pass!
LTP MEMORY geteuid01 after_run: free_frames=157837 allocated_frames=38329
LTP MEMORY geteuid01 after_cleanup: free_frames=157837 allocated_frames=38329
LTP CASE RUNTIME geteuid01: 1089 ms
========== END ltp geteuid01 ==========
========== START ltp getgid01 ==========
RUN LTP CASE getgid01
LTP MEMORY getgid01 before: free_frames=157837 allocated_frames=38329
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
[37m[308.221536 0:792 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getgid01 : 0
Pass!
LTP MEMORY getgid01 after_run: free_frames=157821 allocated_frames=38345
LTP MEMORY getgid01 after_cleanup: free_frames=157821 allocated_frames=38345
LTP CASE RUNTIME getgid01: 1066 ms
========== END ltp getgid01 ==========
========== START ltp getegid01 ==========
RUN LTP CASE getegid01
LTP MEMORY getegid01 before: free_frames=157821 allocated_frames=38345
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
[37m[309.356110 0:796 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getegid01 : 0
Pass!
LTP MEMORY getegid01 after_run: free_frames=157805 allocated_frames=38361
LTP MEMORY getegid01 after_cleanup: free_frames=157805 allocated_frames=38361
LTP CASE RUNTIME getegid01: 1145 ms
========== END ltp getegid01 ==========
========== START ltp getresuid01 ==========
RUN LTP CASE getresuid01
LTP MEMORY getresuid01 before: free_frames=157805 allocated_frames=38361
getresuid01    1  [1;32mTPASS[0m  :  Functionality of getresuid() successful
FAIL LTP CASE getresuid01 : 0
Pass!
LTP MEMORY getresuid01 after_run: free_frames=157797 allocated_frames=38369
LTP MEMORY getresuid01 after_cleanup: free_frames=157797 allocated_frames=38369
LTP CASE RUNTIME getresuid01: 1051 ms
========== END ltp getresuid01 ==========
========== START ltp getresuid02 ==========
RUN LTP CASE getresuid02
LTP MEMORY getresuid02 before: free_frames=157797 allocated_frames=38369
getresuid02    1  [1;32mTPASS[0m  :  Functionality of getresuid() successful
FAIL LTP CASE getresuid02 : 0
Pass!
LTP MEMORY getresuid02 after_run: free_frames=157789 allocated_frames=38377
LTP MEMORY getresuid02 after_cleanup: free_frames=157789 allocated_frames=38377
LTP CASE RUNTIME getresuid02: 1009 ms
========== END ltp getresuid02 ==========
========== START ltp getresuid03 ==========
RUN LTP CASE getresuid03
LTP MEMORY getresuid03 before: free_frames=157789 allocated_frames=38377
getresuid03    1  [1;32mTPASS[0m  :  Functionality of getresuid() successful
FAIL LTP CASE getresuid03 : 0
Pass!
LTP MEMORY getresuid03 after_run: free_frames=157781 allocated_frames=38385
LTP MEMORY getresuid03 after_cleanup: free_frames=157781 allocated_frames=38385
LTP CASE RUNTIME getresuid03: 1082 ms
========== END ltp getresuid03 ==========
========== START ltp getresgid01 ==========
RUN LTP CASE getresgid01
LTP MEMORY getresgid01 before: free_frames=157781 allocated_frames=38385
getresgid01    1  [1;32mTPASS[0m  :  Functionality of getresgid() successful
FAIL LTP CASE getresgid01 : 0
Pass!
LTP MEMORY getresgid01 after_run: free_frames=157773 allocated_frames=38393
LTP MEMORY getresgid01 after_cleanup: free_frames=157773 allocated_frames=38393
LTP CASE RUNTIME getresgid01: 1060 ms
========== END ltp getresgid01 ==========
========== START ltp getresgid02 ==========
RUN LTP CASE getresgid02
LTP MEMORY getresgid02 before: free_frames=157773 allocated_frames=38393
getresgid02    1  [1;32mTPASS[0m  :  Functionality of getresgid() successful
FAIL LTP CASE getresgid02 : 0
Pass!
LTP MEMORY getresgid02 after_run: free_frames=157765 allocated_frames=38401
LTP MEMORY getresgid02 after_cleanup: free_frames=157765 allocated_frames=38401
LTP CASE RUNTIME getresgid02: 996 ms
========== END ltp getresgid02 ==========
========== START ltp getresgid03 ==========
RUN LTP CASE getresgid03
LTP MEMORY getresgid03 before: free_frames=157765 allocated_frames=38401
getresgid03    1  [1;32mTPASS[0m  :  Functionality of getresgid() successful
FAIL LTP CASE getresgid03 : 0
Pass!
LTP MEMORY getresgid03 after_run: free_frames=157757 allocated_frames=38409
LTP MEMORY getresgid03 after_cleanup: free_frames=157757 allocated_frames=38409
LTP CASE RUNTIME getresgid03: 1005 ms
========== END ltp getresgid03 ==========
========== START ltp lseek01 ==========
RUN LTP CASE lseek01
LTP MEMORY lseek01 before: free_frames=157757 allocated_frames=38409
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
[37m[316.677177 0:806 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[316.680713 0:806 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE lseek01 : 0
Pass!
LTP MEMORY lseek01 after_run: free_frames=157741 allocated_frames=38425
LTP MEMORY lseek01 after_cleanup: free_frames=157741 allocated_frames=38425
LTP CASE RUNTIME lseek01: 1092 ms
========== END ltp lseek01 ==========
========== START ltp read02 ==========
RUN LTP CASE read02
LTP MEMORY read02 before: free_frames=157741 allocated_frames=38425
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
[37m[317.747715 0:810 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[317.749975 0:810 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE read02 : 0
Pass!
LTP MEMORY read02 after_run: free_frames=157725 allocated_frames=38441
LTP MEMORY read02 after_cleanup: free_frames=157725 allocated_frames=38441
LTP CASE RUNTIME read02: 1065 ms
========== END ltp read02 ==========
========== START ltp write02 ==========
RUN LTP CASE write02
LTP MEMORY write02 before: free_frames=157725 allocated_frames=38441
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
[37m[318.893403 0:814 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[318.895821 0:814 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE write02 : 0
Pass!
LTP MEMORY write02 after_run: free_frames=157709 allocated_frames=38457
LTP MEMORY write02 after_cleanup: free_frames=157709 allocated_frames=38457
LTP CASE RUNTIME write02: 1129 ms
========== END ltp write02 ==========
========== START ltp creat01 ==========
RUN LTP CASE creat01
LTP MEMORY creat01 before: free_frames=157709 allocated_frames=38457
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
[37m[319.972874 0:818 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[319.974137 0:818 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE creat01 : 0
Pass!
LTP MEMORY creat01 after_run: free_frames=157693 allocated_frames=38473
LTP MEMORY creat01 after_cleanup: free_frames=157693 allocated_frames=38473
LTP CASE RUNTIME creat01: 1076 ms
========== END ltp creat01 ==========
========== START ltp creat03 ==========
RUN LTP CASE creat03
LTP MEMORY creat03 before: free_frames=157693 allocated_frames=38473
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
[37m[321.067462 0:822 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[321.068592 0:822 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE creat03 : 0
Pass!
LTP MEMORY creat03 after_run: free_frames=157677 allocated_frames=38489
LTP MEMORY creat03 after_cleanup: free_frames=157677 allocated_frames=38489
LTP CASE RUNTIME creat03: 1093 ms
========== END ltp creat03 ==========
========== START ltp open02 ==========
RUN LTP CASE open02
LTP MEMORY open02 before: free_frames=157677 allocated_frames=38489
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
[37m[322.116822 0:826 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[322.120195 0:826 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE open02 : 0
Pass!
LTP MEMORY open02 after_run: free_frames=157661 allocated_frames=38505
LTP MEMORY open02 after_cleanup: free_frames=157661 allocated_frames=38505
LTP CASE RUNTIME open02: 1053 ms
========== END ltp open02 ==========
========== START ltp open03 ==========
RUN LTP CASE open03
LTP MEMORY open03 before: free_frames=157661 allocated_frames=38505
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
[37m[323.222261 0:830 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE open03 : 0
Pass!
LTP MEMORY open03 after_run: free_frames=157645 allocated_frames=38521
LTP MEMORY open03 after_cleanup: free_frames=157645 allocated_frames=38521
LTP CASE RUNTIME open03: 1109 ms
========== END ltp open03 ==========
========== START ltp stat02 ==========
RUN LTP CASE stat02
LTP MEMORY stat02 before: free_frames=157645 allocated_frames=38521
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
[37m[324.321452 0:834 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE stat02 : 0
Pass!
LTP MEMORY stat02 after_run: free_frames=157629 allocated_frames=38537
LTP MEMORY stat02 after_cleanup: free_frames=157629 allocated_frames=38537
LTP CASE RUNTIME stat02: 1092 ms
========== END ltp stat02 ==========
========== START ltp lstat01 ==========
RUN LTP CASE lstat01
LTP MEMORY lstat01 before: free_frames=157629 allocated_frames=38537
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
[37m[325.468640 0:838 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[325.479603 0:838 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE lstat01 : 0
Pass!
LTP MEMORY lstat01 after_run: free_frames=157613 allocated_frames=38553
LTP MEMORY lstat01 after_cleanup: free_frames=157613 allocated_frames=38553
LTP CASE RUNTIME lstat01: 1153 ms
========== END ltp lstat01 ==========
========== START ltp chmod01 ==========
RUN LTP CASE chmod01
LTP MEMORY chmod01 before: free_frames=157613 allocated_frames=38553
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
[37m[326.593293 0:842 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[326.594370 0:842 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[326.595554 0:842 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE chmod01 : 0
Pass!
LTP MEMORY chmod01 after_run: free_frames=157589 allocated_frames=38577
LTP MEMORY chmod01 after_cleanup: free_frames=157589 allocated_frames=38577
LTP CASE RUNTIME chmod01: 1113 ms
========== END ltp chmod01 ==========
========== START ltp fchmod01 ==========
RUN LTP CASE fchmod01
LTP MEMORY fchmod01 before: free_frames=157589 allocated_frames=38577
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
[37m[327.689399 0:849 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[327.692375 0:849 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE fchmod01 : 0
Pass!
LTP MEMORY fchmod01 after_run: free_frames=157573 allocated_frames=38593
LTP MEMORY fchmod01 after_cleanup: free_frames=157573 allocated_frames=38593
LTP CASE RUNTIME fchmod01: 1107 ms
========== END ltp fchmod01 ==========
========== START ltp rmdir01 ==========
RUN LTP CASE rmdir01
LTP MEMORY rmdir01 before: free_frames=157573 allocated_frames=38593
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
[37m[328.703839 0:853 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE rmdir01 : 0
Pass!
LTP MEMORY rmdir01 after_run: free_frames=157557 allocated_frames=38609
LTP MEMORY rmdir01 after_cleanup: free_frames=157557 allocated_frames=38609
LTP CASE RUNTIME rmdir01: 1011 ms
========== END ltp rmdir01 ==========
========== START ltp symlink01 ==========
RUN LTP CASE symlink01
LTP MEMORY symlink01 before: free_frames=157557 allocated_frames=38609
symlink01    1  [1;32mTPASS[0m  :  Creation of symbolic link file to no object file is ok
symlink01    2  [1;32mTPASS[0m  :  Creation of symbolic link file to no object file is ok
symlink01    3  [1;32mTPASS[0m  :  Creation of symbolic link file and object file via symbolic link is ok
symlink01    4  [1;32mTPASS[0m  :  Creating an existing symbolic link file error is caught
symlink01    5  [1;32mTPASS[0m  :  Creating a symbolic link which exceeds maximum pathname error is caught
[37m[329.652486 0:857 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE symlink01 : 0
Pass!
LTP MEMORY symlink01 after_run: free_frames=157549 allocated_frames=38617
LTP MEMORY symlink01 after_cleanup: free_frames=157549 allocated_frames=38617
LTP CASE RUNTIME symlink01: 936 ms
========== END ltp symlink01 ==========
========== START ltp readlink01 ==========
RUN LTP CASE readlink01
LTP MEMORY readlink01 before: free_frames=157549 allocated_frames=38617
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
[37m[330.832486 0:858 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[330.834773 0:858 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE readlink01 : 0
Pass!
LTP MEMORY readlink01 after_run: free_frames=157525 allocated_frames=38641
LTP MEMORY readlink01 after_cleanup: free_frames=157525 allocated_frames=38641
LTP CASE RUNTIME readlink01: 1182 ms
========== END ltp readlink01 ==========
========== START ltp ftruncate01 ==========
RUN LTP CASE ftruncate01
LTP MEMORY ftruncate01 before: free_frames=157525 allocated_frames=38641
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
[37m[331.894144 0:863 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[331.895290 0:863 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE ftruncate01 : 0
Pass!
LTP MEMORY ftruncate01 after_run: free_frames=157509 allocated_frames=38657
LTP MEMORY ftruncate01 after_cleanup: free_frames=157509 allocated_frames=38657
LTP CASE RUNTIME ftruncate01: 1041 ms
========== END ltp ftruncate01 ==========
========== START ltp umask01 ==========
RUN LTP CASE umask01
LTP MEMORY umask01 before: free_frames=157509 allocated_frames=38657
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
[37m[333.157914 0:867 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE umask01 : 0
Pass!
LTP MEMORY umask01 after_run: free_frames=157493 allocated_frames=38673
LTP MEMORY umask01 after_cleanup: free_frames=157493 allocated_frames=38673
LTP CASE RUNTIME umask01: 1270 ms
========== END ltp umask01 ==========
========== START ltp alarm02 ==========
RUN LTP CASE alarm02
LTP MEMORY alarm02 before: free_frames=157493 allocated_frames=38673
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
[37m[334.397920 0:871 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE alarm02 : 0
Pass!
LTP MEMORY alarm02 after_run: free_frames=157477 allocated_frames=38689
LTP MEMORY alarm02 after_cleanup: free_frames=157477 allocated_frames=38689
LTP CASE RUNTIME alarm02: 1248 ms
========== END ltp alarm02 ==========
========== START ltp alarm03 ==========
RUN LTP CASE alarm03
LTP MEMORY alarm03 before: free_frames=157477 allocated_frames=38689
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
[37m[335.587786 0:878 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE alarm03 : 0
Pass!
LTP MEMORY alarm03 after_run: free_frames=157453 allocated_frames=38713
LTP MEMORY alarm03 after_cleanup: free_frames=157453 allocated_frames=38713
LTP CASE RUNTIME alarm03: 1183 ms
========== END ltp alarm03 ==========
========== START ltp clock_gettime02 ==========
RUN LTP CASE clock_gettime02
LTP MEMORY clock_gettime02 before: free_frames=157453 allocated_frames=38713
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
[37m[336.733413 0:884 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE clock_gettime02 : 0
Pass!
LTP MEMORY clock_gettime02 after_run: free_frames=157437 allocated_frames=38729
LTP MEMORY clock_gettime02 after_cleanup: free_frames=157437 allocated_frames=38729
LTP CASE RUNTIME clock_gettime02: 1131 ms
========== END ltp clock_gettime02 ==========
========== START ltp gettimeofday01 ==========
RUN LTP CASE gettimeofday01
LTP MEMORY gettimeofday01 before: free_frames=157437 allocated_frames=38729
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
[37m[337.880565 0:888 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE gettimeofday01 : 0
Pass!
LTP MEMORY gettimeofday01 after_run: free_frames=157421 allocated_frames=38745
LTP MEMORY gettimeofday01 after_cleanup: free_frames=157421 allocated_frames=38745
LTP CASE RUNTIME gettimeofday01: 1128 ms
========== END ltp gettimeofday01 ==========
========== START ltp time01 ==========
RUN LTP CASE time01
LTP MEMORY time01 before: free_frames=157421 allocated_frames=38745
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
time01.c:36: [1;32mTPASS: [0mtime() returned value 338
time01.c:38: [1;32mTPASS: [0mtime() returned value 338, stored value 338 are same

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[338.945381 0:892 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE time01 : 0
Pass!
LTP MEMORY time01 after_run: free_frames=157405 allocated_frames=38761
LTP MEMORY time01 after_cleanup: free_frames=157405 allocated_frames=38761
LTP CASE RUNTIME time01: 1062 ms
========== END ltp time01 ==========
========== START ltp times01 ==========
RUN LTP CASE times01
LTP MEMORY times01 before: free_frames=157405 allocated_frames=38761
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
times01.c:25: [1;32mTPASS: [0mtimes(&mytimes) returned 339984

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[339.998120 0:896 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE times01 : 0
Pass!
LTP MEMORY times01 after_run: free_frames=157389 allocated_frames=38777
LTP MEMORY times01 after_cleanup: free_frames=157389 allocated_frames=38777
LTP CASE RUNTIME times01: 1050 ms
========== END ltp times01 ==========
========== START ltp kill03 ==========
RUN LTP CASE kill03
LTP MEMORY kill03 before: free_frames=157389 allocated_frames=38777
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
[37m[341.096052 0:900 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE kill03 : 0
Pass!
LTP MEMORY kill03 after_run: free_frames=157373 allocated_frames=38793
LTP MEMORY kill03 after_cleanup: free_frames=157373 allocated_frames=38793
LTP CASE RUNTIME kill03: 1096 ms
========== END ltp kill03 ==========
========== START ltp rt_sigaction01 ==========
RUN LTP CASE rt_sigaction01
LTP MEMORY rt_sigaction01 before: free_frames=157373 allocated_frames=38793
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
[37m[342.880691 0:904 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE rt_sigaction01 : 0
Pass!
LTP MEMORY rt_sigaction01 after_run: free_frames=157365 allocated_frames=38801
LTP MEMORY rt_sigaction01 after_cleanup: free_frames=157365 allocated_frames=38801
LTP CASE RUNTIME rt_sigaction01: 1778 ms
========== END ltp rt_sigaction01 ==========
========== START ltp rt_sigaction02 ==========
RUN LTP CASE rt_sigaction02
LTP MEMORY rt_sigaction02 before: free_frames=157365 allocated_frames=38801
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 35
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02    1  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 35 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02    2  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 35 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02    3  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 35 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02    4  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 35 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02    5  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 35 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 36
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02    6  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 36 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02    7  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 36 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02    8  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 36 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02    9  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 36 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   10  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 36 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 37
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   11  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 37 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   12  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 37 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   13  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 37 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   14  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 37 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   15  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 37 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 38
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   16  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 38 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   17  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 38 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   18  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 38 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   19  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 38 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   20  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 38 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 39
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   21  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 39 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   22  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 39 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   23  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 39 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   24  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 39 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   25  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 39 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 40
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   26  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 40 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   27  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 40 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   28  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 40 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   29  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 40 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   30  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 40 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 41
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   31  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 41 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   32  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 41 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   33  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 41 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   34  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 41 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   35  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 41 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 42
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   36  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 42 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   37  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 42 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   38  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 42 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   39  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 42 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   40  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 42 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 43
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   41  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 43 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   42  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 43 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   43  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 43 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   44  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 43 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   45  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 43 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 44
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   46  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 44 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   47  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 44 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   48  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 44 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   49  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 44 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   50  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 44 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 45
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   51  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 45 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   52  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 45 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   53  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 45 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   54  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 45 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   55  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 45 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 46
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   56  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 46 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   57  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 46 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   58  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 46 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   59  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 46 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   60  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 46 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 47
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   61  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 47 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   62  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 47 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   63  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 47 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   64  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 47 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   65  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 47 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 48
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   66  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 48 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   67  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 48 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   68  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 48 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   69  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 48 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   70  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 48 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 49
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   71  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 49 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   72  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 49 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   73  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 49 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   74  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 49 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   75  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 49 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 50
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   76  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 50 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   77  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 50 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   78  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 50 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   79  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 50 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   80  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 50 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 51
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   81  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 51 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   82  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 51 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   83  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 51 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   84  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 51 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   85  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 51 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 52
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   86  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 52 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   87  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 52 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   88  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 52 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   89  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 52 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   90  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 52 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 53
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   91  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 53 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   92  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 53 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   93  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 53 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   94  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 53 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   95  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 53 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 54
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   96  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 54 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   97  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 54 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   98  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 54 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   99  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 54 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  100  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 54 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 55
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  101  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 55 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  102  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 55 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  103  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 55 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  104  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 55 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  105  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 55 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 56
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  106  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 56 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  107  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 56 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  108  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 56 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  109  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 56 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  110  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 56 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 57
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  111  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 57 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  112  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 57 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  113  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 57 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  114  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 57 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  115  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 57 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 58
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  116  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 58 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  117  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 58 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  118  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 58 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  119  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 58 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  120  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 58 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 59
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  121  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 59 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  122  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 59 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  123  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 59 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  124  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 59 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  125  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 59 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 60
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  126  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 60 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  127  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 60 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  128  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 60 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  129  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 60 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  130  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 60 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 61
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  131  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 61 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  132  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 61 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  133  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 61 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  134  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 61 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  135  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 61 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 62
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  136  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 62 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  137  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 62 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  138  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 62 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  139  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 62 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  140  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 62 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 63
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  141  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 63 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  142  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 63 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  143  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 63 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  144  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 63 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  145  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 63 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  Signal 64
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  146  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 64 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  147  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 64 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  148  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 64 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  149  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 64 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  [1;34mTINFO[0m  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  150  [1;32mTPASS[0m  :  rt_sigaction02 failure with sig: 64 as expected errno  = EFAULT : Bad address
[37m[344.110719 0:905 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE rt_sigaction02 : 0
Pass!
LTP MEMORY rt_sigaction02 after_run: free_frames=157357 allocated_frames=38809
LTP MEMORY rt_sigaction02 after_cleanup: free_frames=157357 allocated_frames=38809
LTP CASE RUNTIME rt_sigaction02: 1257 ms
========== END ltp rt_sigaction02 ==========
========== START ltp sigaction01 ==========
RUN LTP CASE sigaction01
LTP MEMORY sigaction01 before: free_frames=157357 allocated_frames=38809
sigaction01    1  [1;32mTPASS[0m  :  SA_RESETHAND did not cause SA_SIGINFO to be cleared
sigaction01    2  [1;32mTPASS[0m  :  SA_RESETHAND was masked when handler executed
sigaction01    3  [1;32mTPASS[0m  :  sig has been masked because sa_mask originally contained sig
sigaction01    4  [1;32mTPASS[0m  :  siginfo pointer non NULL
FAIL LTP CASE sigaction01 : 0
Pass!
LTP MEMORY sigaction01 after_run: free_frames=157349 allocated_frames=38817
LTP MEMORY sigaction01 after_cleanup: free_frames=157349 allocated_frames=38817
LTP CASE RUNTIME sigaction01: 1383 ms
========== END ltp sigaction01 ==========
========== START ltp proc01 ==========
RUN LTP CASE proc01
LTP MEMORY proc01 before: free_frames=157349 allocated_frames=38817
proc01      1  [1;32mTPASS[0m  :  readproc() completed successfully, total read: 872 bytes, 20 objs
[37m[346.782262 0:907 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE proc01 : 0
Pass!
LTP MEMORY proc01 after_run: free_frames=157341 allocated_frames=38825
LTP MEMORY proc01 after_cleanup: free_frames=157341 allocated_frames=38825
LTP CASE RUNTIME proc01: 1268 ms
========== END ltp proc01 ==========
========== START ltp exit01 ==========
RUN LTP CASE exit01
LTP MEMORY exit01 before: free_frames=157341 allocated_frames=38825
exit01      1  [1;32mTPASS[0m  :  exit() test PASSED
FAIL LTP CASE exit01 : 0
Pass!
LTP MEMORY exit01 after_run: free_frames=157325 allocated_frames=38841
LTP MEMORY exit01 after_cleanup: free_frames=157325 allocated_frames=38841
LTP CASE RUNTIME exit01: 1043 ms
========== END ltp exit01 ==========
========== START ltp exit02 ==========
RUN LTP CASE exit02
LTP MEMORY exit02 before: free_frames=157325 allocated_frames=38841
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
[37m[348.955163 0:910 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE exit02 : 0
Pass!
LTP MEMORY exit02 after_run: free_frames=157301 allocated_frames=38865
LTP MEMORY exit02 after_cleanup: free_frames=157301 allocated_frames=38865
LTP CASE RUNTIME exit02: 1112 ms
========== END ltp exit02 ==========
========== START ltp exit_group01 ==========
RUN LTP CASE exit_group01
LTP MEMORY exit_group01 before: free_frames=157301 allocated_frames=38865
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
[37m[350.130945 0:915 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[350.132728 0:915 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE exit_group01 : 0
Pass!
LTP MEMORY exit_group01 after_run: free_frames=157277 allocated_frames=38889
LTP MEMORY exit_group01 after_cleanup: free_frames=157277 allocated_frames=38889
LTP CASE RUNTIME exit_group01: 1179 ms
========== END ltp exit_group01 ==========
========== START ltp getpgrp01 ==========
RUN LTP CASE getpgrp01
LTP MEMORY getpgrp01 before: free_frames=157277 allocated_frames=38889
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getpgrp01.c:18: [1;32mTPASS: [0mgetpgrp() returned pid 924
getpgrp01.c:19: [1;32mTPASS: [0mTST_RET == SAFE_GETPGID(0) (924)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[351.257736 0:922 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getpgrp01 : 0
Pass!
LTP MEMORY getpgrp01 after_run: free_frames=157261 allocated_frames=38905
LTP MEMORY getpgrp01 after_cleanup: free_frames=157261 allocated_frames=38905
LTP CASE RUNTIME getpgrp01: 1125 ms
========== END ltp getpgrp01 ==========
========== START ltp getsid01 ==========
RUN LTP CASE getsid01
LTP MEMORY getsid01 before: free_frames=157261 allocated_frames=38905
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getsid01.c:41: [1;32mTPASS: [0mp_sid == c_sid (926)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[352.409307 0:926 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getsid01 : 0
Pass!
LTP MEMORY getsid01 after_run: free_frames=157237 allocated_frames=38929
LTP MEMORY getsid01 after_cleanup: free_frames=157237 allocated_frames=38929
LTP CASE RUNTIME getsid01: 1148 ms
========== END ltp getsid01 ==========
========== START ltp gettid01 ==========
RUN LTP CASE gettid01
LTP MEMORY gettid01 before: free_frames=157237 allocated_frames=38929
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
gettid01.c:26: [1;32mTPASS: [0mtst_syscall(__NR_gettid) == tst_syscall(__NR_getpid) (933)
gettid01.c:27: [1;32mTPASS: [0mtst_syscall(__NR_gettid) == pid (933)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[353.467998 0:931 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE gettid01 : 0
Pass!
LTP MEMORY gettid01 after_run: free_frames=157221 allocated_frames=38945
LTP MEMORY gettid01 after_cleanup: free_frames=157221 allocated_frames=38945
LTP CASE RUNTIME gettid01: 1057 ms
========== END ltp gettid01 ==========
========== START ltp uname01 ==========
RUN LTP CASE uname01
LTP MEMORY uname01 before: free_frames=157221 allocated_frames=38945
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
[37m[354.571693 0:935 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE uname01 : 0
Pass!
LTP MEMORY uname01 after_run: free_frames=157205 allocated_frames=38961
LTP MEMORY uname01 after_cleanup: free_frames=157205 allocated_frames=38961
LTP CASE RUNTIME uname01: 1110 ms
========== END ltp uname01 ==========
========== START ltp uname04 ==========
RUN LTP CASE uname04
LTP MEMORY uname04 before: free_frames=157205 allocated_frames=38961
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
uname04.c:70: [1;34mTINFO: [0mCalling uname with default personality
uname04.c:62: [1;32mTPASS: [0mNo bytes leaked
uname04.c:73: [1;34mTINFO: [0mCalling uname with UNAME26 personality
uname04.c:62: [1;32mTPASS: [0mNo bytes leaked

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[355.727920 0:939 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE uname04 : 0
Pass!
LTP MEMORY uname04 after_run: free_frames=157189 allocated_frames=38977
LTP MEMORY uname04 after_cleanup: free_frames=157189 allocated_frames=38977
LTP CASE RUNTIME uname04: 1138 ms
========== END ltp uname04 ==========
========== START ltp getrlimit01 ==========
RUN LTP CASE getrlimit01
LTP MEMORY getrlimit01 before: free_frames=157189 allocated_frames=38977
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
[37m[356.874323 0:943 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getrlimit01 : 0
Pass!
LTP MEMORY getrlimit01 after_run: free_frames=157173 allocated_frames=38993
LTP MEMORY getrlimit01 after_cleanup: free_frames=157173 allocated_frames=38993
LTP CASE RUNTIME getrlimit01: 1139 ms
========== END ltp getrlimit01 ==========
========== START ltp getrusage01 ==========
RUN LTP CASE getrusage01
LTP MEMORY getrusage01 before: free_frames=157173 allocated_frames=38993
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
[37m[357.969452 0:947 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getrusage01 : 0
Pass!
LTP MEMORY getrusage01 after_run: free_frames=157157 allocated_frames=39009
LTP MEMORY getrusage01 after_cleanup: free_frames=157157 allocated_frames=39009
LTP CASE RUNTIME getrusage01: 1095 ms
========== END ltp getrusage01 ==========
========== START ltp sched_getscheduler01 ==========
RUN LTP CASE sched_getscheduler01
LTP MEMORY sched_getscheduler01 before: free_frames=157157 allocated_frames=39009
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
sched_getscheduler01.c:58: [1;34mTINFO: [0mTesting libc variant
sched_getscheduler01.c:51: [1;32mTPASS: [0mgot expected policy 2
sched_getscheduler01.c:51: [1;32mTPASS: [0mgot expected policy 0
sched_getscheduler01.c:51: [1;32mTPASS: [0mgot expected policy 1
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
sched_getscheduler01.c:58: [1;34mTINFO: [0mTesting syscall variant
sched_getscheduler01.c:51: [1;32mTPASS: [0mgot expected policy 2
sched_getscheduler01.c:51: [1;32mTPASS: [0mgot expected policy 0
sched_getscheduler01.c:51: [1;32mTPASS: [0mgot expected policy 1

Summary:
passed   6
failed   0
broken   0
skipped  0
warnings 0
[37m[359.032410 0:951 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE sched_getscheduler01 : 0
Pass!
LTP MEMORY sched_getscheduler01 after_run: free_frames=157133 allocated_frames=39033
LTP MEMORY sched_getscheduler01 after_cleanup: free_frames=157133 allocated_frames=39033
LTP CASE RUNTIME sched_getscheduler01: 1061 ms
========== END ltp sched_getscheduler01 ==========
========== START ltp sched_yield01 ==========
RUN LTP CASE sched_yield01
LTP MEMORY sched_yield01 before: free_frames=157133 allocated_frames=39033
sched_yield01    1  [1;32mTPASS[0m  :  sched_yield() call succeeded
FAIL LTP CASE sched_yield01 : 0
Pass!
LTP MEMORY sched_yield01 after_run: free_frames=157125 allocated_frames=39041
LTP MEMORY sched_yield01 after_cleanup: free_frames=157125 allocated_frames=39041
LTP CASE RUNTIME sched_yield01: 1015 ms
========== END ltp sched_yield01 ==========
========== START ltp getpgid02 ==========
RUN LTP CASE getpgid02
LTP MEMORY getpgid02 before: free_frames=157125 allocated_frames=39041
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
[37m[361.108786 0:959 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getpgid02 : 0
Pass!
LTP MEMORY getpgid02 after_run: free_frames=157109 allocated_frames=39057
LTP MEMORY getpgid02 after_cleanup: free_frames=157109 allocated_frames=39057
LTP CASE RUNTIME getpgid02: 1059 ms
========== END ltp getpgid02 ==========
========== START ltp getsid02 ==========
RUN LTP CASE getsid02
LTP MEMORY getsid02 before: free_frames=157109 allocated_frames=39057
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
[37m[362.191787 0:963 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getsid02 : 0
Pass!
LTP MEMORY getsid02 after_run: free_frames=157093 allocated_frames=39073
LTP MEMORY getsid02 after_cleanup: free_frames=157093 allocated_frames=39073
LTP CASE RUNTIME getsid02: 1093 ms
========== END ltp getsid02 ==========
========== START ltp getppid02 ==========
RUN LTP CASE getppid02
LTP MEMORY getppid02 before: free_frames=157093 allocated_frames=39073
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: [1;34mTINFO: [0mLTP version: 20240524
tst_test.c:1617: [1;34mTINFO: [0mTimeout per run is 0h 00m 30s
tst_memutils.c:152: [1;34mTINFO: [0moom_score_adj does not exist, skipping the adjustment
getppid02.c:31: [1;32mTPASS: [0mgetppid() returned parent pid (969)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[363.366442 0:967 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getppid02 : 0
Pass!
LTP MEMORY getppid02 after_run: free_frames=157069 allocated_frames=39097
LTP MEMORY getppid02 after_cleanup: free_frames=157069 allocated_frames=39097
LTP CASE RUNTIME getppid02: 1166 ms
========== END ltp getppid02 ==========
========== START ltp getuid03 ==========
RUN LTP CASE getuid03
LTP MEMORY getuid03 before: free_frames=157069 allocated_frames=39097
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
[37m[364.477538 0:972 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getuid03 : 0
Pass!
LTP MEMORY getuid03 after_run: free_frames=157053 allocated_frames=39113
LTP MEMORY getuid03 after_cleanup: free_frames=157053 allocated_frames=39113
LTP CASE RUNTIME getuid03: 1112 ms
========== END ltp getuid03 ==========
========== START ltp geteuid02 ==========
RUN LTP CASE geteuid02
LTP MEMORY geteuid02 before: free_frames=157053 allocated_frames=39113
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
[37m[365.630054 0:976 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE geteuid02 : 0
Pass!
LTP MEMORY geteuid02 after_run: free_frames=157037 allocated_frames=39129
LTP MEMORY geteuid02 after_cleanup: free_frames=157037 allocated_frames=39129
LTP CASE RUNTIME geteuid02: 1153 ms
========== END ltp geteuid02 ==========
========== START ltp getgid03 ==========
RUN LTP CASE getgid03
LTP MEMORY getgid03 before: free_frames=157037 allocated_frames=39129
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
[37m[366.734314 0:980 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getgid03 : 0
Pass!
LTP MEMORY getgid03 after_run: free_frames=157021 allocated_frames=39145
LTP MEMORY getgid03 after_cleanup: free_frames=157021 allocated_frames=39145
LTP CASE RUNTIME getgid03: 1101 ms
========== END ltp getgid03 ==========
========== START ltp getegid02 ==========
RUN LTP CASE getegid02
LTP MEMORY getegid02 before: free_frames=157021 allocated_frames=39145
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
[37m[367.744160 0:984 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getegid02 : 0
Pass!
LTP MEMORY getegid02 after_run: free_frames=157005 allocated_frames=39161
LTP MEMORY getegid02 after_cleanup: free_frames=157005 allocated_frames=39161
LTP CASE RUNTIME getegid02: 1014 ms
========== END ltp getegid02 ==========
========== START ltp getgroups03 ==========
RUN LTP CASE getgroups03
LTP MEMORY getgroups03 before: free_frames=157005 allocated_frames=39161
getgroups03    1  [1;32mTPASS[0m  :  getgroups functionality correct
FAIL LTP CASE getgroups03 : 0
Pass!
LTP MEMORY getgroups03 after_run: free_frames=156997 allocated_frames=39169
LTP MEMORY getgroups03 after_cleanup: free_frames=156997 allocated_frames=39169
LTP CASE RUNTIME getgroups03: 997 ms
========== END ltp getgroups03 ==========
========== START ltp uname02 ==========
RUN LTP CASE uname02
LTP MEMORY uname02 before: free_frames=156997 allocated_frames=39169
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
[37m[369.854047 0:989 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE uname02 : 0
Pass!
LTP MEMORY uname02 after_run: free_frames=156981 allocated_frames=39185
LTP MEMORY uname02 after_cleanup: free_frames=156981 allocated_frames=39185
LTP CASE RUNTIME uname02: 1100 ms
========== END ltp uname02 ==========
========== START ltp wait01 ==========
RUN LTP CASE wait01
LTP MEMORY wait01 before: free_frames=156981 allocated_frames=39185
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
[37m[370.945578 0:993 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE wait01 : 0
Pass!
LTP MEMORY wait01 after_run: free_frames=156965 allocated_frames=39201
LTP MEMORY wait01 after_cleanup: free_frames=156965 allocated_frames=39201
LTP CASE RUNTIME wait01: 1082 ms
========== END ltp wait01 ==========
========== START ltp wait02 ==========
RUN LTP CASE wait02
LTP MEMORY wait02 before: free_frames=156965 allocated_frames=39201
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
[37m[372.062883 0:997 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE wait02 : 0
Pass!
LTP MEMORY wait02 after_run: free_frames=156941 allocated_frames=39225
LTP MEMORY wait02 after_cleanup: free_frames=156941 allocated_frames=39225
LTP CASE RUNTIME wait02: 1111 ms
========== END ltp wait02 ==========
========== START ltp getrlimit02 ==========
RUN LTP CASE getrlimit02
LTP MEMORY getrlimit02 before: free_frames=156941 allocated_frames=39225
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
[37m[373.191495 0:1002 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getrlimit02 : 0
Pass!
LTP MEMORY getrlimit02 after_run: free_frames=156925 allocated_frames=39241
LTP MEMORY getrlimit02 after_cleanup: free_frames=156925 allocated_frames=39241
LTP CASE RUNTIME getrlimit02: 1133 ms
========== END ltp getrlimit02 ==========
ltp cases: 85 passed, 0 failed, 0 timed out
#### OS COMP TEST GROUP END ltp-musl ####
#### OS COMP TEST GROUP START ltp-glibc ####
ltp case list: stable (85 cases, timeout 15s)
========== START ltp access01 ==========
RUN LTP CASE access01
LTP MEMORY access01 before: free_frames=156925 allocated_frames=39241
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
[37m[379.056962 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.057825 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.058647 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.059849 0:1006 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[379.061051 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.061733 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.062385 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.063122 0:1006 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[379.064135 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.064785 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.065433 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.066185 0:1006 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[379.067162 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.067832 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.068631 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.069360 0:1006 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[379.070339 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.070983 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.071626 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.072376 0:1006 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[379.073337 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.073980 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.074616 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.075359 0:1006 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[379.075984 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.076612 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.077207 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.077793 0:1006 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[379.078835 0:1006 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE access01 : 0
Pass!
LTP MEMORY access01 after_run: free_frames=155995 allocated_frames=40171
LTP MEMORY access01 after_cleanup: free_frames=155995 allocated_frames=40171
LTP CASE RUNTIME access01: 5856 ms
========== END ltp access01 ==========
========== START ltp brk01 ==========
RUN LTP CASE brk01
LTP MEMORY brk01 before: free_frames=155995 allocated_frames=40171
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
[37m[381.053568 0:1111 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE brk01 : 0
Pass!
LTP MEMORY brk01 after_run: free_frames=155965 allocated_frames=40201
LTP MEMORY brk01 after_cleanup: free_frames=155965 allocated_frames=40201
LTP CASE RUNTIME brk01: 1976 ms
========== END ltp brk01 ==========
========== START ltp chdir01 ==========
RUN LTP CASE chdir01
LTP MEMORY chdir01 before: free_frames=155965 allocated_frames=40201
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
tst_test.c:1120: TINFO: Mounting ltp-tmpfs to /tmp/ltp-work/LTP_chdFx6IfI/mntpoint fstyp=tmpfs flags=0
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
[37m[383.188902 0:1118 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[383.191432 0:1118 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[383.192516 0:1118 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[383.193770 0:1118 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[383.194711 0:1118 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE chdir01 : 0
Pass!
LTP MEMORY chdir01 after_run: free_frames=155944 allocated_frames=40222
LTP MEMORY chdir01 after_cleanup: free_frames=155944 allocated_frames=40222
LTP CASE RUNTIME chdir01: 2151 ms
========== END ltp chdir01 ==========
========== START ltp clone01 ==========
RUN LTP CASE clone01
LTP MEMORY clone01 before: free_frames=155944 allocated_frames=40222
tst_buffers.c:57: TINFO: Test is using guarded buffers
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
clone01.c:37: TPASS: clone returned 1125
clone01.c:43: TPASS: Child exited with 0

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[385.290775 0:1122 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE clone01 : 0
Pass!
LTP MEMORY clone01 after_run: free_frames=155914 allocated_frames=40252
LTP MEMORY clone01 after_cleanup: free_frames=155914 allocated_frames=40252
LTP CASE RUNTIME clone01: 2094 ms
========== END ltp clone01 ==========
========== START ltp close01 ==========
RUN LTP CASE close01
LTP MEMORY close01 before: free_frames=155914 allocated_frames=40252
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
[37m[387.365707 0:1127 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[387.367610 0:1127 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE close01 : 0
Pass!
LTP MEMORY close01 after_run: free_frames=155893 allocated_frames=40273
LTP MEMORY close01 after_cleanup: free_frames=155893 allocated_frames=40273
LTP CASE RUNTIME close01: 2078 ms
========== END ltp close01 ==========
========== START ltp dup01 ==========
RUN LTP CASE dup01
LTP MEMORY dup01 before: free_frames=155893 allocated_frames=40273
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
dup01.c:24: TPASS: dup(fd) returned fd 4
dup01.c:27: TPASS: buf1.st_ino == buf2.st_ino (9242274199192812947)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[389.637283 0:1131 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[389.639664 0:1131 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE dup01 : 0
Pass!
LTP MEMORY dup01 after_run: free_frames=155872 allocated_frames=40294
LTP MEMORY dup01 after_cleanup: free_frames=155872 allocated_frames=40294
LTP CASE RUNTIME dup01: 2269 ms
========== END ltp dup01 ==========
========== START ltp fcntl01 ==========
RUN LTP CASE fcntl01
LTP MEMORY fcntl01 before: free_frames=155872 allocated_frames=40294
[37m[391.738569 0:1135 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE fcntl01 : 0
Pass!
LTP MEMORY fcntl01 after_run: free_frames=155860 allocated_frames=40306
LTP MEMORY fcntl01 after_cleanup: free_frames=155860 allocated_frames=40306
LTP CASE RUNTIME fcntl01: 2106 ms
========== END ltp fcntl01 ==========
========== START ltp fcntl02 ==========
RUN LTP CASE fcntl02
LTP MEMORY fcntl02 before: free_frames=155860 allocated_frames=40306
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
fcntl02.c:41: TPASS: fcntl(fcntl02_1138, F_DUPFD, 0) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1138, F_DUPFD, 1) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1138, F_DUPFD, 2) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1138, F_DUPFD, 3) returned 4
fcntl02.c:41: TPASS: fcntl(fcntl02_1138, F_DUPFD, 10) returned 10
fcntl02.c:41: TPASS: fcntl(fcntl02_1138, F_DUPFD, 100) returned 100

Summary:
passed   6
failed   0
broken   0
skipped  0
warnings 0
[37m[393.913971 0:1136 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[393.915374 0:1136 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE fcntl02 : 0
Pass!
LTP MEMORY fcntl02 after_run: free_frames=155839 allocated_frames=40327
LTP MEMORY fcntl02 after_cleanup: free_frames=155839 allocated_frames=40327
LTP CASE RUNTIME fcntl02: 2149 ms
========== END ltp fcntl02 ==========
========== START ltp fork01 ==========
RUN LTP CASE fork01
LTP MEMORY fork01 before: free_frames=155839 allocated_frames=40327
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
fork01.c:47: TPASS: correct child status returned 42
fork01.c:50: TPASS: child_pid == pid (1143)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[396.037910 0:1140 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[396.039227 0:1140 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE fork01 : 0
Pass!
LTP MEMORY fork01 after_run: free_frames=155809 allocated_frames=40357
LTP MEMORY fork01 after_cleanup: free_frames=155809 allocated_frames=40357
LTP CASE RUNTIME fork01: 2122 ms
========== END ltp fork01 ==========
========== START ltp getpid01 ==========
RUN LTP CASE getpid01
LTP MEMORY getpid01 before: free_frames=155809 allocated_frames=40357
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getpid01.c:34: TPASS: getpid() returns 1148
getpid01.c:34: TPASS: getpid() returns 1149
getpid01.c:34: TPASS: getpid() returns 1150
getpid01.c:34: TPASS: getpid() returns 1151
getpid01.c:34: TPASS: getpid() returns 1152
getpid01.c:34: TPASS: getpid() returns 1153
getpid01.c:34: TPASS: getpid() returns 1154
getpid01.c:34: TPASS: getpid() returns 1155
getpid01.c:34: TPASS: getpid() returns 1156
getpid01.c:34: TPASS: getpid() returns 1157
getpid01.c:34: TPASS: getpid() returns 1158
getpid01.c:34: TPASS: getpid() returns 1159
getpid01.c:34: TPASS: getpid() returns 1160
getpid01.c:34: TPASS: getpid() returns 1161
getpid01.c:34: TPASS: getpid() returns 1162
getpid01.c:34: TPASS: getpid() returns 1163
getpid01.c:34: TPASS: getpid() returns 1164
getpid01.c:34: TPASS: getpid() returns 1165
getpid01.c:34: TPASS: getpid() returns 1166
getpid01.c:34: TPASS: getpid() returns 1167
getpid01.c:34: TPASS: getpid() returns 1168
getpid01.c:34: TPASS: getpid() returns 1169
getpid01.c:34: TPASS: getpid() returns 1170
getpid01.c:34: TPASS: getpid() returns 1171
getpid01.c:34: TPASS: getpid() returns 1172
getpid01.c:34: TPASS: getpid() returns 1173
getpid01.c:34: TPASS: getpid() returns 1174
getpid01.c:34: TPASS: getpid() returns 1175
getpid01.c:34: TPASS: getpid() returns 1176
getpid01.c:34: TPASS: getpid() returns 1177
getpid01.c:34: TPASS: getpid() returns 1178
getpid01.c:34: TPASS: getpid() returns 1179
getpid01.c:34: TPASS: getpid() returns 1180
getpid01.c:34: TPASS: getpid() returns 1181
getpid01.c:34: TPASS: getpid() returns 1182
getpid01.c:34: TPASS: getpid() returns 1183
getpid01.c:34: TPASS: getpid() returns 1184
getpid01.c:34: TPASS: getpid() returns 1185
getpid01.c:34: TPASS: getpid() returns 1186
getpid01.c:34: TPASS: getpid() returns 1187
getpid01.c:34: TPASS: getpid() returns 1188
getpid01.c:34: TPASS: getpid() returns 1189
getpid01.c:34: TPASS: getpid() returns 1190
getpid01.c:34: TPASS: getpid() returns 1191
getpid01.c:34: TPASS: getpid() returns 1192
getpid01.c:34: TPASS: getpid() returns 1193
getpid01.c:34: TPASS: getpid() returns 1194
getpid01.c:34: TPASS: getpid() returns 1195
getpid01.c:34: TPASS: getpid() returns 1196
getpid01.c:34: TPASS: getpid() returns 1197
getpid01.c:34: TPASS: getpid() returns 1198
getpid01.c:34: TPASS: getpid() returns 1199
getpid01.c:34: TPASS: getpid() returns 1200
getpid01.c:34: TPASS: getpid() returns 1201
getpid01.c:34: TPASS: getpid() returns 1202
getpid01.c:34: TPASS: getpid() returns 1203
getpid01.c:34: TPASS: getpid() returns 1204
getpid01.c:34: TPASS: getpid() returns 1205
getpid01.c:34: TPASS: getpid() returns 1206
getpid01.c:34: TPASS: getpid() returns 1207
getpid01.c:34: TPASS: getpid() returns 1208
getpid01.c:34: TPASS: getpid() returns 1209
getpid01.c:34: TPASS: getpid() returns 1210
getpid01.c:34: TPASS: getpid() returns 1211
getpid01.c:34: TPASS: getpid() returns 1212
getpid01.c:34: TPASS: getpid() returns 1213
getpid01.c:34: TPASS: getpid() returns 1214
getpid01.c:34: TPASS: getpid() returns 1215
getpid01.c:34: TPASS: getpid() returns 1216
getpid01.c:34: TPASS: getpid() returns 1217
getpid01.c:34: TPASS: getpid() returns 1218
getpid01.c:34: TPASS: getpid() returns 1219
getpid01.c:34: TPASS: getpid() returns 1220
getpid01.c:34: TPASS: getpid() returns 1221
getpid01.c:34: TPASS: getpid() returns 1222
getpid01.c:34: TPASS: getpid() returns 1223
getpid01.c:34: TPASS: getpid() returns 1224
getpid01.c:34: TPASS: getpid() returns 1225
getpid01.c:34: TPASS: getpid() returns 1226
getpid01.c:34: TPASS: getpid() returns 1227
getpid01.c:34: TPASS: getpid() returns 1228
getpid01.c:34: TPASS: getpid() returns 1229
getpid01.c:34: TPASS: getpid() returns 1230
getpid01.c:34: TPASS: getpid() returns 1231
getpid01.c:34: TPASS: getpid() returns 1232
getpid01.c:34: TPASS: getpid() returns 1233
getpid01.c:34: TPASS: getpid() returns 1234
getpid01.c:34: TPASS: getpid() returns 1235
getpid01.c:34: TPASS: getpid() returns 1236
getpid01.c:34: TPASS: getpid() returns 1237
getpid01.c:34: TPASS: getpid() returns 1238
getpid01.c:34: TPASS: getpid() returns 1239
getpid01.c:34: TPASS: getpid() returns 1240
getpid01.c:34: TPASS: getpid() returns 1241
getpid01.c:34: TPASS: getpid() returns 1242
getpid01.c:34: TPASS: getpid() returns 1243
getpid01.c:34: TPASS: getpid() returns 1244
getpid01.c:34: TPASS: getpid() returns 1245
getpid01.c:34: TPASS: getpid() returns 1246
getpid01.c:34: TPASS: getpid() returns 1247

Summary:
passed   100
failed   0
broken   0
skipped  0
warnings 0
[37m[401.301913 0:1145 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getpid01 : 0
Pass!
LTP MEMORY getpid01 after_run: free_frames=154888 allocated_frames=41278
LTP MEMORY getpid01 after_cleanup: free_frames=154888 allocated_frames=41278
LTP CASE RUNTIME getpid01: 5279 ms
========== END ltp getpid01 ==========
========== START ltp mmap01 ==========
RUN LTP CASE mmap01
LTP MEMORY mmap01 before: free_frames=154888 allocated_frames=41278
[37m[403.396623 0:1249 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[403.401507 0:1249 axfs::root:433] [33m[AxError::IsADirectory][m
[mmmap01      1  TPASS  :  Functionality of mmap() successful
FAIL LTP CASE mmap01 : 0
Pass!
LTP MEMORY mmap01 after_run: free_frames=154867 allocated_frames=41299
LTP MEMORY mmap01 after_cleanup: free_frames=154867 allocated_frames=41299
LTP CASE RUNTIME mmap01: 2114 ms
========== END ltp mmap01 ==========
========== START ltp open01 ==========
RUN LTP CASE open01
LTP MEMORY open01 before: free_frames=154867 allocated_frames=41299
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
[37m[405.495821 0:1251 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[405.498610 0:1251 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE open01 : 0
Pass!
LTP MEMORY open01 after_run: free_frames=154846 allocated_frames=41320
LTP MEMORY open01 after_cleanup: free_frames=154846 allocated_frames=41320
LTP CASE RUNTIME open01: 2074 ms
========== END ltp open01 ==========
========== START ltp pipe01 ==========
RUN LTP CASE pipe01
LTP MEMORY pipe01 before: free_frames=154846 allocated_frames=41320
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
[37m[407.468701 0:1255 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE pipe01 : 0
Pass!
LTP MEMORY pipe01 after_run: free_frames=154825 allocated_frames=41341
LTP MEMORY pipe01 after_cleanup: free_frames=154825 allocated_frames=41341
LTP CASE RUNTIME pipe01: 1963 ms
========== END ltp pipe01 ==========
========== START ltp read01 ==========
RUN LTP CASE read01
LTP MEMORY read01 before: free_frames=154825 allocated_frames=41341
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
[37m[409.474550 0:1259 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[409.476316 0:1259 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE read01 : 0
Pass!
LTP MEMORY read01 after_run: free_frames=154804 allocated_frames=41362
LTP MEMORY read01 after_cleanup: free_frames=154804 allocated_frames=41362
LTP CASE RUNTIME read01: 2007 ms
========== END ltp read01 ==========
========== START ltp stat01 ==========
RUN LTP CASE stat01
LTP MEMORY stat01 before: free_frames=154804 allocated_frames=41362
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
[37m[411.706710 0:1263 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[411.710770 0:1263 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[411.715551 0:1263 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE stat01 : 0
Pass!
LTP MEMORY stat01 after_run: free_frames=154783 allocated_frames=41383
LTP MEMORY stat01 after_cleanup: free_frames=154783 allocated_frames=41383
LTP CASE RUNTIME stat01: 2255 ms
========== END ltp stat01 ==========
========== START ltp wait401 ==========
RUN LTP CASE wait401
LTP MEMORY wait401 before: free_frames=154783 allocated_frames=41383
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
wait401.c:40: TPASS: wait4() returned correct pid 1271
wait401.c:49: TPASS: WIFEXITED() is set in status
wait401.c:54: TPASS: WEXITSTATUS() == 0

Summary:
passed   3
failed   0
broken   0
skipped  0
warnings 0
[37m[413.875579 0:1268 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE wait401 : 0
Pass!
LTP MEMORY wait401 after_run: free_frames=154753 allocated_frames=41413
LTP MEMORY wait401 after_cleanup: free_frames=154753 allocated_frames=41413
LTP CASE RUNTIME wait401: 2125 ms
========== END ltp wait401 ==========
========== START ltp write01 ==========
RUN LTP CASE write01
LTP MEMORY write01 before: free_frames=154753 allocated_frames=41413
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
[37m[416.283843 0:1273 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[416.285781 0:1273 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE write01 : 0
Pass!
LTP MEMORY write01 after_run: free_frames=121964 allocated_frames=74202
LTP MEMORY write01 after_cleanup: free_frames=121964 allocated_frames=74202
LTP CASE RUNTIME write01: 2419 ms
========== END ltp write01 ==========
========== START ltp access03 ==========
RUN LTP CASE access03
LTP MEMORY access03 before: free_frames=121964 allocated_frames=74202
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
[37m[418.603146 0:1277 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE access03 : 0
Pass!
LTP MEMORY access03 after_run: free_frames=121907 allocated_frames=74259
LTP MEMORY access03 after_cleanup: free_frames=121907 allocated_frames=74259
LTP CASE RUNTIME access03: 2316 ms
========== END ltp access03 ==========
========== START ltp close02 ==========
RUN LTP CASE close02
LTP MEMORY close02 before: free_frames=121907 allocated_frames=74259
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
[37m[420.754807 0:1285 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE close02 : 0
Pass!
LTP MEMORY close02 after_run: free_frames=121886 allocated_frames=74280
LTP MEMORY close02 after_cleanup: free_frames=121886 allocated_frames=74280
LTP CASE RUNTIME close02: 2163 ms
========== END ltp close02 ==========
========== START ltp dup02 ==========
RUN LTP CASE dup02
LTP MEMORY dup02 before: free_frames=121886 allocated_frames=74280
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
[37m[422.857805 0:1289 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE dup02 : 0
Pass!
LTP MEMORY dup02 after_run: free_frames=121865 allocated_frames=74301
LTP MEMORY dup02 after_cleanup: free_frames=121865 allocated_frames=74301
LTP CASE RUNTIME dup02: 2076 ms
========== END ltp dup02 ==========
========== START ltp fcntl03 ==========
RUN LTP CASE fcntl03
LTP MEMORY fcntl03 before: free_frames=121865 allocated_frames=74301
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
fcntl03.c:32: TPASS: fcntl(fcntl03_1295, F_GETFD, 0) returned 0

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[424.925483 0:1293 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[424.926822 0:1293 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE fcntl03 : 0
Pass!
LTP MEMORY fcntl03 after_run: free_frames=121844 allocated_frames=74322
LTP MEMORY fcntl03 after_cleanup: free_frames=121844 allocated_frames=74322
LTP CASE RUNTIME fcntl03: 2067 ms
========== END ltp fcntl03 ==========
========== START ltp getcwd01 ==========
RUN LTP CASE getcwd01
LTP MEMORY getcwd01 before: free_frames=121844 allocated_frames=74322
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
[37m[426.934230 0:1297 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getcwd01 : 0
Pass!
LTP MEMORY getcwd01 after_run: free_frames=121823 allocated_frames=74343
LTP MEMORY getcwd01 after_cleanup: free_frames=121823 allocated_frames=74343
LTP CASE RUNTIME getcwd01: 2007 ms
========== END ltp getcwd01 ==========
========== START ltp getpid02 ==========
RUN LTP CASE getpid02
LTP MEMORY getpid02 before: free_frames=121823 allocated_frames=74343
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getpid02.c:37: TPASS: child getppid() == parent getpid() (1303)
getpid02.c:50: TPASS: child getpid() == parent fork() (1304)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[429.117731 0:1301 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getpid02 : 0
Pass!
LTP MEMORY getpid02 after_run: free_frames=121793 allocated_frames=74373
LTP MEMORY getpid02 after_cleanup: free_frames=121793 allocated_frames=74373
LTP CASE RUNTIME getpid02: 2192 ms
========== END ltp getpid02 ==========
========== START ltp getppid01 ==========
RUN LTP CASE getppid01
LTP MEMORY getppid01 before: free_frames=121793 allocated_frames=74373
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getppid01.c:31: TPASS: getppid() returned 1306

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[432.568297 0:1306 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getppid01 : 0
Pass!
LTP MEMORY getppid01 after_run: free_frames=121772 allocated_frames=74394
LTP MEMORY getppid01 after_cleanup: free_frames=121772 allocated_frames=74394
LTP CASE RUNTIME getppid01: 3459 ms
========== END ltp getppid01 ==========
========== START ltp getuid01 ==========
RUN LTP CASE getuid01
LTP MEMORY getuid01 before: free_frames=121772 allocated_frames=74394
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
[37m[435.611241 0:1310 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getuid01 : 0
Pass!
LTP MEMORY getuid01 after_run: free_frames=121751 allocated_frames=74415
LTP MEMORY getuid01 after_cleanup: free_frames=121751 allocated_frames=74415
LTP CASE RUNTIME getuid01: 3041 ms
========== END ltp getuid01 ==========
========== START ltp geteuid01 ==========
RUN LTP CASE geteuid01
LTP MEMORY geteuid01 before: free_frames=121751 allocated_frames=74415
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
[37m[437.715313 0:1314 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE geteuid01 : 0
Pass!
LTP MEMORY geteuid01 after_run: free_frames=121730 allocated_frames=74436
LTP MEMORY geteuid01 after_cleanup: free_frames=121730 allocated_frames=74436
LTP CASE RUNTIME geteuid01: 2103 ms
========== END ltp geteuid01 ==========
========== START ltp getgid01 ==========
RUN LTP CASE getgid01
LTP MEMORY getgid01 before: free_frames=121730 allocated_frames=74436
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
[37m[439.807929 0:1318 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getgid01 : 0
Pass!
LTP MEMORY getgid01 after_run: free_frames=121709 allocated_frames=74457
LTP MEMORY getgid01 after_cleanup: free_frames=121709 allocated_frames=74457
LTP CASE RUNTIME getgid01: 2093 ms
========== END ltp getgid01 ==========
========== START ltp getegid01 ==========
RUN LTP CASE getegid01
LTP MEMORY getegid01 before: free_frames=121709 allocated_frames=74457
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
[37m[441.960483 0:1322 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getegid01 : 0
Pass!
LTP MEMORY getegid01 after_run: free_frames=121688 allocated_frames=74478
LTP MEMORY getegid01 after_cleanup: free_frames=121688 allocated_frames=74478
LTP CASE RUNTIME getegid01: 2122 ms
========== END ltp getegid01 ==========
========== START ltp getresuid01 ==========
RUN LTP CASE getresuid01
LTP MEMORY getresuid01 before: free_frames=121688 allocated_frames=74478
getresuid01    1  TPASS  :  Functionality of getresuid() successful
FAIL LTP CASE getresuid01 : 0
Pass!
LTP MEMORY getresuid01 after_run: free_frames=121676 allocated_frames=74490
LTP MEMORY getresuid01 after_cleanup: free_frames=121676 allocated_frames=74490
LTP CASE RUNTIME getresuid01: 2006 ms
========== END ltp getresuid01 ==========
========== START ltp getresuid02 ==========
RUN LTP CASE getresuid02
LTP MEMORY getresuid02 before: free_frames=121676 allocated_frames=74490
getresuid02    1  TPASS  :  Functionality of getresuid() successful
FAIL LTP CASE getresuid02 : 0
Pass!
LTP MEMORY getresuid02 after_run: free_frames=121664 allocated_frames=74502
LTP MEMORY getresuid02 after_cleanup: free_frames=121664 allocated_frames=74502
LTP CASE RUNTIME getresuid02: 1957 ms
========== END ltp getresuid02 ==========
========== START ltp getresuid03 ==========
RUN LTP CASE getresuid03
LTP MEMORY getresuid03 before: free_frames=121664 allocated_frames=74502
getresuid03    1  TPASS  :  Functionality of getresuid() successful
FAIL LTP CASE getresuid03 : 0
Pass!
LTP MEMORY getresuid03 after_run: free_frames=121652 allocated_frames=74514
LTP MEMORY getresuid03 after_cleanup: free_frames=121652 allocated_frames=74514
LTP CASE RUNTIME getresuid03: 1907 ms
========== END ltp getresuid03 ==========
========== START ltp getresgid01 ==========
RUN LTP CASE getresgid01
LTP MEMORY getresgid01 before: free_frames=121652 allocated_frames=74514
getresgid01    1  TPASS  :  Functionality of getresgid() successful
FAIL LTP CASE getresgid01 : 0
Pass!
LTP MEMORY getresgid01 after_run: free_frames=121640 allocated_frames=74526
LTP MEMORY getresgid01 after_cleanup: free_frames=121640 allocated_frames=74526
LTP CASE RUNTIME getresgid01: 1852 ms
========== END ltp getresgid01 ==========
========== START ltp getresgid02 ==========
RUN LTP CASE getresgid02
LTP MEMORY getresgid02 before: free_frames=121640 allocated_frames=74526
getresgid02    1  TPASS  :  Functionality of getresgid() successful
FAIL LTP CASE getresgid02 : 0
Pass!
LTP MEMORY getresgid02 after_run: free_frames=121628 allocated_frames=74538
LTP MEMORY getresgid02 after_cleanup: free_frames=121628 allocated_frames=74538
LTP CASE RUNTIME getresgid02: 1988 ms
========== END ltp getresgid02 ==========
========== START ltp getresgid03 ==========
RUN LTP CASE getresgid03
LTP MEMORY getresgid03 before: free_frames=121628 allocated_frames=74538
getresgid03    1  TPASS  :  Functionality of getresgid() successful
FAIL LTP CASE getresgid03 : 0
Pass!
LTP MEMORY getresgid03 after_run: free_frames=121616 allocated_frames=74550
LTP MEMORY getresgid03 after_cleanup: free_frames=121616 allocated_frames=74550
LTP CASE RUNTIME getresgid03: 1921 ms
========== END ltp getresgid03 ==========
========== START ltp lseek01 ==========
RUN LTP CASE lseek01
LTP MEMORY lseek01 before: free_frames=121616 allocated_frames=74550
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
[37m[455.540995 0:1332 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[455.543552 0:1332 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE lseek01 : 0
Pass!
LTP MEMORY lseek01 after_run: free_frames=121595 allocated_frames=74571
LTP MEMORY lseek01 after_cleanup: free_frames=121595 allocated_frames=74571
LTP CASE RUNTIME lseek01: 1942 ms
========== END ltp lseek01 ==========
========== START ltp read02 ==========
RUN LTP CASE read02
LTP MEMORY read02 before: free_frames=121595 allocated_frames=74571
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
[37m[457.679385 0:1336 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[457.681003 0:1336 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE read02 : 0
Pass!
LTP MEMORY read02 after_run: free_frames=121574 allocated_frames=74592
LTP MEMORY read02 after_cleanup: free_frames=121574 allocated_frames=74592
LTP CASE RUNTIME read02: 2141 ms
========== END ltp read02 ==========
========== START ltp write02 ==========
RUN LTP CASE write02
LTP MEMORY write02 before: free_frames=121574 allocated_frames=74592
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
[37m[459.663368 0:1340 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[459.668138 0:1340 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE write02 : 0
Pass!
LTP MEMORY write02 after_run: free_frames=121553 allocated_frames=74613
LTP MEMORY write02 after_cleanup: free_frames=121553 allocated_frames=74613
LTP CASE RUNTIME write02: 1984 ms
========== END ltp write02 ==========
========== START ltp creat01 ==========
RUN LTP CASE creat01
LTP MEMORY creat01 before: free_frames=121553 allocated_frames=74613
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
[37m[461.773644 0:1344 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[461.775816 0:1344 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE creat01 : 0
Pass!
LTP MEMORY creat01 after_run: free_frames=121532 allocated_frames=74634
LTP MEMORY creat01 after_cleanup: free_frames=121532 allocated_frames=74634
LTP CASE RUNTIME creat01: 2099 ms
========== END ltp creat01 ==========
========== START ltp creat03 ==========
RUN LTP CASE creat03
LTP MEMORY creat03 before: free_frames=121532 allocated_frames=74634
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
[37m[463.797807 0:1348 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[463.799564 0:1348 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE creat03 : 0
Pass!
LTP MEMORY creat03 after_run: free_frames=121511 allocated_frames=74655
LTP MEMORY creat03 after_cleanup: free_frames=121511 allocated_frames=74655
LTP CASE RUNTIME creat03: 2013 ms
========== END ltp creat03 ==========
========== START ltp open02 ==========
RUN LTP CASE open02
LTP MEMORY open02 before: free_frames=121511 allocated_frames=74655
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
[37m[465.889560 0:1352 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[465.890752 0:1352 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE open02 : 0
Pass!
LTP MEMORY open02 after_run: free_frames=121490 allocated_frames=74676
LTP MEMORY open02 after_cleanup: free_frames=121490 allocated_frames=74676
LTP CASE RUNTIME open02: 2081 ms
========== END ltp open02 ==========
========== START ltp open03 ==========
RUN LTP CASE open03
LTP MEMORY open03 before: free_frames=121490 allocated_frames=74676
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
[37m[467.860980 0:1356 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE open03 : 0
Pass!
LTP MEMORY open03 after_run: free_frames=121469 allocated_frames=74697
LTP MEMORY open03 after_cleanup: free_frames=121469 allocated_frames=74697
LTP CASE RUNTIME open03: 1970 ms
========== END ltp open03 ==========
========== START ltp stat02 ==========
RUN LTP CASE stat02
LTP MEMORY stat02 before: free_frames=121469 allocated_frames=74697
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
[37m[469.851772 0:1360 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE stat02 : 0
Pass!
LTP MEMORY stat02 after_run: free_frames=121448 allocated_frames=74718
LTP MEMORY stat02 after_cleanup: free_frames=121448 allocated_frames=74718
LTP CASE RUNTIME stat02: 1992 ms
========== END ltp stat02 ==========
========== START ltp lstat01 ==========
RUN LTP CASE lstat01
LTP MEMORY lstat01 before: free_frames=121448 allocated_frames=74718
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
[37m[471.857631 0:1364 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[471.859505 0:1364 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE lstat01 : 0
Pass!
LTP MEMORY lstat01 after_run: free_frames=121427 allocated_frames=74739
LTP MEMORY lstat01 after_cleanup: free_frames=121427 allocated_frames=74739
LTP CASE RUNTIME lstat01: 2012 ms
========== END ltp lstat01 ==========
========== START ltp chmod01 ==========
RUN LTP CASE chmod01
LTP MEMORY chmod01 before: free_frames=121427 allocated_frames=74739
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
[37m[473.946236 0:1368 axfs::root:433] [33m[AxError::IsADirectory][m
[m[37m[473.947135 0:1368 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[473.947984 0:1368 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE chmod01 : 0
Pass!
LTP MEMORY chmod01 after_run: free_frames=121397 allocated_frames=74769
LTP MEMORY chmod01 after_cleanup: free_frames=121397 allocated_frames=74769
LTP CASE RUNTIME chmod01: 2078 ms
========== END ltp chmod01 ==========
========== START ltp fchmod01 ==========
RUN LTP CASE fchmod01
LTP MEMORY fchmod01 before: free_frames=121397 allocated_frames=74769
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
[37m[475.900722 0:1375 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[475.901917 0:1375 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE fchmod01 : 0
Pass!
LTP MEMORY fchmod01 after_run: free_frames=121376 allocated_frames=74790
LTP MEMORY fchmod01 after_cleanup: free_frames=121376 allocated_frames=74790
LTP CASE RUNTIME fchmod01: 1953 ms
========== END ltp fchmod01 ==========
========== START ltp rmdir01 ==========
RUN LTP CASE rmdir01
LTP MEMORY rmdir01 before: free_frames=121376 allocated_frames=74790
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
[37m[477.913331 0:1379 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE rmdir01 : 0
Pass!
LTP MEMORY rmdir01 after_run: free_frames=121355 allocated_frames=74811
LTP MEMORY rmdir01 after_cleanup: free_frames=121355 allocated_frames=74811
LTP CASE RUNTIME rmdir01: 2011 ms
========== END ltp rmdir01 ==========
========== START ltp symlink01 ==========
RUN LTP CASE symlink01
LTP MEMORY symlink01 before: free_frames=121355 allocated_frames=74811
[37m[479.959004 0:1383 axfs::root:433] [33m[AxError::IsADirectory][m
[msymlink01    1  TPASS  :  Creation of symbolic link file to no object file is ok
symlink01    2  TPASS  :  Creation of symbolic link file to no object file is ok
symlink01    3  TPASS  :  Creation of symbolic link file and object file via symbolic link is ok
symlink01    4  TPASS  :  Creating an existing symbolic link file error is caught
symlink01    5  TPASS  :  Creating a symbolic link which exceeds maximum pathname error is caught
FAIL LTP CASE symlink01 : 0
Pass!
LTP MEMORY symlink01 after_run: free_frames=121343 allocated_frames=74823
LTP MEMORY symlink01 after_cleanup: free_frames=121343 allocated_frames=74823
LTP CASE RUNTIME symlink01: 2045 ms
========== END ltp symlink01 ==========
========== START ltp readlink01 ==========
RUN LTP CASE readlink01
LTP MEMORY readlink01 before: free_frames=121343 allocated_frames=74823
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
[37m[482.030229 0:1384 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[482.031815 0:1384 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE readlink01 : 0
Pass!
LTP MEMORY readlink01 after_run: free_frames=121313 allocated_frames=74853
LTP MEMORY readlink01 after_cleanup: free_frames=121313 allocated_frames=74853
LTP CASE RUNTIME readlink01: 2070 ms
========== END ltp readlink01 ==========
========== START ltp ftruncate01 ==========
RUN LTP CASE ftruncate01
LTP MEMORY ftruncate01 before: free_frames=121313 allocated_frames=74853
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
[37m[484.005255 0:1389 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[484.006472 0:1389 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE ftruncate01 : 0
Pass!
LTP MEMORY ftruncate01 after_run: free_frames=121292 allocated_frames=74874
LTP MEMORY ftruncate01 after_cleanup: free_frames=121292 allocated_frames=74874
LTP CASE RUNTIME ftruncate01: 1974 ms
========== END ltp ftruncate01 ==========
========== START ltp umask01 ==========
RUN LTP CASE umask01
LTP MEMORY umask01 before: free_frames=121292 allocated_frames=74874
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
[37m[486.031478 0:1393 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE umask01 : 0
Pass!
LTP MEMORY umask01 after_run: free_frames=121271 allocated_frames=74895
LTP MEMORY umask01 after_cleanup: free_frames=121271 allocated_frames=74895
LTP CASE RUNTIME umask01: 2041 ms
========== END ltp umask01 ==========
========== START ltp alarm02 ==========
RUN LTP CASE alarm02
LTP MEMORY alarm02 before: free_frames=121271 allocated_frames=74895
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
[37m[487.954506 0:1397 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE alarm02 : 0
Pass!
LTP MEMORY alarm02 after_run: free_frames=121250 allocated_frames=74916
LTP MEMORY alarm02 after_cleanup: free_frames=121250 allocated_frames=74916
LTP CASE RUNTIME alarm02: 1901 ms
========== END ltp alarm02 ==========
========== START ltp alarm03 ==========
RUN LTP CASE alarm03
LTP MEMORY alarm03 before: free_frames=121250 allocated_frames=74916
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
[37m[489.917388 0:1404 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE alarm03 : 0
Pass!
LTP MEMORY alarm03 after_run: free_frames=121220 allocated_frames=74946
LTP MEMORY alarm03 after_cleanup: free_frames=121220 allocated_frames=74946
LTP CASE RUNTIME alarm03: 1961 ms
========== END ltp alarm03 ==========
========== START ltp clock_gettime02 ==========
RUN LTP CASE clock_gettime02
LTP MEMORY clock_gettime02 before: free_frames=121220 allocated_frames=74946
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
[37m[491.945374 0:1410 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE clock_gettime02 : 0
Pass!
LTP MEMORY clock_gettime02 after_run: free_frames=121199 allocated_frames=74967
LTP MEMORY clock_gettime02 after_cleanup: free_frames=121199 allocated_frames=74967
LTP CASE RUNTIME clock_gettime02: 2027 ms
========== END ltp clock_gettime02 ==========
========== START ltp gettimeofday01 ==========
RUN LTP CASE gettimeofday01
LTP MEMORY gettimeofday01 before: free_frames=121199 allocated_frames=74967
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
[37m[494.013859 0:1414 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE gettimeofday01 : 0
Pass!
LTP MEMORY gettimeofday01 after_run: free_frames=121178 allocated_frames=74988
LTP MEMORY gettimeofday01 after_cleanup: free_frames=121178 allocated_frames=74988
LTP CASE RUNTIME gettimeofday01: 2068 ms
========== END ltp gettimeofday01 ==========
========== START ltp time01 ==========
RUN LTP CASE time01
LTP MEMORY time01 before: free_frames=121178 allocated_frames=74988
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
time01.c:36: TPASS: time() returned value 495
time01.c:38: TPASS: time() returned value 495, stored value 495 are same

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[495.943336 0:1418 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE time01 : 0
Pass!
LTP MEMORY time01 after_run: free_frames=121157 allocated_frames=75009
LTP MEMORY time01 after_cleanup: free_frames=121157 allocated_frames=75009
LTP CASE RUNTIME time01: 1928 ms
========== END ltp time01 ==========
========== START ltp times01 ==========
RUN LTP CASE times01
LTP MEMORY times01 before: free_frames=121157 allocated_frames=75009
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
times01.c:25: TPASS: times(&mytimes) returned 497941

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[497.960899 0:1422 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE times01 : 0
Pass!
LTP MEMORY times01 after_run: free_frames=121136 allocated_frames=75030
LTP MEMORY times01 after_cleanup: free_frames=121136 allocated_frames=75030
LTP CASE RUNTIME times01: 2020 ms
========== END ltp times01 ==========
========== START ltp kill03 ==========
RUN LTP CASE kill03
LTP MEMORY kill03 before: free_frames=121136 allocated_frames=75030
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
[37m[500.000261 0:1426 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE kill03 : 0
Pass!
LTP MEMORY kill03 after_run: free_frames=121115 allocated_frames=75051
LTP MEMORY kill03 after_cleanup: free_frames=121115 allocated_frames=75051
LTP CASE RUNTIME kill03: 2033 ms
========== END ltp kill03 ==========
========== START ltp rt_sigaction01 ==========
RUN LTP CASE rt_sigaction01
LTP MEMORY rt_sigaction01 before: free_frames=121115 allocated_frames=75051
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
rt_sigaction01    0  TINFO  : [37m[502.965495 0:1430 axfs::root:433] [33m[AxError::IsADirectory][m
[m sa.sa_flags = SA_NOMASK
rt_sigaction01    0  TINFO  :  Signal Handler Called with signal number 64
FAIL LTP CASE rt_sigaction01 : 0
Pass!
LTP MEMORY rt_sigaction01 after_run: free_frames=121103 allocated_frames=75063
LTP MEMORY rt_sigaction01 after_cleanup: free_frames=121103 allocated_frames=75063
LTP CASE RUNTIME rt_sigaction01: 2987 ms
========== END ltp rt_sigaction01 ==========
========== START ltp rt_sigaction02 ==========
RUN LTP CASE rt_sigaction02
LTP MEMORY rt_sigaction02 before: free_frames=121103 allocated_frames=75063
rt_sigaction02    0  TINFO  :  Signal 34
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02    1  TPASS  :  rt_sigaction02 failure with sig: 34 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02    2  TPASS  :  rt_sigaction02 failure with sig: 34 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02    3  TPASS  :  rt_sigaction02 failure with sig: 34 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02    4  TPASS  :  rt_sigaction02 failure with sig: 34 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02    5  TPASS  :  rt_sigaction02 failure with sig: 34 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 35
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02    6  TPASS  :  rt_sigaction02 failure with sig: 35 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02    7  TPASS  :  rt_sigaction02 failure with sig: 35 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02    8  TPASS  :  rt_sigaction02 failure with sig: 35 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02    9  TPASS  :  rt_sigaction02 failure with sig: 35 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   10  TPASS  :  rt_sigaction02 failure with sig: 35 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 36
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   11  TPASS  :  rt_sigaction02 failure with sig: 36 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   12  TPASS  :  rt_sigaction02 failure with sig: 36 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   13  TPASS  :  rt_sigaction02 failure with sig: 36 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   14  TPASS  :  rt_sigaction02 failure with sig: 36 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   15  TPASS  :  rt_sigaction02 failure with sig: 36 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 37
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   16  TPASS  :  rt_sigaction02 failure with sig: 37 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   17  TPASS  :  rt_sigaction02 failure with sig: 37 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   18  TPASS  :  rt_sigaction02 failure with sig: 37 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   19  TPASS  :  rt_sigaction02 failure with sig: 37 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   20  TPASS  :  rt_sigaction02 failure with sig: 37 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 38
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   21  TPASS  :  rt_sigaction02 failure with sig: 38 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   22  TPASS  :  rt_sigaction02 failure with sig: 38 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   23  TPASS  :  rt_sigaction02 failure with sig: 38 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   24  TPASS  :  rt_sigaction02 failure with sig: 38 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   25  TPASS  :  rt_sigaction02 failure with sig: 38 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 39
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   26  TPASS  :  rt_sigaction02 failure with sig: 39 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   27  TPASS  :  rt_sigaction02 failure with sig: 39 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   28  TPASS  :  rt_sigaction02 failure with sig: 39 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   29  TPASS  :  rt_sigaction02 failure with sig: 39 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   30  TPASS  :  rt_sigaction02 failure with sig: 39 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 40
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   31  TPASS  :  rt_sigaction02 failure with sig: 40 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   32  TPASS  :  rt_sigaction02 failure with sig: 40 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   33  TPASS  :  rt_sigaction02 failure with sig: 40 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   34  TPASS  :  rt_sigaction02 failure with sig: 40 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   35  TPASS  :  rt_sigaction02 failure with sig: 40 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 41
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   36  TPASS  :  rt_sigaction02 failure with sig: 41 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   37  TPASS  :  rt_sigaction02 failure with sig: 41 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   38  TPASS  :  rt_sigaction02 failure with sig: 41 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   39  TPASS  :  rt_sigaction02 failure with sig: 41 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   40  TPASS  :  rt_sigaction02 failure with sig: 41 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 42
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   41  TPASS  :  rt_sigaction02 failure with sig: 42 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   42  TPASS  :  rt_sigaction02 failure with sig: 42 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   43  TPASS  :  rt_sigaction02 failure with sig: 42 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   44  TPASS  :  rt_sigaction02 failure with sig: 42 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   45  TPASS  :  rt_sigaction02 failure with sig: 42 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 43
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   46  TPASS  :  rt_sigaction02 failure with sig: 43 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   47  TPASS  :  rt_sigaction02 failure with sig: 43 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   48  TPASS  :  rt_sigaction02 failure with sig: 43 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   49  TPASS  :  rt_sigaction02 failure with sig: 43 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   50  TPASS  :  rt_sigaction02 failure with sig: 43 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 44
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   51  TPASS  :  rt_sigaction02 failure with sig: 44 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   52  TPASS  :  rt_sigaction02 failure with sig: 44 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   53  TPASS  :  rt_sigaction02 failure with sig: 44 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   54  TPASS  :  rt_sigaction02 failure with sig: 44 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   55  TPASS  :  rt_sigaction02 failure with sig: 44 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 45
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   56  TPASS  :  rt_sigaction02 failure with sig: 45 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   57  TPASS  :  rt_sigaction02 failure with sig: 45 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   58  TPASS  :  rt_sigaction02 failure with sig: 45 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   59  TPASS  :  rt_sigaction02 failure with sig: 45 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   60  TPASS  :  rt_sigaction02 failure with sig: 45 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 46
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   61  TPASS  :  rt_sigaction02 failure with sig: 46 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   62  TPASS  :  rt_sigaction02 failure with sig: 46 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   63  TPASS  :  rt_sigaction02 failure with sig: 46 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   64  TPASS  :  rt_sigaction02 failure with sig: 46 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   65  TPASS  :  rt_sigaction02 failure with sig: 46 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 47
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   66  TPASS  :  rt_sigaction02 failure with sig: 47 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   67  TPASS  :  rt_sigaction02 failure with sig: 47 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   68  TPASS  :  rt_sigaction02 failure with sig: 47 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   69  TPASS  :  rt_sigaction02 failure with sig: 47 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   70  TPASS  :  rt_sigaction02 failure with sig: 47 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 48
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   71  TPASS  :  rt_sigaction02 failure with sig: 48 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   72  TPASS  :  rt_sigaction02 failure with sig: 48 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   73  TPASS  :  rt_sigaction02 failure with sig: 48 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   74  TPASS  :  rt_sigaction02 failure with sig: 48 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   75  TPASS  :  rt_sigaction02 failure with sig: 48 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 49
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   76  TPASS  :  rt_sigaction02 failure with sig: 49 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   77  TPASS  :  rt_sigaction02 failure with sig: 49 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   78  TPASS  :  rt_sigaction02 failure with sig: 49 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   79  TPASS  :  rt_sigaction02 failure with sig: 49 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   80  TPASS  :  rt_sigaction02 failure with sig: 49 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 50
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   81  TPASS  :  rt_sigaction02 failure with sig: 50 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   82  TPASS  :  rt_sigaction02 failure with sig: 50 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   83  TPASS  :  rt_sigaction02 failure with sig: 50 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   84  TPASS  :  rt_sigaction02 failure with sig: 50 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   85  TPASS  :  rt_sigaction02 failure with sig: 50 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 51
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   86  TPASS  :  rt_sigaction02 failure with sig: 51 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   87  TPASS  :  rt_sigaction02 failure with sig: 51 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   88  TPASS  :  rt_sigaction02 failure with sig: 51 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   89  TPASS  :  rt_sigaction02 failure with sig: 51 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   90  TPASS  :  rt_sigaction02 failure with sig: 51 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 52
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   91  TPASS  :  rt_sigaction02 failure with sig: 52 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   92  TPASS  :  rt_sigaction02 failure with sig: 52 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   93  TPASS  :  rt_sigaction02 failure with sig: 52 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   94  TPASS  :  rt_sigaction02 failure with sig: 52 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02   95  TPASS  :  rt_sigaction02 failure with sig: 52 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 53
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   96  TPASS  :  rt_sigaction02 failure with sig: 53 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02   97  TPASS  :  rt_sigaction02 failure with sig: 53 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   98  TPASS  :  rt_sigaction02 failure with sig: 53 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02   99  TPASS  :  rt_sigaction02 failure with sig: 53 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  100  TPASS  :  rt_sigaction02 failure with sig: 53 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 54
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  101  TPASS  :  rt_sigaction02 failure with sig: 54 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  102  TPASS  :  rt_sigaction02 failure with sig: 54 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  103  TPASS  :  rt_sigaction02 failure with sig: 54 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  104  TPASS  :  rt_sigaction02 failure with sig: 54 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  105  TPASS  :  rt_sigaction02 failure with sig: 54 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 55
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  106  TPASS  :  rt_sigaction02 failure with sig: 55 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  107  TPASS  :  rt_sigaction02 failure with sig: 55 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  108  TPASS  :  rt_sigaction02 failure with sig: 55 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  109  TPASS  :  rt_sigaction02 failure with sig: 55 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  110  TPASS  :  rt_sigaction02 failure with sig: 55 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 56
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  111  TPASS  :  rt_sigaction02 failure with sig: 56 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  112  TPASS  :  rt_sigaction02 failure with sig: 56 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  113  TPASS  :  rt_sigaction02 failure with sig: 56 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  114  TPASS  :  rt_sigaction02 failure with sig: 56 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  115  TPASS  :  rt_sigaction02 failure with sig: 56 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 57
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  116  TPASS  :  rt_sigaction02 failure with sig: 57 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  117  TPASS  :  rt_sigaction02 failure with sig: 57 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  118  TPASS  :  rt_sigaction02 failure with sig: 57 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  119  TPASS  :  rt_sigaction02 failure with sig: 57 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  120  TPASS  :  rt_sigaction02 failure with sig: 57 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 58
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  121  TPASS  :  rt_sigaction02 failure with sig: 58 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  122  TPASS  :  rt_sigaction02 failure with sig: 58 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  123  TPASS  :  rt_sigaction02 failure with sig: 58 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  124  TPASS  :  rt_sigaction02 failure with sig: 58 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  125  TPASS  :  rt_sigaction02 failure with sig: 58 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 59
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  126  TPASS  :  rt_sigaction02 failure with sig: 59 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  127  TPASS  :  rt_sigaction02 failure with sig: 59 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  128  TPASS  :  rt_sigaction02 failure with sig: 59 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  129  TPASS  :  rt_sigaction02 failure with sig: 59 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  130  TPASS  :  rt_sigaction02 failure with sig: 59 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 60
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  131  TPASS  :  rt_sigaction02 failure with sig: 60 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  132  TPASS  :  rt_sigaction02 failure with sig: 60 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  133  TPASS  :  rt_sigaction02 failure with sig: 60 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  134  TPASS  :  rt_sigaction02 failure with sig: 60 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  135  TPASS  :  rt_sigaction02 failure with sig: 60 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 61
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  136  TPASS  :  rt_sigaction02 failure with sig: 61 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  137  TPASS  :  rt_sigaction02 failure with sig: 61 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  138  TPASS  :  rt_sigaction02 failure with sig: 61 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  139  TPASS  :  rt_sigaction02 failure with sig: 61 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  140  TPASS  :  rt_sigaction02 failure with sig: 61 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 62
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  141  TPASS  :  rt_sigaction02 failure with sig: 62 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  142  TPASS  :  rt_sigaction02 failure with sig: 62 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  143  TPASS  :  rt_sigaction02 failure with sig: 62 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  144  TPASS  :  rt_sigaction02 failure with sig: 62 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  145  TPASS  :  rt_sigaction02 failure with sig: 62 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 63
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  146  TPASS  :  rt_sigaction02 failure with sig: 63 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  147  TPASS  :  rt_sigaction02 failure with sig: 63 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  148  TPASS  :  rt_sigaction02 failure with sig: 63 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  149  TPASS  :  rt_sigaction02 failure with sig: 63 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  150  TPASS  :  rt_sigaction02 failure with sig: 63 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  Signal 64
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  151  TPASS  :  rt_sigaction02 failure with sig: 64 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND
rt_sigaction02  152  TPASS  :  rt_sigaction02 failure with sig: 64 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  153  TPASS  :  rt_sigaction02 failure with sig: 64 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_RESETHAND|SA_SIGINFO
rt_sigaction02  154  TPASS  :  rt_sigaction02 failure with sig: 64 as expected errno  = EFAULT : Bad address
rt_sigaction02    0  TINFO  :  sa.sa_flags = SA_NOMASK
rt_sigaction02  155  TPASS  [37m[505.324483 0:1431 axfs::root:433] [33m[AxError::IsADirectory][m
[m:  rt_sigaction02 failure with sig: 64 as expected errno  = EFAULT : Bad address
FAIL LTP CASE rt_sigaction02 : 0
Pass!
LTP MEMORY rt_sigaction02 after_run: free_frames=121091 allocated_frames=75075
LTP MEMORY rt_sigaction02 after_cleanup: free_frames=121091 allocated_frames=75075
LTP CASE RUNTIME rt_sigaction02: 2345 ms
========== END ltp rt_sigaction02 ==========
========== START ltp sigaction01 ==========
RUN LTP CASE sigaction01
LTP MEMORY sigaction01 before: free_frames=121091 allocated_frames=75075
sigaction01    1  TPASS  :  SA_RESETHAND did not cause SA_SIGINFO to be cleared
sigaction01    2  TPASS  :  SA_RESETHAND was masked when handler executed
sigaction01    3  TPASS  :  sig has been masked because sa_mask originally contained sig
sigaction01    4  TPASS  :  siginfo pointer non NULL
FAIL LTP CASE sigaction01 : 0
Pass!
LTP MEMORY sigaction01 after_run: free_frames=121079 allocated_frames=75087
LTP MEMORY sigaction01 after_cleanup: free_frames=121079 allocated_frames=75087
LTP CASE RUNTIME sigaction01: 1981 ms
========== END ltp sigaction01 ==========
========== START ltp proc01 ==========
RUN LTP CASE proc01
LTP MEMORY proc01 before: free_frames=121079 allocated_frames=75087
[37m[509.469236 0:1433 axfs::root:433] [33m[AxError::IsADirectory][m
[mproc01      1  TPASS  :  readproc() completed successfully, total read: 875 bytes, 20 objs
FAIL LTP CASE proc01 : 0
Pass!
LTP MEMORY proc01 after_run: free_frames=121067 allocated_frames=75099
LTP MEMORY proc01 after_cleanup: free_frames=121067 allocated_frames=75099
LTP CASE RUNTIME proc01: 2180 ms
========== END ltp proc01 ==========
========== START ltp exit01 ==========
RUN LTP CASE exit01
LTP MEMORY exit01 before: free_frames=121067 allocated_frames=75099
exit01      1  TPASS  :  exit() test PASSED
FAIL LTP CASE exit01 : 0
Pass!
LTP MEMORY exit01 after_run: free_frames=121046 allocated_frames=75120
LTP MEMORY exit01 after_cleanup: free_frames=121046 allocated_frames=75120
LTP CASE RUNTIME exit01: 2173 ms
========== END ltp exit01 ==========
========== START ltp exit02 ==========
RUN LTP CASE exit02
LTP MEMORY exit02 before: free_frames=121046 allocated_frames=75120
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
[37m[513.472116 0:1436 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE exit02 : 0
Pass!
LTP MEMORY exit02 after_run: free_frames=121016 allocated_frames=75150
LTP MEMORY exit02 after_cleanup: free_frames=121016 allocated_frames=75150
LTP CASE RUNTIME exit02: 1804 ms
========== END ltp exit02 ==========
========== START ltp exit_group01 ==========
RUN LTP CASE exit_group01
LTP MEMORY exit_group01 before: free_frames=121016 allocated_frames=75150
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
[37m[515.596073 0:1441 axfs::fops:297] [33m[AxError::NotADirectory][m
[m[37m[515.598069 0:1441 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE exit_group01 : 0
Pass!
LTP MEMORY exit_group01 after_run: free_frames=120984 allocated_frames=75182
LTP MEMORY exit_group01 after_cleanup: free_frames=120984 allocated_frames=75182
LTP CASE RUNTIME exit_group01: 2107 ms
========== END ltp exit_group01 ==========
========== START ltp getpgrp01 ==========
RUN LTP CASE getpgrp01
LTP MEMORY getpgrp01 before: free_frames=120984 allocated_frames=75182
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getpgrp01.c:18: TPASS: getpgrp() returned pid 1450
getpgrp01.c:19: TPASS: TST_RET == SAFE_GETPGID(0) (1450)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[517.532663 0:1448 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getpgrp01 : 0
Pass!
LTP MEMORY getpgrp01 after_run: free_frames=120963 allocated_frames=75203
LTP MEMORY getpgrp01 after_cleanup: free_frames=120963 allocated_frames=75203
LTP CASE RUNTIME getpgrp01: 1941 ms
========== END ltp getpgrp01 ==========
========== START ltp getsid01 ==========
RUN LTP CASE getsid01
LTP MEMORY getsid01 before: free_frames=120963 allocated_frames=75203
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getsid01.c:41: TPASS: p_sid == c_sid (1452)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[519.653994 0:1452 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getsid01 : 0
Pass!
LTP MEMORY getsid01 after_run: free_frames=120933 allocated_frames=75233
LTP MEMORY getsid01 after_cleanup: free_frames=120933 allocated_frames=75233
LTP CASE RUNTIME getsid01: 2121 ms
========== END ltp getsid01 ==========
========== START ltp gettid01 ==========
RUN LTP CASE gettid01
LTP MEMORY gettid01 before: free_frames=120933 allocated_frames=75233
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
gettid01.c:26: TPASS: tst_syscall(__NR_gettid) == tst_syscall(__NR_getpid) (1459)
gettid01.c:27: TPASS: tst_syscall(__NR_gettid) == pid (1459)

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[521.568728 0:1457 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE gettid01 : 0
Pass!
LTP MEMORY gettid01 after_run: free_frames=120912 allocated_frames=75254
LTP MEMORY gettid01 after_cleanup: free_frames=120912 allocated_frames=75254
LTP CASE RUNTIME gettid01: 1905 ms
========== END ltp gettid01 ==========
========== START ltp uname01 ==========
RUN LTP CASE uname01
LTP MEMORY uname01 before: free_frames=120912 allocated_frames=75254
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
[37m[523.508788 0:1461 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE uname01 : 0
Pass!
LTP MEMORY uname01 after_run: free_frames=120891 allocated_frames=75275
LTP MEMORY uname01 after_cleanup: free_frames=120891 allocated_frames=75275
LTP CASE RUNTIME uname01: 1934 ms
========== END ltp uname01 ==========
========== START ltp uname04 ==========
RUN LTP CASE uname04
LTP MEMORY uname04 before: free_frames=120891 allocated_frames=75275
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
uname04.c:70: TINFO: Calling uname with default personality
uname04.c:62: TPASS: No bytes leaked
uname04.c:73: TINFO: Calling uname with UNAME26 personality
uname04.c:62: TPASS: No bytes leaked

Summary:
passed   2
failed   0
broken   0
skipped  0
warnings 0
[37m[525.433393 0:1465 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE uname04 : 0
Pass!
LTP MEMORY uname04 after_run: free_frames=120870 allocated_frames=75296
LTP MEMORY uname04 after_cleanup: free_frames=120870 allocated_frames=75296
LTP CASE RUNTIME uname04: 1925 ms
========== END ltp uname04 ==========
========== START ltp getrlimit01 ==========
RUN LTP CASE getrlimit01
LTP MEMORY getrlimit01 before: free_frames=120870 allocated_frames=75296
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
[37m[527.162613 0:1469 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getrlimit01 : 0
Pass!
LTP MEMORY getrlimit01 after_run: free_frames=120849 allocated_frames=75317
LTP MEMORY getrlimit01 after_cleanup: free_frames=120849 allocated_frames=75317
LTP CASE RUNTIME getrlimit01: 1740 ms
========== END ltp getrlimit01 ==========
========== START ltp getrusage01 ==========
RUN LTP CASE getrusage01
LTP MEMORY getrusage01 before: free_frames=120849 allocated_frames=75317
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
[37m[529.181035 0:1473 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getrusage01 : 0
Pass!
LTP MEMORY getrusage01 after_run: free_frames=120828 allocated_frames=75338
LTP MEMORY getrusage01 after_cleanup: free_frames=120828 allocated_frames=75338
LTP CASE RUNTIME getrusage01: 2007 ms
========== END ltp getrusage01 ==========
========== START ltp sched_getscheduler01 ==========
RUN LTP CASE sched_getscheduler01
LTP MEMORY sched_getscheduler01 before: free_frames=120828 allocated_frames=75338
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
sched_getscheduler01.c:58: TINFO: Testing libc variant
sched_getscheduler01.c:51: TPASS: got expected policy 2
sched_getscheduler01.c:51: TPASS: got expected policy 0
sched_getscheduler01.c:51: TPASS: got expected policy 1
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
sched_getscheduler01.c:58: TINFO: Testing syscall variant
sched_getscheduler01.c:51: TPASS: got expected policy 2
sched_getscheduler01.c:51: TPASS: got expected policy 0
sched_getscheduler01.c:51: TPASS: got expected policy 1

Summary:
passed   6
failed   0
broken   0
skipped  0
warnings 0
[37m[531.468753 0:1477 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE sched_getscheduler01 : 0
Pass!
LTP MEMORY sched_getscheduler01 after_run: free_frames=120798 allocated_frames=75368
LTP MEMORY sched_getscheduler01 after_cleanup: free_frames=120798 allocated_frames=75368
LTP CASE RUNTIME sched_getscheduler01: 2283 ms
========== END ltp sched_getscheduler01 ==========
========== START ltp sched_yield01 ==========
RUN LTP CASE sched_yield01
LTP MEMORY sched_yield01 before: free_frames=120798 allocated_frames=75368
sched_yield01    1  TPASS  :  sched_yield() call succeeded
FAIL LTP CASE sched_yield01 : 0
Pass!
LTP MEMORY sched_yield01 after_run: free_frames=120786 allocated_frames=75380
LTP MEMORY sched_yield01 after_cleanup: free_frames=120786 allocated_frames=75380
LTP CASE RUNTIME sched_yield01: 1984 ms
========== END ltp sched_yield01 ==========
========== START ltp getpgid02 ==========
RUN LTP CASE getpgid02
LTP MEMORY getpgid02 before: free_frames=120786 allocated_frames=75380
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
[37m[535.400536 0:1485 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getpgid02 : 0
Pass!
LTP MEMORY getpgid02 after_run: free_frames=120765 allocated_frames=75401
LTP MEMORY getpgid02 after_cleanup: free_frames=120765 allocated_frames=75401
LTP CASE RUNTIME getpgid02: 1950 ms
========== END ltp getpgid02 ==========
========== START ltp getsid02 ==========
RUN LTP CASE getsid02
LTP MEMORY getsid02 before: free_frames=120765 allocated_frames=75401
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
[37m[537.531884 0:1489 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getsid02 : 0
Pass!
LTP MEMORY getsid02 after_run: free_frames=120744 allocated_frames=75422
LTP MEMORY getsid02 after_cleanup: free_frames=120744 allocated_frames=75422
LTP CASE RUNTIME getsid02: 2130 ms
========== END ltp getsid02 ==========
========== START ltp getppid02 ==========
RUN LTP CASE getppid02
LTP MEMORY getppid02 before: free_frames=120744 allocated_frames=75422
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
tst_test.c:1733: TINFO: LTP version: 20240524
tst_test.c:1617: TINFO: Timeout per run is 0h 00m 30s
tst_memutils.c:152: TINFO: oom_score_adj does not exist, skipping the adjustment
getppid02.c:31: TPASS: getppid() returned parent pid (1495)

Summary:
passed   1
failed   0
broken   0
skipped  0
warnings 0
[37m[539.630144 0:1493 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getppid02 : 0
Pass!
LTP MEMORY getppid02 after_run: free_frames=120714 allocated_frames=75452
LTP MEMORY getppid02 after_cleanup: free_frames=120714 allocated_frames=75452
LTP CASE RUNTIME getppid02: 2094 ms
========== END ltp getppid02 ==========
========== START ltp getuid03 ==========
RUN LTP CASE getuid03
LTP MEMORY getuid03 before: free_frames=120714 allocated_frames=75452
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
[37m[541.712167 0:1498 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getuid03 : 0
Pass!
LTP MEMORY getuid03 after_run: free_frames=120693 allocated_frames=75473
LTP MEMORY getuid03 after_cleanup: free_frames=120693 allocated_frames=75473
LTP CASE RUNTIME getuid03: 2111 ms
========== END ltp getuid03 ==========
========== START ltp geteuid02 ==========
RUN LTP CASE geteuid02
LTP MEMORY geteuid02 before: free_frames=120693 allocated_frames=75473
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
[37m[543.776581 0:1502 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE geteuid02 : 0
Pass!
LTP MEMORY geteuid02 after_run: free_frames=120672 allocated_frames=75494
LTP MEMORY geteuid02 after_cleanup: free_frames=120672 allocated_frames=75494
LTP CASE RUNTIME geteuid02: 2025 ms
========== END ltp geteuid02 ==========
========== START ltp getgid03 ==========
RUN LTP CASE getgid03
LTP MEMORY getgid03 before: free_frames=120672 allocated_frames=75494
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
[37m[545.830577 0:1506 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getgid03 : 0
Pass!
LTP MEMORY getgid03 after_run: free_frames=120651 allocated_frames=75515
LTP MEMORY getgid03 after_cleanup: free_frames=120651 allocated_frames=75515
LTP CASE RUNTIME getgid03: 2035 ms
========== END ltp getgid03 ==========
========== START ltp getegid02 ==========
RUN LTP CASE getegid02
LTP MEMORY getegid02 before: free_frames=120651 allocated_frames=75515
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
[37m[547.854538 0:1510 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getegid02 : 0
Pass!
LTP MEMORY getegid02 after_run: free_frames=120630 allocated_frames=75536
LTP MEMORY getegid02 after_cleanup: free_frames=120630 allocated_frames=75536
LTP CASE RUNTIME getegid02: 2041 ms
========== END ltp getegid02 ==========
========== START ltp getgroups03 ==========
RUN LTP CASE getgroups03
LTP MEMORY getgroups03 before: free_frames=120630 allocated_frames=75536
getgroups03    1  TPASS  :  getgroups functionality correct
FAIL LTP CASE getgroups03 : 0
Pass!
LTP MEMORY getgroups03 after_run: free_frames=120618 allocated_frames=75548
LTP MEMORY getgroups03 after_cleanup: free_frames=120618 allocated_frames=75548
LTP CASE RUNTIME getgroups03: 1997 ms
========== END ltp getgroups03 ==========
========== START ltp uname02 ==========
RUN LTP CASE uname02
LTP MEMORY uname02 before: free_frames=120618 allocated_frames=75548
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
[37m[551.898280 0:1515 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE uname02 : 0
Pass!
LTP MEMORY uname02 after_run: free_frames=120597 allocated_frames=75569
LTP MEMORY uname02 after_cleanup: free_frames=120597 allocated_frames=75569
LTP CASE RUNTIME uname02: 2025 ms
========== END ltp uname02 ==========
========== START ltp wait01 ==========
RUN LTP CASE wait01
LTP MEMORY wait01 before: free_frames=120597 allocated_frames=75569
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
[37m[553.876427 0:1519 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE wait01 : 0
Pass!
LTP MEMORY wait01 after_run: free_frames=120576 allocated_frames=75590
LTP MEMORY wait01 after_cleanup: free_frames=120576 allocated_frames=75590
LTP CASE RUNTIME wait01: 1972 ms
========== END ltp wait01 ==========
========== START ltp wait02 ==========
RUN LTP CASE wait02
LTP MEMORY wait02 before: free_frames=120576 allocated_frames=75590
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
[37m[555.875672 0:1523 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE wait02 : 0
Pass!
LTP MEMORY wait02 after_run: free_frames=120546 allocated_frames=75620
LTP MEMORY wait02 after_cleanup: free_frames=120546 allocated_frames=75620
LTP CASE RUNTIME wait02: 2003 ms
========== END ltp wait02 ==========
========== START ltp getrlimit02 ==========
RUN LTP CASE getrlimit02
LTP MEMORY getrlimit02 before: free_frames=120546 allocated_frames=75620
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
[37m[557.879247 0:1528 axfs::root:433] [33m[AxError::IsADirectory][m
[mFAIL LTP CASE getrlimit02 : 0
Pass!
LTP MEMORY getrlimit02 after_run: free_frames=120525 allocated_frames=75641
LTP MEMORY getrlimit02 after_cleanup: free_frames=120525 allocated_frames=75641
LTP CASE RUNTIME getrlimit02: 1997 ms
========== END ltp getrlimit02 ==========
ltp cases: 85 passed, 0 failed, 0 timed out
#### OS COMP TEST GROUP END ltp-glibc ####
#### OS COMP TEST GROUP START libcbench-musl ####
b_malloc_sparse (0)
  time: 1.872349590, virt: 0, res: 0, dirty: 0

b_malloc_bubble (0)
  time: 1.927983180, virt: 0, res: 0, dirty: 0

b_malloc_tiny1 (0)
  time: 0.027109550, virt: 0, res: 0, dirty: 0

b_malloc_tiny2 (0)
  time: 0.022761060, virt: 0, res: 0, dirty: 0

b_malloc_big1 (0)
  time: 0.723155220, virt: 0, res: 0, dirty: 0

b_malloc_big2 (0)
  time: 0.554140540, virt: 0, res: 0, dirty: 0

b_malloc_thread_stress (0)
  time: 0.308541770, virt: 0, res: 0, dirty: 0

b_malloc_thread_local (0)
  time: 0.176818340, virt: 0, res: 0, dirty: 0

b_string_strstr ("abcdefghijklmnopqrstuvwxyz")
  time: 0.028343110, virt: 0, res: 0, dirty: 0

b_string_strstr ("azbycxdwevfugthsirjqkplomn")
  time: 0.042039500, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaacccccccccccc")
  time: 0.027202550, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.026567340, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.033730540, virt: 0, res: 0, dirty: 0

b_string_memset (0)
  time: 0.019890240, virt: 0, res: 0, dirty: 0

b_string_strchr (0)
  time: 0.036785600, virt: 0, res: 0, dirty: 0

b_string_strlen (0)
  time: 0.030853260, virt: 0, res: 0, dirty: 0

b_pthread_createjoin_serial1 (0)
  time: 0.005756320, virt: 0, res: 0, dirty: 0

b_pthread_createjoin_serial2 (0)
  time: 1.107819520, virt: 0, res: 0, dirty: 0

b_pthread_create_serial1 (0)
  time: 0.002968320, virt: 0, res: 0, dirty: 0

b_pthread_uselesslock (0)
  time: 0.203411350, virt: 0, res: 0, dirty: 0

b_utf8_bigbuf (0)
  time: 0.149511170, virt: 0, res: 0, dirty: 0

b_utf8_onebyone (0)
  time: 0.281350550, virt: 0, res: 0, dirty: 0

b_stdio_putcgetc (0)
  time: 0.607624660, virt: 0, res: 0, dirty: 0

b_stdio_putcgetc_unlocked (0)
  time: 0.551166670, virt: 0, res: 0, dirty: 0

b_regex_compile ("(a|b|c)*d*b")
  time: 0.920762420, virt: 0, res: 0, dirty: 0

b_regex_search ("(a|b|c)*d*b")
  time: 0.267550040, virt: 0, res: 0, dirty: 0

b_regex_search ("a{25}b")
  time: 0.585118590, virt: 0, res: 0, dirty: 0

#### OS COMP TEST GROUP END libcbench-musl ####
#### OS COMP TEST GROUP START libcbench-glibc ####
b_malloc_sparse (0)
  time: 1.743131530, virt: 0, res: 0, dirty: 0

b_malloc_bubble (0)
  time: 1.670904420, virt: 0, res: 0, dirty: 0

b_malloc_tiny1 (0)
  time: 0.026481770, virt: 0, res: 0, dirty: 0

b_malloc_tiny2 (0)
  time: 0.033076240, virt: 0, res: 0, dirty: 0

b_malloc_big1 (0)
  time: 0.376283010, virt: 0, res: 0, dirty: 0

b_malloc_big2 (0)
  time: 0.402991140, virt: 0, res: 0, dirty: 0

The futex facility returned an unexpected error code.
The futex facility returned an unexpected error code.
b_string_strstr ("abcdefghijklmnopqrstuvwxyz")
  time: 0.024760650, virt: 0, res: 0, dirty: 0

b_string_strstr ("azbycxdwevfugthsirjqkplomn")
  time: 0.023663260, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaacccccccccccc")
  time: 0.049226340, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.035255950, virt: 0, res: 0, dirty: 0

b_string_strstr ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaac")
  time: 0.042708010, virt: 0, res: 0, dirty: 0

b_string_memset (0)
  time: 0.036478440, virt: 0, res: 0, dirty: 0

b_string_strchr (0)
  time: 0.059729720, virt: 0, res: 0, dirty: 0

b_string_strlen (0)
  time: 0.049426130, virt: 0, res: 0, dirty: 0

The futex facility returned an unexpected error code.
b_pthread_create_serial1 (0)
  time: 0.005118300, virt: 0, res: 0, dirty: 0

The futex facility returned an unexpected error code.
b_utf8_bigbuf (0)
  time: 0.115239410, virt: 0, res: 0, dirty: 0

b_utf8_onebyone (0)
  time: 0.113072930, virt: 0, res: 0, dirty: 0

b_regex_compile ("(a|b|c)*d*b")
  time: 0.042801310, virt: 0, res: 0, dirty: 0

b_regex_search ("(a|b|c)*d*b")
  time: 0.016564640, virt: 0, res: 0, dirty: 0

b_regex_search ("a{25}b")
  time: 0.251125330, virt: 0, res: 0, dirty: 0

#### OS COMP TEST GROUP END libcbench-glibc ####
#### OS COMP TEST GROUP START iperf-musl ####
====== iperf BASIC_UDP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 0.0.0.0 port 49152 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Total Datagrams
[  5]   0.00-2.00   sec  2.43 MBytes  10.2 Mbits/sec  1744
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.00   sec  2.43 MBytes  10.2 Mbits/sec  0.000 ms  0/1744 (0%)  sender
[  5]   0.00-2.00   sec  2.43 MBytes  10.2 Mbits/sec  0.094 ms  0/1744 (0%)  receiver

iperf Done.
====== iperf BASIC_UDP end: success ======

====== iperf BASIC_TCP begin ======
Connecting to host 127.0.0.1, port 5001
[  5] local 127.0.0.1 port 49154 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.01   sec  56.5 MBytes   236 Mbits/sec    0   0.00 Bytes
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.01   sec  56.5 MBytes   236 Mbits/sec    0             sender
[  5]   0.00-2.01   sec  55.6 MBytes   232 Mbits/sec                  receiver

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
[  5]   0.00-2.00   sec   699 KBytes  2.86 Mbits/sec  490
[  7]   0.00-2.00   sec   699 KBytes  2.86 Mbits/sec  490
[  9]   0.00-2.00   sec   699 KBytes  2.86 Mbits/sec  490
[ 11]   0.00-2.00   sec   699 KBytes  2.86 Mbits/sec  490
[ 13]   0.00-2.00   sec   699 KBytes  2.86 Mbits/sec  490
[SUM]   0.00-2.00   sec  3.41 MBytes  14.3 Mbits/sec  2450
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.00   sec   699 KBytes  2.86 Mbits/sec  0.000 ms  0/490 (0%)  sender
[  5]   0.00-2.00   sec   699 KBytes  2.85 Mbits/sec  0.285 ms  0/490 (0%)  receiver
[  7]   0.00-2.00   sec   699 KBytes  2.86 Mbits/sec  0.000 ms  0/490 (0%)  sender
[  7]   0.00-2.00   sec   699 KBytes  2.85 Mbits/sec  0.249 ms  0/490 (0%)  receiver
[  9]   0.00-2.00   sec   699 KBytes  2.86 Mbits/sec  0.000 ms  0/490 (0%)  sender
[  9]   0.00-2.00   sec   699 KBytes  2.85 Mbits/sec  0.727 ms  0/490 (0%)  receiver
[ 11]   0.00-2.00   sec   699 KBytes  2.86 Mbits/sec  0.000 ms  0/490 (0%)  sender
[ 11]   0.00-2.00   sec   699 KBytes  2.85 Mbits/sec  0.806 ms  0/490 (0%)  receiver
[ 13]   0.00-2.00   sec   699 KBytes  2.86 Mbits/sec  0.000 ms  0/490 (0%)  sender
[ 13]   0.00-2.00   sec   699 KBytes  2.85 Mbits/sec  0.382 ms  0/490 (0%)  receiver
[SUM]   0.00-2.00   sec  3.41 MBytes  14.3 Mbits/sec  0.000 ms  0/2450 (0%)  sender
[SUM]   0.00-2.00   sec  3.41 MBytes  14.3 Mbits/sec  0.490 ms  0/2450 (0%)  receiver

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
[  5]   0.00-2.03   sec  13.8 MBytes  56.9 Mbits/sec    0   0.00 Bytes
[  7]   0.00-2.03   sec  13.8 MBytes  56.9 Mbits/sec    0   0.00 Bytes
[  9]   0.00-2.03   sec  13.8 MBytes  56.9 Mbits/sec    0   0.00 Bytes
[ 11]   0.00-2.03   sec  13.8 MBytes  56.9 Mbits/sec    0   0.00 Bytes
[ 13]   0.00-2.03   sec  13.8 MBytes  56.9 Mbits/sec    0   0.00 Bytes
[SUM]   0.00-2.03   sec  68.8 MBytes   284 Mbits/sec    0
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.03   sec  13.8 MBytes  56.9 Mbits/sec    0             sender
[  5]   0.00-2.04   sec  12.9 MBytes  53.0 Mbits/sec                  receiver
[  7]   0.00-2.03   sec  13.8 MBytes  56.9 Mbits/sec    0             sender
[  7]   0.00-2.04   sec  12.9 MBytes  53.0 Mbits/sec                  receiver
[  9]   0.00-2.03   sec  13.8 MBytes  56.9 Mbits/sec    0             sender
[  9]   0.00-2.04   sec  12.9 MBytes  53.0 Mbits/sec                  receiver
[ 11]   0.00-2.03   sec  13.8 MBytes  56.9 Mbits/sec    0             sender
[ 11]   0.00-2.04   sec  12.9 MBytes  53.0 Mbits/sec                  receiver
[ 13]   0.00-2.03   sec  13.8 MBytes  56.9 Mbits/sec    0             sender
[ 13]   0.00-2.04   sec  12.9 MBytes  53.0 Mbits/sec                  receiver
[SUM]   0.00-2.03   sec  68.8 MBytes   284 Mbits/sec    0             sender
[SUM]   0.00-2.04   sec  64.4 MBytes   265 Mbits/sec                  receiver

iperf Done.
====== iperf PARALLEL_TCP end: success ======

====== iperf REVERSE_UDP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 0.0.0.0 port 49158 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.00   sec  2.52 MBytes  10.5 Mbits/sec  0.052 ms  0/1807 (0%)
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Jitter    Lost/Total Datagrams
[  5]   0.00-2.00   sec  2.52 MBytes  10.5 Mbits/sec  0.000 ms  0/1808 (0%)  sender
[  5]   0.00-2.00   sec  2.52 MBytes  10.5 Mbits/sec  0.052 ms  0/1807 (0%)  receiver

iperf Done.
====== iperf REVERSE_UDP end: success ======

====== iperf REVERSE_TCP begin ======
Connecting to host 127.0.0.1, port 5001
Reverse mode, remote host 127.0.0.1 is sending
[  5] local 127.0.0.1 port 49164 connected to 127.0.0.1 port 5001
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-2.01   sec  51.6 MBytes   216 Mbits/sec
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-2.03   sec  52.5 MBytes   217 Mbits/sec    0             sender
[  5]   0.00-2.01   sec  51.6 MBytes   216 Mbits/sec                  receiver

iperf Done.
====== iperf REVERSE_TCP end: success ======

#### OS COMP TEST GROUP END iperf-musl ####
#### OS COMP TEST GROUP START iperf-glibc ####
====== iperf BASIC_UDP begin ======
iperf3: error - control socket has closed unexpectedly
====== iperf BASIC_UDP end: fail ======

====== iperf BASIC_TCP begin ======
[37m[611.061734 0:4192 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[611.062577 0:4192 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf BASIC_TCP end: fail ======

====== iperf PARALLEL_UDP begin ======
[37m[611.145229 0:4193 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[611.151033 0:4193 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf PARALLEL_UDP end: fail ======

====== iperf PARALLEL_TCP begin ======
[37m[611.292344 0:4194 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[611.295350 0:4194 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf PARALLEL_TCP end: fail ======

====== iperf REVERSE_UDP begin ======
[37m[611.424558 0:4195 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[611.428628 0:4195 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf REVERSE_UDP end: fail ======

====== iperf REVERSE_TCP begin ======
[37m[611.512096 0:4196 axnet::smoltcp_impl::listen_table:107] [33m[AxError::ConnectionRefused] loopback socket connect() failed[m
[m[37m[611.515528 0:4196 arceos_posix_api::imp::net:422] [32msys_connect => Err(ECONNREFUSED)[m
[miperf3: error - unable to connect to server - server may have stopped running or use a different port, firewall issue, etc.: Connection refused
====== iperf REVERSE_TCP end: fail ======

#### OS COMP TEST GROUP END iperf-glibc ####
autorun: skip disabled test group /musl/lmbench_testcode.sh
autorun: skip disabled test group /glibc/lmbench_testcode.sh
#### OS COMP TEST GROUP START netperf-musl ####
====== netperf UDP_STREAM begin ======
Starting netserver with host '127.0.0.1' port '12865' and family AF_UNSPEC
MIGRATED UDP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Socket  Message  Elapsed      Messages
Size    Size     Time         Okay Errors   Throughput
bytes   bytes    secs            #      #   10^6bits/sec

 65536    1000   1.00         1440      0      11.46
 65536           1.00         1440             11.46

====== netperf UDP_STREAM end: success ======
====== netperf TCP_STREAM begin ======
MIGRATED TCP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Recv   Send    Send
Socket Socket  Message  Elapsed
Size   Size    Size     Time     Throughput
bytes  bytes   bytes    secs.    10^6bits/sec

 65536  65536   1000    0.05      154.12
====== netperf TCP_STREAM end: success ======
====== netperf UDP_RR begin ======
MIGRATED UDP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate
bytes  Bytes  bytes    bytes   secs.    per sec

65536  65536  64       64      1.00     1174.55
65536  65536
====== netperf UDP_RR end: success ======
====== netperf TCP_RR begin ======
MIGRATED TCP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate
bytes  Bytes  bytes    bytes   secs.    per sec

65536  65536  64       64      1.01     1310.11
65536  65536
====== netperf TCP_RR end: success ======
====== netperf TCP_CRR begin ======
MIGRATED TCP Connect/Request/Response TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 (127.0.0) port 0 AF_INET
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate
bytes  Bytes  bytes    bytes   secs.    per sec

65536  65536  64       64      1.00      819.08
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

 65536    1000   1.01         1505      0      11.97
 65536           1.01         1505             11.97

====== netperf UDP_STREAM end: success ======
====== netperf TCP_STREAM begin ======
MIGRATED TCP STREAM TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET
Recv   Send    Send
Socket Socket  Message  Elapsed
Size   Size    Size     Time     Throughput
bytes  bytes   bytes    secs.    10^6bits/sec

 65536  65536   1000    0.05      164.00
====== netperf TCP_STREAM end: success ======
====== netperf UDP_RR begin ======
MIGRATED UDP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate
bytes  Bytes  bytes    bytes   secs.    per sec

65536  65536  64       64      1.01     1123.22
65536  65536
====== netperf UDP_RR end: success ======
====== netperf TCP_RR begin ======
MIGRATED TCP REQUEST/RESPONSE TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET : first burst 0
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate
bytes  Bytes  bytes    bytes   secs.    per sec

65536  65536  64       64      1.00     1373.31
65536  65536
====== netperf TCP_RR end: success ======
====== netperf TCP_CRR begin ======
MIGRATED TCP Connect/Request/Response TEST from 0.0.0.0 (0.0.0.0) port 0 AF_INET to 127.0.0.1 () port 0 AF_INET
Local /Remote
Socket Size   Request  Resp.   Elapsed  Trans.
Send   Recv   Size     Size    Time     Rate
bytes  Bytes  bytes    bytes   secs.    per sec

65536  65536  64       64      1.00      854.35
65536  65536
====== netperf TCP_CRR end: success ======
#### OS COMP TEST GROUP END netperf-glibc ####
autorun: skip disabled test group /musl/cyclictest_testcode.sh
autorun: skip disabled test group /glibc/cyclictest_testcode.sh
autorun: skip disabled test group /musl/iozone_testcode.sh
autorun: skip disabled test group /glibc/iozone_testcode.sh
autorun: skip disabled test group /musl/unixbench_testcode.sh
autorun: skip disabled test group /glibc/unixbench_testcode.sh
[37m[650.663293 0:2 axplat_loongarch64_qemu_virt::power:23] [32mShutting down...[m
[m
