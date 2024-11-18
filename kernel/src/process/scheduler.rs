use alloc::collections::VecDeque;
use alloc::vec::Vec;
use x86_64::instructions::interrupts;
use super::{Process, ProcessState};

pub struct Scheduler {
    processes: Vec<Process>,
    ready_queue: VecDeque<usize>,
    current_process: Option<usize>,
    time_slice: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            processes: Vec::new(),
            ready_queue: VecDeque::new(),
            current_process: None,
            time_slice: 100, // Default time slice in milliseconds
        }
    }

    pub fn init(&mut self) {
        // Create initial kernel process
        let kernel_process = Process::new(0, None);
        self.processes.push(kernel_process);
        self.current_process = Some(0);
    }

    pub fn add_process(&mut self, process: Process) {
        let pid = process.id;
        self.processes.push(process);
        self.ready_queue.push_back(pid);
    }

    pub fn schedule(&mut self) {
        interrupts::disable();

        if let Some(current_pid) = self.current_process {
            // Move current process to ready state if it's still running
            if let Some(current_process) = self.processes.get_mut(current_pid) {
                if current_process.state == ProcessState::Running {
                    current_process.state = ProcessState::Ready;
                    self.ready_queue.push_back(current_pid);
                }
            }
        }

        // Get next process from ready queue
        if let Some(next_pid) = self.ready_queue.pop_front() {
            if let Some(next_process) = self.processes.get_mut(next_pid) {
                next_process.state = ProcessState::Running;
                self.current_process = Some(next_pid);
                
                // Switch to the new process's address space
                unsafe {
                    switch_address_space(next_process.page_table);
                    switch_stack(next_process.stack_pointer);
                }
            }
        }

        interrupts::enable();
    }

    pub fn exit_process(&mut self, pid: usize) {
        if let Some(process) = self.processes.get_mut(pid) {
            process.state = ProcessState::Terminated;
            
            // Remove from ready queue if present
            self.ready_queue.retain(|&p| p != pid);
            
            // Wake up parent if waiting
            if let Some(parent_pid) = process.parent_id {
                if let Some(parent) = self.processes.get_mut(parent_pid) {
                    if parent.state == ProcessState::Blocked {
                        parent.state = ProcessState::Ready;
                        self.ready_queue.push_back(parent_pid);
                    }
                }
            }
            
            // Schedule next process
            self.schedule();
        }
    }

    pub fn get_current_process(&self) -> Option<&Process> {
        self.current_process.and_then(|pid| self.processes.get(pid))
    }
}

unsafe fn switch_address_space(page_table: PhysFrame) {
    // TODO: Implement address space switching
    unimplemented!()
}

unsafe fn switch_stack(stack_pointer: VirtAddr) {
    // TODO: Implement stack switching
    unimplemented!()
}
