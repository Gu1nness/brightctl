use crate::consts;
use crate::enums::ValueUpdate;
use crate::utils::{percent_to_val, val_to_percent};
use glob::glob;
use std::borrow::Cow;
use std::cmp::max;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Device<'a> {
    class: Cow<'a, str>,
    id: Cow<'a, str>,
    curr_brightness: i64,
    max_brightness: i64,
}

impl Device<'_> {
    pub fn get_max_brightness(&self) -> i64 {
        self.max_brightness
    }
    pub fn get_curr_brightness(&self) -> i64 {
        self.curr_brightness
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_class(&self) -> &str {
        &self.class
    }

    fn compute_for_val(&self, value: i64, update: &ValueUpdate) -> i64 {
        match update {
            ValueUpdate::Delta(x) => value + x,
            ValueUpdate::Direct(x) => *x,
            ValueUpdate::Relative(x) => percent_to_val(val_to_percent(value, self) + *x, self),
            ValueUpdate::Absolute(x) => percent_to_val(*x, self),
        }
    }

    pub fn compute_min_value(&self, min_value: &ValueUpdate) -> i64 {
        self.compute_for_val(self.max_brightness, min_value)
    }

    pub fn compute_from_update(&self, update_value: &ValueUpdate) -> i64 {
        let new_val = self.compute_for_val(self.curr_brightness, update_value);
        std::cmp::max(0, std::cmp::min(new_val, self.get_max_brightness()))
    }

    pub fn restore(&mut self) {
        let path = format!(
            "{}/{}/{}/{}",
            consts::RUNTIME_DIR,
            self.get_class(),
            self.get_id(),
            "brightness"
        );
        if let Ok(x) = fs::read_to_string(path)
            .unwrap_or(String::from(""))
            .trim_end_matches('\n')
            .parse::<i64>()
        {
            self.curr_brightness = x
        }
        write_device(self, consts::NONE_UPDATE, 1, false);
    }

    pub fn store(&self) {
        let prefix = format!(
            "{}/{}/{}",
            consts::RUNTIME_DIR,
            self.get_class(),
            self.get_id(),
        );
        let path = format!("{}/{}", &prefix, "brightness");
        fs::create_dir_all(prefix).unwrap();
        match fs::write(&path, format!("{}", self.curr_brightness)) {
            Ok(_) => (),
            Err(err) => {
                println!("Failed to write device {}: {}", &path, err);
            }
        }
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
        let val_1 = ValueUpdate::Relative(50);
        assert_eq!(DEVICE.compute_from_update(&val_1), 200);
    }

    #[test]
    fn test_compute_new_val_no_overflow() {
        let val_1 = ValueUpdate::Relative(70);

        assert_eq!(DEVICE.compute_from_update(&val_1), 200);
    }

    #[test]
    fn test_compute_new_val_no_negative() {
        let val_1 = ValueUpdate::Relative(-70);

        assert_eq!(DEVICE.compute_from_update(&val_1), 0);
    }

    #[test]
    fn test_compute_new_val_50_percent_minus() {
        let val_1 = ValueUpdate::Relative(-50);

        assert_eq!(DEVICE.compute_from_update(&val_1), 0);
    }

    #[test]
    fn test_compute_new_val_50_minus() {
        let val_1 = ValueUpdate::Delta(-50);

        assert_eq!(DEVICE.compute_from_update(&val_1), 50);
    }

    #[test]
    fn test_compute_new_val_50_percent() {
        let val_1 = ValueUpdate::Absolute(50);

        assert_eq!(DEVICE.compute_from_update(&val_1), 100);
    }

    #[test]
    fn test_compute_new_val_50() {
        let val_1 = ValueUpdate::Direct(50);

        assert_eq!(DEVICE.compute_from_update(&val_1), 50);
    }
}

impl std::fmt::Display for Device<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.sign_plus() {
            writeln!(
                f,
                "{},{},{},{}%,{}",
                self.id,
                self.class,
                self.curr_brightness,
                val_to_percent(self.curr_brightness, self),
                self.max_brightness
            )
        } else {
            writeln!(
                f,
                "Device '{}' of class '{}':\n\tCurrent brightness {} ({}%)\n\tMax brightness: {}\n",
                self.id,
                self.class,
                self.curr_brightness,
                val_to_percent(self.curr_brightness, self),
                self.max_brightness
            )
        }
    }
}

pub fn read_device(path: PathBuf, class: &str, id: String) -> Device {
    let read_brightness =
        fs::read_to_string(format!("{}/{}", path.display(), "brightness")).unwrap();
    let read_max_brightness =
        fs::read_to_string(format!("{}/{}", path.display(), "max_brightness")).unwrap();
    let curr_brightness = match read_brightness.trim_end_matches('\n').parse() {
        Ok(value) => value,
        Err(err) => {
            println!("{:?}, read {:?}", err, read_brightness);
            panic!()
        }
    };
    let max_brightness = match read_max_brightness.trim_end_matches('\n').parse() {
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

pub fn write_device<'a>(
    device: &Device<'a>,
    update_value: &ValueUpdate,
    min_value: i64,
    pretend: bool,
) -> Option<Device<'a>> {
    //let new_val = max(device.compute_new_val(value), min_value);
    let new_val_bis = max(device.compute_from_update(update_value), min_value);
    let prefix = format!(
        "{}/{}/{}",
        consts::PATH,
        device.get_class(),
        device.get_id()
    );
    let path = format!("{}/{}", prefix, "brightness");
    if pretend {
        println!(
            "Would set {} brightness to {}",
            device.get_class(),
            new_val_bis
        );
        Option::None
    } else {
        match fs::write(&path, format!("{}", new_val_bis)) {
            Ok(_) => Some(read_device(
                PathBuf::from(prefix),
                match &device.class {
                    Cow::Borrowed(class) => class,
                    Cow::Owned(_) => panic!("String is owned here, shouldn't happen"),
                },
                device.get_id().to_string(),
            )),
            Err(err) => {
                println!("Failed to write device {}: {}", path, err);
                Option::None
            }
        }
    }
}

pub fn read_class<'a>(class: &'a str, device: Option<&'a String>) -> Vec<Device<'a>> {
    let concat_path: &String = &format!(
        "{}/{}/{}",
        consts::PATH,
        class,
        device.unwrap_or(&String::from("*"))
    );
    let mut device_ret: Vec<Device> = Vec::new();
    for entry in glob(concat_path).unwrap() {
        match entry {
            Ok(path) => device_ret.push(read_device(
                path.clone(),
                class,
                path.file_name().unwrap().to_string_lossy().into_owned(),
            )),
            Err(error) => println!("{:?}", error),
        }
    }
    device_ret
}

pub fn read_devices<'a>(class: &'a Option<String>, device: &'a Option<String>) -> Vec<Device<'a>> {
    let mut devices: Vec<Device> = Vec::new();
    match class {
        None => {
            for class in consts::CLASSES {
                devices.extend(read_class(class, device.as_ref()))
            }
        }
        Some(class_) => devices.extend(read_class(class_, device.as_ref())),
    }

    devices
}
