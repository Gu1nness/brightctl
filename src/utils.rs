use crate::device::Device;

pub fn percent_to_val(val: i64, device: &Device) -> i64 {
    (val * device.get_max_brightness() + 50) / 100
}
pub fn val_to_percent(val: i64, device: &Device) -> i64 {
    (val * 100 + device.get_max_brightness() / 2) / device.get_max_brightness()
}
