extern crate termui;

use termui::{Event, Key, Window};

fn main() {
    const MSG: &str = "My potions are too strong for you traveller";
    const EXIT_MSG: &str = "Press Q to exit";

    let mut window = Window::new();

    loop {
        window.erase();

        let (rows, cols) = window.get_size();

        window.print(rows / 2, (cols / 2) - (MSG.len() / 2), MSG);
        window.print(rows - 1, cols - EXIT_MSG.len(), EXIT_MSG);

        window.refresh();

        match window.poll_event() {
            Some(Event::Key {
                key: Key::Char(ch), ..
            }) if ch == 'Q' || ch == 'q' => break,
            _ => continue,
        }
    }
}
