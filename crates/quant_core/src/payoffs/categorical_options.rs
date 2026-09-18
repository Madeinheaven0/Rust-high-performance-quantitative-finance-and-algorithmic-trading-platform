//! # The type of the option's strategy ( Call or Put)

#[derive(Clone, PartialEq, Debug)]
pub enum CallPutCategory {
    Call,
    Put,
}

#[derive(Clone, PartialEq, Debug)]
pub enum LongShortCategory {
    Long,
    Short,
}
