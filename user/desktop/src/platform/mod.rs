pub mod display;
pub mod filesystem;
pub mod input;
pub mod system;
pub mod time;

#[cfg(feature = "orays")]
mod virtio;

#[cfg(any(feature = "orays", test))]
const DMA_PAGE_SIZE: usize = 0x1000;

#[cfg(any(feature = "orays", test))]
pub(crate) fn checked_dma_byte_len(pages: usize) -> usize {
    assert!(
        pages != 0,
        "VirtIO DMA allocation requires at least one page"
    );
    pages
        .checked_mul(DMA_PAGE_SIZE)
        .expect("VirtIO DMA allocation size overflow")
}

#[cfg(any(feature = "orays", test))]
pub(crate) fn require_dma_allocation<T, E>(allocation: Result<T, E>) -> T {
    allocation.unwrap_or_else(|_| panic!("VirtIO DMA allocation failed"))
}

#[cfg(any(feature = "orays", test))]
pub(crate) fn zero_dma_bytes(bytes: &mut [u8]) {
    bytes.fill(0);
}

#[cfg(any(all(feature = "orays", feature = "bus-pci"), test))]
pub(crate) fn claim_initialized<T, E>(
    result: Result<T, E>,
    register: impl FnOnce(T) -> bool,
    report_error: impl FnOnce(E),
) -> bool {
    match result {
        Ok(value) => register(value),
        Err(error) => {
            report_error(error);
            false
        }
    }
}

#[cfg(any(feature = "orays", test))]
pub(crate) fn store_once<T>(slot: &mut Option<T>, value: T) -> bool {
    if slot.is_some() {
        return false;
    }
    *slot = Some(value);
    true
}

#[cfg(any(feature = "orays", test))]
pub(crate) fn store_first_empty<T>(slots: &mut [Option<T>], value: T) -> bool {
    let Some(slot) = slots.iter_mut().find(|slot| slot.is_none()) else {
        return false;
    };
    *slot = Some(value);
    true
}

#[cfg(any(feature = "orays", test))]
pub(crate) fn reserve_first_empty<T>(slots: &[Option<T>], reservations: &mut usize) -> bool {
    let available = slots.iter().filter(|slot| slot.is_none()).count();
    if *reservations >= available {
        return false;
    }
    *reservations += 1;
    true
}

#[cfg(any(feature = "orays", test))]
pub(crate) fn cancel_reservation(reservations: &mut usize) -> bool {
    if *reservations == 0 {
        return false;
    }
    *reservations -= 1;
    true
}

#[cfg(any(feature = "orays", test))]
pub(crate) fn commit_reserved<T>(
    slots: &mut [Option<T>],
    reservations: &mut usize,
    value: T,
) -> bool {
    if !cancel_reservation(reservations) {
        return false;
    }
    store_first_empty(slots, value)
}

#[cfg(all(feature = "orays", feature = "bus-mmio"))]
#[unsafe(no_mangle)]
pub extern "C" fn orays_desktop_probe_device_mmio(base: usize, size: usize) -> bool {
    use core::ptr::NonNull;

    use axhal::mem::phys_to_virt;
    use virtio_drivers::transport::mmio::{MmioTransport, VirtIOHeader};
    use virtio_drivers::transport::{DeviceType, Transport};

    let address = phys_to_virt(base.into());
    let Some(header) = NonNull::new(address.as_mut_ptr().cast::<VirtIOHeader>()) else {
        return false;
    };
    // SAFETY: axdriver invokes this hook synchronously for the unique active
    // MMIO range before creating any other owner for an unclaimed device. The
    // aligned mapped range comes directly from platform configuration and
    // remains valid for the desktop driver's lifetime. One transport is
    // created and moved to exactly one device owner below.
    let Ok(transport) = (unsafe { MmioTransport::new(header, size) }) else {
        return false;
    };
    match transport.device_type() {
        DeviceType::Input => input::register_orays_input_mmio(transport),
        DeviceType::GPU => display::register_orays_display_mmio(transport),
        _ => false,
    }
}

#[cfg(all(feature = "orays", feature = "bus-pci"))]
#[unsafe(no_mangle)]
/// # Safety
///
/// `root` must point to axdriver's unique active
/// `PciRoot<MmioCam<'static>>`, and `info` must point to the matching current
/// `DeviceFunctionInfo`. Both pointers must remain valid and exclusively
/// borrowable for this synchronous call and must not be retained afterwards.
pub unsafe extern "C" fn orays_desktop_probe_device_pci(
    root: *mut core::ffi::c_void,
    bus: u8,
    device: u8,
    function: u8,
    info: *const core::ffi::c_void,
) -> bool {
    use axdriver_pci::DeviceFunction;
    use virtio_drivers::transport::DeviceType;
    use virtio_drivers::transport::pci::virtio_device_type;

    // SAFETY: The caller contract above guarantees validity, exclusivity and
    // the exact pointee types for the duration of this call.
    let Some(root) = (unsafe {
        root.cast::<axdriver_pci::PciRoot<axdriver_pci::MmioCam<'static>>>()
            .as_mut()
    }) else {
        return false;
    };
    // SAFETY: The caller contract guarantees a valid matching immutable
    // DeviceFunctionInfo for the duration of this call.
    let Some(info) = (unsafe { info.cast::<axdriver_pci::DeviceFunctionInfo>().as_ref() }) else {
        return false;
    };
    let bdf = DeviceFunction {
        bus,
        device,
        function,
    };
    match virtio_device_type(info) {
        Some(DeviceType::Input) => input::register_orays_input_pci(root, bdf),
        Some(DeviceType::GPU) => display::register_orays_display_pci(root, bdf),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DMA_PAGE_SIZE, cancel_reservation, checked_dma_byte_len, claim_initialized,
        commit_reserved, require_dma_allocation, reserve_first_empty, store_first_empty,
        store_once, zero_dma_bytes,
    };

    #[test]
    #[should_panic(expected = "requires at least one page")]
    fn zero_page_dma_allocation_is_rejected() {
        let _ = checked_dma_byte_len(0);
    }

    #[test]
    #[should_panic(expected = "allocation size overflow")]
    fn overflowing_dma_allocation_is_rejected() {
        let _ = checked_dma_byte_len(usize::MAX);
    }

    #[test]
    #[should_panic(expected = "VirtIO DMA allocation failed")]
    fn dma_allocator_failure_uses_the_explicit_panic_policy() {
        let _: usize = require_dma_allocation(Err::<usize, _>("simulated OOM"));
    }

    #[test]
    fn dma_zeroing_clears_every_allocated_byte() {
        let mut allocation = [0xa5; DMA_PAGE_SIZE * 2];
        zero_dma_bytes(&mut allocation);
        assert!(allocation.iter().all(|byte| *byte == 0));
    }

    #[test]
    fn initialization_failure_never_claims_a_device() {
        let mut registered = false;
        let mut reported = false;
        let claimed = claim_initialized(
            Err::<u8, _>("init failed"),
            |_| {
                registered = true;
                true
            },
            |_| reported = true,
        );
        assert!(!claimed);
        assert!(!registered);
        assert!(reported);
    }

    #[test]
    fn occupied_single_slot_is_not_claimed_or_replaced() {
        let mut slot = None;
        assert!(store_once(&mut slot, 1));
        assert!(!store_once(&mut slot, 2));
        assert_eq!(slot, Some(1));
    }

    #[test]
    fn full_registry_is_not_claimed_or_replaced() {
        let mut slots = [None, None];
        assert!(store_first_empty(&mut slots, 1));
        assert!(store_first_empty(&mut slots, 2));
        assert!(!store_first_empty(&mut slots, 3));
        assert_eq!(slots, [Some(1), Some(2)]);
    }

    #[test]
    fn full_registry_is_rejected_before_device_initialization() {
        let slots = [Some(1), Some(2)];
        let mut reservations = 0;
        let mut initialized = false;
        if reserve_first_empty(&slots, &mut reservations) {
            initialized = true;
        }
        assert!(!initialized);
        assert_eq!(reservations, 0);
    }

    #[test]
    fn reservations_are_committed_or_cancelled_exactly_once() {
        let mut slots = [None, None];
        let mut reservations = 0;
        assert!(reserve_first_empty(&slots, &mut reservations));
        assert!(reserve_first_empty(&slots, &mut reservations));
        assert!(!reserve_first_empty(&slots, &mut reservations));
        assert!(commit_reserved(&mut slots, &mut reservations, 1));
        assert!(cancel_reservation(&mut reservations));
        assert!(!cancel_reservation(&mut reservations));
        assert_eq!(slots, [Some(1), None]);
    }
}
