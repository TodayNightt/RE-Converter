use std::fmt::Display;

pub trait OptionStringExt {
    fn unwrap_or_empty_string(self) -> String;
}

impl<T: Display> OptionStringExt for Option<T> {
    fn unwrap_or_empty_string(self) -> String {
        match self {
            Some(val) => val.to_string(),
            None => "".to_string(),
        }
    }
}

pub trait OptionValueExt<T> {
    fn unwrap_or_value<U>(self, default_val: U) -> U
    where
        T: Into<U>;
}

impl<T> OptionValueExt<T> for Option<T> {
    fn unwrap_or_value<U>(self, val: U) -> U
    where
        T: Into<U>,
    {
        match self {
            Some(val) => val.into(),
            None => val,
        }
    }
}
