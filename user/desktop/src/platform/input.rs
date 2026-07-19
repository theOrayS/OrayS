use crate::graphics::geometry::Point;

pub const EV_SYN: u16 = 0;
pub const EV_KEY: u16 = 1;
pub const EV_REL: u16 = 2;
pub const EV_ABS: u16 = 3;

pub const SYN_REPORT: u16 = 0;
pub const REL_X: u16 = 0;
pub const REL_Y: u16 = 1;
pub const REL_WHEEL: u16 = 8;
pub const ABS_X: u16 = 0;
pub const ABS_Y: u16 = 1;

pub const BTN_LEFT: u16 = 0x110;
pub const BTN_RIGHT: u16 = 0x111;
pub const BTN_MIDDLE: u16 = 0x112;
pub const KEY_TAB: u16 = 15;

const KEY_LEFT_CTRL: u16 = 29;
const KEY_LEFT_SHIFT: u16 = 42;
const KEY_LEFT_ALT: u16 = 56;
const KEY_RIGHT_SHIFT: u16 = 54;
const KEY_RIGHT_CTRL: u16 = 97;
const KEY_RIGHT_ALT: u16 = 100;
const KEY_LEFT_META: u16 = 125;
const KEY_RIGHT_META: u16 = 126;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawInputEvent {
    pub event_type: u16,
    pub code: u16,
    pub value: i32,
    pub minimum: Option<u32>,
    pub maximum: Option<u32>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub super_key: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Released,
    Pressed,
    Repeated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    Key {
        code: u16,
        state: KeyState,
        modifiers: Modifiers,
        text: Option<char>,
    },
    PointerMoved {
        position: Point,
        delta_x: i32,
        delta_y: i32,
    },
    PointerButton {
        button: PointerButton,
        state: KeyState,
        position: Point,
    },
    Scroll {
        lines: i32,
        position: Point,
    },
}

pub struct InputQueue<const CAPACITY: usize> {
    events: [Option<InputEvent>; CAPACITY],
    head: usize,
    len: usize,
    dropped: u64,
}

impl<const CAPACITY: usize> InputQueue<CAPACITY> {
    pub const fn new() -> Self {
        Self {
            events: [None; CAPACITY],
            head: 0,
            len: 0,
            dropped: 0,
        }
    }

    pub fn push(&mut self, event: InputEvent) {
        if CAPACITY == 0 {
            self.dropped = self.dropped.saturating_add(1);
            return;
        }
        if self.len == CAPACITY {
            self.events[self.head] = None;
            self.head = (self.head + 1) % CAPACITY;
            self.len -= 1;
            self.dropped = self.dropped.saturating_add(1);
        }
        let tail = (self.head + self.len) % CAPACITY;
        self.events[tail] = Some(event);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<InputEvent> {
        if self.len == 0 || CAPACITY == 0 {
            return None;
        }
        let event = self.events[self.head].take();
        self.head = (self.head + 1) % CAPACITY;
        self.len -= 1;
        event
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn dropped(&self) -> u64 {
        self.dropped
    }
}

impl<const CAPACITY: usize> Default for InputQueue<CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct InputTranslator<const CAPACITY: usize> {
    queue: InputQueue<CAPACITY>,
    modifiers: Modifiers,
    modifier_mask: u8,
    pointer: Point,
    width: u32,
    height: u32,
    pending_x: Option<i32>,
    pending_y: Option<i32>,
    relative_x: i32,
    relative_y: i32,
    scroll: i32,
}

impl<const CAPACITY: usize> InputTranslator<CAPACITY> {
    pub const fn new(width: u32, height: u32) -> Self {
        Self {
            queue: InputQueue::new(),
            modifiers: Modifiers {
                shift: false,
                control: false,
                alt: false,
                super_key: false,
            },
            modifier_mask: 0,
            pointer: Point::new(0, 0),
            width,
            height,
            pending_x: None,
            pending_y: None,
            relative_x: 0,
            relative_y: 0,
            scroll: 0,
        }
    }

    pub fn feed(&mut self, raw: RawInputEvent) {
        match (raw.event_type, raw.code) {
            (EV_SYN, SYN_REPORT) => self.flush_pointer(),
            (EV_REL, REL_X) => self.relative_x = self.relative_x.saturating_add(raw.value),
            (EV_REL, REL_Y) => self.relative_y = self.relative_y.saturating_add(raw.value),
            (EV_REL, REL_WHEEL) => self.scroll = self.scroll.saturating_add(raw.value),
            (EV_ABS, ABS_X) => {
                self.pending_x = normalize_axis(raw, self.width);
            }
            (EV_ABS, ABS_Y) => {
                self.pending_y = normalize_axis(raw, self.height);
            }
            (EV_KEY, BTN_LEFT | BTN_RIGHT | BTN_MIDDLE) => self.push_pointer_button(raw),
            (EV_KEY, _) => self.push_key(raw),
            _ => {}
        }
    }

    pub fn pop(&mut self) -> Option<InputEvent> {
        self.queue.pop()
    }

    pub const fn pointer(&self) -> Point {
        self.pointer
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.pointer.x = self.pointer.x.clamp(0, width.saturating_sub(1) as i32);
        self.pointer.y = self.pointer.y.clamp(0, height.saturating_sub(1) as i32);
        self.pending_x = None;
        self.pending_y = None;
        self.relative_x = 0;
        self.relative_y = 0;
    }

    pub const fn dropped(&self) -> u64 {
        self.queue.dropped()
    }

    fn push_key(&mut self, raw: RawInputEvent) {
        let Some(state) = key_state(raw.value) else {
            return;
        };
        let pressed = state != KeyState::Released;
        if let Some(bit) = modifier_bit(raw.code) {
            if pressed {
                self.modifier_mask |= bit;
            } else {
                self.modifier_mask &= !bit;
            }
            self.modifiers.shift = self.modifier_mask & 0b0000_0011 != 0;
            self.modifiers.control = self.modifier_mask & 0b0000_1100 != 0;
            self.modifiers.alt = self.modifier_mask & 0b0011_0000 != 0;
            self.modifiers.super_key = self.modifier_mask & 0b1100_0000 != 0;
        }
        let text = if pressed && !self.modifiers.control && !self.modifiers.alt {
            key_text(raw.code, self.modifiers.shift)
        } else {
            None
        };
        self.queue.push(InputEvent::Key {
            code: raw.code,
            state,
            modifiers: self.modifiers,
            text,
        });
    }

    fn push_pointer_button(&mut self, raw: RawInputEvent) {
        let Some(state) = key_state(raw.value) else {
            return;
        };
        self.flush_pointer();
        let button = match raw.code {
            BTN_LEFT => PointerButton::Left,
            BTN_RIGHT => PointerButton::Right,
            _ => PointerButton::Middle,
        };
        self.queue.push(InputEvent::PointerButton {
            button,
            state,
            position: self.pointer,
        });
    }

    fn flush_pointer(&mut self) {
        let old = self.pointer;
        let max_x = self.width.saturating_sub(1) as i32;
        let max_y = self.height.saturating_sub(1) as i32;
        let x = self
            .pending_x
            .take()
            .unwrap_or_else(|| old.x.saturating_add(self.relative_x));
        let y = self
            .pending_y
            .take()
            .unwrap_or_else(|| old.y.saturating_add(self.relative_y));
        self.pointer = Point::new(x.clamp(0, max_x), y.clamp(0, max_y));
        let delta_x = self.pointer.x - old.x;
        let delta_y = self.pointer.y - old.y;
        if delta_x != 0 || delta_y != 0 {
            self.queue.push(InputEvent::PointerMoved {
                position: self.pointer,
                delta_x,
                delta_y,
            });
        }
        if self.scroll != 0 {
            self.queue.push(InputEvent::Scroll {
                lines: self.scroll,
                position: self.pointer,
            });
        }
        self.relative_x = 0;
        self.relative_y = 0;
        self.scroll = 0;
    }
}

fn key_state(value: i32) -> Option<KeyState> {
    match value {
        0 => Some(KeyState::Released),
        1 => Some(KeyState::Pressed),
        2 => Some(KeyState::Repeated),
        _ => None,
    }
}

fn modifier_bit(code: u16) -> Option<u8> {
    Some(match code {
        KEY_LEFT_SHIFT => 1 << 0,
        KEY_RIGHT_SHIFT => 1 << 1,
        KEY_LEFT_CTRL => 1 << 2,
        KEY_RIGHT_CTRL => 1 << 3,
        KEY_LEFT_ALT => 1 << 4,
        KEY_RIGHT_ALT => 1 << 5,
        KEY_LEFT_META => 1 << 6,
        KEY_RIGHT_META => 1 << 7,
        _ => return None,
    })
}

fn normalize_axis(raw: RawInputEvent, extent: u32) -> Option<i32> {
    let (Some(minimum), Some(maximum)) = (raw.minimum, raw.maximum) else {
        return None;
    };
    if extent == 0 || maximum <= minimum {
        return None;
    }
    let value = (raw.value as i64).clamp(minimum as i64, maximum as i64) - minimum as i64;
    let span = (maximum - minimum) as i64;
    Some((value * extent.saturating_sub(1) as i64 / span) as i32)
}

fn key_text(code: u16, shift: bool) -> Option<char> {
    let (normal, shifted) = match code {
        2 => ('1', '!'),
        3 => ('2', '@'),
        4 => ('3', '#'),
        5 => ('4', '$'),
        6 => ('5', '%'),
        7 => ('6', '^'),
        8 => ('7', '&'),
        9 => ('8', '*'),
        10 => ('9', '('),
        11 => ('0', ')'),
        12 => ('-', '_'),
        13 => ('=', '+'),
        15 => ('\t', '\t'),
        16 => ('q', 'Q'),
        17 => ('w', 'W'),
        18 => ('e', 'E'),
        19 => ('r', 'R'),
        20 => ('t', 'T'),
        21 => ('y', 'Y'),
        22 => ('u', 'U'),
        23 => ('i', 'I'),
        24 => ('o', 'O'),
        25 => ('p', 'P'),
        26 => ('[', '{'),
        27 => (']', '}'),
        28 => ('\n', '\n'),
        30 => ('a', 'A'),
        31 => ('s', 'S'),
        32 => ('d', 'D'),
        33 => ('f', 'F'),
        34 => ('g', 'G'),
        35 => ('h', 'H'),
        36 => ('j', 'J'),
        37 => ('k', 'K'),
        38 => ('l', 'L'),
        39 => (';', ':'),
        40 => ('\'', '"'),
        41 => ('`', '~'),
        43 => ('\\', '|'),
        44 => ('z', 'Z'),
        45 => ('x', 'X'),
        46 => ('c', 'C'),
        47 => ('v', 'V'),
        48 => ('b', 'B'),
        49 => ('n', 'N'),
        50 => ('m', 'M'),
        51 => (',', '<'),
        52 => ('.', '>'),
        53 => ('/', '?'),
        57 => (' ', ' '),
        _ => return None,
    };
    Some(if shift { shifted } else { normal })
}

#[cfg(feature = "orays")]
pub fn poll_orays_raw() -> Option<RawInputEvent> {
    orays::poll()
}

#[cfg(feature = "orays")]
pub fn initialize_orays_input() {
    orays::initialize()
}

#[cfg(all(feature = "orays", feature = "bus-mmio"))]
pub(crate) fn register_orays_input_mmio(
    transport: virtio_drivers::transport::mmio::MmioTransport<'static>,
) -> bool {
    orays::register_transport(transport, "mmio")
}

#[cfg(all(feature = "orays", feature = "bus-pci"))]
pub(crate) fn register_orays_input_pci(
    root: &mut axdriver_pci::PciRoot<axdriver_pci::MmioCam<'static>>,
    bdf: axdriver_pci::DeviceFunction,
) -> bool {
    orays::register_pci(root, bdf)
}

#[cfg(feature = "orays")]
mod orays {
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(feature = "bus-pci")]
    use axdriver_pci::{DeviceFunction, MmioCam, PciRoot};
    use kspin::SpinNoIrq;
    use virtio_drivers::device::input::{AbsInfo, InputEvent, VirtIOInput};

    use super::RawInputEvent;
    use crate::platform::virtio::DesktopVirtIoHal;

    #[cfg(feature = "bus-mmio")]
    type InputTransport = virtio_drivers::transport::mmio::MmioTransport<'static>;
    #[cfg(feature = "bus-pci")]
    type InputTransport = virtio_drivers::transport::pci::PciTransport;
    type InputDriver = VirtIOInput<DesktopVirtIoHal, InputTransport>;

    const MAX_INPUT_DEVICES: usize = 4;

    struct InputDevice {
        inner: InputDriver,
        abs_x: Option<AbsInfo>,
        abs_y: Option<AbsInfo>,
    }

    impl InputDevice {
        fn new(mut inner: InputDriver) -> Self {
            let abs_x = inner.abs_info(0).ok();
            let abs_y = inner.abs_info(1).ok();
            let _ = inner.ack_interrupt();
            Self {
                inner,
                abs_x,
                abs_y,
            }
        }

        fn pop(&mut self) -> Option<RawInputEvent> {
            let _ = self.inner.ack_interrupt();
            let InputEvent {
                event_type,
                code,
                value,
            } = self.inner.pop_pending_event()?;
            let limits = match (event_type, code) {
                (3, 0) => self.abs_x.as_ref(),
                (3, 1) => self.abs_y.as_ref(),
                _ => None,
            };
            Some(RawInputEvent {
                event_type,
                code,
                value: value as i32,
                minimum: limits.map(|info| info.min),
                maximum: limits.map(|info| info.max),
            })
        }
    }

    struct InputRegistry {
        devices: [Option<InputDevice>; MAX_INPUT_DEVICES],
        reservations: usize,
        next: usize,
    }

    impl InputRegistry {
        const fn new() -> Self {
            Self {
                devices: [const { None }; MAX_INPUT_DEVICES],
                reservations: 0,
                next: 0,
            }
        }
    }

    static INITIALIZED: AtomicBool = AtomicBool::new(false);
    static INPUTS: SpinNoIrq<InputRegistry> = SpinNoIrq::new(InputRegistry::new());

    fn reserve_registration() -> bool {
        let mut inputs = INPUTS.lock();
        let InputRegistry {
            devices,
            reservations,
            ..
        } = &mut *inputs;
        if crate::platform::reserve_first_empty(devices, reservations) {
            true
        } else {
            axstd::println!("ORAYS_DESKTOP_INPUT_DEVICE ignored capacity={MAX_INPUT_DEVICES}");
            false
        }
    }

    fn cancel_registration() {
        let cancelled = crate::platform::cancel_reservation(&mut INPUTS.lock().reservations);
        debug_assert!(
            cancelled,
            "input reservation must exist before cancellation"
        );
    }

    fn register_reserved(driver: InputDriver) -> bool {
        let device = InputDevice::new(driver);
        let mut inputs = INPUTS.lock();
        let InputRegistry {
            devices,
            reservations,
            ..
        } = &mut *inputs;
        if crate::platform::commit_reserved(devices, reservations, device) {
            axstd::println!("ORAYS_DESKTOP_INPUT_DEVICE registered");
            true
        } else {
            axstd::println!("ORAYS_DESKTOP_INPUT_DEVICE registration_invariant_failed");
            false
        }
    }

    fn initialize_reserved(transport: InputTransport, bus: &str) -> bool {
        match InputDriver::new(transport) {
            Ok(driver) => register_reserved(driver),
            Err(error) => {
                cancel_registration();
                axstd::println!("ORAYS_DESKTOP_INPUT_DEVICE init_failed bus={bus} error={error:?}");
                false
            }
        }
    }

    #[cfg(feature = "bus-mmio")]
    pub fn register_transport(transport: InputTransport, bus: &str) -> bool {
        reserve_registration() && initialize_reserved(transport, bus)
    }

    pub fn poll() -> Option<RawInputEvent> {
        initialize();
        let mut inputs = INPUTS.lock();
        for offset in 0..MAX_INPUT_DEVICES {
            let index = (inputs.next + offset) % MAX_INPUT_DEVICES;
            if let Some(event) = inputs.devices[index].as_mut().and_then(InputDevice::pop) {
                inputs.next = (index + 1) % MAX_INPUT_DEVICES;
                return Some(event);
            }
        }
        None
    }

    pub fn initialize() {
        if INITIALIZED.swap(true, Ordering::AcqRel) {
            return;
        }
        let device_count = registered_count();
        axstd::println!("ORAYS_DESKTOP_INPUT_READY devices={device_count}");
    }

    fn registered_count() -> usize {
        INPUTS
            .lock()
            .devices
            .iter()
            .filter(|device| device.is_some())
            .count()
    }

    #[cfg(feature = "bus-pci")]
    pub fn register_pci(root: &mut PciRoot<MmioCam<'static>>, bdf: DeviceFunction) -> bool {
        use virtio_drivers::transport::pci::PciTransport;

        if !reserve_registration() {
            return false;
        }
        match PciTransport::new::<DesktopVirtIoHal, _>(root, bdf) {
            Ok(transport) => initialize_reserved(transport, "pci"),
            Err(error) => {
                cancel_registration();
                axstd::println!(
                    "ORAYS_DESKTOP_INPUT_DEVICE transport_failed bus=pci error={error:?}"
                );
                false
            }
        }
    }
}
