use alloc::string::String;
use alloc::vec::Vec;
use crate::{print, println};
use crate::drivers::vga::{self, Color};

const MAX_CMD_LENGTH: usize = 1024;

pub struct Shell {
    input_buffer: String,
    command_history: Vec<String>,
    history_index: usize,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            input_buffer: String::with_capacity(MAX_CMD_LENGTH),
            command_history: Vec::new(),
            history_index: 0,
        }
    }

    pub fn handle_key(&mut self, c: char) {
        match c {
            '\n' => self.execute_command(),
            '\x08' => self.backspace(), // Backspace
            c if c.is_ascii() && !c.is_control() => self.add_char(c),
            _ => {}
        }
    }

    fn add_char(&mut self, c: char) {
        if self.input_buffer.len() < MAX_CMD_LENGTH {
            self.input_buffer.push(c);
            print!("{}", c);
        }
    }

    fn backspace(&mut self) {
        if !self.input_buffer.is_empty() {
            self.input_buffer.pop();
            print!("\x08 \x08"); // Move back, print space, move back again
        }
    }

    fn execute_command(&mut self) {
        println!();
        if !self.input_buffer.is_empty() {
            self.command_history.push(self.input_buffer.clone());
            self.history_index = self.command_history.len();

            match self.input_buffer.as_str() {
                "help" => self.cmd_help(),
                "clear" => self.cmd_clear(),
                "version" => self.cmd_version(),
                "memory" => self.cmd_memory(),
                "uptime" => self.cmd_uptime(),
                _ => println!("Unknown command: {}", self.input_buffer),
            }
        }
        self.input_buffer.clear();
        print!("\nNexOS> ");
    }

    fn cmd_help(&self) {
        let mut writer = vga::WRITER.lock();
        writer.set_color(Color::Yellow, Color::Black);
        println!("Available commands:");
        writer.set_color(Color::White, Color::Black);
        println!("  help     - Show this help message");
        println!("  clear    - Clear the screen");
        println!("  version  - Show kernel version");
        println!("  memory   - Show memory information");
        println!("  uptime   - Show system uptime");
    }

    fn cmd_clear(&self) {
        vga::WRITER.lock().clear_screen();
    }

    fn cmd_version(&self) {
        let mut writer = vga::WRITER.lock();
        writer.set_color(Color::LightGreen, Color::Black);
        println!("NexOS v0.1.0");
        println!("Developed by Codeium User");
        writer.set_color(Color::White, Color::Black);
    }

    fn cmd_memory(&self) {
        let mut writer = vga::WRITER.lock();
        writer.set_color(Color::LightCyan, Color::Black);
        println!("Memory Information:");
        writer.set_color(Color::White, Color::Black);
        println!("  Total Physical: Unknown");
        println!("  Used: Unknown");
        println!("  Free: Unknown");
        // TODO: Implement actual memory stats
    }

    fn cmd_uptime(&self) {
        println!("System Uptime: Unknown");
        // TODO: Implement actual uptime tracking
    }
}

pub fn init() {
    let mut writer = vga::WRITER.lock();
    writer.clear_screen();
    writer.set_color(Color::LightGreen, Color::Black);
    println!("================================");
    println!("         NexOS v0.1.0           ");
    println!("================================");
    writer.set_color(Color::White, Color::Black);
    println!("\nType 'help' for available commands");
    print!("\nNexOS> ");
}
