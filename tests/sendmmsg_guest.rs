#![no_main]
#![no_std]

use core::ffi::c_void;
use core::sync::atomic::{AtomicUsize, Ordering};

const SYS_CLOSE: usize = 57;
const SYS_OPENAT: usize = 56;
const SYS_PIPE2: usize = 59;
const SYS_WRITE: usize = 64;
const SYS_EXIT_GROUP: usize = 94;
const SYS_SOCKET: usize = 198;
const SYS_SOCKETPAIR: usize = 199;
const SYS_BIND: usize = 200;
const SYS_CONNECT: usize = 203;
const SYS_GETSOCKNAME: usize = 204;
const SYS_SENDMSG: usize = 211;
const SYS_RECVMSG: usize = 212;
const SYS_MUNMAP: usize = 215;
const SYS_MMAP: usize = 222;
const SYS_MPROTECT: usize = 226;
const SYS_SENDMMSG: usize = 269;

const AF_UNIX: usize = 1;
const AF_INET: usize = 2;
const SOCK_DGRAM: usize = 2;
const SOCK_NONBLOCK: usize = 0x800;
const PROT_NONE: usize = 0;
const PROT_READ: usize = 1;
const PROT_WRITE: usize = 2;
const MAP_PRIVATE: usize = 2;
const MAP_ANONYMOUS: usize = 0x20;
const AT_FDCWD: usize = (-100isize) as usize;
const O_WRONLY: usize = 1;

const EBADF: isize = 9;
const EFAULT: isize = 14;
const ENOTSOCK: isize = 88;
const EOPNOTSUPP: isize = 95;
const PAGE_SIZE: usize = 4096;
const SENTINEL: u32 = 0xa5a5_a5a5;

static FAILURES: AtomicUsize = AtomicUsize::new(0);

#[repr(C)]
#[derive(Clone, Copy)]
struct Iovec {
    base: *mut c_void,
    len: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Msghdr {
    name: *mut c_void,
    name_len: u32,
    iov: *mut Iovec,
    iov_len: usize,
    control: *mut c_void,
    control_len: usize,
    flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Mmsghdr {
    msg: Msghdr,
    msg_len: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SockaddrIn {
    family: u16,
    port: u16,
    addr: u32,
    zero: [u8; 8],
}

const EMPTY_IOV: Iovec = Iovec {
    base: core::ptr::null_mut(),
    len: 0,
};
const EMPTY_MSGHDR: Msghdr = Msghdr {
    name: core::ptr::null_mut(),
    name_len: 0,
    iov: core::ptr::null_mut(),
    iov_len: 0,
    control: core::ptr::null_mut(),
    control_len: 0,
    flags: 0,
};
const EMPTY_MMSGHDR: Mmsghdr = Mmsghdr {
    msg: EMPTY_MSGHDR,
    msg_len: SENTINEL,
};

const _: () = {
    assert!(core::mem::size_of::<Msghdr>() == 56);
    assert!(core::mem::align_of::<Msghdr>() == 8);
    assert!(core::mem::offset_of!(Msghdr, iov) == 16);
    assert!(core::mem::size_of::<Mmsghdr>() == 64);
    assert!(core::mem::align_of::<Mmsghdr>() == 8);
    assert!(core::mem::offset_of!(Mmsghdr, msg_len) == 56);
};

#[cfg(target_arch = "riscv64")]
#[inline(always)]
fn syscall(number: usize, args: [usize; 6]) -> isize {
    let ret: isize;
    // SAFETY: this is the RISC-V64 Linux syscall ABI; all pointer validity is
    // intentionally decided by the kernel under test.
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") args[0] => ret,
            in("a1") args[1],
            in("a2") args[2],
            in("a3") args[3],
            in("a4") args[4],
            in("a5") args[5],
            in("a7") number,
            options(nostack)
        );
    }
    ret
}

#[cfg(target_arch = "loongarch64")]
#[inline(always)]
fn syscall(number: usize, args: [usize; 6]) -> isize {
    let ret: isize;
    // SAFETY: this is the LoongArch64 Linux syscall ABI; all pointer validity
    // is intentionally decided by the kernel under test.
    unsafe {
        core::arch::asm!(
            "syscall 0",
            inlateout("$a0") args[0] => ret,
            in("$a1") args[1],
            in("$a2") args[2],
            in("$a3") args[3],
            in("$a4") args[4],
            in("$a5") args[5],
            in("$a7") number,
            options(nostack)
        );
    }
    ret
}

fn write_text(text: &[u8]) {
    let _ = syscall(SYS_WRITE, [1, text.as_ptr() as usize, text.len(), 0, 0, 0]);
}

fn check(condition: bool, name: &[u8]) {
    if condition {
        return;
    }
    FAILURES.fetch_add(1, Ordering::Relaxed);
    write_text(b"FAIL: ");
    write_text(name);
    write_text(b"\n");
}

fn exit(code: usize) -> ! {
    let _ = syscall(SYS_EXIT_GROUP, [code, 0, 0, 0, 0, 0]);
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    write_text(b"FAIL: panic\n");
    exit(101)
}

fn close(fd: isize) {
    if fd >= 0 {
        let _ = syscall(SYS_CLOSE, [fd as usize, 0, 0, 0, 0, 0]);
    }
}

fn socketpair(kind: usize) -> Result<[i32; 2], isize> {
    let mut fds = [-1i32; 2];
    let ret = syscall(
        SYS_SOCKETPAIR,
        [AF_UNIX, kind, 0, fds.as_mut_ptr() as usize, 0, 0],
    );
    if ret < 0 { Err(ret) } else { Ok(fds) }
}

fn make_message(iov: &mut Iovec, data: *mut u8, len: usize) -> Mmsghdr {
    iov.base = data.cast();
    iov.len = len;
    Mmsghdr {
        msg: Msghdr {
            iov: iov as *mut Iovec,
            iov_len: 1,
            ..EMPTY_MSGHDR
        },
        msg_len: SENTINEL,
    }
}

fn sendmmsg(fd: i32, msgs: *mut Mmsghdr, count: usize, flags: usize) -> isize {
    syscall(
        SYS_SENDMMSG,
        [fd as usize, msgs as usize, count, flags, 0, 0],
    )
}

fn recv_byte(fd: i32, byte: &mut u8) -> isize {
    let mut iov = Iovec {
        base: (byte as *mut u8).cast(),
        len: 1,
    };
    let mut msg = Msghdr {
        iov: &mut iov,
        iov_len: 1,
        ..EMPTY_MSGHDR
    };
    syscall(
        SYS_RECVMSG,
        [fd as usize, &mut msg as *mut Msghdr as usize, 0, 0, 0, 0],
    )
}

fn test_sendmsg_regression() {
    let Ok(fds) = socketpair(SOCK_DGRAM) else {
        check(false, b"sendmsg socketpair");
        return;
    };
    let mut data = b's';
    let mut iov = EMPTY_IOV;
    let msg = make_message(&mut iov, &mut data, 1).msg;
    let ret = syscall(
        SYS_SENDMSG,
        [fds[0] as usize, &msg as *const Msghdr as usize, 0, 0, 0, 0],
    );
    let mut received = 0;
    check(ret == 1, b"sendmsg return");
    check(
        recv_byte(fds[1], &mut received) == 1 && received == data,
        b"sendmsg payload",
    );
    close(fds[0] as isize);
    close(fds[1] as isize);
}

fn test_vlen_zero_and_errors() {
    let Ok(fds) = socketpair(SOCK_DGRAM) else {
        check(false, b"zero socketpair");
        return;
    };
    check(
        sendmmsg(fds[0], 1usize as *mut Mmsghdr, 0, usize::MAX) == 0,
        b"vlen zero",
    );
    check(
        sendmmsg(-1, 1usize as *mut Mmsghdr, 0, 0) == -EBADF,
        b"vlen zero bad fd",
    );
    check(
        sendmmsg(fds[0], 1usize as *mut Mmsghdr, 1, 0) == -EFAULT,
        b"invalid msgvec",
    );
    let overflow = usize::MAX - core::mem::size_of::<Mmsghdr>() + 2;
    check(
        sendmmsg(fds[0], overflow as *mut Mmsghdr, 2, 0) == -EFAULT,
        b"msgvec overflow",
    );
    close(fds[0] as isize);
    close(fds[1] as isize);
}

fn test_unix_success() {
    let Ok(fds) = socketpair(SOCK_DGRAM) else {
        check(false, b"unix socketpair");
        return;
    };
    let mut data = [b'x', b'y', b'z'];
    let mut iovs = [EMPTY_IOV; 3];
    let mut msgs = [EMPTY_MMSGHDR; 3];
    for index in 0..3 {
        msgs[index] = make_message(&mut iovs[index], &mut data[index], 1);
    }
    check(
        sendmmsg(fds[0], msgs.as_mut_ptr(), 3, 0) == 3,
        b"unix count",
    );
    for index in 0..3 {
        let mut received = 0;
        check(msgs[index].msg_len == 1, b"unix msg_len");
        check(
            recv_byte(fds[1], &mut received) == 1 && received == data[index],
            b"unix payload",
        );
    }
    close(fds[0] as isize);
    close(fds[1] as isize);
}

fn udp_pair() -> Result<[i32; 2], isize> {
    let receiver = syscall(SYS_SOCKET, [AF_INET, SOCK_DGRAM, 0, 0, 0, 0]);
    let sender = syscall(SYS_SOCKET, [AF_INET, SOCK_DGRAM, 0, 0, 0, 0]);
    if receiver < 0 || sender < 0 {
        close(receiver);
        close(sender);
        return Err(if receiver < 0 { receiver } else { sender });
    }
    let mut addr = SockaddrIn {
        family: AF_INET as u16,
        port: 0,
        addr: u32::from_ne_bytes([127, 0, 0, 1]),
        zero: [0; 8],
    };
    let mut addr_len = core::mem::size_of::<SockaddrIn>() as u32;
    let bind = syscall(
        SYS_BIND,
        [
            receiver as usize,
            &addr as *const SockaddrIn as usize,
            addr_len as usize,
            0,
            0,
            0,
        ],
    );
    let name = syscall(
        SYS_GETSOCKNAME,
        [
            receiver as usize,
            &mut addr as *mut SockaddrIn as usize,
            &mut addr_len as *mut u32 as usize,
            0,
            0,
            0,
        ],
    );
    let connect = syscall(
        SYS_CONNECT,
        [
            sender as usize,
            &addr as *const SockaddrIn as usize,
            addr_len as usize,
            0,
            0,
            0,
        ],
    );
    if bind < 0 || name < 0 || connect < 0 {
        close(receiver);
        close(sender);
        return Err(bind.min(name).min(connect));
    }
    Ok([sender as i32, receiver as i32])
}

fn test_udp_success() {
    let Ok(fds) = udp_pair() else {
        check(false, b"udp setup");
        return;
    };
    let mut data = [b'a', b'b', b'c'];
    let mut iovs = [EMPTY_IOV; 3];
    let mut msgs = [EMPTY_MMSGHDR; 3];
    for index in 0..3 {
        msgs[index] = make_message(&mut iovs[index], &mut data[index], 1);
    }
    check(sendmmsg(fds[0], msgs.as_mut_ptr(), 3, 0) == 3, b"udp count");
    for index in 0..3 {
        let mut received = 0;
        check(msgs[index].msg_len == 1, b"udp msg_len");
        check(
            recv_byte(fds[1], &mut received) == 1 && received == data[index],
            b"udp payload",
        );
    }
    close(fds[0] as isize);
    close(fds[1] as isize);
}

fn test_partial_failure(failing: usize) {
    let Ok(fds) = socketpair(SOCK_DGRAM) else {
        check(false, b"partial socketpair");
        return;
    };
    let mut data = [b'p', b'q', b'r'];
    let mut iovs = [EMPTY_IOV; 3];
    let mut msgs = [EMPTY_MMSGHDR; 3];
    for index in 0..3 {
        msgs[index] = make_message(&mut iovs[index], &mut data[index], 1);
    }
    iovs[failing].base = 1usize as *mut c_void;
    check(
        sendmmsg(fds[0], msgs.as_mut_ptr(), 3, 0) == failing as isize,
        b"partial count",
    );
    for index in 0..failing {
        let mut received = 0;
        check(msgs[index].msg_len == 1, b"partial completed msg_len");
        check(
            recv_byte(fds[1], &mut received) == 1 && received == data[index],
            b"partial payload",
        );
    }
    for msg in &msgs[failing..] {
        check(msg.msg_len == SENTINEL, b"partial untouched msg_len");
    }
    close(fds[0] as isize);
    close(fds[1] as isize);
}

fn test_first_failure_and_flags() {
    let Ok(fds) = socketpair(SOCK_DGRAM) else {
        check(false, b"error socketpair");
        return;
    };
    let mut data = b'e';
    let mut iov = Iovec {
        base: 1usize as *mut c_void,
        len: 1,
    };
    let mut msg = make_message(&mut iov, 1usize as *mut u8, 1);
    check(
        sendmmsg(fds[0], &mut msg, 1, 0) == -EFAULT,
        b"first failure errno",
    );
    check(msg.msg_len == SENTINEL, b"first failure msg_len");
    msg = make_message(&mut iov, &mut data, 1);
    check(
        sendmmsg(fds[0], &mut msg, 1, usize::MAX) == -EOPNOTSUPP,
        b"invalid flags errno",
    );
    check(msg.msg_len == SENTINEL, b"invalid flags msg_len");
    close(fds[0] as isize);
    close(fds[1] as isize);
}

fn test_regular_fd() {
    let path = b"/dev/null\0";
    let file = syscall(
        SYS_OPENAT,
        [AT_FDCWD, path.as_ptr() as usize, O_WRONLY, 0, 0, 0],
    );
    if file < 0 {
        check(false, b"ordinary file setup");
    } else {
        let mut data = b'f';
        let mut iov = EMPTY_IOV;
        let mut msg = make_message(&mut iov, &mut data, 1);
        check(
            sendmmsg(file as i32, &mut msg, 1, 0) == -ENOTSOCK,
            b"ordinary file errno",
        );
        check(msg.msg_len == SENTINEL, b"ordinary file msg_len");
        close(file);
    }

    let mut pipe_fds = [-1i32; 2];
    let pipe_ret = syscall(SYS_PIPE2, [pipe_fds.as_mut_ptr() as usize, 0, 0, 0, 0, 0]);
    if pipe_ret < 0 {
        check(false, b"pipe setup");
        return;
    }
    let mut data = b'f';
    let mut iov = EMPTY_IOV;
    let mut msg = make_message(&mut iov, &mut data, 1);
    check(
        sendmmsg(pipe_fds[1], &mut msg, 1, 0) == -ENOTSOCK,
        b"non-socket fd errno",
    );
    check(msg.msg_len == SENTINEL, b"non-socket fd msg_len");
    close(pipe_fds[0] as isize);
    close(pipe_fds[1] as isize);
}

fn test_vlen_limit() {
    const COUNT: usize = 1025;
    let Ok(fds) = socketpair(SOCK_DGRAM) else {
        check(false, b"limit socketpair");
        return;
    };
    let mut iovs = [EMPTY_IOV; COUNT];
    let mut msgs = [EMPTY_MMSGHDR; COUNT];
    let mut byte = 0u8;
    for index in 0..COUNT {
        msgs[index] = make_message(&mut iovs[index], &mut byte, 0);
    }
    check(
        sendmmsg(fds[0], msgs.as_mut_ptr(), COUNT, 0) == 1024,
        b"vlen cap count",
    );
    check(msgs[1023].msg_len == 0, b"vlen cap last allowed");
    check(msgs[1024].msg_len == SENTINEL, b"vlen cap untouched");
    close(fds[0] as isize);
    close(fds[1] as isize);
}

fn test_nonblocking_partial() {
    const COUNT: usize = 70;
    const SIZE: usize = 1024;
    let Ok(fds) = socketpair(SOCK_DGRAM | SOCK_NONBLOCK) else {
        check(false, b"nonblock socketpair");
        return;
    };
    let mut data = [0u8; SIZE];
    let mut iovs = [EMPTY_IOV; COUNT];
    let mut msgs = [EMPTY_MMSGHDR; COUNT];
    for index in 0..COUNT {
        msgs[index] = make_message(&mut iovs[index], data.as_mut_ptr(), SIZE);
    }
    let sent = sendmmsg(fds[0], msgs.as_mut_ptr(), COUNT, 0);
    check(sent > 0 && sent < COUNT as isize, b"nonblock partial count");
    if sent > 0 && sent < COUNT as isize {
        for msg in &msgs[..sent as usize] {
            check(msg.msg_len == SIZE as u32, b"nonblock completed msg_len");
        }
        check(
            msgs[sent as usize].msg_len == SENTINEL,
            b"nonblock untouched msg_len",
        );
    }
    close(fds[0] as isize);
    close(fds[1] as isize);
}

fn map_pages(length: usize) -> Result<usize, isize> {
    let ret = syscall(
        SYS_MMAP,
        [
            0,
            length,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            usize::MAX,
            0,
        ],
    );
    if ret < 0 { Err(ret) } else { Ok(ret as usize) }
}

fn test_crossing_invalid_page() {
    let Ok(fds) = socketpair(SOCK_DGRAM) else {
        check(false, b"cross-page socketpair");
        return;
    };
    let Ok(mapping) = map_pages(PAGE_SIZE * 2) else {
        check(false, b"cross-page mmap");
        close(fds[0] as isize);
        close(fds[1] as isize);
        return;
    };
    let mut data = b'v';
    let mut iov = EMPTY_IOV;
    let msg = make_message(&mut iov, &mut data, 1);
    let msgs = (mapping + PAGE_SIZE - core::mem::size_of::<Mmsghdr>()) as *mut Mmsghdr;
    // SAFETY: the first mapped page is writable and `msgs` is aligned within it.
    unsafe { core::ptr::write(msgs, msg) };
    let protect = syscall(
        SYS_MPROTECT,
        [mapping + PAGE_SIZE, PAGE_SIZE, PROT_NONE, 0, 0, 0],
    );
    check(protect == 0, b"cross-page mprotect");
    check(
        sendmmsg(fds[0], msgs, 2, 0) == 1,
        b"cross-page partial count",
    );
    // SAFETY: the first page remains readable after protecting only the second page.
    let first = unsafe { core::ptr::read(msgs) };
    check(first.msg_len == 1, b"cross-page msg_len");
    let _ = syscall(
        SYS_MPROTECT,
        [
            mapping + PAGE_SIZE,
            PAGE_SIZE,
            PROT_READ | PROT_WRITE,
            0,
            0,
            0,
        ],
    );
    let _ = syscall(SYS_MUNMAP, [mapping, PAGE_SIZE * 2, 0, 0, 0, 0]);
    close(fds[0] as isize);
    close(fds[1] as isize);
}

fn test_copyout_failure() {
    let Ok(fds) = socketpair(SOCK_DGRAM) else {
        check(false, b"copyout socketpair");
        return;
    };
    let Ok(mapping) = map_pages(PAGE_SIZE) else {
        check(false, b"copyout mmap");
        close(fds[0] as isize);
        close(fds[1] as isize);
        return;
    };
    let mut data = b'w';
    let mut iov = EMPTY_IOV;
    let msg = make_message(&mut iov, &mut data, 1);
    let msgs = mapping as *mut Mmsghdr;
    // SAFETY: the anonymous mapping is writable and aligned for `Mmsghdr`.
    unsafe { core::ptr::write(msgs, msg) };
    check(
        syscall(SYS_MPROTECT, [mapping, PAGE_SIZE, PROT_READ, 0, 0, 0]) == 0,
        b"copyout mprotect",
    );
    check(sendmmsg(fds[0], msgs, 1, 0) == -EFAULT, b"copyout errno");
    // SAFETY: the mapping remains readable.
    let unchanged = unsafe { core::ptr::read(msgs) };
    check(unchanged.msg_len == SENTINEL, b"copyout msg_len");
    let mut received = 0;
    check(
        recv_byte(fds[1], &mut received) == 1 && received == data,
        b"copyout delivered payload",
    );
    let _ = syscall(
        SYS_MPROTECT,
        [mapping, PAGE_SIZE, PROT_READ | PROT_WRITE, 0, 0, 0],
    );
    let _ = syscall(SYS_MUNMAP, [mapping, PAGE_SIZE, 0, 0, 0, 0]);
    close(fds[0] as isize);
    close(fds[1] as isize);
}

fn test_partial_copyout_failure() {
    let Ok(fds) = socketpair(SOCK_DGRAM) else {
        check(false, b"partial copyout socketpair");
        return;
    };
    let Ok(mapping) = map_pages(PAGE_SIZE * 2) else {
        check(false, b"partial copyout mmap");
        close(fds[0] as isize);
        close(fds[1] as isize);
        return;
    };
    let mut data = [b'1', b'2'];
    let mut iovs = [EMPTY_IOV; 2];
    let msgs = (mapping + PAGE_SIZE - core::mem::size_of::<Mmsghdr>()) as *mut Mmsghdr;
    let first = make_message(&mut iovs[0], &mut data[0], 1);
    let second = make_message(&mut iovs[1], &mut data[1], 1);
    // SAFETY: both anonymous pages are writable and `msgs` is aligned; the two
    // writes occupy exactly the final record of page one and the first of page two.
    unsafe {
        core::ptr::write(msgs, first);
        core::ptr::write(msgs.add(1), second);
    }
    check(
        syscall(
            SYS_MPROTECT,
            [mapping + PAGE_SIZE, PAGE_SIZE, PROT_READ, 0, 0, 0],
        ) == 0,
        b"partial copyout mprotect",
    );
    check(sendmmsg(fds[0], msgs, 2, 0) == 1, b"partial copyout count");
    // SAFETY: both pages remain readable.
    let first = unsafe { core::ptr::read(msgs) };
    // SAFETY: the second page is read-only, not inaccessible.
    let second = unsafe { core::ptr::read(msgs.add(1)) };
    check(first.msg_len == 1, b"partial copyout completed msg_len");
    check(
        second.msg_len == SENTINEL,
        b"partial copyout untouched msg_len",
    );
    for expected in data {
        let mut received = 0;
        check(
            recv_byte(fds[1], &mut received) == 1 && received == expected,
            b"partial copyout delivered payload",
        );
    }
    let _ = syscall(
        SYS_MPROTECT,
        [
            mapping + PAGE_SIZE,
            PAGE_SIZE,
            PROT_READ | PROT_WRITE,
            0,
            0,
            0,
        ],
    );
    let _ = syscall(SYS_MUNMAP, [mapping, PAGE_SIZE * 2, 0, 0, 0, 0]);
    close(fds[0] as isize);
    close(fds[1] as isize);
}

fn run() {
    test_sendmsg_regression();
    test_vlen_zero_and_errors();
    test_unix_success();
    test_udp_success();
    test_first_failure_and_flags();
    test_partial_failure(1);
    test_partial_failure(2);
    test_regular_fd();
    test_vlen_limit();
    test_nonblocking_partial();
    test_crossing_invalid_page();
    test_copyout_failure();
    test_partial_copyout_failure();
}

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    run();
    let failures = FAILURES.load(Ordering::Relaxed);
    if failures == 0 {
        write_text(b"sendmmsg guest: PASS\n");
        exit(0)
    } else {
        write_text(b"sendmmsg guest: FAIL\n");
        exit(1)
    }
}
