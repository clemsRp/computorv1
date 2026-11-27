
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, Default)]
pub struct FormatConfig {
    pub fraction: bool,
    pub simple: bool,
}

const FRACTION_BIT: u8 = 1 << 0;
const SIMPLE_BIT: u8   = 1 << 1;
static CONFIG: AtomicU8 = AtomicU8::new(0);

pub fn set_config(cfg: FormatConfig) {
    let mut bits = 0;
    if cfg.fraction { bits |= FRACTION_BIT; }
    if cfg.simple   { bits |= SIMPLE_BIT; }
    CONFIG.store(bits, Ordering::Relaxed);
}

pub fn get_config() -> FormatConfig {
    let bits = CONFIG.load(Ordering::Relaxed);
    FormatConfig {
        fraction: bits & FRACTION_BIT != 0,
        simple:   bits & SIMPLE_BIT != 0,
    }
}

// Constants
pub const BP_UNAIRE: u8 = 5;
