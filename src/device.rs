use crate::consts;
use crate::enums::{DeltaType, Sign, ValueType};
use crate::structs::Value;
use crate::utils::{percent_to_val, val_to_percent};
use glob::glob;
use std::borrow::Cow;
use std::fs;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Clone)]
pub struct Device {
    class: Cow<'static, str>,
    id: Cow<'static, str>,
    curr_brightness: u64,
    max_brightness: u64,
}

impl Device {
    pub fn get_max_brightness(&self) -> u64 {
        self.max_brightness
    }
    pub fn get_curr_brightness(&self) -> u64 {
        self.curr_brightness
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_class(&self) -> &str {
        &self.class
    }

    pub fn compute_new_val(&self, value: &Value) -> u64 {
        let new_val = match value.d_type {
            DeltaType::DIRECT => match value.v_type {
                ValueType::ABSOLUTE => value.val,
                ValueType::RELATIVE => percent_to_val(value.val, self),
            },
            DeltaType::DELTA => match value.v_type {
                ValueType::ABSOLUTE => match value.sign {
                    Sign::MINUS => match self.curr_brightness.checked_sub(value.val) {
                        None => 0,
                        Some(v) => v,
                    },
                    Sign::PLUS => value.val + self.curr_brightness,
                },
                ValueType::RELATIVE => {
                    let new_perc = match value.sign {
                        Sign::MINUS => {
                            match val_to_percent(self.curr_brightness, self).checked_sub(value.val)
                            {
                                None => 0,
                                Some(v) => v,
                            }
                        }
                        Sign::PLUS => val_to_percent(self.curr_brightness, self) + value.val,
                    };
                    percent_to_val(new_perc, self)
                }
            },
        };
        std::cmp::min(new_val, self.get_max_brightness())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static DEVICE: Device = Device {
        id: Cow::Borrowed("blah"),
        class: Cow::Borrowed("blah"),
        curr_brightness: 100,
        max_brightness: 200,
    };
    #[test]
    fn test_compute_new_val_50_percent_plus() {
        let val_1 = Value {
            val: 50,
            v_type: ValueType::RELATIVE,
            d_type: DeltaType::DELTA,
            sign: Sign::PLUS,
        };
        assert_eq!(DEVICE.compute_new_val(&val_1), 200);
    }

    #[test]
    fn test_compute_new_val_no_overflow() {
        let val_1 = Value {
            val: 70,
            v_type: ValueType::RELATIVE,
            d_type: DeltaType::DELTA,
            sign: Sign::PLUS,
        };

        assert_eq!(DEVICE.compute_new_val(&val_1), 200);
    }

    #[test]
    fn test_compute_new_val_no_negative() {
        let val_1 = Value {
            val: 70,
            v_type: ValueType::RELATIVE,
            d_type: DeltaType::DELTA,
            sign: Sign::MINUS,
        };

        assert_eq!(DEVICE.compute_new_val(&val_1), 0);
    }

    #[test]
    fn test_compute_new_val_50_percent_minus() {
        let val_1 = Value {
            val: 50,
            v_type: ValueType::RELATIVE,
            d_type: DeltaType::DELTA,
            sign: Sign::MINUS,
        };

        assert_eq!(DEVICE.compute_new_val(&val_1), 0);
    }

    #[test]
    fn test_compute_new_val_50_minus() {
        let val_1 = Value {
            val: 50,
            v_type: ValueType::ABSOLUTE,
            d_type: DeltaType::DELTA,
            sign: Sign::MINUS,
        };

        assert_eq!(DEVICE.compute_new_val(&val_1), 50);
    }

    #[test]
    fn test_compute_new_val_50_percent() {
        let val_1 = Value {
            val: 50,
            v_type: ValueType::RELATIVE,
            d_type: DeltaType::DIRECT,
            sign: Sign::PLUS,
        };

        assert_eq!(DEVICE.compute_new_val(&val_1), 100);
    }

    #[test]
    fn test_compute_new_val_50() {
        let val_1 = Value {
            val: 50,
            v_type: ValueType::ABSOLUTE,
            d_type: DeltaType::DIRECT,
            sign: Sign::PLUS,
        };

        assert_eq!(DEVICE.compute_new_val(&val_1), 50);
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.sign_plus() {
            write!(
                f,
                "{},{},{},{}%,{}\n",
                self.id,
                self.class,
                self.curr_brightness,
                val_to_percent(self.curr_brightness, self),
                self.max_brightness
            )
        } else {
            write!(
            f,
            "Device '{}' of class '{}':\n\tCurrent brightness {} ({}%)\n\tMax brightness: {}\n\n",
            self.id,
            self.class,
            self.curr_brightness,
            val_to_percent(self.curr_brightness, self),
            self.max_brightness
            )
        }
    }
}

pub fn read_device(path: PathBuf, class: &'static str, id: String) -> Device {
    let read_brightness =
        fs::read_to_string(format!("{}/{}", path.display(), "brightness")).unwrap();
    let read_max_brightness =
        fs::read_to_string(format!("{}/{}", path.display(), "max_brightness")).unwrap();
    let curr_brightness = match u64::from_str(read_brightness.trim_end_matches('\n')) {
        Ok(value) => value,
        Err(err) => {
            println!("{:?}, read {:?}", err, read_brightness);
            panic!()
        }
    };
    let max_brightness = match u64::from_str(&read_max_brightness.trim_end_matches('\n')) {
        Ok(value) => value,
        Err(err) => {
            println!("{:?}, read {:?}", err, read_max_brightness);
            panic!()
        }
    };
    Device {
        class: Cow::Borrowed(class),
        id: Cow::Owned(id),
        curr_brightness,
        max_brightness,
    }
}

pub fn write_device(device: &Device, value: &Value) -> Option<Device> {
    let new_val = device.compute_new_val(value);
    let prefix = format!(
        "{}/{}/{}",
        consts::PATH,
        device.get_class(),
        device.get_id()
    );
    let path = format!("{}/{}", prefix, "brightness");
    match fs::write(path, format!("{}", new_val)) {
        Ok(_) => Some(read_device(
            PathBuf::from(prefix),
            match &device.class {
                Cow::Borrowed(class) => class,
                Cow::Owned(_) => panic!("String is owned here, shouldn't happen"),
            },
            device.get_id().to_string()
        )),
        Err(err) => {
            println!("Failed to write device: {}", err);
            Option::None
        }
    }
}

pub fn read_class(class: &'static str) -> Vec<Device> {
    let concat_path: &String = &format!("{}/{}/*", consts::PATH, class);
    let mut device_ret: Vec<Device> = Vec::new();
    for entry in glob(concat_path).unwrap() {
        match entry {
            Ok(path) => {
                device_ret.push(
                    read_device(
                        path.clone(),
                        class,
                        path.file_name().unwrap().to_string_lossy().into_owned()
                        )
                    )
            },
            Err(error) => println!("{:?}", error),
        }
    }
    device_ret
}

pub fn read_devices() -> Vec<Device> {
    let mut devices: Vec<Device> = Vec::new();
    for class in consts::CLASSES {
        devices.extend(read_class(class))
    }
    devices
}
