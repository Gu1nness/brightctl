use crate::device::Device;

pub fn percent_to_val(val: u64, device: &Device) -> u64 {
    ((val * device.get_max_brightness()) + 50) / 100
}
pub fn val_to_percent(val: u64, device: &Device) -> u64 {
    (val * 100 + device.get_max_brightness() / 2) / device.get_max_brightness()
}

