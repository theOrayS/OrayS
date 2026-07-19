use alloc::vec;
use alloc::vec::Vec;

use crate::graphics::geometry::Rect;
use crate::graphics::surface::{Surface, SurfaceError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Bgra8888,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayDescriptor {
    pub width: u32,
    pub height: u32,
    pub stride_bytes: usize,
    pub pixel_format: PixelFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayError {
    Empty,
    InvalidGeometry,
    UnsupportedFormat,
    DeviceUnavailable,
    DeviceFailure,
    Surface(SurfaceError),
}

impl From<SurfaceError> for DisplayError {
    fn from(value: SurfaceError) -> Self {
        Self::Surface(value)
    }
}

pub trait DisplayBackend {
    fn info(&self) -> DisplayDescriptor;
    fn refresh_info(&mut self) -> Result<DisplayDescriptor, DisplayError> {
        Ok(self.info())
    }
    fn present(&mut self, surface: &Surface, damage: &[Rect]) -> Result<(), DisplayError>;
}

pub struct MemoryDisplay {
    info: DisplayDescriptor,
    framebuffer: Vec<u8>,
    frame_count: u64,
    last_damage: Vec<Rect>,
}

impl MemoryDisplay {
    pub fn new(width: u32, height: u32, stride_bytes: usize) -> Result<Self, DisplayError> {
        if width == 0 || height == 0 {
            return Err(DisplayError::Empty);
        }
        let minimum_stride = width as usize * 4;
        if stride_bytes < minimum_stride {
            return Err(DisplayError::InvalidGeometry);
        }
        let size = (height as usize)
            .checked_mul(stride_bytes)
            .ok_or(DisplayError::InvalidGeometry)?;
        Ok(Self {
            info: DisplayDescriptor {
                width,
                height,
                stride_bytes,
                pixel_format: PixelFormat::Bgra8888,
            },
            framebuffer: vec![0; size],
            frame_count: 0,
            last_damage: Vec::new(),
        })
    }

    pub fn framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }

    pub const fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn last_damage(&self) -> &[Rect] {
        &self.last_damage
    }
}

impl DisplayBackend for MemoryDisplay {
    fn info(&self) -> DisplayDescriptor {
        self.info
    }

    fn present(&mut self, surface: &Surface, damage: &[Rect]) -> Result<(), DisplayError> {
        if surface.width() != self.info.width || surface.height() != self.info.height {
            return Err(DisplayError::InvalidGeometry);
        }
        surface.copy_bgra8888_regions_to(&mut self.framebuffer, self.info.stride_bytes, damage)?;
        self.last_damage.clear();
        self.last_damage.extend_from_slice(damage);
        self.frame_count = self.frame_count.saturating_add(1);
        Ok(())
    }
}

#[cfg(feature = "orays")]
pub struct OraysDisplay {
    inner: OraysGpu,
    info: DisplayDescriptor,
    framebuffer: Option<core::ptr::NonNull<u8>>,
    framebuffer_len: usize,
    refresh_count: u8,
}

#[cfg(feature = "orays")]
impl OraysDisplay {
    pub fn new() -> Result<Self, DisplayError> {
        let transport = probe_gpu_transport()?;
        let mut inner = OraysGpu::new(transport).map_err(|_| DisplayError::DeviceFailure)?;
        let (width, height) = inner
            .resolution()
            .map_err(|_| DisplayError::DeviceFailure)?;
        let (framebuffer, framebuffer_len) = allocate_framebuffer(&mut inner, width, height)?;
        let info = descriptor_from_dimensions(width, height, framebuffer_len)?;
        Ok(Self {
            inner,
            info,
            framebuffer: Some(framebuffer),
            framebuffer_len,
            refresh_count: 0,
        })
    }

    fn reconfigure(&mut self, width: u32, height: u32) -> Result<(), DisplayError> {
        let visible_size = checked_visible_size(width, height)?;
        let proposed = descriptor_from_dimensions(width, height, visible_size)?;
        // `VirtIOGpu::change_resolution` tears down the old DMA allocation
        // before it creates the replacement and may fail after that point.
        // Invalidate our raw view first so every error path leaves `present`
        // unable to dereference the former allocation.
        self.framebuffer = None;
        self.framebuffer_len = 0;
        let (framebuffer, framebuffer_len) = allocate_framebuffer(&mut self.inner, width, height)?;
        if framebuffer_len < visible_size {
            return Err(DisplayError::InvalidGeometry);
        }
        self.info = proposed;
        self.framebuffer = Some(framebuffer);
        self.framebuffer_len = framebuffer_len;
        axstd::println!(
            "ORAYS_DESKTOP_DISPLAY_CHANGED width={} height={}",
            width,
            height
        );
        Ok(())
    }
}

#[cfg(any(feature = "orays", test))]
fn checked_visible_size(width: u32, height: u32) -> Result<usize, DisplayError> {
    if width == 0 || height == 0 {
        return Err(DisplayError::Empty);
    }
    // virtio-drivers 0.13 computes the allocation size in `u32`, so reject
    // overflow before calling `VirtIOGpu::change_resolution`.
    let bytes = width
        .checked_mul(height)
        .and_then(|pixels| pixels.checked_mul(4))
        .ok_or(DisplayError::InvalidGeometry)?;
    Ok(bytes as usize)
}

#[cfg(feature = "orays")]
fn descriptor_from_dimensions(
    width: u32,
    height: u32,
    framebuffer_len: usize,
) -> Result<DisplayDescriptor, DisplayError> {
    if width == 0 || height == 0 || framebuffer_len == 0 {
        return Err(DisplayError::Empty);
    }
    let stride_bytes = (width as usize)
        .checked_mul(4)
        .ok_or(DisplayError::InvalidGeometry)?;
    let visible_size = stride_bytes
        .checked_mul(height as usize)
        .ok_or(DisplayError::InvalidGeometry)?;
    if visible_size > framebuffer_len {
        return Err(DisplayError::InvalidGeometry);
    }
    Ok(DisplayDescriptor {
        width,
        height,
        stride_bytes,
        pixel_format: PixelFormat::Bgra8888,
    })
}

#[cfg(feature = "orays")]
fn allocate_framebuffer(
    inner: &mut OraysGpu,
    width: u32,
    height: u32,
) -> Result<(core::ptr::NonNull<u8>, usize), DisplayError> {
    let visible_size = checked_visible_size(width, height)?;
    let framebuffer = inner
        .change_resolution(width, height)
        .map_err(|_| DisplayError::DeviceFailure)?;
    if framebuffer.len() < visible_size {
        return Err(DisplayError::InvalidGeometry);
    }
    let base = core::ptr::NonNull::new(framebuffer.as_mut_ptr()).ok_or(DisplayError::Empty)?;
    Ok((base, framebuffer.len()))
}

#[cfg(feature = "orays")]
impl DisplayBackend for OraysDisplay {
    fn info(&self) -> DisplayDescriptor {
        self.info
    }

    fn refresh_info(&mut self) -> Result<DisplayDescriptor, DisplayError> {
        use virtio_drivers::transport::InterruptStatus;

        let interrupt = self.inner.ack_interrupt();
        self.refresh_count = self.refresh_count.wrapping_add(1);
        let periodic_poll = self.refresh_count % 32 == 0;
        if !periodic_poll && !interrupt.contains(InterruptStatus::DEVICE_CONFIGURATION_INTERRUPT) {
            return Ok(self.info);
        }

        let (width, height) = self
            .inner
            .resolution()
            .map_err(|_| DisplayError::DeviceFailure)?;
        if width != self.info.width || height != self.info.height {
            self.reconfigure(width, height)?;
        }
        Ok(self.info)
    }

    fn present(&mut self, surface: &Surface, damage: &[Rect]) -> Result<(), DisplayError> {
        if damage.is_empty() {
            return Ok(());
        }
        if surface.width() != self.info.width || surface.height() != self.info.height {
            return Err(DisplayError::InvalidGeometry);
        }
        // SAFETY: `framebuffer` and `framebuffer_len` come from the slice
        // returned by this instance's `VirtIOGpu::change_resolution`. The DMA
        // allocation remains owned by `self.inner`; `reconfigure` replaces the
        // pointer when that allocation changes, and invalidates the `Option`
        // before a fallible reconfiguration. `&mut self` prevents concurrent
        // framebuffer access.
        let framebuffer_base = self.framebuffer.ok_or(DisplayError::DeviceFailure)?;
        let framebuffer = unsafe {
            core::slice::from_raw_parts_mut(framebuffer_base.as_ptr(), self.framebuffer_len)
        };
        surface.copy_bgra8888_regions_to(framebuffer, self.info.stride_bytes, damage)?;
        self.inner
            .flush()
            .map_err(|_| DisplayError::DeviceFailure)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{DisplayError, checked_visible_size};

    #[test]
    fn virtio_framebuffer_size_is_checked_before_device_reconfiguration() {
        assert_eq!(checked_visible_size(900, 650), Ok(900 * 650 * 4));
        assert_eq!(checked_visible_size(0, 650), Err(DisplayError::Empty));
        assert_eq!(
            checked_visible_size(u32::MAX, 2),
            Err(DisplayError::InvalidGeometry)
        );
    }
}

#[cfg(all(feature = "orays", feature = "bus-mmio"))]
type GpuTransport = virtio_drivers::transport::mmio::MmioTransport<'static>;

#[cfg(all(feature = "orays", feature = "bus-pci"))]
type GpuTransport = virtio_drivers::transport::pci::PciTransport;

#[cfg(feature = "orays")]
type OraysGpu =
    virtio_drivers::device::gpu::VirtIOGpu<crate::platform::virtio::DesktopVirtIoHal, GpuTransport>;

#[cfg(feature = "orays")]
static GPU_TRANSPORT: kspin::SpinNoIrq<Option<GpuTransport>> = kspin::SpinNoIrq::new(None);

#[cfg(feature = "orays")]
fn register_gpu_transport(transport: GpuTransport) -> bool {
    let mut slot = GPU_TRANSPORT.lock();
    if crate::platform::store_once(&mut slot, transport) {
        axstd::println!("ORAYS_DESKTOP_DISPLAY_DEVICE registered");
        true
    } else {
        axstd::println!("ORAYS_DESKTOP_DISPLAY_DEVICE ignored reason=already_registered");
        false
    }
}

#[cfg(feature = "orays")]
fn probe_gpu_transport() -> Result<GpuTransport, DisplayError> {
    GPU_TRANSPORT
        .lock()
        .take()
        .ok_or(DisplayError::DeviceUnavailable)
}

#[cfg(all(feature = "orays", feature = "bus-mmio"))]
pub(crate) fn register_orays_display_mmio(
    transport: virtio_drivers::transport::mmio::MmioTransport<'static>,
) -> bool {
    register_gpu_transport(transport)
}

#[cfg(all(feature = "orays", feature = "bus-pci"))]
pub(crate) fn register_orays_display_pci(
    root: &mut axdriver_pci::PciRoot<axdriver_pci::MmioCam<'static>>,
    bdf: axdriver_pci::DeviceFunction,
) -> bool {
    use virtio_drivers::transport::pci::PciTransport;

    crate::platform::claim_initialized(
        PciTransport::new::<crate::platform::virtio::DesktopVirtIoHal, _>(root, bdf),
        register_gpu_transport,
        |error| {
            axstd::println!(
                "ORAYS_DESKTOP_DISPLAY_DEVICE transport_failed bus=pci error={error:?}"
            );
        },
    )
}
