//! Events module.

mod key;
mod mouse;

pub use self::key::{Key, Modifier};
pub use self::mouse::{MouseButton, MouseEvent};

/// Represents an event in the window.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Event {
    /// A terminal refresh event.
    Refresh,
    /// A terminal resize event.
    Resize,
    /// A key press event.
    Key {
        /// The key pressed.
        key: Key,
        /// The key press modifier.
        modifier: Modifier,
    },
    /// A mouse event.
    Mouse {
        /// The position of the mouse.
        pos: (usize, usize),
        /// The mouse event.
        event: MouseEvent,
    },
    /// An unknown event.
    Unknown(Vec<u8>),
}

impl Event {
    pub(crate) fn key(key: Key) -> Event {
        Event::Key {
            key,
            modifier: Modifier::None,
        }
    }
}
