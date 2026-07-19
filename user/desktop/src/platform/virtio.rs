use core::ptr::NonNull;

use axalloc::global_allocator;
use axhal::mem::{phys_to_virt, virt_to_phys};
use virtio_drivers::{BufferDirection, Hal, PhysAddr};

pub(super) struct DesktopVirtIoHal;

const DMA_PAGE_SIZE: usize = 0x1000;

// SAFETY: DMA allocations are page-aligned, exclusive kernel page allocations
// and are zeroed in full before being returned. RV64 and LA64 QEMU virt use the
// platform's coherent direct physical mapping without an IOMMU, so `share` and
// `mmio_phys_to_virt` translate through the same axhal mapping and `unshare` is
// a no-op. Callers of the unsafe methods must provide the unchanged allocation
// tuple or a valid device BAR as required by `Hal`; the implementations do not
// broaden those caller obligations. The two mappings and DMA-backed GPU/input
// queues are exercised by the RV64 and LA64 headless runtime suites.
unsafe impl Hal for DesktopVirtIoHal {
    fn dma_alloc(pages: usize, _direction: BufferDirection) -> (PhysAddr, NonNull<u8>) {
        let Some(byte_len) = pages.checked_mul(DMA_PAGE_SIZE) else {
            return (0, NonNull::dangling());
        };
        if byte_len == 0 {
            return (0, NonNull::dangling());
        }
        let Ok(vaddr) = global_allocator().alloc_pages(pages, DMA_PAGE_SIZE) else {
            return (0, NonNull::dangling());
        };
        let Some(vaddr_ptr) = NonNull::new(vaddr as *mut u8) else {
            global_allocator().dealloc_pages(vaddr, pages);
            return (0, NonNull::dangling());
        };
        // SAFETY: `alloc_pages` returned `pages` exclusive contiguous pages at
        // `vaddr`; `byte_len` is the checked size of exactly that allocation.
        unsafe { core::ptr::write_bytes(vaddr_ptr.as_ptr(), 0, byte_len) };
        let paddr = virt_to_phys(vaddr.into());
        (paddr.as_usize() as PhysAddr, vaddr_ptr)
    }

    unsafe fn dma_dealloc(_paddr: PhysAddr, vaddr: NonNull<u8>, pages: usize) -> i32 {
        // SAFETY: `vaddr` and `pages` are the unchanged pair returned by
        // `dma_alloc` to virtio-drivers for this allocation.
        global_allocator().dealloc_pages(vaddr.as_ptr() as usize, pages);
        0
    }

    unsafe fn mmio_phys_to_virt(paddr: PhysAddr, _size: usize) -> NonNull<u8> {
        NonNull::new(phys_to_virt((paddr as usize).into()).as_mut_ptr()).unwrap()
    }

    unsafe fn share(buffer: NonNull<[u8]>, _direction: BufferDirection) -> PhysAddr {
        let vaddr = buffer.as_ptr() as *mut u8 as usize;
        virt_to_phys(vaddr.into()).as_usize() as PhysAddr
    }

    unsafe fn unshare(_paddr: PhysAddr, _buffer: NonNull<[u8]>, _direction: BufferDirection) {}
}
