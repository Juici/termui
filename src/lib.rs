//! A small terminal ui library.

#![deny(missing_docs, warnings)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;
pub extern crate pancurses as curses;

pub mod event;
pub mod window;

#[doc(no_inline)]
pub use event::{Event, Key};
#[doc(no_inline)]
pub use window::Window;
