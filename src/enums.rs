use std::{error::Error, str::FromStr};

use regex::Regex;

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

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(?<value>\d+)(?<percent>\%?)(?<sign>[\+\-]?)").unwrap();
        let cap_opt = re.captures(data);
        let cap = match &cap_opt {
            None => {
                return Err(format!("Wrong format, expecting {}", re).into());
            }
            Some(data) => data,
        };
        let value = cap["value"].parse()?;
        let sign = cap.name("sign").map(|x| x.as_str());
        match (cap["percent"].is_empty(), sign) {
            (true, Some("")) => Ok(Self::Direct(value)),
            (true, Some("+")) => Ok(Self::Delta(value)),
            (true, Some("-")) => Ok(Self::Delta(-value)),
            (false, Some("")) => Ok(Self::Absolute(value)),
            (false, Some("+")) => Ok(Self::Relative(value)),
            (false, Some("-")) => Ok(Self::Relative(-value)),
            (_, _) => panic!("This should be unreachable."),
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
}
