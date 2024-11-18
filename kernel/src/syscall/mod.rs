use x86_64::structures::idt::InterruptStackFrame;

/// System call numbers
#[repr(usize)]
pub enum SyscallNumber {
    Write = 1,
    Read = 2,
    Open = 3,
    Close = 4,
    Exit = 5,
    Fork = 6,
    Exec = 7,
    CreateProcess = 8,
}

/// System call handler
pub extern "x86-interrupt" fn syscall_handler(stack_frame: InterruptStackFrame) {
    let syscall_number: SyscallNumber;
    let arg1: usize;
    let arg2: usize;
    let arg3: usize;

    // Get syscall number and arguments from registers
    unsafe {
        asm!(
            "mov {0}, rax",
            "mov {1}, rdi",
            "mov {2}, rsi",
            "mov {3}, rdx",
            out(reg) syscall_number,
            out(reg) arg1,
            out(reg) arg2,
            out(reg) arg3,
        );
    }

    // Handle system call
    match syscall_number {
        SyscallNumber::Write => handle_write(arg1, arg2, arg3),
        SyscallNumber::Read => handle_read(arg1, arg2, arg3),
        SyscallNumber::Open => handle_open(arg1, arg2),
        SyscallNumber::Close => handle_close(arg1),
        SyscallNumber::Exit => handle_exit(arg1),
        SyscallNumber::Fork => handle_fork(),
        SyscallNumber::Exec => handle_exec(arg1),
        SyscallNumber::CreateProcess => handle_create_process(arg1, arg2),
        _ => println!("Unknown syscall: {:?}", syscall_number),
    }
}

fn handle_write(fd: usize, buffer: usize, size: usize) -> isize {
    // TODO: Implement write syscall
    0
}

fn handle_read(fd: usize, buffer: usize, size: usize) -> isize {
    // TODO: Implement read syscall
    0
}

fn handle_open(path: usize, flags: usize) -> isize {
    // TODO: Implement open syscall
    0
}

fn handle_close(fd: usize) -> isize {
    // TODO: Implement close syscall
    0
}

fn handle_exit(status: usize) -> ! {
    // TODO: Implement exit syscall
    loop {}
}

fn handle_fork() -> isize {
    // TODO: Implement fork syscall
    0
}

fn handle_exec(path: usize) -> isize {
    // TODO: Implement exec syscall
    0
}

fn handle_create_process(path: usize, args: usize) -> isize {
    // TODO: Implement process creation
    0
}
