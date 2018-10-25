/// Represents a key on a keyboard.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Key {
    /// A character.
    Char(char),

    /// Enter key.
    Enter,
    /// Backspace.
    Backspace,
    /// Tab.
    Tab,
    /// Escape.
    Escape,

    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,

    /// Pause/Break.
    Break,
    /// Insert.
    Insert,
    /// Delete.
    Delete,
    /// Home.
    Home,
    /// End.
    End,
    /// Page up.
    PageUp,
    /// Page down.
    PageDown,

    /// Function 0.
    F0,
    /// Function 1.
    F1,
    /// Function 2.
    F2,
    /// Function 3.
    F3,
    /// Function 4.
    F4,
    /// Function 5.
    F5,
    /// Function 6.
    F6,
    /// Function 7.
    F7,
    /// Function 8.
    F8,
    /// Function 9.
    F9,
    /// Function 10.
    F10,
    /// Function 11.
    F11,
    /// Function 12.
    F12,
    /// Function 13.
    F13,
    /// Function 14.
    F14,
    /// Function 15.
    F15,
}

bitflags! {
    /// Represents modifier keys pressed during a key event.
    pub struct Modifier: u8 {
        /// No modifiers.
        const None = 0b000;

        /// Ctrl.
        const Ctrl = 0b001;
        /// Shift.
        const Shift = 0b010;
        /// Alt.
        const Alt = 0b100;
    }
}
