use alloc::collections::VecDeque;
use spin::Mutex;
use lazy_static::lazy_static;
use crate::process::{Process, ProcessState};

pub struct Scheduler {
    ready_queue: VecDeque<Process>,
    current_process: Option<Process>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Scheduler {
            ready_queue: VecDeque::new(),
            current_process: None,
        }
    }

    pub fn init(&mut self) {
        // Create kernel process
        let kernel_process = Process::new(0);
        self.current_process = Some(kernel_process);
    }

    pub fn schedule(&mut self) -> Option<&Process> {
        if let Some(mut current) = self.current_process.take() {
            if current.state == ProcessState::Running {
                current.state = ProcessState::Ready;
                self.ready_queue.push_back(current);
            }
        }

        self.current_process = self.ready_queue.pop_front();
        if let Some(ref mut process) = self.current_process {
            process.state = ProcessState::Running;
        }

        self.current_process.as_ref()
    }

    pub fn add_process(&mut self, mut process: Process) {
        process.state = ProcessState::Ready;
        self.ready_queue.push_back(process);
    }
}

lazy_static! {
    pub static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
}

pub fn init() {
    SCHEDULER.lock().init();
}
