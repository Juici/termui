/// Represents a mouse event.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MouseEvent {
    /// Button press.
    Press(MouseButton),
    /// Button release.
    Release(MouseButton),
    /// Mouse drag.
    Hold(MouseButton),
    /// Scroll up.
    WheelUp,
    /// Scroll down.
    WheelDown,
}

impl MouseEvent {
    /// Gets the mouse button pressed during event.
    pub fn button(&self) -> Option<MouseButton> {
        match *self {
            MouseEvent::Press(btn) | MouseEvent::Release(btn) | MouseEvent::Hold(btn) => Some(btn),
            _ => None,
        }
    }
}

/// Represents a button on a mouse.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MouseButton {
    /// Left click.
    Left,
    /// Middle click.
    Middle,
    /// Right click.
    Right,

    /// Button 4.
    Button4,
    /// Button 5.
    Button5,

    #[doc(hidden)]
    Other,
}
