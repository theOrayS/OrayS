//! [ArceOS](https://github.com/arceos-org/arceos) global memory allocator.
//!
//! It provides [`GlobalAllocator`], which implements the trait
//! [`core::alloc::GlobalAlloc`]. A static global variable of type
//! [`GlobalAllocator`] is defined with the `#[global_allocator]` attribute, to
//! be registered as the standard library’s default allocator.

#![no_std]

#[macro_use]
extern crate log;
extern crate alloc;

mod page;

use axallocator::{AllocResult, BaseAllocator, BitmapPageAllocator, ByteAllocator, PageAllocator};
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};
use kspin::SpinNoIrq;

const PAGE_SIZE: usize = 0x1000;
const MIN_HEAP_SIZE: usize = 0x8000; // 32 K
const MAX_SMALL_HEAP_EXPAND_SIZE: usize = 0x400000; // 4 MiB
// Keep medium, short-lived buffers (task stacks, page-table side buffers,
// executable/load scratch vectors in long LTP runs) page-backed so
// deallocation returns frames to the page allocator instead of permanently
// growing the byte heap.  A 4 KiB threshold is intentionally page-sized: the
// page allocator already rounds these layouts up, while smaller metadata stays
// in TLSF to avoid pathological one-page-per-node waste.
const LARGE_DIRECT_ALLOC_THRESHOLD: usize = PAGE_SIZE;

pub use page::GlobalPage;

/// A snapshot of the frame allocator state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameAllocatorStats {
    /// Frames currently available for allocation.
    pub free_frames: usize,
    /// Frames currently allocated from the frame allocator.
    pub allocated_frames: usize,
}

/// A coarse snapshot of currently live global-allocation requests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AllocationBucketStats {
    /// Inclusive upper bound for request sizes covered by this bucket.
    pub max_size: usize,
    /// Number of live allocations in this bucket.
    pub active_count: usize,
    /// Sum of requested bytes for live allocations in this bucket.
    pub active_bytes: usize,
    /// Number of live allocations in this bucket that bypass the byte heap and
    /// are backed directly by pages.
    pub direct_count: usize,
    /// Sum of requested bytes for live direct-page allocations in this bucket.
    pub direct_bytes: usize,
}

const ALLOCATION_BUCKET_LIMITS: [usize; 14] = [
    8,
    16,
    32,
    64,
    128,
    256,
    512,
    1024,
    2048,
    4096,
    8192,
    16384,
    65536,
    usize::MAX,
];

static ALLOCATION_BUCKET_COUNTS: [AtomicUsize; 14] = [
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
];

static ALLOCATION_BUCKET_BYTES: [AtomicUsize; 14] = [
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
];

static ALLOCATION_BUCKET_DIRECT_COUNTS: [AtomicUsize; 14] = [
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
];

static ALLOCATION_BUCKET_DIRECT_BYTES: [AtomicUsize; 14] = [
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
];

cfg_if::cfg_if! {
    if #[cfg(feature = "slab")] {
        /// The default byte allocator.
        pub type DefaultByteAllocator = axallocator::SlabByteAllocator;
    } else if #[cfg(feature = "buddy")] {
        /// The default byte allocator.
        pub type DefaultByteAllocator = axallocator::BuddyByteAllocator;
    } else if #[cfg(feature = "tlsf")] {
        /// The default byte allocator.
        pub type DefaultByteAllocator = axallocator::TlsfByteAllocator;
    }
}

/// The global allocator used by ArceOS.
///
/// It combines a [`ByteAllocator`] and a [`PageAllocator`] into a simple
/// two-level allocator: firstly tries allocate from the byte allocator, if
/// there is no memory, asks the page allocator for more memory and adds it to
/// the byte allocator.
///
/// Currently, [`TlsfByteAllocator`] is used as the byte allocator, while
/// [`BitmapPageAllocator`] is used as the page allocator.
///
/// [`TlsfByteAllocator`]: axallocator::TlsfByteAllocator
pub struct GlobalAllocator {
    balloc: SpinNoIrq<DefaultByteAllocator>,
    palloc: SpinNoIrq<BitmapPageAllocator<PAGE_SIZE>>,
}

impl GlobalAllocator {
    /// Creates an empty [`GlobalAllocator`].
    pub const fn new() -> Self {
        Self {
            balloc: SpinNoIrq::new(DefaultByteAllocator::new()),
            palloc: SpinNoIrq::new(BitmapPageAllocator::new()),
        }
    }

    /// Returns the name of the allocator.
    pub const fn name(&self) -> &'static str {
        cfg_if::cfg_if! {
            if #[cfg(feature = "slab")] {
                "slab"
            } else if #[cfg(feature = "buddy")] {
                "buddy"
            } else if #[cfg(feature = "tlsf")] {
                "TLSF"
            }
        }
    }

    /// Initializes the allocator with the given region.
    ///
    /// It firstly adds the whole region to the page allocator, then allocates
    /// a small region (32 KB) to initialize the byte allocator. Therefore,
    /// the given region must be larger than 32 KB.
    pub fn init(&self, start_vaddr: usize, size: usize) {
        assert!(size > MIN_HEAP_SIZE);
        let init_heap_size = MIN_HEAP_SIZE;
        self.palloc.lock().init(start_vaddr, size);
        let heap_ptr = self
            .alloc_pages(init_heap_size / PAGE_SIZE, PAGE_SIZE)
            .unwrap();
        self.balloc.lock().init(heap_ptr, init_heap_size);
    }

    /// Add the given region to the allocator.
    ///
    /// It will add the whole region to the byte allocator.
    pub fn add_memory(&self, start_vaddr: usize, size: usize) -> AllocResult {
        self.balloc.lock().add_memory(start_vaddr, size)
    }

    /// Allocate arbitrary number of bytes. Returns the left bound of the
    /// allocated region.
    ///
    /// It firstly tries to allocate from the byte allocator. If there is no
    /// memory, it asks the page allocator for more memory and adds it to the
    /// byte allocator.
    pub fn alloc(&self, layout: Layout) -> AllocResult<NonNull<u8>> {
        if should_direct_page_alloc(layout) {
            let pages = pages_for_layout(layout);
            let align = layout.align().max(PAGE_SIZE);
            return self.alloc_pages(pages, align).map(|ptr| {
                record_allocation(layout, true);
                // SAFETY: the page allocator never returns null on success.
                unsafe { NonNull::new_unchecked(ptr as *mut u8) }
            });
        }

        // simple two-level allocator: if no heap memory, allocate from the page allocator.
        let mut balloc = self.balloc.lock();
        loop {
            if let Ok(ptr) = balloc.alloc(layout) {
                record_allocation(layout, false);
                return Ok(ptr);
            } else {
                let request_size = layout.size().saturating_add(layout.align()).max(PAGE_SIZE);
                let min_expand = align_up_to_page(request_size);
                let desired_expand = balloc
                    .total_bytes()
                    .max(request_size)
                    .checked_next_power_of_two()
                    .unwrap_or(request_size);
                let mut expand_size = if request_size <= MAX_SMALL_HEAP_EXPAND_SIZE {
                    desired_expand.min(MAX_SMALL_HEAP_EXPAND_SIZE)
                } else {
                    desired_expand
                };
                expand_size = align_up_to_page(expand_size).max(min_expand);

                loop {
                    let pages = expand_size / PAGE_SIZE;
                    match self.alloc_pages(pages, PAGE_SIZE) {
                        Ok(heap_ptr) => match balloc.add_memory(heap_ptr, expand_size) {
                            Ok(()) => {
                                debug!(
                                    "expand heap memory: [{:#x}, {:#x})",
                                    heap_ptr,
                                    heap_ptr + expand_size
                                );
                                break;
                            }
                            Err(err) => {
                                self.dealloc_pages(heap_ptr, pages);
                                if expand_size <= min_expand {
                                    return Err(err);
                                }
                            }
                        },
                        Err(err) => {
                            if expand_size <= min_expand {
                                return Err(err);
                            }
                        }
                    }
                    expand_size = align_up_to_page((expand_size / 2).max(min_expand));
                }
            }
        }
    }

    /// Gives back the allocated region to the byte allocator.
    ///
    /// The region should be allocated by [`alloc`], and `align_pow2` should be
    /// the same as the one used in [`alloc`]. Otherwise, the behavior is
    /// undefined.
    ///
    /// [`alloc`]: GlobalAllocator::alloc
    pub fn dealloc(&self, pos: NonNull<u8>, layout: Layout) {
        if should_direct_page_alloc(layout) {
            record_deallocation(layout, true);
            self.dealloc_pages(pos.as_ptr() as usize, pages_for_layout(layout));
        } else {
            record_deallocation(layout, false);
            self.balloc.lock().dealloc(pos, layout)
        }
    }

    /// Allocates contiguous pages.
    ///
    /// It allocates `num_pages` pages from the page allocator.
    ///
    /// `align_pow2` must be a power of 2, and the returned region bound will be
    /// aligned to it.
    pub fn alloc_pages(&self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        self.palloc.lock().alloc_pages(num_pages, align_pow2)
    }

    /// Allocates contiguous pages starting from the given address.
    ///
    /// It allocates `num_pages` pages from the page allocator starting from the
    /// given address.
    ///
    /// `align_pow2` must be a power of 2, and the returned region bound will be
    /// aligned to it.
    pub fn alloc_pages_at(
        &self,
        start: usize,
        num_pages: usize,
        align_pow2: usize,
    ) -> AllocResult<usize> {
        self.palloc
            .lock()
            .alloc_pages_at(start, num_pages, align_pow2)
    }

    /// Gives back the allocated pages starts from `pos` to the page allocator.
    ///
    /// The pages should be allocated by [`alloc_pages`], and `align_pow2`
    /// should be the same as the one used in [`alloc_pages`]. Otherwise, the
    /// behavior is undefined.
    ///
    /// [`alloc_pages`]: GlobalAllocator::alloc_pages
    pub fn dealloc_pages(&self, pos: usize, num_pages: usize) {
        self.palloc.lock().dealloc_pages(pos, num_pages)
    }

    /// Returns the number of allocated bytes in the byte allocator.
    pub fn used_bytes(&self) -> usize {
        self.balloc.lock().used_bytes()
    }

    /// Returns the number of available bytes in the byte allocator.
    pub fn available_bytes(&self) -> usize {
        self.balloc.lock().available_bytes()
    }

    /// Returns the number of allocated pages in the page allocator.
    pub fn used_pages(&self) -> usize {
        self.palloc.lock().used_pages()
    }

    /// Returns the number of available pages in the page allocator.
    pub fn available_pages(&self) -> usize {
        self.palloc.lock().available_pages()
    }

    /// Returns a snapshot of the page-frame allocator counters.
    pub fn frame_stats(&self) -> FrameAllocatorStats {
        let palloc = self.palloc.lock();
        FrameAllocatorStats {
            free_frames: palloc.available_pages(),
            allocated_frames: palloc.used_pages(),
        }
    }
}

fn allocation_bucket_index(size: usize) -> usize {
    let size = size.max(1);
    let mut index = 0;
    while index + 1 < ALLOCATION_BUCKET_LIMITS.len() && size > ALLOCATION_BUCKET_LIMITS[index] {
        index += 1;
    }
    index
}

fn record_allocation(layout: Layout, direct: bool) {
    let index = allocation_bucket_index(layout.size());
    ALLOCATION_BUCKET_COUNTS[index].fetch_add(1, Ordering::Relaxed);
    ALLOCATION_BUCKET_BYTES[index].fetch_add(layout.size(), Ordering::Relaxed);
    if direct {
        ALLOCATION_BUCKET_DIRECT_COUNTS[index].fetch_add(1, Ordering::Relaxed);
        ALLOCATION_BUCKET_DIRECT_BYTES[index].fetch_add(layout.size(), Ordering::Relaxed);
    }
}

fn record_deallocation(layout: Layout, direct: bool) {
    let index = allocation_bucket_index(layout.size());
    ALLOCATION_BUCKET_COUNTS[index].fetch_sub(1, Ordering::Relaxed);
    ALLOCATION_BUCKET_BYTES[index].fetch_sub(layout.size(), Ordering::Relaxed);
    if direct {
        ALLOCATION_BUCKET_DIRECT_COUNTS[index].fetch_sub(1, Ordering::Relaxed);
        ALLOCATION_BUCKET_DIRECT_BYTES[index].fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

const fn align_up_to_page(value: usize) -> usize {
    value.saturating_add(PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

const fn pages_for_layout(layout: Layout) -> usize {
    let size = if layout.size() == 0 { 1 } else { layout.size() };
    let bytes = align_up_to_page(size);
    bytes / PAGE_SIZE
}

const fn should_direct_page_alloc(layout: Layout) -> bool {
    layout.size() >= LARGE_DIRECT_ALLOC_THRESHOLD || layout.align() > PAGE_SIZE
}

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if let Ok(ptr) = GlobalAllocator::alloc(self, layout) {
            ptr.as_ptr()
        } else {
            // The `GlobalAlloc` contract signals allocation failure by
            // returning a null pointer.  Higher-level infallible allocation
            // APIs will still call `handle_alloc_error`, while fallible APIs
            // such as `Vec::try_reserve*` can now propagate ENOMEM to syscall
            // callers instead of panicking the kernel during long evaluation
            // runs.
            core::ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        GlobalAllocator::dealloc(self, NonNull::new(ptr).expect("dealloc null ptr"), layout)
    }
}

#[cfg_attr(all(target_os = "none", not(test)), global_allocator)]
static GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator::new();

/// Returns the reference to the global allocator.
pub fn global_allocator() -> &'static GlobalAllocator {
    &GLOBAL_ALLOCATOR
}

/// Returns a snapshot of the global frame allocator counters.
pub fn frame_allocator_stats() -> FrameAllocatorStats {
    GLOBAL_ALLOCATOR.frame_stats()
}

/// Returns live allocation counters grouped by request size.
pub fn allocation_bucket_stats() -> [AllocationBucketStats; 14] {
    core::array::from_fn(|index| AllocationBucketStats {
        max_size: ALLOCATION_BUCKET_LIMITS[index],
        active_count: ALLOCATION_BUCKET_COUNTS[index].load(Ordering::Relaxed),
        active_bytes: ALLOCATION_BUCKET_BYTES[index].load(Ordering::Relaxed),
        direct_count: ALLOCATION_BUCKET_DIRECT_COUNTS[index].load(Ordering::Relaxed),
        direct_bytes: ALLOCATION_BUCKET_DIRECT_BYTES[index].load(Ordering::Relaxed),
    })
}

/// Initializes the global allocator with the given memory region.
///
/// Note that the memory region bounds are just numbers, and the allocator
/// does not actually access the region. Users should ensure that the region
/// is valid and not being used by others, so that the allocated memory is also
/// valid.
///
/// This function should be called only once, and before any allocation.
pub fn global_init(start_vaddr: usize, size: usize) {
    debug!(
        "initialize global allocator at: [{:#x}, {:#x})",
        start_vaddr,
        start_vaddr + size
    );
    GLOBAL_ALLOCATOR.init(start_vaddr, size);
}

/// Add the given memory region to the global allocator.
///
/// Users should ensure that the region is valid and not being used by others,
/// so that the allocated memory is also valid.
///
/// It's similar to [`global_init`], but can be called multiple times.
pub fn global_add_memory(start_vaddr: usize, size: usize) -> AllocResult {
    debug!(
        "add a memory region to global allocator: [{:#x}, {:#x})",
        start_vaddr,
        start_vaddr + size
    );
    GLOBAL_ALLOCATOR.add_memory(start_vaddr, size)
}
