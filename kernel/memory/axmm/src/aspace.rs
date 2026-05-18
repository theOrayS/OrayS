use core::fmt;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use axerrno::{AxError, AxResult, ax_err};
use axhal::mem::phys_to_virt;
use axhal::paging::{MappingFlags, PageSize, PageTable};
use axhal::trap::PageFaultFlags;
use memory_addr::{
    MemoryAddr, PAGE_SIZE_4K, PageIter4K, PhysAddr, VirtAddr, VirtAddrRange, is_aligned_4k,
};
use memory_set::{MemoryArea, MemorySet};

use crate::backend::{Backend, retain_shared_frame};
use crate::mapping_err_to_ax_err;

/// The virtual memory address space.
pub struct AddrSpace {
    va_range: VirtAddrRange,
    areas: MemorySet<Backend>,
    pt: PageTable,
}

/// Diagnostic information for a single virtual address lookup.
#[derive(Clone, Copy, Debug)]
pub struct AddrSpaceQuery {
    pub contains: bool,
    pub area_found: bool,
    pub area_start: usize,
    pub area_end: usize,
    pub area_flags: MappingFlags,
    pub backend: &'static str,
    pub pte_mapped: bool,
    pub paddr: usize,
    pub pte_flags: MappingFlags,
    pub page_size: Option<PageSize>,
    pub shared_metadata: bool,
}

impl AddrSpace {
    /// Returns the address space base.
    pub const fn base(&self) -> VirtAddr {
        self.va_range.start
    }

    /// Returns the address space end.
    pub const fn end(&self) -> VirtAddr {
        self.va_range.end
    }

    /// Returns the address space size.
    pub fn size(&self) -> usize {
        self.va_range.size()
    }

    /// Returns the reference to the inner page table.
    pub const fn page_table(&self) -> &PageTable {
        &self.pt
    }

    /// Returns the root physical address of the inner page table.
    pub const fn page_table_root(&self) -> PhysAddr {
        self.pt.root_paddr()
    }

    /// Checks if the address space contains the given address range.
    pub fn contains_range(&self, start: VirtAddr, size: usize) -> bool {
        self.va_range
            .contains_range(VirtAddrRange::from_start_size(start, size))
    }

    /// Creates a new empty address space.
    pub(crate) fn new_empty(base: VirtAddr, size: usize) -> AxResult<Self> {
        Ok(Self {
            va_range: VirtAddrRange::from_start_size(base, size),
            areas: MemorySet::new(),
            pt: PageTable::try_new().map_err(|_| AxError::NoMemory)?,
        })
    }

    /// Copies page table mappings from another address space.
    ///
    /// It copies the page table entries only rather than the memory regions,
    /// usually used to copy a portion of the kernel space mapping to the
    /// user space.
    ///
    /// Returns an error if the two address spaces overlap.
    pub fn copy_mappings_from(&mut self, other: &AddrSpace) -> AxResult {
        if self.va_range.overlaps(other.va_range) {
            return ax_err!(InvalidInput, "address space overlap");
        }
        self.pt
            .cursor()
            .copy_from(&other.pt, other.base(), other.size());
        Ok(())
    }

    /// Clones all user mappings from another address space.
    ///
    /// Unlike [`Self::copy_mappings_from`], this duplicates user mappings even
    /// when the two address spaces cover the same virtual range. Allocation
    /// backends are recreated in the destination. Resident pages are retained
    /// as lazy shared metadata; writable pages are write-protected in both
    /// parent and child so copy-on-write can preserve isolation without
    /// eagerly duplicating the process's writable working set.
    pub fn clone_user_mappings_from(&mut self, other: &mut AddrSpace) -> AxResult {
        if self.va_range != other.va_range {
            return ax_err!(InvalidInput, "address space range mismatch");
        }

        self.clear();

        for area in other.areas.iter() {
            let mut shared_pages = BTreeMap::new();
            let mut retained_frames = Vec::new();
            let mut parent_protect_pages = Vec::new();
            let cow_pages = area.flags().contains(MappingFlags::WRITE);
            let alloc_missing = area.backend().alloc_missing_on_fault();
            for vaddr in PageIter4K::new(area.start(), area.end())
                .expect("memory area bounds must be 4K aligned")
            {
                let parent_pte = other.pt.query(vaddr).ok();
                let query = parent_pte.or_else(|| {
                    area.backend()
                        .shared_page(vaddr)
                        .map(|(paddr, flags)| (paddr, flags, PageSize::Size4K))
                });
                let Some((src_paddr, src_flags, page_size)) = query else {
                    continue;
                };
                if page_size.is_huge() {
                    warn!(
                        "clone_user_mappings_from: huge user page is not supported area_start={:#x} area_end={:#x} vaddr={:#x} src_paddr={:#x} flags={:?} backend={}",
                        area.start(),
                        area.end(),
                        vaddr,
                        src_paddr,
                        src_flags,
                        area.backend().kind_name()
                    );
                    self.clear();
                    return ax_err!(BadState, "failed to clone huge user page");
                }
                let mut child_flags = src_flags;
                if cow_pages {
                    child_flags.remove(MappingFlags::WRITE);
                    if parent_pte.is_some() {
                        parent_protect_pages.push((vaddr, child_flags));
                    }
                }
                shared_pages.insert(vaddr.as_usize(), (src_paddr, child_flags));
                retained_frames.push(src_paddr);
            }

            let cloned_area = MemoryArea::new(
                area.start(),
                area.size(),
                area.flags(),
                Backend::new_shared(shared_pages.clone(), alloc_missing),
            );
            if let Err(err) = self.areas.map(cloned_area, &mut self.pt, false) {
                warn!(
                    "clone_user_mappings_from: map cloned area failed start={:#x} end={:#x} size={:#x} flags={:?} backend={} err={:?}",
                    area.start(),
                    area.end(),
                    area.size(),
                    area.flags(),
                    area.backend().kind_name(),
                    err
                );
                self.clear();
                return Err(mapping_err_to_ax_err(err));
            }

            for frame in retained_frames {
                retain_shared_frame(frame);
            }

            for (vaddr, flags) in parent_protect_pages {
                if let Err(err) = other.pt.cursor().protect(vaddr, flags) {
                    warn!(
                        "clone_user_mappings_from: COW parent protect failed area_start={:#x} area_end={:#x} vaddr={:#x} flags={:?} err={:?}",
                        area.start(),
                        area.end(),
                        vaddr,
                        flags,
                        err
                    );
                    self.clear();
                    return ax_err!(BadState, "failed to protect parent COW page");
                }
            }
        }
        Ok(())
    }

    /// Finds a free area that can accommodate the given size.
    ///
    /// The search starts from the given hint address, and the area should be within the given limit range.
    ///
    /// Returns the start address of the free area. Returns None if no such area is found.
    pub fn find_free_area(
        &self,
        hint: VirtAddr,
        size: usize,
        limit: VirtAddrRange,
    ) -> Option<VirtAddr> {
        self.areas.find_free_area(hint, size, limit, PAGE_SIZE_4K)
    }

    /// Add a new linear mapping.
    ///
    /// See [`Backend`] for more details about the mapping backends.
    ///
    /// The `flags` parameter indicates the mapping permissions and attributes.
    ///
    /// Returns an error if the address range is out of the address space or not
    /// aligned.
    pub fn map_linear(
        &mut self,
        start_vaddr: VirtAddr,
        start_paddr: PhysAddr,
        size: usize,
        flags: MappingFlags,
    ) -> AxResult {
        if !self.contains_range(start_vaddr, size) {
            return ax_err!(InvalidInput, "address out of range");
        }
        if !start_vaddr.is_aligned_4k() || !start_paddr.is_aligned_4k() || !is_aligned_4k(size) {
            return ax_err!(InvalidInput, "address not aligned");
        }

        let offset = start_vaddr.as_usize() - start_paddr.as_usize();
        let area = MemoryArea::new(start_vaddr, size, flags, Backend::new_linear(offset));
        self.areas
            .map(area, &mut self.pt, false)
            .map_err(mapping_err_to_ax_err)?;
        Ok(())
    }

    /// Add a new allocation mapping.
    ///
    /// See [`Backend`] for more details about the mapping backends.
    ///
    /// The `flags` parameter indicates the mapping permissions and attributes.
    ///
    /// Returns an error if the address range is out of the address space or not
    /// aligned.
    pub fn map_alloc(
        &mut self,
        start: VirtAddr,
        size: usize,
        flags: MappingFlags,
        populate: bool,
    ) -> AxResult {
        if !self.contains_range(start, size) {
            return ax_err!(InvalidInput, "address out of range");
        }
        if !start.is_aligned_4k() || !is_aligned_4k(size) {
            return ax_err!(InvalidInput, "address not aligned");
        }

        let area = MemoryArea::new(start, size, flags, Backend::new_alloc(populate));
        self.areas
            .map(area, &mut self.pt, false)
            .map_err(mapping_err_to_ax_err)?;
        Ok(())
    }

    /// Removes mappings within the specified virtual address range.
    ///
    /// Returns an error if the address range is out of the address space or not
    /// aligned.
    pub fn unmap(&mut self, start: VirtAddr, size: usize) -> AxResult {
        if !self.contains_range(start, size) {
            return ax_err!(InvalidInput, "address out of range");
        }
        if !start.is_aligned_4k() || !is_aligned_4k(size) {
            return ax_err!(InvalidInput, "address not aligned");
        }

        self.areas
            .unmap(start, size, &mut self.pt)
            .map_err(mapping_err_to_ax_err)?;
        Ok(())
    }

    /// To process data in this area with the given function.
    ///
    /// Now it supports reading and writing data in the given interval.
    fn process_area_data<F>(
        &mut self,
        start: VirtAddr,
        size: usize,
        access_flags: MappingFlags,
        fault_flags: PageFaultFlags,
        mut f: F,
    ) -> AxResult
    where
        F: FnMut(VirtAddr, usize, usize),
    {
        if !self.contains_range(start, size) {
            return ax_err!(InvalidInput, "address out of range");
        }
        let mut cnt = 0;
        // If start is aligned to 4K, start_align_down will be equal to start_align_up.
        let end_align_up = (start + size).align_up_4k();
        for vaddr in PageIter4K::new(start.align_down_4k(), end_align_up)
            .expect("Failed to create page iterator")
        {
            let (mut paddr, _, _) = self.ensure_page_access(vaddr, access_flags, fault_flags)?;

            let mut copy_size = (size - cnt).min(PAGE_SIZE_4K);

            if copy_size == 0 {
                break;
            }
            if vaddr == start.align_down_4k() && start.align_offset_4k() != 0 {
                let align_offset = start.align_offset_4k();
                copy_size = copy_size.min(PAGE_SIZE_4K - align_offset);
                paddr += align_offset;
            }
            f(phys_to_virt(paddr), cnt, copy_size);
            cnt += copy_size;
        }
        Ok(())
    }

    fn ensure_page_access(
        &mut self,
        vaddr: VirtAddr,
        access_flags: MappingFlags,
        fault_flags: PageFaultFlags,
    ) -> AxResult<(PhysAddr, MappingFlags, PageSize)> {
        let needs_write = access_flags.contains(MappingFlags::WRITE);
        match self.pt.query(vaddr) {
            Ok((paddr, flags, page_size)) => {
                if needs_write && !flags.contains(MappingFlags::WRITE) {
                    if self.handle_page_fault(vaddr, PageFaultFlags::WRITE) {
                        return self.ensure_write_resolved(vaddr);
                    }
                    if !self.area_allows_write(vaddr) {
                        return Ok((paddr, flags, page_size));
                    }
                    return ax_err!(BadAddress, "write fault was not handled");
                }
                if !needs_write && !flags.contains(access_flags) {
                    return ax_err!(BadAddress, "page permissions do not allow access");
                }
                Ok((paddr, flags, page_size))
            }
            _ => {
                let handled = self.handle_page_fault(vaddr, fault_flags)
                    || (needs_write && self.handle_page_fault(vaddr, PageFaultFlags::READ));
                if !handled {
                    return ax_err!(BadAddress, "page fault was not handled");
                }
                if needs_write {
                    return self.ensure_write_resolved(vaddr);
                }
                let (paddr, flags, page_size) =
                    self.pt.query(vaddr).map_err(|_| AxError::BadAddress)?;
                if !flags.contains(access_flags) {
                    return ax_err!(BadAddress, "page permissions do not allow access");
                }
                Ok((paddr, flags, page_size))
            }
        }
    }

    fn ensure_write_resolved(
        &mut self,
        vaddr: VirtAddr,
    ) -> AxResult<(PhysAddr, MappingFlags, PageSize)> {
        let (paddr, flags, page_size) = self.pt.query(vaddr).map_err(|_| AxError::BadAddress)?;
        if flags.contains(MappingFlags::WRITE) || !self.area_allows_write(vaddr) {
            return Ok((paddr, flags, page_size));
        }
        if self.handle_page_fault(vaddr, PageFaultFlags::WRITE) {
            let (paddr, flags, page_size) =
                self.pt.query(vaddr).map_err(|_| AxError::BadAddress)?;
            if flags.contains(MappingFlags::WRITE) {
                return Ok((paddr, flags, page_size));
            }
        }
        ax_err!(BadAddress, "write fault was not resolved")
    }

    fn area_allows_write(&self, vaddr: VirtAddr) -> bool {
        self.areas
            .find(vaddr)
            .is_some_and(|area| area.flags().contains(MappingFlags::WRITE))
    }

    /// To read data from the address space.
    ///
    /// # Arguments
    ///
    /// * `start` - The start virtual address to read.
    /// * `buf` - The buffer to store the data.
    pub fn read(&mut self, start: VirtAddr, buf: &mut [u8]) -> AxResult {
        self.process_area_data(
            start,
            buf.len(),
            MappingFlags::READ,
            PageFaultFlags::READ,
            |src, offset, read_size| unsafe {
                core::ptr::copy_nonoverlapping(
                    src.as_ptr(),
                    buf.as_mut_ptr().add(offset),
                    read_size,
                );
            },
        )
    }

    /// To write data to the address space.
    ///
    /// # Arguments
    ///
    /// * `start_vaddr` - The start virtual address to write.
    /// * `buf` - The buffer to write to the address space.
    pub fn write(&mut self, start: VirtAddr, buf: &[u8]) -> AxResult {
        self.process_area_data(
            start,
            buf.len(),
            MappingFlags::WRITE,
            PageFaultFlags::WRITE,
            |dst, offset, write_size| unsafe {
                core::ptr::copy_nonoverlapping(
                    buf.as_ptr().add(offset),
                    dst.as_mut_ptr(),
                    write_size,
                );
            },
        )
    }

    /// Populates lazy pages in the specified virtual address range.
    pub fn populate_range(
        &mut self,
        start: VirtAddr,
        size: usize,
        access_flags: PageFaultFlags,
    ) -> AxResult {
        if !self.contains_range(start, size) {
            return ax_err!(InvalidInput, "address out of range");
        }
        let end_align_up = (start + size).align_up_4k();
        for vaddr in PageIter4K::new(start.align_down_4k(), end_align_up)
            .expect("Failed to create page iterator")
        {
            if self.pt.query(vaddr).is_err() && !self.handle_page_fault(vaddr, access_flags) {
                return ax_err!(BadState, "failed to populate lazy page");
            }
        }
        Ok(())
    }

    /// Updates mapping within the specified virtual address range.
    ///
    /// Returns an error if the address range is out of the address space or not
    /// aligned.
    pub fn protect(&mut self, start: VirtAddr, size: usize, flags: MappingFlags) -> AxResult {
        if !self.contains_range(start, size) {
            return ax_err!(InvalidInput, "address out of range");
        }
        if !start.is_aligned_4k() || !is_aligned_4k(size) {
            return ax_err!(InvalidInput, "address not aligned");
        }

        self.areas
            .protect(start, size, |_| Some(flags), &mut self.pt)
            .map_err(mapping_err_to_ax_err)?;
        Ok(())
    }

    /// Removes all mappings in the address space.
    pub fn clear(&mut self) {
        self.areas.clear(&mut self.pt).unwrap();
    }

    /// Checks whether an access to the specified memory region is valid.
    ///
    /// Returns `true` if the memory region given by `range` is all mapped and
    /// has proper permission flags (i.e. containing `access_flags`).
    pub fn can_access_range(
        &self,
        start: VirtAddr,
        size: usize,
        access_flags: MappingFlags,
    ) -> bool {
        let mut range = VirtAddrRange::from_start_size(start, size);
        for area in self.areas.iter() {
            if area.end() <= range.start {
                continue;
            }
            if area.start() > range.start {
                return false;
            }

            // This area overlaps with the memory region
            if !area.flags().contains(access_flags) {
                return false;
            }

            range.start = area.end();
            if range.is_empty() {
                return true;
            }
        }

        false
    }

    /// Returns diagnostic information for one virtual address.
    pub fn query_address(&self, vaddr: VirtAddr) -> AddrSpaceQuery {
        let contains = self.va_range.contains(vaddr);
        let mut query = AddrSpaceQuery {
            contains,
            area_found: false,
            area_start: 0,
            area_end: 0,
            area_flags: MappingFlags::empty(),
            backend: "none",
            pte_mapped: false,
            paddr: 0,
            pte_flags: MappingFlags::empty(),
            page_size: None,
            shared_metadata: false,
        };
        if let Some(area) = self.areas.find(vaddr) {
            query.area_found = true;
            query.area_start = area.start().as_usize();
            query.area_end = area.end().as_usize();
            query.area_flags = area.flags();
            query.backend = area.backend().kind_name();
            query.shared_metadata = area.backend().shared_page(vaddr.align_down_4k()).is_some();
        }
        if let Ok((paddr, flags, page_size)) = self.pt.query(vaddr.align_down_4k()) {
            query.pte_mapped = true;
            query.paddr = paddr.as_usize();
            query.pte_flags = flags;
            query.page_size = Some(page_size);
        }
        query
    }

    /// Handles a page fault at the given address.
    ///
    /// `access_flags` indicates the access type that caused the page fault.
    ///
    /// Returns `true` if the page fault is handled successfully (not a real
    /// fault).
    pub fn handle_page_fault(&mut self, vaddr: VirtAddr, access_flags: PageFaultFlags) -> bool {
        if !self.va_range.contains(vaddr) {
            return false;
        }
        if let Some(area) = self.areas.find(vaddr) {
            let orig_flags = area.flags();
            let access_flags = MappingFlags::from_bits_truncate(access_flags.bits());
            if orig_flags.contains(access_flags) {
                return area
                    .backend()
                    .handle_page_fault(vaddr, orig_flags, &mut self.pt);
            }
        }
        false
    }
}

impl fmt::Debug for AddrSpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AddrSpace")
            .field("va_range", &self.va_range)
            .field("page_table_root", &self.pt.root_paddr())
            .field("areas", &self.areas)
            .finish()
    }
}

impl Drop for AddrSpace {
    fn drop(&mut self) {
        self.clear();
    }
}
