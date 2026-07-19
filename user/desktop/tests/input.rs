use orays_desktop::platform::input::*;

fn raw(event_type: u16, code: u16, value: i32) -> RawInputEvent {
    RawInputEvent {
        event_type,
        code,
        value,
        minimum: None,
        maximum: None,
    }
}

#[test]
fn shift_updates_modifiers_and_text_on_press_and_release() {
    let mut input = InputTranslator::<16>::new(1280, 720);
    input.feed(raw(EV_KEY, 42, 1));
    input.feed(raw(EV_KEY, 30, 1));
    input.feed(raw(EV_KEY, 30, 0));
    input.feed(raw(EV_KEY, 42, 0));

    assert!(matches!(
        input.pop(),
        Some(InputEvent::Key {
            code: 42,
            state: KeyState::Pressed,
            modifiers: Modifiers { shift: true, .. },
            text: None,
        })
    ));
    assert!(matches!(
        input.pop(),
        Some(InputEvent::Key {
            code: 30,
            state: KeyState::Pressed,
            text: Some('A'),
            ..
        })
    ));
    assert_eq!(input.pop().unwrap().unwrap_key_state(), KeyState::Released);
    assert_eq!(input.pop().unwrap().unwrap_key_state(), KeyState::Released);
}

#[test]
fn modifier_sides_are_independent_and_control_suppresses_text() {
    let mut input = InputTranslator::<16>::new(640, 480);
    input.feed(raw(EV_KEY, 42, 1));
    input.feed(raw(EV_KEY, 54, 1));
    input.feed(raw(EV_KEY, 42, 0));
    input.feed(raw(EV_KEY, 30, 1));
    for _ in 0..3 {
        input.pop().unwrap();
    }
    assert!(matches!(
        input.pop(),
        Some(InputEvent::Key {
            modifiers: Modifiers { shift: true, .. },
            text: Some('A'),
            ..
        })
    ));

    input.feed(raw(EV_KEY, 29, 1));
    input.feed(raw(EV_KEY, 31, 1));
    input.pop().unwrap();
    assert!(matches!(
        input.pop(),
        Some(InputEvent::Key {
            modifiers: Modifiers { control: true, .. },
            text: None,
            ..
        })
    ));
}

#[test]
fn relative_pointer_scroll_and_buttons_are_preserved() {
    let mut input = InputTranslator::<16>::new(100, 80);
    input.feed(raw(EV_REL, REL_X, 12));
    input.feed(raw(EV_REL, REL_Y, -4));
    input.feed(raw(EV_REL, REL_WHEEL, -2));
    input.feed(raw(EV_SYN, SYN_REPORT, 0));
    input.feed(raw(EV_KEY, BTN_LEFT, 1));

    assert_eq!(
        input.pop(),
        Some(InputEvent::PointerMoved {
            position: orays_desktop::graphics::geometry::Point::new(12, 0),
            delta_x: 12,
            delta_y: 0,
        })
    );
    assert!(matches!(
        input.pop(),
        Some(InputEvent::Scroll { lines: -2, .. })
    ));
    assert!(matches!(
        input.pop(),
        Some(InputEvent::PointerButton {
            button: PointerButton::Left,
            state: KeyState::Pressed,
            ..
        })
    ));
}

#[test]
fn absolute_pointer_uses_device_reported_ranges() {
    let mut input = InputTranslator::<8>::new(101, 51);
    input.feed(RawInputEvent {
        event_type: EV_ABS,
        code: ABS_X,
        value: 500,
        minimum: Some(0),
        maximum: Some(1000),
    });
    input.feed(RawInputEvent {
        event_type: EV_ABS,
        code: ABS_Y,
        value: 1000,
        minimum: Some(0),
        maximum: Some(1000),
    });
    input.feed(raw(EV_SYN, SYN_REPORT, 0));
    assert!(matches!(
        input.pop(),
        Some(InputEvent::PointerMoved {
            position: orays_desktop::graphics::geometry::Point { x: 50, y: 50 },
            ..
        })
    ));
}

#[test]
fn display_resize_clamps_pointer_and_updates_absolute_mapping() {
    let mut input = InputTranslator::<8>::new(101, 51);
    input.feed(raw(EV_REL, REL_X, 100));
    input.feed(raw(EV_REL, REL_Y, 50));
    input.feed(raw(EV_SYN, SYN_REPORT, 0));
    input.pop().unwrap();
    input.resize(41, 21);
    assert_eq!(
        input.pointer(),
        orays_desktop::graphics::geometry::Point::new(40, 20)
    );
    input.feed(RawInputEvent {
        event_type: EV_ABS,
        code: ABS_X,
        value: 500,
        minimum: Some(0),
        maximum: Some(1000),
    });
    input.feed(RawInputEvent {
        event_type: EV_ABS,
        code: ABS_Y,
        value: 500,
        minimum: Some(0),
        maximum: Some(1000),
    });
    input.feed(raw(EV_SYN, SYN_REPORT, 0));
    assert!(matches!(
        input.pop(),
        Some(InputEvent::PointerMoved {
            position: orays_desktop::graphics::geometry::Point { x: 20, y: 10 },
            ..
        })
    ));
}

#[test]
fn bounded_queue_reports_and_drops_oldest_event() {
    let mut queue = InputQueue::<2>::new();
    queue.push(InputEvent::Scroll {
        lines: 1,
        position: orays_desktop::graphics::geometry::Point::new(0, 0),
    });
    queue.push(InputEvent::Scroll {
        lines: 2,
        position: orays_desktop::graphics::geometry::Point::new(0, 0),
    });
    queue.push(InputEvent::Scroll {
        lines: 3,
        position: orays_desktop::graphics::geometry::Point::new(0, 0),
    });
    assert_eq!(queue.dropped(), 1);
    assert!(matches!(
        queue.pop(),
        Some(InputEvent::Scroll { lines: 2, .. })
    ));
    assert!(matches!(
        queue.pop(),
        Some(InputEvent::Scroll { lines: 3, .. })
    ));
}

trait KeyEventExt {
    fn unwrap_key_state(self) -> KeyState;
}

impl KeyEventExt for InputEvent {
    fn unwrap_key_state(self) -> KeyState {
        match self {
            InputEvent::Key { state, .. } => state,
            event => panic!("expected key event, got {event:?}"),
        }
    }
}
