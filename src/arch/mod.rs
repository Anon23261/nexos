pub mod x86_64;

pub use self::x86_64::*;

pub fn init() {
    x86_64::init();
}
