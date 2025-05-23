use crate::enums::ValueUpdate;

pub const PATH: &str = "/sys/class";
pub const CLASSES: &[&str] = &["backlight", "leds"];

pub const RUNTIME_DIR: &str = "~/.local/share/brightctl";

pub const NONE_UPDATE: &ValueUpdate = &ValueUpdate::Relative(0);
