
use std::sync::atomic::{AtomicBool, Ordering};

// Exact fraction flag
static FRACTION_MODE: AtomicBool = AtomicBool::new(false);
pub fn set_fraction_mode(value: bool) {
    FRACTION_MODE.store(value, Ordering::Relaxed);
}

pub fn get_fraction_mode() -> bool {
    FRACTION_MODE.load(Ordering::Relaxed)
}

// Simplified display flag
static SIMPLIFIED_MODE: AtomicBool = AtomicBool::new(false);
pub fn set_simple_mode(value: bool) {
    SIMPLIFIED_MODE.store(value, Ordering::Relaxed);
}

pub fn get_simple_mode() -> bool {
    SIMPLIFIED_MODE.load(Ordering::Relaxed)
}

// Constants
pub const BP_UNAIRE: u8 = 5;
