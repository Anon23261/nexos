use alloc::vec::Vec;
use spin::Mutex;

mod scheduler;
mod task;

pub use task::{Process, ProcessState};
use scheduler::RoundRobinScheduler;

lazy_static::lazy_static! {
    static ref SCHEDULER: Mutex<RoundRobinScheduler> = Mutex::new(RoundRobinScheduler::new());
}

/// Initialize process management subsystem
pub fn init() {
    // Initialize the scheduler
    SCHEDULER.lock().init();
    println!("Process management initialized");
}

/// Create a new process
pub fn create_process(entry_point: usize, priority: u8) -> Result<Process, &'static str> {
    let process = Process::new(entry_point, priority);
    SCHEDULER.lock().add_process(process.clone());
    Ok(process)
}

/// Schedule the next process to run
pub fn schedule() -> Option<Process> {
    SCHEDULER.lock().next_process()
}

/// Block the current process
pub fn block_current(reason: ProcessState) {
    SCHEDULER.lock().block_current(reason);
}

/// Wake up a blocked process
pub fn wake_up(pid: usize) {
    SCHEDULER.lock().wake_up(pid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_process_creation() {
        let process = create_process(0x1000, 1).unwrap();
        assert_eq!(process.state(), ProcessState::Ready);
    }
}
