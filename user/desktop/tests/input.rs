use orays_desktop::graphics::geometry::Point;
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

#[test]
fn bounded_queue_never_evicts_a_release_for_non_release_input() {
    let position = orays_desktop::graphics::geometry::Point::new(0, 0);
    let release = InputEvent::PointerButton {
        button: PointerButton::Left,
        state: KeyState::Released,
        position,
    };
    let mut queue = InputQueue::<2>::new();
    queue.push(release);
    queue.push(InputEvent::Scroll { lines: 1, position });
    queue.push(InputEvent::Scroll { lines: 2, position });

    assert_eq!(queue.dropped(), 1);
    assert_eq!(queue.pop(), Some(release));
    assert!(matches!(
        queue.pop(),
        Some(InputEvent::Scroll { lines: 2, .. })
    ));
}

#[test]
fn incoming_release_evicts_an_older_non_release_event() {
    let position = orays_desktop::graphics::geometry::Point::new(0, 0);
    let mut queue = InputQueue::<2>::new();
    queue.push(InputEvent::Scroll { lines: 1, position });
    queue.push(InputEvent::Scroll { lines: 2, position });
    queue.push(InputEvent::Key {
        code: 30,
        state: KeyState::Released,
        modifiers: Modifiers::default(),
        text: None,
    });

    assert_eq!(queue.dropped(), 1);
    assert!(matches!(
        queue.pop(),
        Some(InputEvent::Scroll { lines: 2, .. })
    ));
    assert!(matches!(
        queue.pop(),
        Some(InputEvent::Key {
            code: 30,
            state: KeyState::Released,
            ..
        })
    ));
}

#[test]
fn full_release_queue_coalesces_the_same_release_identity() {
    let old_position = orays_desktop::graphics::geometry::Point::new(1, 1);
    let new_position = orays_desktop::graphics::geometry::Point::new(9, 7);
    let mut queue = InputQueue::<2>::new();
    queue.push(InputEvent::PointerButton {
        button: PointerButton::Left,
        state: KeyState::Released,
        position: old_position,
    });
    queue.push(InputEvent::Key {
        code: 30,
        state: KeyState::Released,
        modifiers: Modifiers::default(),
        text: None,
    });
    queue.push(InputEvent::PointerButton {
        button: PointerButton::Left,
        state: KeyState::Released,
        position: new_position,
    });

    assert_eq!(queue.dropped(), 0);
    assert_eq!(
        queue.pop(),
        Some(InputEvent::PointerButton {
            button: PointerButton::Left,
            state: KeyState::Released,
            position: new_position,
        })
    );
    assert!(matches!(
        queue.pop(),
        Some(InputEvent::Key {
            code: 30,
            state: KeyState::Released,
            ..
        })
    ));
}

#[test]
fn distinct_release_overflow_emits_state_reset_before_new_release() {
    let position = orays_desktop::graphics::geometry::Point::new(4, 5);
    let mut queue = InputQueue::<2>::new();
    queue.push(InputEvent::Key {
        code: 30,
        state: KeyState::Released,
        modifiers: Modifiers::default(),
        text: None,
    });
    queue.push(InputEvent::PointerButton {
        button: PointerButton::Right,
        state: KeyState::Released,
        position,
    });
    queue.push(InputEvent::PointerButton {
        button: PointerButton::Left,
        state: KeyState::Released,
        position,
    });

    assert_eq!(queue.dropped(), 2);
    assert_eq!(queue.pop(), Some(InputEvent::StateReset));
    assert_eq!(
        queue.pop(),
        Some(InputEvent::PointerButton {
            button: PointerButton::Left,
            state: KeyState::Released,
            position,
        })
    );
}

#[test]
fn state_reset_clears_translator_modifiers() {
    let mut input = InputTranslator::<2>::new(100, 80);
    input.feed(raw(EV_KEY, 42, 1));
    assert!(matches!(
        input.pop(),
        Some(InputEvent::Key {
            state: KeyState::Pressed,
            modifiers: Modifiers { shift: true, .. },
            ..
        })
    ));
    input.feed(raw(EV_KEY, 30, 0));
    input.feed(raw(EV_KEY, BTN_RIGHT, 0));
    input.feed(raw(EV_KEY, BTN_LEFT, 0));
    assert_eq!(input.pop(), Some(InputEvent::StateReset));
    input.pop();
    input.feed(raw(EV_KEY, 48, 1));
    assert!(matches!(
        input.pop(),
        Some(InputEvent::Key {
            modifiers: Modifiers { shift: false, .. },
            text: Some('b'),
            ..
        })
    ));
}

#[test]
fn pending_state_reset_cannot_be_evicted_before_consumption() {
    let position = Point::new(9, 11);
    let mut queue = InputQueue::<2>::new();
    queue.push(InputEvent::Key {
        code: 30,
        state: KeyState::Released,
        modifiers: Modifiers::default(),
        text: None,
    });
    queue.push(InputEvent::PointerButton {
        button: PointerButton::Right,
        state: KeyState::Released,
        position,
    });
    assert!(queue.push(InputEvent::PointerButton {
        button: PointerButton::Left,
        state: KeyState::Released,
        position,
    }));

    queue.push(InputEvent::PointerMoved {
        position: Point::new(20, 21),
        delta_x: 11,
        delta_y: 10,
    });
    assert_eq!(queue.pop(), Some(InputEvent::StateReset));
    assert_eq!(
        queue.pop(),
        Some(InputEvent::PointerButton {
            button: PointerButton::Left,
            state: KeyState::Released,
            position,
        })
    );
}

#[test]
fn repeated_resync_does_not_count_the_internal_reset_as_dropped_input() {
    let position = Point::new(4, 5);
    let mut queue = InputQueue::<2>::new();
    queue.push(InputEvent::Key {
        code: 30,
        state: KeyState::Released,
        modifiers: Modifiers::default(),
        text: None,
    });
    queue.push(InputEvent::PointerButton {
        button: PointerButton::Right,
        state: KeyState::Released,
        position,
    });
    assert!(queue.push(InputEvent::PointerButton {
        button: PointerButton::Left,
        state: KeyState::Released,
        position,
    }));
    assert!(queue.push(InputEvent::PointerButton {
        button: PointerButton::Middle,
        state: KeyState::Released,
        position,
    }));

    assert_eq!(queue.dropped(), 3);
    assert_eq!(queue.pop(), Some(InputEvent::StateReset));
    assert_eq!(
        queue.pop(),
        Some(InputEvent::PointerButton {
            button: PointerButton::Middle,
            state: KeyState::Released,
            position,
        })
    );
}

#[test]
fn zero_capacity_queue_drops_each_input_without_panicking() {
    let mut queue = InputQueue::<0>::new();
    assert!(!queue.push(InputEvent::StateReset));
    assert!(!queue.push(InputEvent::Scroll {
        lines: 1,
        position: Point::new(0, 0),
    }));
    assert_eq!(queue.dropped(), 2);
    assert_eq!(queue.len(), 0);
    assert_eq!(queue.pop(), None);
}

#[test]
fn one_capacity_release_overflow_keeps_reset_and_counts_every_lost_input() {
    let position = Point::new(4, 5);
    let mut queue = InputQueue::<1>::new();
    assert!(!queue.push(InputEvent::Key {
        code: 30,
        state: KeyState::Released,
        modifiers: Modifiers::default(),
        text: None,
    }));
    assert!(queue.push(InputEvent::PointerButton {
        button: PointerButton::Left,
        state: KeyState::Released,
        position,
    }));
    assert_eq!(queue.dropped(), 2);
    assert_eq!(queue.pop(), Some(InputEvent::StateReset));
    assert_eq!(queue.pop(), None);
}

#[test]
fn one_capacity_pending_reset_counts_each_additional_release() {
    let position = Point::new(4, 5);
    let mut queue = InputQueue::<1>::new();
    queue.push(InputEvent::Key {
        code: 30,
        state: KeyState::Released,
        modifiers: Modifiers::default(),
        text: None,
    });
    queue.push(InputEvent::PointerButton {
        button: PointerButton::Left,
        state: KeyState::Released,
        position,
    });
    assert!(queue.push(InputEvent::PointerButton {
        button: PointerButton::Right,
        state: KeyState::Released,
        position,
    }));
    assert_eq!(queue.dropped(), 3);
    assert_eq!(queue.pop(), Some(InputEvent::StateReset));
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
