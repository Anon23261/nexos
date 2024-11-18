pub mod x86_64;

pub use self::x86_64::*;

/// Initialize architecture-specific features
pub fn init() {
    x86_64::init();
}
