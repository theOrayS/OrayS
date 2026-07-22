use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;

use axalloc::{frame_allocator_stats, global_allocator};
use axhal::mem::{phys_to_virt, virt_to_phys};
use axhal::paging::{MappingFlags, PageSize, PageTable};
use kspin::SpinNoIrq;
use lazyinit::LazyInit;
use memory_addr::{MemoryAddr, PAGE_SIZE_4K, PageIter4K, PhysAddr, VirtAddr};

use super::{Backend, SharedPages, pte_flags_for_mapping};

static SHARED_FRAMES: LazyInit<SpinNoIrq<BTreeMap<usize, usize>>> = LazyInit::new();

/// Reference-count statistics for shared physical frames.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SharedFrameStats {
    /// Number of tracked shared frames.
    pub entries: usize,
    /// Sum of reference counts over all tracked frames.
    pub total_refs: usize,
    /// Highest reference count of any tracked frame.
    pub max_refcount: usize,
}

fn shared_frames() -> &'static SpinNoIrq<BTreeMap<usize, usize>> {
    let _ = SHARED_FRAMES.call_once(|| SpinNoIrq::new(BTreeMap::new()));
    &SHARED_FRAMES
}

pub(crate) fn shared_frame_stats() -> SharedFrameStats {
    let frames = shared_frames().lock();
    let mut total_refs = 0usize;
    let mut max_refcount = 0usize;
    for count in frames.values().copied() {
        total_refs = total_refs.saturating_add(count);
        max_refcount = max_refcount.max(count);
    }
    SharedFrameStats {
        entries: frames.len(),
        total_refs,
        max_refcount,
    }
}

fn alloc_frame(zeroed: bool) -> Option<PhysAddr> {
    let vaddr = VirtAddr::from(global_allocator().alloc_pages(1, PAGE_SIZE_4K).ok()?);
    if zeroed {
        unsafe { core::ptr::write_bytes(vaddr.as_mut_ptr(), 0, PAGE_SIZE_4K) };
    }
    let paddr = virt_to_phys(vaddr);
    Some(paddr)
}

fn dealloc_frame(frame: PhysAddr) {
    let vaddr = phys_to_virt(frame);
    global_allocator().dealloc_pages(vaddr.as_usize(), 1);
}

pub(crate) fn retain_shared_frame(frame: PhysAddr) {
    let mut frames = shared_frames().lock();
    let key = frame.as_usize();
    if let Some(count) = frames.get_mut(&key) {
        *count += 1;
    } else {
        frames.insert(key, 2);
    }
}

fn shared_frame_count(frame: PhysAddr) -> Option<usize> {
    shared_frames().lock().get(&frame.as_usize()).copied()
}

fn forget_shared_frame(frame: PhysAddr) {
    shared_frames().lock().remove(&frame.as_usize());
}

pub(crate) fn release_owned_frame(frame: PhysAddr) {
    let should_dealloc = {
        let mut frames = shared_frames().lock();
        let key = frame.as_usize();
        match frames.get_mut(&key) {
            Some(count) if *count > 1 => {
                *count -= 1;
                false
            }
            Some(_) => {
                frames.remove(&key);
                true
            }
            None => true,
        }
    };
    if should_dealloc {
        dealloc_frame(frame);
    }
}

impl Backend {
    /// Creates a new allocation mapping backend.
    pub const fn new_alloc(populate: bool) -> Self {
        Self::Alloc { populate }
    }

    pub(crate) fn new_shared(
        pages: BTreeMap<usize, (PhysAddr, MappingFlags)>,
        alloc_missing: bool,
    ) -> Self {
        Self::Shared {
            pages: Arc::new(SpinNoIrq::new(pages)),
            alloc_missing,
        }
    }

    pub(crate) fn map_alloc(
        &self,
        start: VirtAddr,
        size: usize,
        flags: MappingFlags,
        pt: &mut PageTable,
        populate: bool,
    ) -> bool {
        debug!(
            "map_alloc: [{:#x}, {:#x}) {:?} (populate={})",
            start,
            start + size,
            flags,
            populate
        );
        if populate {
            // allocate all possible physical frames for populated mapping.
            let mut cursor = pt.cursor();
            for addr in PageIter4K::new(start, start + size).unwrap() {
                let Some(frame) = alloc_frame(true) else {
                    let stats = frame_allocator_stats();
                    warn!(
                        "map_alloc: frame allocation failed start={:#x} size={:#x} at={:#x} flags={:?} populate=true free_frames={} allocated_frames={}",
                        start, size, addr, flags, stats.free_frames, stats.allocated_frames
                    );
                    for rollback_addr in PageIter4K::new(start, addr).unwrap() {
                        if let Ok((mapped_frame, _, page_size)) = cursor.unmap(rollback_addr)
                            && !page_size.is_huge()
                        {
                            release_owned_frame(mapped_frame);
                        }
                    }
                    return false;
                };
                let pte_flags = pte_flags_for_mapping(flags);
                if cursor
                    .map(addr, frame, PageSize::Size4K, pte_flags)
                    .is_err()
                {
                    warn!(
                        "map_alloc: page-table map failed start={:#x} size={:#x} at={:#x} frame={:#x} flags={:?} populate=true",
                        start, size, addr, frame, flags
                    );
                    for rollback_addr in PageIter4K::new(start, addr).unwrap() {
                        if let Ok((mapped_frame, _, page_size)) = cursor.unmap(rollback_addr)
                            && !page_size.is_huge()
                        {
                            release_owned_frame(mapped_frame);
                        }
                    }
                    release_owned_frame(frame);
                    return false;
                }
            }
            true
        } else {
            // Leave page-table entries absent until the first page fault.
            true
        }
    }

    pub(crate) fn unmap_alloc(
        &self,
        start: VirtAddr,
        size: usize,
        pt: &mut PageTable,
        _populate: bool,
    ) -> bool {
        debug!("unmap_alloc: [{:#x}, {:#x})", start, start + size);
        for addr in PageIter4K::new(start, start + size).unwrap() {
            if let Ok((frame, _, page_size)) = pt.cursor().unmap(addr) {
                // Deallocate the physical frame if there is a mapping in the
                // page table.
                if page_size.is_huge() {
                    return false;
                }
                release_owned_frame(frame);
            }
        }
        true
    }

    pub(crate) fn unmap_shared(
        &self,
        start: VirtAddr,
        size: usize,
        pt: &mut PageTable,
        pages: &SharedPages,
    ) -> bool {
        for addr in PageIter4K::new(start, start + size).unwrap() {
            if let Ok((frame, _, page_size)) = pt.cursor().unmap(addr) {
                if page_size.is_huge() {
                    return false;
                }
                let shared_frame = pages
                    .lock()
                    .get(&addr.as_usize())
                    .is_some_and(|(source_frame, _)| *source_frame == frame);
                if !shared_frame {
                    release_owned_frame(frame);
                }
            }
        }
        let mut pages = pages.lock();
        let keys = pages
            .range(start.as_usize()..(start + size).as_usize())
            .map(|(addr, _)| *addr)
            .collect::<Vec<_>>();
        for key in keys {
            if let Some((frame, _)) = pages.remove(&key) {
                release_owned_frame(frame);
            }
        }
        true
    }

    pub(crate) fn map_shared(
        &self,
        start: VirtAddr,
        size: usize,
        pt: &mut PageTable,
        pages: &SharedPages,
    ) -> bool {
        let end = start + size;
        let entries = pages
            .lock()
            .range(start.as_usize()..end.as_usize())
            .map(|(addr, (frame, flags))| (VirtAddr::from(*addr), *frame, *flags))
            .collect::<Vec<_>>();

        let mut cursor = pt.cursor();
        for (addr, frame, flags) in entries.iter().copied() {
            if cursor.map(addr, frame, PageSize::Size4K, flags).is_err() {
                warn!(
                    "map_shared: page-table map failed start={:#x} size={:#x} at={:#x} frame={:#x} flags={:?}",
                    start, size, addr, frame, flags
                );
                for (rollback_addr, _, _) in entries
                    .iter()
                    .copied()
                    .take_while(|(rollback_addr, _, _)| *rollback_addr != addr)
                {
                    let _ = cursor.unmap(rollback_addr);
                }
                return false;
            }
        }
        true
    }

    fn handle_cow_fault(
        &self,
        vaddr: VirtAddr,
        orig_flags: MappingFlags,
        pt: &mut PageTable,
    ) -> bool {
        let Ok((old_frame, flags, page_size)) = pt.query(vaddr) else {
            return false;
        };
        if page_size.is_huge()
            || flags.contains(MappingFlags::WRITE)
            || !orig_flags.contains(MappingFlags::WRITE)
        {
            return false;
        }
        let count = shared_frame_count(old_frame).unwrap_or(1);
        if count <= 1 {
            let res = pt.cursor().protect(vaddr, orig_flags);
            if let Err(err) = &res {
                warn!(
                    "handle_page_fault_alloc: COW protect failed for {:#x}: {:?}",
                    vaddr, err
                );
            } else {
                forget_shared_frame(old_frame);
                axhal::asm::flush_tlb(Some(vaddr));
            }
            return res.is_ok();
        }
        let Some(new_frame) = alloc_frame(false) else {
            let stats = frame_allocator_stats();
            warn!(
                "handle_page_fault_alloc: COW frame allocation failed for {:#x} flags={:?} free_frames={} allocated_frames={}",
                vaddr, orig_flags, stats.free_frames, stats.allocated_frames
            );
            return false;
        };
        unsafe {
            core::ptr::copy_nonoverlapping(
                phys_to_virt(old_frame).as_ptr(),
                phys_to_virt(new_frame).as_mut_ptr(),
                PAGE_SIZE_4K,
            );
        }
        let res = pt.cursor().remap(vaddr, new_frame, orig_flags);
        if let Err(err) = &res {
            warn!(
                "handle_page_fault_alloc: COW remap failed for {:#x}: {:?}",
                vaddr, err
            );
            dealloc_frame(new_frame);
            return false;
        }
        axhal::asm::flush_tlb(Some(vaddr));
        release_owned_frame(old_frame);
        true
    }

    fn map_fresh_frame(&self, vaddr: VirtAddr, flags: MappingFlags, pt: &mut PageTable) -> bool {
        if let Some(frame) = alloc_frame(true) {
            let pte_flags = pte_flags_for_mapping(flags);
            let res = pt.cursor().map(vaddr, frame, PageSize::Size4K, pte_flags);
            if let Err(e) = &res {
                warn!(
                    "handle_page_fault_alloc: map failed for {:#x}: {:?}",
                    vaddr, e
                );
                release_owned_frame(frame);
            }
            res.is_ok()
        } else {
            let stats = frame_allocator_stats();
            warn!(
                "handle_page_fault_alloc: frame allocation failed for {:#x} flags={:?} free_frames={} allocated_frames={}",
                vaddr, flags, stats.free_frames, stats.allocated_frames
            );
            false
        }
    }

    pub(crate) fn handle_page_fault_alloc(
        &self,
        vaddr: VirtAddr,
        orig_flags: MappingFlags,
        pt: &mut PageTable,
        populate: bool,
    ) -> bool {
        let vaddr = vaddr.align_down_4k();
        if pt.query(vaddr).is_ok() {
            self.handle_cow_fault(vaddr, orig_flags, pt)
        } else if populate {
            false // Populated mappings should not trigger page faults.
        } else {
            self.map_fresh_frame(vaddr, orig_flags, pt)
        }
    }

    pub(crate) fn handle_page_fault_shared(
        &self,
        vaddr: VirtAddr,
        orig_flags: MappingFlags,
        pt: &mut PageTable,
        pages: &SharedPages,
        alloc_missing: bool,
    ) -> bool {
        let vaddr = vaddr.align_down_4k();
        if pt.query(vaddr).is_ok() {
            let handled = self.handle_cow_fault(vaddr, orig_flags, pt);
            if handled {
                pages.lock().remove(&vaddr.as_usize());
            }
            return handled;
        }
        if let Some((frame, flags)) = pages.lock().get(&vaddr.as_usize()).copied() {
            let res = pt.cursor().map(vaddr, frame, PageSize::Size4K, flags);
            if let Err(err) = &res {
                warn!(
                    "handle_page_fault_shared: map shared page failed for {:#x}: {:?}",
                    vaddr, err
                );
            }
            return res.is_ok();
        }
        if alloc_missing {
            self.map_fresh_frame(vaddr, orig_flags, pt)
        } else {
            false
        }
    }
}
