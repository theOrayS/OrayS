#[allow(unused_imports)]
use crate::{AllDevices, prelude::*};

#[cfg(feature = "desktop-device-hook")]
unsafe extern "C" {
    fn orays_desktop_probe_device_mmio(base: usize, size: usize) -> bool;
}

impl AllDevices {
    pub(crate) fn probe_bus_devices(&mut self) {
        // TODO: parse device tree
        #[cfg(feature = "virtio")]
        for reg in axconfig::devices::VIRTIO_MMIO_RANGES {
            for_each_drivers!(type Driver, {
                if let Some(dev) = Driver::probe_mmio(reg.0, reg.1) {
                    info!(
                        "registered a new {:?} device at [PA:{:#x}, PA:{:#x}): {:?}",
                        dev.device_type(),
                        reg.0, reg.0 + reg.1,
                        dev.device_name(),
                    );
                    self.add_device(dev);
                    continue; // skip to the next device
                }
            });
            #[cfg(feature = "desktop-device-hook")]
            {
                // SAFETY: Enabling the private desktop hook requires the final
                // desktop binary to provide this exact C ABI symbol. The
                // physical range is copied directly from platform config and
                // remains valid for the lifetime of the kernel.
                if unsafe { orays_desktop_probe_device_mmio(reg.0, reg.1) } {
                    continue;
                }
            }
        }
    }
}
