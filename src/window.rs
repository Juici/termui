//! Window module.

use std::collections::{HashMap, VecDeque};
use std::io::{self, Write};
use std::{env, ptr};

use curses;
use event::{Event, Key, Modifier, MouseButton, MouseEvent};

/// Represents the terminal window.
pub struct Window {
    /// The inner curses window.
    pub window: curses::Window,

    event_queue: VecDeque<Event>,
    last_mouse_button: Option<MouseButton>,
    key_codes: HashMap<i32, Event>,
}

impl Window {
    /// Creates a new window.
    pub fn new() -> Window {
        env::set_var("ESCDELAY", "25");

        let window = curses::initscr();
        window.keypad(true);
        window.nodelay(true);

        curses::noecho();
        curses::cbreak();

        curses::start_color();
        curses::use_default_colors();

        curses::curs_set(0);

        curses::mouseinterval(0);
        curses::mousemask(
            curses::ALL_MOUSE_EVENTS | curses::REPORT_MOUSE_POSITION,
            ptr::null_mut(),
        );

        print!("\x1B[?1002h");
        io::stdout().flush().expect("could not flush stdout");

        Window {
            window,

            event_queue: VecDeque::new(),
            last_mouse_button: None,
            key_codes: init_keymap(),
        }
    }

    /// Polls the window for an event.
    ///
    /// *Handles key press modifiers and mouse events.*
    pub fn poll_event(&mut self) -> Option<Event> {
        use self::curses::Input;

        let ev = self.event_queue.pop_front();
        if ev.is_some() {
            return ev;
        }

        match self.window.getch() {
            Some(input) => {
                let ev = match input {
                    Input::Character('\n') | Input::KeyEnter => Event::key(Key::Enter),
                    Input::Character('\u{7f}')
                    | Input::Character('\u{8}')
                    | Input::KeyBackspace => Event::key(Key::Backspace),
                    Input::Character('\u{9}') => Event::key(Key::Tab),
                    Input::Character('\u{1b}') => Event::key(Key::Escape),

                    Input::KeyBTab | Input::KeySTab => Event::Key {
                        key: Key::Tab,
                        modifier: Modifier::Shift,
                    },
                    Input::KeyCTab => Event::Key {
                        key: Key::Tab,
                        modifier: Modifier::Ctrl,
                    },
                    Input::KeyCATab => Event::Key {
                        key: Key::Tab,
                        modifier: Modifier::Ctrl | Modifier::Alt,
                    },

                    Input::Character(c) if (c as u32) <= 26 => Event::Key {
                        key: Key::Char((b'a' - 1 + c as u8) as char),
                        modifier: Modifier::Ctrl,
                    },
                    Input::Character(c) => Event::key(Key::Char(c)),
                    Input::Unknown(code) => self
                        .key_codes
                        .get(&(code + 256 + 48))
                        .cloned()
                        .unwrap_or_else(|| {
                            warn!("unknown key: {}", code);
                            Event::Unknown(split_i32(code))
                        }),

                    Input::KeyUp => Event::key(Key::Up),
                    Input::KeyDown => Event::key(Key::Down),
                    Input::KeyLeft => Event::key(Key::Left),
                    Input::KeyRight => Event::key(Key::Right),

                    Input::KeySR => Event::Key {
                        key: Key::Up,
                        modifier: Modifier::Shift,
                    },
                    Input::KeySF => Event::Key {
                        key: Key::Down,
                        modifier: Modifier::Shift,
                    },
                    Input::KeySLeft => Event::Key {
                        key: Key::Left,
                        modifier: Modifier::Shift,
                    },
                    Input::KeySRight => Event::Key {
                        key: Key::Right,
                        modifier: Modifier::Shift,
                    },

                    Input::KeyBreak => Event::key(Key::Break),
                    Input::KeyIC => Event::key(Key::Insert),
                    Input::KeyDC => Event::key(Key::Delete),
                    Input::KeyHome => Event::key(Key::Home),
                    Input::KeyEnd => Event::key(Key::End),
                    Input::KeyPPage => Event::key(Key::PageUp),
                    Input::KeyNPage => Event::key(Key::PageDown),

                    Input::KeySIC => Event::Key {
                        key: Key::Insert,
                        modifier: Modifier::Shift,
                    },
                    Input::KeySDC => Event::Key {
                        key: Key::Delete,
                        modifier: Modifier::Shift,
                    },
                    Input::KeySHome => Event::Key {
                        key: Key::Home,
                        modifier: Modifier::Shift,
                    },
                    Input::KeySEnd => Event::Key {
                        key: Key::End,
                        modifier: Modifier::Shift,
                    },
                    Input::KeySPrevious => Event::Key {
                        key: Key::PageUp,
                        modifier: Modifier::Shift,
                    },
                    Input::KeySNext => Event::Key {
                        key: Key::PageDown,
                        modifier: Modifier::Shift,
                    },

                    Input::KeyF0 => Event::key(Key::F0),
                    Input::KeyF1 => Event::key(Key::F1),
                    Input::KeyF2 => Event::key(Key::F2),
                    Input::KeyF3 => Event::key(Key::F3),
                    Input::KeyF4 => Event::key(Key::F4),
                    Input::KeyF5 => Event::key(Key::F5),
                    Input::KeyF6 => Event::key(Key::F6),
                    Input::KeyF7 => Event::key(Key::F7),
                    Input::KeyF8 => Event::key(Key::F8),
                    Input::KeyF9 => Event::key(Key::F9),
                    Input::KeyF10 => Event::key(Key::F10),
                    Input::KeyF11 => Event::key(Key::F11),
                    Input::KeyF12 => Event::key(Key::F12),

                    Input::KeyResize => {
                        curses::resize_term(0, 0);
                        Event::Resize
                    }

                    Input::KeyMouse => self.parse_mouse_event(),

                    _ => Event::Refresh,
                };

                Some(ev)
            }
            None => None,
        }
    }

    /// Clears the window, without refreshing.
    pub fn erase(&self) {
        self.window.erase();
    }

    /// Refreshes the window.
    pub fn refresh(&self) {
        self.window.refresh();
    }

    /// Gets the size of the window in rows and columns.
    pub fn get_size(&self) -> (usize, usize) {
        let (rows, cols) = self.window.get_max_yx();
        (rows as usize, cols as usize)
    }

    /// Prints a message to window at the given position.
    pub fn print<S: AsRef<str>>(&self, row: usize, col: usize, msg: S) {
        self.window.mvprintw(row as i32, col as i32, msg);
    }

    /// Prints a character to window at the given position.
    pub fn printch(&self, row: usize, col: usize, ch: char) {
        self.window.mvaddch(row as i32, col as i32, ch);
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        print!("\x1B[?1002l");
        io::stdout().flush().expect("could not flush stdout");

        curses::endwin();
    }
}

impl Window {
    fn parse_mouse_event(&mut self) -> Event {
        let mut mevent = match curses::getmouse() {
            Ok(event) => event,
            Err(code) => return Event::Unknown(split_i32(code)),
        };

        let _ctrl = (mevent.bstate & curses::BUTTON_CTRL) != 0;
        let _shift = (mevent.bstate & curses::BUTTON_SHIFT) != 0;
        let _alt = (mevent.bstate & curses::BUTTON_ALT) != 0;

        mevent.bstate &= !(curses::BUTTON_CTRL | curses::BUTTON_SHIFT | curses::BUTTON_ALT);

        let make_event = |event| Event::Mouse {
            pos: (mevent.x as usize, mevent.y as usize),
            event,
        };

        if mevent.bstate == curses::REPORT_MOUSE_POSITION {
            self.last_mouse_button
                .map(MouseEvent::Hold)
                .map(&make_event)
                .unwrap_or_else(|| {
                    debug!("received a mouse drag, but not last mouse button");
                    Event::Unknown(Vec::new())
                })
        } else {
            let mut bare_event = mevent.bstate & ((1 << 25) - 1);

            let mut event = None;
            while bare_event != 0 {
                let single_event = 1 << bare_event.trailing_zeros();
                bare_event ^= single_event;

                on_mouse_event(single_event, |e| {
                    if event.is_none() {
                        event = Some(e);
                    } else {
                        self.event_queue.push_back(make_event(e));
                    }
                });
            }

            match event {
                Some(event) => {
                    if let Some(button) = event.button() {
                        self.last_mouse_button = Some(button);
                    }
                    make_event(event)
                }
                None => {
                    debug!("no event parsed");
                    Event::Unknown(Vec::new())
                }
            }
        }
    }
}

fn init_keymap() -> HashMap<i32, Event> {
    let mut map = HashMap::new();

    let key_names = {
        let mut key_names = HashMap::new();

        key_names.insert("DC", Key::Delete);
        key_names.insert("DN", Key::Down);
        key_names.insert("END", Key::End);
        key_names.insert("HOM", Key::Home);
        key_names.insert("IC", Key::Insert);
        key_names.insert("LFT", Key::Left);
        key_names.insert("NXT", Key::PageDown);
        key_names.insert("PRV", Key::PageUp);
        key_names.insert("RIT", Key::Right);
        key_names.insert("UP", Key::Up);

        key_names
    };

    for code in 512..1024 {
        let name = match curses::keyname(code) {
            Some(name) => name,
            None => continue,
        };

        if !name.starts_with('k') {
            continue;
        }

        let (key_name, modifier) = name[1..].split_at(name.len() - 2);
        let key = match key_names.get(key_name) {
            Some(&key) => key,
            None => continue,
        };
        let modifier = match modifier {
            "3" => Modifier::Alt,
            "4" => Modifier::Shift | Modifier::Alt,
            "5" => Modifier::Ctrl,
            "6" => Modifier::Ctrl | Modifier::Shift,
            "7" => Modifier::Ctrl | Modifier::Shift | Modifier::Alt,
            _ => continue,
        };

        map.insert(code, Event::Key { key, modifier });
    }

    map
}

fn split_i32(code: i32) -> Vec<u8> {
    (0..4).map(|i| ((code >> (8 * i)) & 0xFF) as u8).collect()
}

fn on_mouse_event<F: FnMut(MouseEvent)>(bare_event: curses::mmask_t, mut f: F) {
    let button = get_mouse_button(bare_event);
    match bare_event {
        curses::BUTTON4_PRESSED => f(MouseEvent::WheelUp),
        curses::BUTTON5_PRESSED => f(MouseEvent::WheelDown),
        curses::BUTTON1_RELEASED
        | curses::BUTTON2_RELEASED
        | curses::BUTTON3_RELEASED
        | curses::BUTTON4_RELEASED
        | curses::BUTTON5_RELEASED => f(MouseEvent::Release(button)),
        curses::BUTTON1_PRESSED | curses::BUTTON2_PRESSED | curses::BUTTON3_PRESSED => {
            f(MouseEvent::Press(button))
        }
        curses::BUTTON1_CLICKED
        | curses::BUTTON2_CLICKED
        | curses::BUTTON3_CLICKED
        | curses::BUTTON4_CLICKED
        | curses::BUTTON5_CLICKED => {
            f(MouseEvent::Press(button));
            f(MouseEvent::Release(button));
        }
        curses::BUTTON1_DOUBLE_CLICKED
        | curses::BUTTON2_DOUBLE_CLICKED
        | curses::BUTTON3_DOUBLE_CLICKED
        | curses::BUTTON4_DOUBLE_CLICKED
        | curses::BUTTON5_DOUBLE_CLICKED => {
            for _ in 0..2 {
                f(MouseEvent::Press(button));
                f(MouseEvent::Release(button));
            }
        }
        curses::BUTTON1_TRIPLE_CLICKED
        | curses::BUTTON2_TRIPLE_CLICKED
        | curses::BUTTON3_TRIPLE_CLICKED
        | curses::BUTTON4_TRIPLE_CLICKED
        | curses::BUTTON5_TRIPLE_CLICKED => {
            for _ in 0..3 {
                f(MouseEvent::Press(button));
                f(MouseEvent::Release(button));
            }
        }
        _ => debug!("unknown event: {:032b}", bare_event),
    }
}

fn get_mouse_button(bare_event: curses::mmask_t) -> MouseButton {
    match bare_event {
        curses::BUTTON1_RELEASED
        | curses::BUTTON1_PRESSED
        | curses::BUTTON1_CLICKED
        | curses::BUTTON1_DOUBLE_CLICKED
        | curses::BUTTON1_TRIPLE_CLICKED => MouseButton::Left,
        curses::BUTTON2_RELEASED
        | curses::BUTTON2_PRESSED
        | curses::BUTTON2_CLICKED
        | curses::BUTTON2_DOUBLE_CLICKED
        | curses::BUTTON2_TRIPLE_CLICKED => MouseButton::Middle,
        curses::BUTTON3_RELEASED
        | curses::BUTTON3_PRESSED
        | curses::BUTTON3_CLICKED
        | curses::BUTTON3_DOUBLE_CLICKED
        | curses::BUTTON3_TRIPLE_CLICKED => MouseButton::Right,
        curses::BUTTON4_RELEASED
        | curses::BUTTON4_PRESSED
        | curses::BUTTON4_CLICKED
        | curses::BUTTON4_DOUBLE_CLICKED
        | curses::BUTTON4_TRIPLE_CLICKED => MouseButton::Button4,
        curses::BUTTON5_RELEASED
        | curses::BUTTON5_PRESSED
        | curses::BUTTON5_CLICKED
        | curses::BUTTON5_DOUBLE_CLICKED
        | curses::BUTTON5_TRIPLE_CLICKED => MouseButton::Button5,
        _ => MouseButton::Other,
    }
}
