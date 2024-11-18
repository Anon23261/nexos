use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::shell::Shell;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    
    static ref SHELL: Mutex<Shell> = Mutex::new(Shell::new());
}

pub fn init() {
    crate::shell::init();
}

pub fn handle_scancode(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();
    
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    SHELL.lock().handle_key(character);
                },
                DecodedKey::RawKey(_key) => {
                    // Handle special keys if needed
                }
            }
        }
    }
}
