use crate::enums::{ValueType, DeltaType, Sign};
use regex::Regex;
use std::process::exit;
use std::str::FromStr;


#[derive(Debug, PartialEq, Eq)]
pub struct Value {
    pub val: u64,
    pub v_type: ValueType,
    pub d_type: DeltaType,
    pub sign: Sign,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{},{:?},{:?},{:?}\n",
            self.val, self.v_type, self.d_type, self.sign
        )
    }
}

pub fn parse_value(data: &String) -> Value {
    let mut new_val = Value {
        val: 0,
        v_type: ValueType::ABSOLUTE,
        d_type: DeltaType::DIRECT,
        sign: Sign::PLUS,
    };
    let re = Regex::new(r"^(?P<value>\d+)(?P<percent>\%?)(?P<sign>[\+\-]?)").unwrap();
    let cap_opt = re.captures(data.as_str());
    let cap = match &cap_opt {
        None => {
            println!("Wrong format, expecting {}", re);
            exit(1)
        }
        Some(data) => data,
    };
    if &cap["value"] == "" {
        panic!("Invalid value")
    } else {
        new_val.val = u64::from_str(&cap["value"]).unwrap();
    }
    if &cap["percent"] != "" {
        new_val.v_type = ValueType::RELATIVE
    }
    match &cap["sign"] {
        "+" => {
            new_val.sign = Sign::PLUS;
            new_val.d_type = DeltaType::DELTA
        }
        "-" => {
            new_val.sign = Sign::MINUS;
            new_val.d_type = DeltaType::DELTA
        }
        _ => (),
    }
    return new_val;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value_50_percent_plus() {
        let val_1 = Value {
            val: 50,
            v_type: ValueType::RELATIVE,
            d_type: DeltaType::DELTA,
            sign: Sign::PLUS,
        };
        assert_eq!(parse_value(&String::from_str("50%+").unwrap()), val_1);
    }

    #[test]
    fn test_parse_value_50_percent_minus() {
        let val_1 = Value {
            val: 50,
            v_type: ValueType::RELATIVE,
            d_type: DeltaType::DELTA,
            sign: Sign::MINUS,
        };
        assert_eq!(parse_value(&String::from_str("50%-").unwrap()), val_1);
    }

    #[test]
    fn test_parse_value_50_minus() {
        let val_1 = Value {
            val: 50,
            v_type: ValueType::ABSOLUTE,
            d_type: DeltaType::DELTA,
            sign: Sign::MINUS,
        };
        assert_eq!(parse_value(&String::from_str("50-").unwrap()), val_1);
    }

    #[test]
    fn test_parse_value_50_percent() {
        let val_1 = Value {
            val: 50,
            v_type: ValueType::RELATIVE,
            d_type: DeltaType::DIRECT,
            sign: Sign::PLUS,
        };
        assert_eq!(parse_value(&String::from_str("50%").unwrap()), val_1);
    }

    #[test]
    fn test_parse_value_50() {
        let val_1 = Value {
            val: 50,
            v_type: ValueType::ABSOLUTE,
            d_type: DeltaType::DIRECT,
            sign: Sign::PLUS,
        };
        assert_eq!(parse_value(&String::from_str("50").unwrap()), val_1);
    }
}

