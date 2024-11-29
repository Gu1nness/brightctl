use std::{error::Error, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum ValueType {
    ABSOLUTE,
    RELATIVE,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeltaType {
    DIRECT,
    DELTA,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Sign {
    PLUS,
    MINUS,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValueUpdate {
    Delta(i64),
    Direct(i64),
    Relative(i64),
    Absolute(i64),
}

impl FromStr for ValueUpdate {
    type Err = Box<dyn Error>;

    //fn from_str(data: &str) -> Result<Self, Self::Err> {
    //    let re = Regex::new(r"^(?<value>\d+)(?<percent>\%)?(?<sign>[\+\-])?").unwrap();
    //    let cap_opt = re.captures(data);
    //    let cap = match &cap_opt {
    //        None => {
    //            return Err(format!("Wrong format, expecting {}", re).into());
    //        }
    //        Some(data) => data,
    //    };
    //    let value = cap["value"].parse()?;
    //    let sign = cap.name("sign").map(|x| x.as_str());
    //    match (cap.name("percent"), sign) {
    //        (None, None) => Ok(Self::Direct(value)),
    //        (None, Some("+")) => Ok(Self::Delta(value)),
    //        (None, Some("-")) => Ok(Self::Delta(-value)),
    //        (Some(_), None) => Ok(Self::Absolute(value)),
    //        (Some(_), Some("+")) => Ok(Self::Relative(value)),
    //        (Some(_), Some("-")) => Ok(Self::Relative(-value)),
    //        (_, _) => panic!("This should be unreachable."),
    //    }
    //}
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let chars = data.chars();
        let mut value: i64 = 0;
        let mut has_percent = false;
        let mut has_sign: bool = false;
        for char in chars {
            if has_sign {
                return Err("Invalid format".into());
            }
            match (has_percent, char) {
                (false, x) if x.is_ascii_digit() => {
                    value = 10 * value + (char.to_digit(10).unwrap() as i64)
                }
                (false, '%') => has_percent = true,
                (_, '+') => has_sign = true,
                (_, '-') => {
                    has_sign = true;
                    value = -value
                }
                (_, _) => return Err("Invalid format".into()),
            }
        }

        match (has_percent, has_sign) {
            (false, false) => Ok(Self::Direct(value)),
            (false, true) => Ok(Self::Delta(value)),
            (true, false) => Ok(Self::Absolute(value)),
            (true, true) => Ok(Self::Relative(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_50_percent_plus() {
        let val_1 = ValueUpdate::Relative(50);
        assert_eq!(ValueUpdate::from_str("50%+").unwrap(), val_1);
    }

    #[test]
    fn test_parse_50_percent_minus() {
        let val_1 = ValueUpdate::Relative(-50);
        assert_eq!(ValueUpdate::from_str("50%-").unwrap(), val_1);
    }

    #[test]
    fn test_parse_50_minus() {
        let val_1 = ValueUpdate::Delta(-50);
        assert_eq!(ValueUpdate::from_str("50-").unwrap(), val_1);
    }

    #[test]
    fn test_parse_50_percent() {
        let val_1 = ValueUpdate::Absolute(50);
        assert_eq!(ValueUpdate::from_str("50%").unwrap(), val_1);
    }

    #[test]
    fn test_parse_50() {
        let val_1 = ValueUpdate::Direct(50);
        assert_eq!(ValueUpdate::from_str("50").unwrap(), val_1);
    }

    #[test]
    fn test_invalid_format() {
        assert!(ValueUpdate::from_str("-50").is_err());
        assert!(ValueUpdate::from_str("5-0").is_err());
        assert!(ValueUpdate::from_str("5a0").is_err());
        assert!(ValueUpdate::from_str("%50").is_err());
    }
}
