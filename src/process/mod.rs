use alloc::vec::Vec;
use crate::println;

pub mod scheduler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

#[derive(Debug)]
pub struct Process {
    pub id: usize,
    pub state: ProcessState,
    pub stack: Vec<u8>,
    pub stack_pointer: usize,
}

impl Process {
    pub fn new(id: usize) -> Self {
        const STACK_SIZE: usize = 4096;
        let mut stack = Vec::with_capacity(STACK_SIZE);
        stack.resize(STACK_SIZE, 0);

        Process {
            id,
            state: ProcessState::Ready,
            stack,
            stack_pointer: 0,
        }
    }

    pub fn set_state(&mut self, state: ProcessState) {
        self.state = state;
    }
}

pub fn init() {
    scheduler::init();
    println!("Process management initialized");
}
