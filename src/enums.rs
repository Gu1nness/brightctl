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
